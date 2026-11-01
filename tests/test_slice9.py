"""Slice 9 tests. Four new seeds and a fifth seed pack format.

Every fixture here is synthetic and written for this repo. Each new
seed gets one target where a gate exercising the defect catches it
and one target where its shape is absent and its row reads not
applicable, and the scoping rules each obligation states get a
content level pin each, at the edges as well as the core.
"""

import ast
import json

import pytest

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

ACCUMULATE_SOURCE = """\
def total(values):
    result = 0
    for value in values:
        result += value
    return result
"""

ACCUMULATE_TEST = """\
from accumulating import total


def test_total():
    assert total([1, 2, 3]) == 6
    assert total([]) == 0
"""

SENTINEL_SOURCE = """\
TOKEN = ["key"]


def is_token(value):
    return value is TOKEN
"""

SENTINEL_TEST = """\
from sentinel import TOKEN, is_token


def test_is_token():
    assert is_token(TOKEN) is True
    assert is_token(["key"]) is False
"""

FIRST_SOURCE = """\
def first(items):
    return items[0]
"""

FIRST_TEST = """\
from indexing import first


def test_first():
    assert first(["a", "b"]) == "a"
"""

BOX_SOURCE = """\
def box(width, height):
    return (width, height)


def unit():
    return box(width=1, height=2)
"""

BOX_TEST = """\
from framing import unit


def test_unit():
    assert unit() == (1, 2)
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


def _write_project(root, files):
    root.mkdir(parents=True, exist_ok=True)
    for name, text in files.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


def _cells(result):
    return {row["seed"]: row["gates"]["tests"] for row in result["rows"]}


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


def _version_four_data():
    return {
        "format": "canaryfield-seeds/4",
        "seeds": [_entry(seed) for seed in SEEDS[:24]],
    }


def test_seed_twenty_four_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"accumulating.py": ACCUMULATE_SOURCE, "test_accumulating.py": ACCUMULATE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-24"] == CAUGHT


def test_seed_twenty_four_swaps_addition_for_subtraction():
    tree = ast.parse("total += value\n")
    assert SEEDS_BY_ID["seed-24"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "total -= value" in rendered


def test_seed_twenty_four_swaps_subtraction_and_multiplication_for_addition():
    tree = ast.parse("total -= value\n")
    assert SEEDS_BY_ID["seed-24"].apply(tree) is True
    assert "total += value" in ast.unparse(tree)
    tree = ast.parse("total *= value\n")
    assert SEEDS_BY_ID["seed-24"].apply(tree) is True
    assert "total += value" in ast.unparse(tree)


def test_seed_twenty_four_plants_every_match_in_one_file():
    source = (
        "def adjust(total):\n"
        "    total += 1\n"
        "    total -= 2\n"
        "    return total\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-24"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "total -= 1" in rendered
    assert "total += 2" in rendered


def test_seed_twenty_four_plants_onto_a_subscript_target():
    tree = ast.parse("counts[0] += 1\n")
    assert SEEDS_BY_ID["seed-24"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "counts[0] -= 1" in rendered


def test_seed_twenty_four_leaves_an_out_of_table_operator_standing():
    source = "total /= 2\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-24"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "total //= 2\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-24"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_four_not_applicable_without_an_augmented_assignment(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-24"] == NOT_APPLICABLE


def test_seed_twenty_five_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"sentinel.py": SENTINEL_SOURCE, "test_sentinel.py": SENTINEL_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-25"] == CAUGHT


def test_seed_twenty_five_turns_is_into_equality():
    tree = ast.parse("def check(a, b):\n    return a is b\n")
    assert SEEDS_BY_ID["seed-25"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "a == b" in rendered


def test_seed_twenty_five_turns_is_not_into_inequality():
    tree = ast.parse("def check(a, b):\n    return a is not b\n")
    assert SEEDS_BY_ID["seed-25"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "a != b" in rendered


def test_seed_twenty_five_exchanges_only_the_first_operator_of_a_chain():
    tree = ast.parse("def check(a, b, c):\n    return a is b is c\n")
    assert SEEDS_BY_ID["seed-25"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "a == b is c" in rendered


def test_seed_twenty_five_leaves_a_chain_led_by_equality_standing():
    source = "def check(a, b, c):\n    return a == b is c\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-25"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_five_not_applicable_without_an_identity_test(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-25"] == NOT_APPLICABLE


def test_seed_twenty_six_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"indexing.py": FIRST_SOURCE, "test_indexing.py": FIRST_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-26"] == CAUGHT


def test_seed_twenty_six_shifts_the_index_by_one():
    tree = ast.parse("def first(items):\n    return items[0]\n")
    assert SEEDS_BY_ID["seed-26"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "items[1]" in rendered


def test_seed_twenty_six_plants_each_level_of_a_nested_subscript():
    tree = ast.parse("def cell(grid):\n    return grid[0][1]\n")
    assert SEEDS_BY_ID["seed-26"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "grid[1][2]" in rendered


def test_seed_twenty_six_leaves_a_negative_index_standing():
    source = "def last(items):\n    return items[-1]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-26"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_six_leaves_a_slice_and_a_tuple_index_standing():
    source = "def part(items):\n    return items[1:2]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-26"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "def pair(items):\n    return items[0, 1]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-26"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_six_leaves_a_boolean_index_standing():
    source = "def pick(items):\n    return items[True]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-26"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_six_leaves_a_name_index_standing():
    source = "def pick(items, index):\n    return items[index]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-26"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_six_not_applicable_without_a_constant_index(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-26"] == NOT_APPLICABLE


def test_seed_twenty_seven_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"framing.py": BOX_SOURCE, "test_framing.py": BOX_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-27"] == CAUGHT


def test_seed_twenty_seven_swaps_the_first_two_keyword_values():
    tree = ast.parse("result = frame(width=1, height=2)\n")
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "frame(width=2, height=1)" in rendered
    tree = ast.parse("result = frame(width=1, height=2, depth=3)\n")
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "frame(width=2, height=1, depth=3)" in rendered


def test_seed_twenty_seven_keeps_a_double_starred_argument_standing_between_them():
    tree = ast.parse("result = frame(a=1, **spread, b=2)\n")
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "frame(a=2, **spread, b=1)" in rendered


def test_seed_twenty_seven_identical_values_exchange_into_themselves():
    source = "result = frame(width=1, height=1)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_seven_plants_a_nested_call_at_its_own_level():
    tree = ast.parse("result = outer(a=inner(x=1, y=2), b=3)\n")
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "outer(a=3, b=inner(x=2, y=1))" in rendered


def test_seed_twenty_seven_leaves_a_single_named_keyword_standing():
    source = "result = frame(a=1, **spread)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-27"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_seven_not_applicable_without_two_named_keywords(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-27"] == NOT_APPLICABLE


def test_version_four_pack_still_loads_the_twenty_four(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_four_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:24]
    ]


def test_a_version_five_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][27]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_four_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_four_data()
    data["seeds"][23] = _entry(SEEDS[24])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_five_is_refused(
    tmp_path,
):
    twenty_four_under_v5 = _version_four_data()
    twenty_four_under_v5["format"] = "canaryfield-seeds/5"
    with pytest.raises(ValueError, match="carries 28"):
        load_seed_pack(_pack_file(tmp_path, twenty_four_under_v5))
    current_under_v4 = seed_pack_data()
    current_under_v4["format"] = "canaryfield-seeds/4"
    with pytest.raises(ValueError, match="carries 24"):
        load_seed_pack(_pack_file(tmp_path, current_under_v4))
