"""Planting and gate running.

A run copies the target tree into scratch copies, one clean copy and one
planted copy per applicable seed. Gates run inside the copies only. The
target tree itself is opened read only, and the run proves that with a
hash walk taken before and after.
"""

import ast
import hashlib
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


def run_field(target, gates, timeout=120, work_root=None, keep_scratch=False):
    """Run every gate against the clean copy and every planted copy.

    gates is a list of (name, command) pairs. The result is a plain dict
    shaped like the JSON report.
    """
    target = Path(target)
    if not target.is_dir():
        raise ValueError(f"target is not a directory: {target}")
    before = hash_tree(target)
    if work_root is None:
        scratch = Path(tempfile.mkdtemp(prefix="canaryfield-"))
    else:
        scratch = Path(work_root)
        scratch.mkdir(parents=True, exist_ok=True)
    try:
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
        summary = {}
        for name, _command in gates:
            counts = {CAUGHT: 0, WAVED: 0, NOT_APPLICABLE: 0, GATE_BROKEN: 0}
            for row in rows:
                counts[row["gates"][name]] += 1
            summary[name] = counts
        return {
            "target": str(target),
            "gates": [name for name, _command in gates],
            "clean_exits": clean_exits,
            "target_unchanged": before == after,
            "rows": rows,
            "summary": summary,
        }
    finally:
        if not keep_scratch:
            shutil.rmtree(scratch, ignore_errors=True)
