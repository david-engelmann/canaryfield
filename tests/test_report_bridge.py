"""Pins for the report bridge of PORT4.

The command line renders its reports through the crate
binary. Where a binary stands, the bridge drives its report
door and carries the rendering back, and a refusal of the
door is raised carrying the door's words. Where the caller
names a binary that does not stand, the refusal names it,
and where no binary stands the refusal names the ground the
renderings stand on. A result the JSON rendering cannot
carry is refused before the door is driven. The binary
standing tests skip where no binary stands, as they do in
the hosted Python job, which builds none.
"""

import pytest

from canaryfield import cli


def _result():
    return {
        "gates": ["alpha"],
        "target": "demo-target",
        "target_unchanged": True,
        "clean_exits": {"alpha": 0},
        "rows": [
            {
                "seed": "seed-01",
                "name": "first seed",
                "planted_file": "mod.py",
                "gates": {"alpha": "caught"},
                "planted_files": ["mod.py"],
            }
        ],
        "unparseable_files": [],
        "summary": {
            "alpha": {
                "caught": 1,
                "waved-through": 0,
                "not-applicable": 0,
                "gate-broken": 0,
            }
        },
    }


def _binary_stands():
    try:
        cli._find_report_binary()
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
        cli._render_report(_result(), "text")
    assert str(caught.value) == (
        f"the named canaryfield binary does not stand: {missing}"
    )


def test_no_binary_refuses_with_the_standing_sentence(tmp_path, monkeypatch):
    monkeypatch.delenv("CANARYFIELD_BINARY", raising=False)
    monkeypatch.setenv("PATH", str(tmp_path))
    monkeypatch.setattr(cli, "_CHECKOUT_ROOT", tmp_path)
    with pytest.raises(NotImplementedError) as caught:
        cli._render_report(_result(), "text")
    assert str(caught.value) == (
        "the report renderings stand in the Rust crate under PORT4"
    )


def test_a_value_with_no_json_kind_is_refused_before_the_door():
    result = _result()
    result["extra"] = {"a", "b"}
    with pytest.raises(ValueError) as caught:
        cli._render_report(result, "json")
    assert str(caught.value) == "the result holds a value with no JSON kind"


def test_a_key_that_is_not_text_is_refused_before_the_door():
    result = _result()
    result["extra"] = {7: "seven"}
    with pytest.raises(ValueError) as caught:
        cli._render_report(result, "json")
    assert str(caught.value) == "the result holds a key that is not text"


def test_a_reference_cycle_is_refused_before_the_door():
    result = _result()
    result["extra"] = []
    result["extra"].append(result["extra"])
    with pytest.raises(ValueError) as caught:
        cli._render_report(result, "text")
    assert str(caught.value) == "the result holds a reference cycle"


def test_a_named_relative_binary_is_resolved_before_it_is_driven(
    tmp_path, monkeypatch
):
    named = tmp_path / "cfbin"
    named.write_text("#!/bin/sh\n", encoding="utf-8")
    monkeypatch.chdir(tmp_path)
    monkeypatch.setenv("CANARYFIELD_BINARY", "./cfbin")
    found = cli._find_report_binary()
    assert found.is_absolute()
    assert found == named.resolve()


@needs_binary
def test_the_bridge_preserves_carriage_returns_in_a_rendering():
    result = _result()
    result["target"] = "demo\rtarget\r\nline"
    rendered = cli._render_report(result, "text")
    assert "demo\rtarget\r\nline" in rendered


@needs_binary
def test_the_bridge_renders_the_text_form_through_the_binary():
    rendered = cli._render_report(_result(), "text")
    assert rendered.startswith("canaryfield catch table\n")
    assert rendered.endswith("gate-broken 0\n")


@needs_binary
def test_the_bridge_renders_the_json_form_through_the_binary():
    rendered = cli._render_report(_result(), "json")
    assert rendered.startswith('{\n  "clean_exits"')
    assert rendered.endswith("}\n")


@needs_binary
def test_a_door_refusal_is_raised_carrying_the_doors_words():
    result = _result()
    del result["gates"]
    with pytest.raises(ValueError) as caught:
        cli._render_report(result, "text")
    assert 'the result holds no key named "gates"' in str(caught.value)
