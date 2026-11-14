"""Pins for the engine bridge of PORT5.

run_field drives the crate binary's run door once and
carries its answer back whole, and a refusal of the door
arrives as the ValueError the command line catches. Where
no binary stands the bridge refuses in the PORT5 words,
and a binary named through the environment that does not
stand refuses in the words that name it. The binary
standing tests skip where no binary stands, as the
standing bridge tests do.
"""

import pytest

from canaryfield import engine


def _binary_stands():
    try:
        engine._find_binary()
    except NotImplementedError:
        return False
    return True


needs_binary = pytest.mark.skipif(
    not _binary_stands(), reason="no canaryfield binary stands"
)


@needs_binary
def test_the_bridge_returns_the_doors_result_whole(tmp_path):
    target = tmp_path / "target"
    target.mkdir()
    (target / "mod.py").write_text(
        "def answer():\n    return 42\n", encoding="utf-8"
    )
    report = engine.run_field(target, [("always", "true")])
    assert len(report["rows"]) == 48
    assert report["target"] == str(target)
    assert report["target_unchanged"] is True
    assert report["clean_exits"] == {"always": 0}
    assert report["summary"]["always"]["caught"] == 0


@needs_binary
def test_a_door_refusal_arrives_as_value_error(tmp_path):
    missing = tmp_path / "no-such-target"
    with pytest.raises(ValueError) as caught:
        engine.run_field(missing, [("always", "true")])
    assert str(caught.value) == f"target is not a directory: {missing}"


@needs_binary
def test_an_unknown_mode_refuses_in_the_doors_words(tmp_path):
    with pytest.raises(ValueError) as caught:
        engine.run_field(tmp_path, [("always", "true")], plant_mode="sideways")
    assert str(caught.value) == "unknown plant mode: sideways"


def test_a_named_missing_binary_refuses_and_names_it(tmp_path, monkeypatch):
    missing = tmp_path / "no-such-canaryfield"
    monkeypatch.setenv("CANARYFIELD_BINARY", str(missing))
    with pytest.raises(NotImplementedError) as caught:
        engine.run_field(tmp_path, [("always", "true")])
    assert str(caught.value) == (
        f"the named canaryfield binary does not stand: {missing}"
    )


def test_no_binary_stands_refuses_in_the_port5_words(tmp_path, monkeypatch):
    monkeypatch.delenv("CANARYFIELD_BINARY", raising=False)
    monkeypatch.setattr(engine.shutil, "which", lambda name: None)
    monkeypatch.setattr(engine, "_CHECKOUT_ROOT", tmp_path)
    with pytest.raises(NotImplementedError) as caught:
        engine.run_field(tmp_path, [("always", "true")])
    assert str(caught.value) == "run_field stands in the Rust crate under PORT5"
