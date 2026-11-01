"""Planting and gate running.

A run works on scratch material only. In copy mode it copies the target
tree into scratch copies, one clean copy and one planted copy per
applicable seed. In git branch mode it clones the target into a scratch
clone and commits each seed on its own throwaway branch there, so git
mode judges the committed HEAD of the target while copy mode judges its
working tree. Gates run inside the scratch material only. The target
tree itself is opened read only, and the run proves that with a hash
walk taken before and after. A supplied work root must start empty and
a run removes only what it created.
"""

import ast
import hashlib
import os
import shutil
import subprocess
import tempfile
from pathlib import Path

from .seeds import SEEDS

CAUGHT = "caught"
WAVED = "waved-through"
NOT_APPLICABLE = "not-applicable"
GATE_BROKEN = "gate-broken"

IGNORED_PARTS = {".git", "__pycache__", ".pytest_cache"}


def _is_test_file(path):
    name = path.name
    return name.startswith("test_") or name.endswith("_test.py") or "tests" in path.parts


def hash_tree(root):
    """Map each file under root to the SHA-256 of its bytes."""
    root = Path(root)
    result = {}
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(root)
        if any(part in IGNORED_PARTS for part in relative.parts):
            continue
        result[str(relative)] = hashlib.sha256(path.read_bytes()).hexdigest()
    return result


def _copy_tree(source, destination):
    def ignore(directory, names):
        return [name for name in names if name in IGNORED_PARTS]

    shutil.copytree(source, destination, ignore=ignore)


def plant_seed(source_root, seed):
    """Plant one seed in the copy rooted at source_root.

    Returns the repo relative path of the changed file, or None when the
    seed does not apply anywhere in the copy.
    """
    root = Path(source_root)
    candidates = sorted(
        path
        for path in root.rglob("*.py")
        if path.is_file()
        and not any(part in IGNORED_PARTS for part in path.relative_to(root).parts)
        and not _is_test_file(path.relative_to(root))
    )
    for path in candidates:
        tree = ast.parse(path.read_text(encoding="utf-8"))
        if seed.apply(tree):
            ast.fix_missing_locations(tree)
            path.write_text(ast.unparse(tree) + "\n", encoding="utf-8")
            return str(path.relative_to(root))
    return None


