"""Pins for the engine bridge of the fix pass, CR1.

The Python engine plants through the crate binary. Where
a binary stands, plant_seed drives it and carries its
answer back, and run_field returns the full report. Where
the caller names a binary that does not stand, the
refusal names it. The binary standing tests skip where
no binary stands, as they do in the hosted Python job,
which builds none.
"""

import pytest

from canaryfield import engine
from canaryfield.registry import SEEDS_BY_ID

MATCH_SOURCE = (
    "def outer():\n"
    "    match y:\n"
    "        case [x]:\n"
    "            pass\n"
    "    def inner():\n"
    "        nonlocal x\n"
    "        flag = True\n"
)


def _binary_stands():
    try:
        engine._find_binary()
    except NotImplementedError:
        return False
    return True


needs_binary = pytest.mark.skipif(
    not _binary_stands(), reason="no canaryfield binary stands"
)


def test_a_named_missing_binary_refuses_and_names_it(tmp_path, monkeypatch):
    missing = tmp_path / "no-such-canaryfield"
    monkeypatch.setenv("CANARYFIELD_BINARY", str(missing))
    with pytest.raises(NotImplementedError) as caught:
        engine.plant_seed(tmp_path, SEEDS_BY_ID["seed-05"])
    assert str(missing) in str(caught.value)


@needs_binary
def test_the_bridge_plants_through_the_binary(tmp_path):
    target = tmp_path / "mod.py"
    target.write_text(MATCH_SOURCE, encoding="utf-8")
    planted = engine.plant_seed(tmp_path, SEEDS_BY_ID["seed-05"])
    assert planted == ["mod.py"]
    assert "flag = False" in target.read_text(encoding="utf-8")


@needs_binary
def test_the_bridge_carries_the_skipped_names_back(tmp_path):
    (tmp_path / "bad.py").write_text("def broken(:\n", encoding="utf-8")
    skipped = []
    planted = engine.plant_seed(tmp_path, SEEDS_BY_ID["seed-05"], skipped)
    assert planted == []
    assert skipped == ["bad.py"]


@needs_binary
def test_run_field_names_an_unparseable_file_once(tmp_path):
    target = tmp_path / "target"
    target.mkdir()
    (target / "mod.py").write_text(MATCH_SOURCE, encoding="utf-8")
    (target / "bad.py").write_text("def broken(:\n", encoding="utf-8")
    report = engine.run_field(target, [("always", "true")])
    assert report["unparseable_files"] == ["bad.py"]


@needs_binary
def test_run_field_returns_the_full_report(tmp_path):
    target = tmp_path / "target"
    target.mkdir()
    (target / "mod.py").write_text(MATCH_SOURCE, encoding="utf-8")
    report = engine.run_field(target, [("always", "true")])
    assert len(report["rows"]) == 48
    assert report["target_unchanged"] is True
    row = next(row for row in report["rows"] if row["seed"] == "seed-05")
    assert row["planted_files"] == ["mod.py"]
    assert row["gates"]["always"] == "waved-through"
