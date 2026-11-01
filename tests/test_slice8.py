"""Slice 8 tests. Four new seeds and a fourth seed pack format.

Every fixture here is synthetic and written for this repo. Each new
seed gets one target where a gate exercising the defect catches it
and one target where its shape is absent and its row reads not
applicable, and the scoping rules each obligation states get a
content level pin each.
"""

import ast
import json

import pytest

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

COMBINE_SOURCE = """\
def combine(left, right):
    return left * 10 + right


def score():
    return combine(3, 4)
"""

COMBINE_TEST = """\
from combining import score


def test_score():
    assert score() == 34
"""

SINGLE_SOURCE = """\
def answer():
    return 42


def show():
    return str(answer())
"""

SINGLE_TEST = """\
from single import answer, show


def test_show():
    assert answer() == 42
    assert show() == "42"
"""

LABEL_SOURCE = """\
def status():
    return "ready"
"""

LABEL_TEST = """\
from labelling import status


def test_status():
    assert status() == "ready"
"""

DOCS_SOURCE = '''\
"""Module notes written for this fixture."""


def answer():
    return 42
'''

DOCS_TEST = """\
from documented import answer


def test_answer():
    assert answer() == 42
"""

OFFSET_SOURCE = """\
def adjust(value):
    return -value
"""

OFFSET_TEST = """\
from offsetting import adjust


def test_adjust():
    assert adjust(5) == -5
    assert adjust(-3) == 3
"""

IDENTITY_SOURCE = """\
def identity(value):
    return value
"""

IDENTITY_TEST = """\
from identity import identity


def test_identity():
    assert identity(7) == 7
"""

SEARCH_SOURCE = """\
def find_first(values, wanted):
    found = -1
    for index, value in enumerate(values):
        if value == wanted:
            found = index
            break
    return found
"""

SEARCH_TEST = """\
from searching import find_first


def test_find_first():
    assert find_first([5, 7, 5], 5) == 0
    assert find_first([1, 2], 9) == -1
"""

SUM_SOURCE = """\
def total(values):
    result = 0
    for value in values:
        result += value
    return result
"""

SUM_TEST = """\
from summing import total


def test_total():
    assert total([1, 2, 3]) == 6
    assert total([]) == 0
"""


def _write_project(root, files):
    root.mkdir(parents=True, exist_ok=True)
    for name, text in files.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


def _cells(result):
    return {row["seed"]: row["gates"]["tests"] for row in result["rows"]}


def test_seed_twenty_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"combining.py": COMBINE_SOURCE, "test_combining.py": COMBINE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-20"] == CAUGHT


