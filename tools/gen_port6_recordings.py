"""Record the PORT6 command line battery from the landed Python.

The recordings at tests/conformance/port6_cases.json are generated
from the landed Python command line in canaryfield/cli.py, driven
as a process with the crate binary named in its discovery, never
from this port. Each case runs in a fresh scene directory with the
scene as its working directory, and the two spellings that vary by
machine, the scene's absolute path and the Python interpreter,
are tokenized in argv and in every recorded byte stream as
{SCENE} and {PYTHON}, the spellings the Rust driver substitutes
back before it drives the shell.

Usage: gen_port6_recordings.py BINARY OUTPUT
"""

import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIXTURE_PATH = ROOT / "tests" / "fixture_project.py"

GIT_ENV = {
    "GIT_AUTHOR_NAME": "canaryfield",
    "GIT_AUTHOR_EMAIL": "canaryfield@example.invalid",
    "GIT_COMMITTER_NAME": "canaryfield",
    "GIT_COMMITTER_EMAIL": "canaryfield@example.invalid",
}


def load_fixture():
    spec = importlib.util.spec_from_file_location("fixture_project", FIXTURE_PATH)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def tokenize(text, scene):
    return text.replace(str(scene), "{SCENE}").replace(sys.executable, "{PYTHON}")


def git(arguments, cwd, env):
    done = subprocess.run(
        ["git", "-C", str(cwd)] + arguments, capture_output=True, text=True, env=env
    )
    if done.returncode != 0:
        raise SystemExit(f"fixture git {arguments} failed: {done.stderr}")


def run_case(spec, binary):
    scene = Path(tempfile.mkdtemp(prefix="canaryfield-port6-"))
    fixture = load_fixture()
    kind = spec.get("fixture")
    if kind == "main":
        fixture.write_main_project(scene / "project")
    elif kind == "main-git":
        fixture.write_main_project(scene / "project")
        env = dict(os.environ)
        env.update(GIT_ENV)
        git(["init", "-q"], scene / "project", env)
        git(["add", "-A"], scene / "project", env)
        git(["commit", "-q", "-m", "fixture"], scene / "project", env)
    argv = [
        part.replace("{SCENE}", str(scene)).replace("{PYTHON}", sys.executable)
        for part in spec["argv"]
    ]
    env = dict(os.environ)
    env["CANARYFIELD_BINARY"] = str(binary)
    env["PYTHONPATH"] = str(ROOT)
    done = subprocess.run(
        [sys.executable, "-m", "canaryfield"] + argv,
        cwd=scene,
        capture_output=True,
        env=env,
    )
    files = {}
    for rel in spec.get("files", []):
        target = scene / rel
        files[rel] = tokenize(target.read_text(encoding="utf-8"), scene)
    return {
        "name": spec["name"],
        "fixture": spec.get("fixture"),
        "argv": spec["argv"],
        "exit": done.returncode,
        "stdout": tokenize(done.stdout.decode("utf-8"), scene),
        "stderr": tokenize(done.stderr.decode("utf-8"), scene),
        "files": files,
    }


def main():
    binary = Path(sys.argv[1]).resolve()
    output = Path(sys.argv[2])
    gate = "tests={PYTHON} -m pytest -q"
    cases = [
        {"name": "help_long", "argv": ["--help"]},
        {"name": "help_short", "argv": ["-h"]},
        {"name": "no_args", "argv": []},
        {"name": "gate_bad_form", "argv": ["--target", "x", "--gate", "badform"]},
        {"name": "gate_empty_command", "argv": ["--target", "x", "--gate", "name="]},
        {"name": "gate_empty_name", "argv": ["--target", "x", "--gate", "=true"]},
        {
            "name": "plant_bad_choice",
            "argv": ["--target", "x", "--gate", "a=b", "--plant", "sideways"],
        },
        {
            "name": "timeout_bad_int",
            "argv": ["--target", "x", "--gate", "a=b", "--timeout", "abc"],
        },
        {
            "name": "timeout_hex",
            "argv": ["--target", "x", "--gate", "a=b", "--timeout", "0x10"],
        },
        {
            "name": "timeout_underscore_accepted",
            "argv": ["--target", "missing", "--gate", "a=b", "--timeout", "1_0"],
        },
        {
            "name": "unrecognized_flag",
            "argv": ["--target", "x", "--gate", "a=b", "--bogus"],
        },
        {"name": "ambiguous_prefix", "argv": ["--t", "x"]},
        {"name": "missing_value", "argv": ["--target"]},
        {"name": "bare_word", "argv": ["hello"]},
        {
            "name": "separator_tail",
            "argv": ["--target", "x", "--gate", "a=b", "--", "extra"],
        },
        {
            "name": "pack_write",
            "argv": ["--write-seed-pack", "out/pack.json"],
            "files": ["out/pack.json"],
        },
        {
            "name": "pack_write_with_target_standing",
            "argv": ["--target", "x", "--write-seed-pack", "out/pack.json"],
            "files": ["out/pack.json"],
        },
        {
            "name": "prefix_flags",
            "argv": ["--tar", "missing", "--ga", "a=b"],
        },
        {
            "name": "repeated_target_last_wins",
            "argv": ["--target", "missing", "--target", "alsomissing", "--gate", "a=b"],
        },
        {
            "name": "run_refusal_not_a_directory",
            "argv": ["--target", "missing", "--gate", "a=true"],
        },
        {
            "name": "run_copy_text",
            "fixture": "main",
            "argv": ["--target", "project", "--gate", gate, "--work-root", "work"],
        },
        {
            "name": "run_copy_json",
            "fixture": "main",
            "argv": [
                "--target", "project", "--gate", gate, "--work-root", "work", "--json",
            ],
        },
        {
            "name": "run_git_text",
            "fixture": "main-git",
            "argv": [
                "--target", "project", "--gate", gate,
                "--plant", "git-branches", "--work-root", "work",
            ],
        },
        {
            "name": "run_out_dir",
            "fixture": "main",
            "argv": [
                "--target", "project", "--gate", gate, "--work-root", "work",
                "--keep-scratch", "--out", "out",
            ],
            "files": ["out/catch-table.txt", "out/catch-table.json"],
        },
        {
            "name": "run_changed_target_exit_3",
            "fixture": "main",
            "argv": [
                "--target", "project",
                "--gate", 'vandal=sh -c "echo vandal >> {SCENE}/project/sample.py"',
                "--work-root", "work",
            ],
        },
    ]
    recorded = [run_case(spec, binary) for spec in cases]
    document = {"format": "canaryfield-port6-cases/1", "cases": recorded}
    output.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
    print(f"recorded {len(recorded)} cases to {output}")


if __name__ == "__main__":
    main()
