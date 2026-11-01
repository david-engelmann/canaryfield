"""Tests for the Round 1 review fix obligations in CONTRACT.md.

Every fixture here is synthetic and written for this repo.
"""

import json
import subprocess
import sys
from pathlib import Path

import pytest

from canaryfield.cli import main as cli_main
from canaryfield.engine import CAUGHT, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data, write_seed_pack

from fixture_project import GATE, write_main_project, write_tiny_project


def _git(cwd, *arguments):
    completed = subprocess.run(
        ["git", "-C", str(cwd)] + list(arguments),
        capture_output=True,
        text=True,
    )
    assert completed.returncode == 0, completed.stderr
    return completed.stdout.strip()


def _write_git_project(root):
    _git(root, "init", "-q", "-b", "main")
    _git(root, "add", "-A")
    _git(
        root,
        "-c",
        "user.name=Fixture",
        "-c",
        "user.email=fixture@example.invalid",
        "commit",
        "-q",
        "-m",
        "Initial fixture commit",
    )
    return root


def test_nonempty_work_root_is_refused_and_left_intact(tmp_path):
    target = write_tiny_project(tmp_path / "project")
    work = tmp_path / "work"
    work.mkdir()
    sentinel = work / "operator-file.txt"
    sentinel.write_text("keep me", encoding="utf-8")
    with pytest.raises(ValueError, match="not empty"):
        run_field(target, [GATE], work_root=work, timeout=60)
    assert sentinel.read_text(encoding="utf-8") == "keep me"
    assert [entry.name for entry in work.iterdir()] == ["operator-file.txt"]


def test_failed_run_never_deletes_a_previously_kept_clone(tmp_path):
    target = _write_git_project(write_main_project(tmp_path / "project"))
    work = tmp_path / "work"
    result = run_field(
        target,
        [GATE],
        work_root=work,
        keep_scratch=True,
        plant_mode="git-branches",
        timeout=60,
    )
    kept_clone = work / "clone"
    assert kept_clone.is_dir()
    with pytest.raises(ValueError, match="not empty"):
        run_field(
            target,
            [GATE],
            work_root=work,
            plant_mode="git-branches",
            timeout=60,
        )
    assert kept_clone.is_dir()
    branches = _git(
        kept_clone, "branch", "--format=%(refname:short)"
    ).splitlines()
    assert "canaryfield/seed-00" in branches


def test_git_mode_survives_a_gate_that_writes_a_tracked_file(tmp_path):
    target = _write_git_project(write_main_project(tmp_path / "project"))
    writing_gate = (
        "writer",
        "printf 'noise\\n' >> sample.py; true",
    )
    result = run_field(
        target,
        [writing_gate],
        plant_mode="git-branches",
        timeout=60,
    )
    assert len(result["rows"]) == 28
    assert result["target_unchanged"] is True


def test_git_mode_refuses_a_subdirectory_of_a_repo(tmp_path):
    repo = _write_git_project(write_main_project(tmp_path / "project"))
    nested = repo / "examples"
    nested.mkdir()
    (nested / "note.txt").write_text("nested", encoding="utf-8")
    with pytest.raises(ValueError, match="not the root"):
        run_field(nested, [GATE], plant_mode="git-branches", timeout=60)


def test_git_mode_refuses_a_repo_with_no_commits(tmp_path):
    empty = tmp_path / "empty"
    empty.mkdir()
    _git(empty, "init", "-q", "-b", "main")
    with pytest.raises(ValueError, match="no commits"):
        run_field(empty, [GATE], plant_mode="git-branches", timeout=60)


def test_cli_refusal_is_an_error_line_not_a_traceback(tmp_path, capsys):
    target = write_tiny_project(tmp_path / "plain")
    code = cli_main(
        [
            "--target",
            str(target),
            "--gate",
            "g=true",
            "--plant",
            "git-branches",
        ]
    )
    captured = capsys.readouterr()
    assert code == 2
    assert "Traceback" not in captured.err
    assert "not a git repository" in captured.err


def _pack_with(tmp_path, mutate):
    data = seed_pack_data()
    mutate(data)
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_seed_pack_rejects_a_duplicate_id(tmp_path):
    def mutate(data):
        data["seeds"][1]["id"] = data["seeds"][0]["id"]

    with pytest.raises(ValueError, match="duplicate"):
        load_seed_pack(_pack_with(tmp_path, mutate))


def test_seed_pack_rejects_an_extra_seed(tmp_path):
    def mutate(data):
        extra = dict(data["seeds"][0])
        extra["id"] = "seed-99"
        data["seeds"].append(extra)

    with pytest.raises(ValueError, match="carries 29"):
        load_seed_pack(_pack_with(tmp_path, mutate))


def test_seed_pack_rejects_a_numeric_id(tmp_path):
    def mutate(data):
        data["seeds"][0]["id"] = 7

    with pytest.raises(ValueError, match="non empty piece of text"):
        load_seed_pack(_pack_with(tmp_path, mutate))
