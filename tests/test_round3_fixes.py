"""Tests for the Round 3 review fix obligations in CONTRACT.md.

Every fixture here is synthetic and written for this test file.
The red team probes are ported here as regression tests. Copy
mode symlink behaviour is pinned here because the Round 2 tests
pinned it only in git mode.
"""

import os
import sys

import pytest

from canaryfield.engine import NOT_APPLICABLE, run_field

needs_symlink = pytest.mark.skipif(
    not hasattr(os, "symlink"), reason="platform cannot create symlinks"
)

GATE = (
    sys.executable + " -m pytest -q tests/test_mod.py -p no:cacheprovider"
)


def _write_project(root, module_text, test_text=None):
    (root / "tests").mkdir(parents=True, exist_ok=True)
    (root / "mod.py").write_text(module_text, encoding="utf-8")
    (root / "tests" / "test_mod.py").write_text(
        test_text
        or "from mod import value\n\n\ndef test_value():\n    assert value() == 1\n",
        encoding="utf-8",
    )
    return root


def _row(result, seed_id):
    return next(r for r in result["rows"] if r["seed"] == seed_id)


@needs_symlink
def test_copy_mode_never_plants_through_a_symlink_alias(tmp_path):
    root = _write_project(
        tmp_path / "proj", "def value():\n    return 1\n"
    )
    os.symlink("mod.py", root / "alias.py")
    result = run_field(root, [("gate", GATE)])
    row = _row(result, "seed-01")
    assert row["planted_files"] == ["mod.py"]


@needs_symlink
def test_copy_mode_never_plants_a_file_outside_the_target(tmp_path):
    outside = tmp_path / "outside.py"
    outside.write_text("def value():\n    return 1\n", encoding="utf-8")
    before = outside.read_bytes()
    root = _write_project(
        tmp_path / "proj", "def value():\n    return 1\n"
    )
    os.symlink(outside, root / "linked.py")
    result = run_field(root, [("gate", GATE)])
    row = _row(result, "seed-01")
    assert "linked.py" not in row["planted_files"]
    assert outside.read_bytes() == before


def test_seed_eleven_judges_a_return_against_its_own_function(tmp_path):
    module = (
        "def outer(a, b):\n"
        "    def inner(c):\n"
        "        return c\n"
        "    return inner(a)\n"
        "\n"
        "\n"
        "def value():\n"
        "    return 1\n"
    )
    root = _write_project(tmp_path / "proj", module)
    result = run_field(root, [("gate", GATE)])
    row = _row(result, "seed-11")
    assert row["gates"]["gate"] == NOT_APPLICABLE
    assert row["planted_files"] == []


def test_seed_eleven_plants_a_nested_function_on_its_own_params(tmp_path):
    module = (
        "def outer():\n"
        "    def inner(a, b):\n"
        "        return a\n"
        "    return inner(1, 2)\n"
        "\n"
        "\n"
        "def value():\n"
        "    return outer()\n"
    )
    test_text = (
        "from mod import value\n"
        "\n"
        "\n"
        "def test_value():\n"
        "    assert value() == 1\n"
    )
    root = _write_project(tmp_path / "proj", module, test_text)
    result = run_field(root, [("gate", GATE)])
    row = _row(result, "seed-11")
    assert row["planted_files"] == ["mod.py"]


def test_seed_zero_four_plants_past_an_already_swallowed_handler(
    tmp_path,
):
    module = (
        "def first():\n"
        "    try:\n"
        "        raise ValueError('boom')\n"
        "    except Exception:\n"
        "        pass\n"
        "\n"
        "\n"
        "def second():\n"
        "    try:\n"
        "        raise ValueError('boom')\n"
        "    except Exception:\n"
        "        return 2\n"
        "\n"
        "\n"
        "def value():\n"
        "    return second()\n"
    )
    test_text = (
        "from mod import value\n"
        "\n"
        "\n"
        "def test_value():\n"
        "    assert value() == 2\n"
    )
    root = _write_project(tmp_path / "proj", module, test_text)
    result = run_field(root, [("gate", GATE)])
    row = _row(result, "seed-04")
    assert row["planted_files"] == ["mod.py"]
    assert row["gates"]["gate"] == "caught"