def test_seed_twenty_swaps_the_first_two_arguments():
    tree = ast.parse("result = combine(left, right)\n")
    assert SEEDS_BY_ID["seed-20"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "combine(right, left)" in rendered


def test_seed_twenty_leaves_keyword_arguments_standing():
    tree = ast.parse("result = combine(left, right, mode='fast')\n")
    assert SEEDS_BY_ID["seed-20"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "combine(right, left, mode='fast')" in rendered


def test_seed_twenty_trades_a_starred_argument_like_any_other():
    tree = ast.parse("result = combine(*pair, 9)\n")
    assert SEEDS_BY_ID["seed-20"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "combine(9, *pair)" in rendered


def test_seed_twenty_identical_arguments_exchange_into_themselves():
    source = "result = combine(same, same)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-20"].apply(tree) is True
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_not_applicable_without_a_two_argument_call(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-20"] == NOT_APPLICABLE


def test_seed_twenty_one_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"labelling.py": LABEL_SOURCE, "test_labelling.py": LABEL_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-21"] == CAUGHT


def test_seed_twenty_one_changes_the_first_character():
    tree = ast.parse('LABEL = "ready"\nOTHER = "xenon"\n')
    assert SEEDS_BY_ID["seed-21"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "'xeady'" in rendered
    assert "'zenon'" in rendered
    assert "'ready'" not in rendered


def test_seed_twenty_one_leaves_docstrings_standing():
    source = (
        '"""Module notes."""\n'
        "def f():\n"
        '    """Function notes."""\n'
        '    return "ready"\n'
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-21"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "Module notes." in rendered
    assert "Function notes." in rendered
    assert "'xeady'" in rendered


def test_seed_twenty_one_leaves_the_version_literal_standing():
    tree = ast.parse('VERSION = "1.4.2"\nLABEL = "ready"\n')
    assert SEEDS_BY_ID["seed-21"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "'1.4.2'" in rendered
    assert "'xeady'" in rendered


def test_seed_twenty_one_alters_the_fixed_text_of_a_format_string():
    tree = ast.parse('MSG = f"total is {n} items"\n')
    assert SEEDS_BY_ID["seed-21"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "xotal is" in rendered
    assert "xitems" in rendered


def test_seed_twenty_one_not_applicable_without_an_in_scope_literal(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"documented.py": DOCS_SOURCE, "test_documented.py": DOCS_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-21"] == NOT_APPLICABLE


def test_seed_twenty_two_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"offsetting.py": OFFSET_SOURCE, "test_offsetting.py": OFFSET_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-22"] == CAUGHT


def test_seed_twenty_two_removes_the_minus_and_keeps_the_operand():
    source = "def adjust(value):\n    return -value\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-22"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return value" in rendered
    assert "-value" not in rendered


def test_seed_twenty_two_drops_a_nested_negation_once_at_each_level():
    source = "def adjust(value):\n    return --value\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-22"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return value" in rendered
    assert rendered.count("-") == 0


def test_seed_twenty_two_leaves_unary_plus_and_logical_not_standing():
    source = (
        "def plus(value):\n"
        "    return +value\n"
        "\n"
        "\n"
        "def negate(flag):\n"
        "    return not flag\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-22"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_two_not_applicable_without_a_negation(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"identity.py": IDENTITY_SOURCE, "test_identity.py": IDENTITY_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-22"] == NOT_APPLICABLE


def test_seed_twenty_three_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"searching.py": SEARCH_SOURCE, "test_searching.py": SEARCH_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-23"] == CAUGHT


def test_seed_twenty_three_turns_a_break_into_a_continue():
    source = (
        "def hunt(items):\n"
        "    for item in items:\n"
        "        if item:\n"
        "            break\n"
        "    return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-23"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "continue" in rendered
    assert "break" not in rendered


def test_seed_twenty_three_plants_the_inner_loop_break_on_its_own_loop():
    source = (
        "for outer in rows:\n"
        "    for inner in outer:\n"
        "        break\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-23"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "for inner in outer:\n        continue" in rendered
    assert "break" not in rendered


def test_seed_twenty_three_leaves_a_break_outside_a_loop_body_standing():
    source = (
        "def hunt(items):\n"
        "    for item in items:\n"
        "        def helper():\n"
        "            break\n"
        "    while items:\n"
        "        items.pop()\n"
        "    else:\n"
        "        break\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-23"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_three_not_applicable_without_a_break(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"summing.py": SUM_SOURCE, "test_summing.py": SUM_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-23"] == NOT_APPLICABLE


def _pack_file(tmp_path, data):
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def _entry(seed):
    return {
        "id": seed.id,
        "name": seed.name,
        "defect_class": seed.defect_class,
        "description": seed.description,
    }


def _version_three_data():
    return {
        "format": "canaryfield-seeds/3",
        "seeds": [_entry(seed) for seed in SEEDS[:20]],
    }


def test_version_three_pack_still_loads_the_twenty(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_three_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:20]
    ]


def test_a_version_four_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][23]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_three_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_three_data()
    data["seeds"][19] = _entry(SEEDS[20])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_four_is_refused(
    tmp_path,
):
    twenty_under_v4 = _version_three_data()
    twenty_under_v4["format"] = "canaryfield-seeds/4"
    with pytest.raises(ValueError, match="carries 24"):
        load_seed_pack(_pack_file(tmp_path, twenty_under_v4))
    current_under_v3 = seed_pack_data()
    current_under_v3["format"] = "canaryfield-seeds/3"
    with pytest.raises(ValueError, match="carries 20"):
        load_seed_pack(_pack_file(tmp_path, current_under_v3))
