"""Slice 2 tests. Git branch planting, the two gate example, the seed pack.

Every fixture here is synthetic and written for this repo. Git fixtures
are throwaway repositories built inside the test temp dirs.
"""

import json
import subprocess
import sys
from pathlib import Path

import pytest

from canaryfield.cli import main as cli_main
from canaryfield.engine import CAUGHT, NOT_APPLICABLE, WAVED, hash_tree, run_field
from canaryfield.report import to_json, to_text
from canaryfield.seedpack import SEED_PACK_FORMAT, load_seed_pack, write_seed_pack
from canaryfield.seeds import SEEDS

from fixture_project import GATE, write_main_project, write_tiny_project

ROOT = Path(__file__).resolve().parents[1]
NOOP_GATE = ("noop", "true")
COMPILE_GATE = ("compile", "python3 -m compileall -q .")


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


def _branches(clone):
    return set(_git(clone, "branch", "--format=%(refname:short)").splitlines())


@pytest.fixture(scope="module")
def git_run(tmp_path_factory):
    target = _write_git_project(
        write_main_project(tmp_path_factory.mktemp("git-target") / "project")
    )
    before = _git_state(target)
    result = run_field(target, [GATE], timeout=60, plant_mode="git-branches")
    after = _git_state(target)
    return target, before, after, result


@pytest.fixture(scope="module")
def git_clone_kept(tmp_path_factory):
    base = tmp_path_factory.mktemp("git-kept")
    target = _write_git_project(write_main_project(base / "project"))
    work_root = base / "work"
    result = run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=work_root,
        keep_scratch=True,
        plant_mode="git-branches",
    )
    return target, work_root / "clone", result


def test_git_mode_scores_caught_and_waved(git_run):
    _target, _before, _after, result = git_run
    assert result["plant_mode"] == "git-branches"
    assert result["clean_exits"]["tests"] == 0
    cells = {row["seed"]: row["gates"]["tests"] for row in result["rows"]}
    assert cells["seed-01"] == CAUGHT
    assert cells["seed-00"] == CAUGHT
    assert cells["seed-07"] == WAVED
    assert cells["seed-09"] == WAVED
    summary = result["summary"]["tests"]
    assert summary[CAUGHT] == 14
    assert summary[WAVED] == 3
    for row in result["rows"]:
        if row["planted_files"]:
            assert row["branch"] == f"canaryfield/{row['seed']}"
        else:
            assert row["branch"] is None


def test_git_mode_target_git_state_untouched(git_run):
    target, before, after, result = git_run
    assert result["target_unchanged"] is True
    assert before == after
    assert _git(target, "branch", "--format=%(refname:short)") == "main"


def test_git_mode_plants_each_seed_on_its_own_branch(git_clone_kept):
    _target, clone, result = git_clone_kept
    assert clone.is_dir()
    branches = _branches(clone)
    expected = {f"canaryfield/seed-{index:02d}" for index in range(13)}
    assert expected <= branches
    planted = _git(clone, "show", "canaryfield/seed-00:sample.py")
    assert "9.9.9" in planted
    assert "1.4.2" not in planted
    base = _git(clone, "show", "main:sample.py")
    assert "1.4.2" in base
    for row in result["rows"]:
        if row["planted_files"]:
            assert row["branch"] in branches
        else:
            assert row["branch"] is None


def test_git_mode_inapplicable_seed_gets_no_branch(tmp_path):
    target = _write_git_project(write_tiny_project(tmp_path / "tiny"))
    work_root = tmp_path / "work"
    result = run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=work_root,
        keep_scratch=True,
        plant_mode="git-branches",
    )
    branches = _branches(work_root / "clone")
    cells = {row["seed"]: row["gates"]["noop"] for row in result["rows"]}
    for seed_id in ("seed-00", "seed-04", "seed-05"):
        assert cells[seed_id] == NOT_APPLICABLE
        assert f"canaryfield/{seed_id}" not in branches
    rows = {row["seed"]: row for row in result["rows"]}
    assert rows["seed-00"]["branch"] is None


def test_git_mode_scratch_clone_deleted_by_default(tmp_path):
    target = _write_git_project(write_tiny_project(tmp_path / "tiny"))
    work_root = tmp_path / "work"
    run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=work_root,
        plant_mode="git-branches",
    )
    assert not work_root.exists()


def test_git_mode_scratch_clone_kept_on_request(git_clone_kept):
    _target, clone, _result = git_clone_kept
    assert (clone / ".git").exists()
    assert (clone / "sample.py").exists()


def test_git_mode_requires_a_git_target(tmp_path):
    target = write_main_project(tmp_path / "plain")
    with pytest.raises(ValueError, match="not a git repository"):
        run_field(target, [NOOP_GATE], timeout=30, plant_mode="git-branches")


def test_copy_mode_example_regenerates_byte_identical(monkeypatch):
    monkeypatch.chdir(ROOT)
    result = run_field(
        Path("examples/demo-project"),
        [("tests", f"{sys.executable} -m pytest -q")],
        timeout=60,
    )
    committed_text = (ROOT / "examples" / "catch-table.txt").read_text(encoding="utf-8")
    committed_json = (ROOT / "examples" / "catch-table.json").read_text(encoding="utf-8")
    assert to_text(result) == committed_text
    assert to_json(result) == committed_json


def test_two_gate_example_shows_an_honest_disagreement():
    table = json.loads(
        (ROOT / "examples" / "two-gates" / "catch-table.json").read_text(encoding="utf-8")
    )
    assert table["gates"] == ["tests", "compile"]
    rows = {row["seed"]: row["gates"] for row in table["rows"]}
    assert rows["seed-00"]["tests"] == CAUGHT
    assert rows["seed-00"]["compile"] == WAVED
    disagreements = [
        seed_id
        for seed_id, gates in rows.items()
        if gates["tests"] != gates["compile"]
    ]
    assert len(disagreements) == 14


def test_two_gate_run_scores_each_gate_on_its_own_evidence():
    result = run_field(
        ROOT / "examples" / "demo-project",
        [("tests", f"{sys.executable} -m pytest -q"), COMPILE_GATE],
        timeout=60,
    )
    rows = {row["seed"]: row["gates"] for row in result["rows"]}
    assert rows["seed-01"]["tests"] == CAUGHT
    assert rows["seed-01"]["compile"] == WAVED
    assert result["summary"]["compile"][WAVED] == 17
    assert result["summary"]["tests"][CAUGHT] == 14


def test_seed_pack_round_trip(tmp_path):
    pack_path = write_seed_pack(tmp_path / "seeds.json")
    data = json.loads(pack_path.read_text(encoding="utf-8"))
    assert data["format"] == "canaryfield-seeds/7"
    entries = load_seed_pack(pack_path)
    assert [entry["id"] for entry in entries] == [seed.id for seed in SEEDS]
    assert [entry["defect_class"] for entry in entries] == [
        seed.defect_class for seed in SEEDS
    ]
    assert [entry["name"] for entry in entries] == [seed.name for seed in SEEDS]
    for entry in entries:
        assert entry["description"].strip(), entry["id"]


def test_seed_pack_via_cli(tmp_path):
    pack_path = tmp_path / "pack.json"
    assert cli_main(["--write-seed-pack", str(pack_path)]) == 0
    entries = load_seed_pack(pack_path)
    assert len(entries) == 36


def test_seed_pack_rejects_a_foreign_format(tmp_path):
    foreign = tmp_path / "foreign.json"
    foreign.write_text(json.dumps({"format": "other/9", "seeds": []}), encoding="utf-8")
    with pytest.raises(ValueError, match="seed pack"):
        load_seed_pack(foreign)
