import pytest

from canaryfield.engine import (
    CAUGHT,
    GATE_BROKEN,
    NOT_APPLICABLE,
    WAVED,
    hash_tree,
    run_field,
)
from canaryfield.report import to_json, to_text

from fixture_project import GATE, write_main_project, write_tiny_project


@pytest.fixture(scope="module")
def main_run(tmp_path_factory):
    target = write_main_project(tmp_path_factory.mktemp("target") / "project")
    result = run_field(target, [GATE], timeout=60)
    return target, result


def _cells(result):
    return {row["seed"]: row["gates"]["tests"] for row in result["rows"]}


def test_caught_and_waved_scoring(main_run):
    _target, result = main_run
    cells = _cells(result)
    for seed_id in (
        "seed-00",
        "seed-01",
        "seed-02",
        "seed-03",
        "seed-04",
        "seed-05",
        "seed-06",
        "seed-08",
        "seed-10",
        "seed-11",
        "seed-12",
    ):
        assert cells[seed_id] == CAUGHT, seed_id
    assert cells["seed-07"] == WAVED
    assert cells["seed-09"] == WAVED
    for seed_id in ("seed-13", "seed-14", "seed-15"):
        assert cells[seed_id] == NOT_APPLICABLE, seed_id


def test_gate_runs_on_clean_and_planted_copies(main_run):
    _target, result = main_run
    assert result["clean_exits"]["tests"] == 0
    planted = 0
    for row in result["rows"]:
        if not row["planted_files"]:
            assert row["planted_file"] is None
            continue
        planted += 1
        assert row["planted_file"] == "sample.py"
        assert row["planted_exits"]["tests"] is not None
    assert planted == 13


def test_catch_table_shape_and_statuses(main_run):
    _target, result = main_run
    assert len(result["rows"]) == 16
    allowed = {CAUGHT, WAVED, NOT_APPLICABLE, GATE_BROKEN}
    for row in result["rows"]:
        assert set(row["gates"].values()) <= allowed
    summary = result["summary"]["tests"]
    assert summary[CAUGHT] == 11
    assert summary[WAVED] == 2
    assert summary[NOT_APPLICABLE] == 3
    assert summary[GATE_BROKEN] == 0


def test_report_renders_text_and_json_with_the_same_facts(main_run):
    _target, result = main_run
    text = to_text(result)
    rendered_json = to_json(result)
    assert "canaryfield catch table" in text
    assert "seed-07" in text and WAVED in text
    assert '"seed-07"' in rendered_json and WAVED in rendered_json
    assert result["summary"]["tests"][CAUGHT] == 11


def test_target_tree_is_byte_identical_after_a_run(tmp_path):
    target = write_main_project(tmp_path / "project")
    before = hash_tree(target)
    result = run_field(target, [("noop", "true")], timeout=30)
    after = hash_tree(target)
    assert before == after
    assert result["target_unchanged"] is True


def test_inapplicable_seed_is_recorded_not_applicable(tmp_path):
    target = write_tiny_project(tmp_path / "tiny")
    result = run_field(target, [("noop", "true")], timeout=30)
    cells = {row["seed"]: row["gates"]["noop"] for row in result["rows"]}
    assert cells["seed-00"] == NOT_APPLICABLE
    assert cells["seed-04"] == NOT_APPLICABLE
    assert cells["seed-05"] == NOT_APPLICABLE
    assert result["rows"][0]["planted_file"] is None


def test_gate_failing_on_clean_copy_is_broken(tmp_path):
    target = write_main_project(tmp_path / "project")
    broken_gate = ("broken", "python3 -c \"import sys; sys.exit(3)\"")
    result = run_field(target, [broken_gate], timeout=30)
    assert result["clean_exits"]["broken"] == 3
    for row in result["rows"]:
        expected = GATE_BROKEN if row["planted_file"] else NOT_APPLICABLE
        assert row["gates"]["broken"] == expected
    assert result["summary"]["broken"][CAUGHT] == 0
    assert result["summary"]["broken"][GATE_BROKEN] == 13