def _run_gate(command, cwd, timeout):
    try:
        completed = subprocess.run(
            command,
            shell=True,
            cwd=cwd,
            timeout=timeout,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        return completed.returncode
    except subprocess.TimeoutExpired:
        return -1


def run_field(
    target, gates, timeout=120, work_root=None, keep_scratch=False, plant_mode="copy"
):
    """Run every gate against the clean copy and every planted copy.

    gates is a list of (name, command) pairs. The result is a plain dict
    shaped like the JSON report. plant_mode is copy for the slice 1
    scratch copies or git-branches for a scratch clone where each seed is
    committed on its own throwaway branch.
    """
    target = Path(target)
    if not target.is_dir():
        raise ValueError(f"target is not a directory: {target}")
    if plant_mode not in ("copy", "git-branches"):
        raise ValueError(f"unknown plant mode: {plant_mode}")
    before = hash_tree(target)
    if work_root is None:
        scratch = Path(tempfile.mkdtemp(prefix="canaryfield-"))
        created_root = True
        preexisting = set()
    else:
        scratch = Path(work_root)
        created_root = not scratch.exists()
        scratch.mkdir(parents=True, exist_ok=True)
        preexisting = set(scratch.iterdir())
        if preexisting:
            raise ValueError(
                f"work root is not empty, refusing to use it: {scratch}"
            )
    try:
        if plant_mode == "git-branches":
            return _run_git_mode(target, gates, timeout, scratch, before)
        clean = scratch / "clean"
        _copy_tree(target, clean)
        clean_exits = {
            name: _run_gate(command, clean, timeout) for name, command in gates
        }
        rows = []
        for seed in SEEDS:
            planted = scratch / seed.id
            _copy_tree(target, planted)
            changed_file = plant_seed(planted, seed)
            cells = {}
            planted_exits = {}
            for name, command in gates:
                if changed_file is None:
                    cells[name] = NOT_APPLICABLE
                    continue
                if clean_exits[name] != 0:
                    cells[name] = GATE_BROKEN
                    continue
                exit_code = _run_gate(command, planted, timeout)
                planted_exits[name] = exit_code
                cells[name] = CAUGHT if exit_code != 0 else WAVED
            rows.append(
                {
                    "seed": seed.id,
                    "name": seed.name,
                    "defect_class": seed.defect_class,
                    "planted_file": changed_file,
                    "planted_exits": planted_exits,
                    "gates": cells,
                }
            )
        after = hash_tree(target)
        return {
            "target": str(target),
            "gates": [name for name, _command in gates],
            "clean_exits": clean_exits,
            "target_unchanged": before == after,
            "rows": rows,
            "summary": _summary(rows, gates),
        }
    finally:
        if not keep_scratch:
            if created_root:
                shutil.rmtree(scratch, ignore_errors=True)
            else:
                for entry in set(scratch.iterdir()) - preexisting:
                    if entry.is_dir():
                        shutil.rmtree(entry, ignore_errors=True)
                    else:
                        entry.unlink(missing_ok=True)


def _summary(rows, gates):
    summary = {}
    for name, _command in gates:
        counts = {CAUGHT: 0, WAVED: 0, NOT_APPLICABLE: 0, GATE_BROKEN: 0}
        for row in rows:
            counts[row["gates"][name]] += 1
        summary[name] = counts
    return summary


_GIT_IDENTITY = {
    "GIT_AUTHOR_NAME": "canaryfield",
    "GIT_AUTHOR_EMAIL": "canaryfield@example.invalid",
    "GIT_COMMITTER_NAME": "canaryfield",
    "GIT_COMMITTER_EMAIL": "canaryfield@example.invalid",
}


def _git(arguments, cwd):
    env = dict(os.environ)
    env.update(_GIT_IDENTITY)
    completed = subprocess.run(
        ["git", "-C", str(cwd)] + arguments,
        capture_output=True,
        text=True,
        env=env,
    )
    if completed.returncode != 0:
        raise ValueError(
            f"git {' '.join(arguments)} failed in {cwd}: {completed.stderr.strip()}"
        )
    return completed.stdout.strip()


def _clean_caches(root):
    for part in (".pytest_cache",):
        shutil.rmtree(root / part, ignore_errors=True)
    for path in sorted(root.rglob("__pycache__")):
        if path.is_dir() and ".git" not in path.parts:
            shutil.rmtree(path, ignore_errors=True)


def _restore_checkout(clone, base):
    """Return the scratch clone checkout to the base commit.

    Gates are expected to be read only judges, but a gate that writes
    tracked or untracked files inside the checkout must not poison the
    next seed, so the checkout is hard reset and cleaned between runs.
    """
    _git(["reset", "-q", "--hard", "HEAD"], clone)
    _git(["clean", "-q", "-fd"], clone)
    _git(["checkout", "-q", base], clone)
    _git(["reset", "-q", "--hard", base], clone)


def _run_git_mode(target, gates, timeout, scratch, before):
    """Plant each seed on its own branch inside a scratch clone.

    The clone is made from the target and the target is only ever read.
    Gates run in the clone, first at the base commit and then in a
    checkout of each seed branch.
    """
    probe = subprocess.run(
        ["git", "-C", str(target), "rev-parse", "--show-toplevel"],
        capture_output=True,
        text=True,
    )
    if probe.returncode != 0:
        raise ValueError(f"target is not a git repository: {target}")
    if Path(probe.stdout.strip()).resolve() != target.resolve():
        raise ValueError(
            f"target is not the root of a git repository: {target}"
        )
    head = subprocess.run(
        ["git", "-C", str(target), "rev-parse", "--verify", "HEAD"],
        capture_output=True,
        text=True,
    )
    if head.returncode != 0:
        raise ValueError(
            f"target git repository has no commits yet: {target}"
        )
    clone = scratch / "clone"
    cloned = subprocess.run(
        ["git", "clone", "--quiet", "--no-hardlinks", str(target), str(clone)],
        capture_output=True,
        text=True,
    )
    if cloned.returncode != 0:
        raise ValueError(f"could not clone the target: {cloned.stderr.strip()}")
    base = _git(["rev-parse", "HEAD"], clone)
    clean_exits = {name: _run_gate(command, clone, timeout) for name, command in gates}
    rows = []
    for seed in SEEDS:
        _restore_checkout(clone, base)
        _clean_caches(clone)
        changed_file = plant_seed(clone, seed)
        cells = {}
        planted_exits = {}
        branch = None
        if changed_file is None:
            cells = {name: NOT_APPLICABLE for name, _command in gates}
        else:
            branch = f"canaryfield/{seed.id}"
            _git(["checkout", "-q", "-b", branch], clone)
            _git(["add", "--", changed_file], clone)
            _git(
                ["commit", "-q", "-m", f"Plant {seed.id} {seed.name}"],
                clone,
            )
            for name, command in gates:
                if clean_exits[name] != 0:
                    cells[name] = GATE_BROKEN
                    continue
                exit_code = _run_gate(command, clone, timeout)
                planted_exits[name] = exit_code
                cells[name] = CAUGHT if exit_code != 0 else WAVED
        rows.append(
            {
                "seed": seed.id,
                "name": seed.name,
                "defect_class": seed.defect_class,
                "planted_file": changed_file,
                "branch": branch,
                "planted_exits": planted_exits,
                "gates": cells,
            }
        )
    _restore_checkout(clone, base)
    after = hash_tree(target)
    return {
        "target": str(target),
        "plant_mode": "git-branches",
        "gates": [name for name, _command in gates],
        "clean_exits": clean_exits,
        "target_unchanged": before == after,
        "rows": rows,
        "summary": _summary(rows, gates),
    }
