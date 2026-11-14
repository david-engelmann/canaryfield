"""Recorded conformance generator for canaryfield PORT5.

Runs the landed Python engine's run_field over synthetic
fixture projects and records its answers, results and
refusals alike. Usage: python3 tools/gen_port5_recordings.py
<repo_root> <out_json>. The repo root is a tree carrying the
landed Python package while canaryfield/engine.py still
carries the machinery. The crate binary must stand named in
CANARYFIELD_BINARY, because the Python's planting already
stands in the crate and the engine drives it through its
bridge. Recordings are never hand written and never taken
from the port.

Two spellings inside a recording are tokens, not paths of
this machine. The scratch root every fixture stands under
is written as <ROOT>, and the interpreter the pytest gate
runs under is written as <PYTHON> in the recorded gate
command. The conformance suite substitutes its own root
and its own interpreter when it replays a case, and the
answers do not depend on either spelling.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2])
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "tests"))

if sys.implementation.name != "cpython" or sys.version_info[:2] != (3, 12):
    raise SystemExit(
        "the recordings stand under CPython 3.12 only and this "
        f"interpreter is {sys.implementation.name} "
        f"{sys.version_info.major}.{sys.version_info.minor}"
    )

BINARY = os.environ.get("CANARYFIELD_BINARY")
if not BINARY or not Path(BINARY).is_file():
    raise SystemExit(
        "CANARYFIELD_BINARY must name the standing crate binary, "
        "because the Python engine plants through it"
    )

from fixture_project import (  # noqa: E402
    GATE,
    write_main_project,
    write_tiny_project,
    write_two_module_project,
)
from canaryfield.engine import run_field  # noqa: E402

SCRATCH = Path(tempfile.mkdtemp(prefix="canaryfield-port5-gen-"))
GIT_ENV = {
    "GIT_AUTHOR_NAME": "canaryfield",
    "GIT_AUTHOR_EMAIL": "canaryfield@example.invalid",
    "GIT_COMMITTER_NAME": "canaryfield",
    "GIT_COMMITTER_EMAIL": "canaryfield@example.invalid",
}


def git(arguments, cwd):
    env = dict(os.environ)
    env.update(GIT_ENV)
    completed = subprocess.run(
        ["git", "-C", str(cwd)] + arguments,
        capture_output=True,
        text=True,
        env=env,
    )
    if completed.returncode != 0:
        raise SystemExit(f"fixture git {arguments} failed: {completed.stderr}")


def build_fixture(kind, root):
    proj = root / "proj"
    if kind == "main":
        write_main_project(proj)
    elif kind == "main-git":
        write_main_project(proj)
        git(["init", "-q"], proj)
        git(["add", "-A"], proj)
        git(["commit", "-q", "-m", "fixture"], proj)
    elif kind in ("tiny", "nonempty-work"):
        write_tiny_project(proj)
    elif kind == "two-module":
        write_two_module_project(proj)
    elif kind == "symlink":
        write_tiny_project(proj)
        os.symlink("tiny.py", proj / "linked.py")
    elif kind == "repo-sub":
        repo = root / "repo"
        write_tiny_project(repo / "proj")
        git(["init", "-q"], repo)
        git(["add", "-A"], repo)
        git(["commit", "-q", "-m", "fixture"], repo)
        return repo / "proj"
    elif kind == "empty-repo":
        proj.mkdir(parents=True, exist_ok=True)
        git(["init", "-q"], proj)
    elif kind == "none":
        pass
    else:
        raise SystemExit(f"unknown fixture kind {kind}")
    return proj


def tokenize(value, root):
    if isinstance(value, str):
        return value.replace(str(root), "<ROOT>").replace(
            sys.executable, "<PYTHON>"
        )
    if isinstance(value, list):
        return [tokenize(item, root) for item in value]
    if isinstance(value, dict):
        return {key: tokenize(item, root) for key, item in value.items()}
    return value


CASES = []


def record(name, fixture, target_of, gates, timeout=60, work="work",
           keep=False, mode="copy"):
    root = SCRATCH / name
    root.mkdir(parents=True)
    target = build_fixture(fixture, root)
    if target_of == "repo-sub":
        pass
    work_root = None if work is None else root / work
    if fixture == "nonempty-work":
        work_root.mkdir(parents=True)
        (work_root / "standing.txt").write_text("standing\n", encoding="utf-8")
    request = {
        "target": str(target),
        "gates": [[gate_name, command] for gate_name, command in gates],
        "timeout": timeout,
        "work_root": None if work_root is None else str(work_root),
        "keep_scratch": keep,
        "plant_mode": mode,
    }
    try:
        result = run_field(
            str(target),
            gates,
            timeout=timeout,
            work_root=work_root,
            keep_scratch=keep,
            plant_mode=mode,
        )
        entry = {"name": name, "fixture": fixture, "request": request,
                 "result": result}
    except ValueError as error:
        entry = {"name": name, "fixture": fixture, "request": request,
                 "error": str(error)}
    if keep:
        standing = sorted(
            entry.name for entry in (work_root).iterdir()
        ) if work_root and work_root.exists() else []
        entry["scratch_standing"] = standing
    CASES.append(tokenize(entry, root))


PYTEST = [GATE]
record("main-copy-three-gates", "main", "proj",
       [GATE, ("always-false", "false"), ("exit-three", "exit 3")])
record("main-git-pytest", "main-git", "proj", PYTEST, mode="git-branches")
record("tiny-copy-default-root", "tiny", "proj", PYTEST, work=None)
record("two-module-copy", "two-module", "proj", PYTEST)
record("main-copy-timeout", "main", "proj", [("slow", "sleep 5")], timeout=1)
record("main-copy-keep-scratch", "main", "proj", PYTEST, keep=True)
record("symlink-copy", "symlink", "proj", PYTEST)
record("refuse-not-a-directory", "none", "proj", PYTEST)
record("refuse-unknown-mode", "tiny", "proj", PYTEST, mode="sideways")
record("refuse-nonempty-work", "nonempty-work", "proj", PYTEST)
record("refuse-git-non-repo", "tiny", "proj", PYTEST, mode="git-branches")
record("refuse-git-below-root", "repo-sub", "repo-sub", PYTEST,
       mode="git-branches")
record("refuse-git-no-commits", "empty-repo", "proj", PYTEST,
       mode="git-branches")

shutil.rmtree(SCRATCH, ignore_errors=True)
document = {"format": "canaryfield-port5-cases/1", "cases": CASES}
OUT.write_text(json.dumps(document, indent=1, sort_keys=True) + "\n",
               encoding="utf-8")
print(f"wrote {len(CASES)} cases to {OUT}")
