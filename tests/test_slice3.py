"""Slice 3 tests. Plant everywhere, planted_files, wider git commits.

Every fixture here is synthetic and written for this repo. Git fixtures
are throwaway repositories built inside the test temp dirs.
"""

import json
import subprocess

import pytest

from canaryfield.engine import (
    CAUGHT,
    NOT_APPLICABLE,
    WAVED,
    hash_tree,
    plant_seed,
    run_field,
)
from canaryfield.report import to_json, to_text
from canaryfield.seeds import SEEDS_BY_ID

from fixture_project import (
    ALPHA_SOURCE,
    BETA_SOURCE,
    GATE,
    write_tiny_project,
    write_two_module_project,
)

NOOP_GATE = ("noop", "true")


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


def _git_state(root):
    return {
        "head": _git(root, "rev-parse", "HEAD"),
        "branches": _git(root, "branch", "--format=%(refname:short)"),
        "status": _git(root, "status", "--porcelain"),
        "hashes": hash_tree(root),
    }


def _rows(result):
    return {row["seed"]: row for row in result["rows"]}


@pytest.fixture(scope="module")
def two_module_run(tmp_path_factory):
    target = write_two_module_project(
        tmp_path_factory.mktemp("two-module") / "project"
    )
    result = run_field(target, [GATE], timeout=60)
    return target, result


def test_plant_seed_covers_every_applicable_file(tmp_path):
    scratch = write_two_module_project(tmp_path / "scratch")
    planted = plant_seed(scratch, SEEDS_BY_ID["seed-01"])
    assert planted == ["alpha.py", "beta.py"]
    alpha = (scratch / "alpha.py").read_text(encoding="utf-8")
    beta = (scratch / "beta.py").read_text(encoding="utf-8")
    assert alpha != ALPHA_SOURCE
    assert beta != BETA_SOURCE
    assert "return 10" not in alpha
    assert "return 20" not in beta
    untouched = write_two_module_project(tmp_path / "untouched")
    assert plant_seed(untouched, SEEDS_BY_ID["seed-00"]) == []
    assert (untouched / "alpha.py").read_text(encoding="utf-8") == ALPHA_SOURCE


def test_multi_file_fixture_names_every_planted_file(two_module_run):
    _target, result = two_module_run
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["alpha.py", "beta.py"]
    assert rows["seed-01"]["planted_file"] == "alpha.py"
    assert rows["seed-06"]["planted_files"] == ["alpha.py", "beta.py"]
    assert rows["seed-00"]["planted_files"] == []
    assert rows["seed-00"]["planted_file"] is None
    assert rows["seed-00"]["gates"]["tests"] == NOT_APPLICABLE


def test_report_lists_every_planted_file_in_text_and_json(two_module_run):
    _target, result = two_module_run
    text = to_text(result)
    assert "planted in alpha.py, beta.py" in text
    data = json.loads(to_json(result))
    rows = {row["seed"]: row for row in data["rows"]}
    assert rows["seed-01"]["planted_files"] == ["alpha.py", "beta.py"]
    assert rows["seed-00"]["planted_files"] == []


def test_multi_file_fixture_scores_the_fully_planted_state(two_module_run):
    _target, result = two_module_run
    rows = _rows(result)
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT
    assert rows["seed-03"]["gates"]["tests"] == CAUGHT
    assert rows["seed-06"]["gates"]["tests"] == CAUGHT
    summary = result["summary"]["tests"]
    assert summary[CAUGHT] == 3
    assert summary[WAVED] == 0
    assert summary[NOT_APPLICABLE] == 45
    assert result["target_unchanged"] is True


def test_single_file_fixture_matches_slice_two_verdicts(tmp_path):
    target = write_tiny_project(tmp_path / "tiny")
    result = run_field(target, [GATE], timeout=60)
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["tiny.py"]
    assert rows["seed-01"]["planted_file"] == "tiny.py"
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT
    assert rows["seed-03"]["planted_files"] == ["tiny.py"]
    assert rows["seed-03"]["gates"]["tests"] == CAUGHT
    assert rows["seed-00"]["planted_files"] == []
    assert rows["seed-00"]["gates"]["tests"] == NOT_APPLICABLE


def test_git_mode_branch_commit_carries_every_planted_file(tmp_path):
    target = _write_git_project(write_two_module_project(tmp_path / "project"))
    before = _git_state(target)
    work_root = tmp_path / "work"
    result = run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=work_root,
        keep_scratch=True,
        plant_mode="git-branches",
    )
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["alpha.py", "beta.py"]
    assert rows["seed-01"]["branch"] == "canaryfield/seed-01"
    clone = work_root / "clone"
    committed = _git(
        clone, "show", "--name-only", "--format=", "canaryfield/seed-01"
    )
    assert sorted(committed.splitlines()) == ["alpha.py", "beta.py"]
    planted_alpha = _git(clone, "show", "canaryfield/seed-01:alpha.py")
    planted_beta = _git(clone, "show", "canaryfield/seed-01:beta.py")
    assert "return 10" not in planted_alpha
    assert "return 20" not in planted_beta
    base_alpha = _git(clone, "show", "main:alpha.py")
    assert "return 10" in base_alpha
    assert result["target_unchanged"] is True
    assert _git_state(target) == before
    assert _git(target, "branch", "--format=%(refname:short)") == "main"


def test_git_mode_multi_file_fixture_scores_caught(tmp_path):
    target = _write_git_project(write_two_module_project(tmp_path / "project"))
    result = run_field(target, [GATE], timeout=60, plant_mode="git-branches")
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["alpha.py", "beta.py"]
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT
