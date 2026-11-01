"""Slice 7 tests. Four new seeds and a third seed pack format.

Every fixture here is synthetic and written for this repo. Each new
seed gets one target where a gate exercising the defect catches it
and one target where its shape is absent and its row reads not
applicable.
"""

import ast
import json

import pytest

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

PARSE_SOURCE = """\
def parse(text):
    try:
        return int(text)
    except ValueError:
        return -1
"""

PARSE_TEST = """\
import pytest

from parsing import parse


def test_parse():
    assert parse("5") == 5
    assert parse("x") == -1
    with pytest.raises(TypeError):
        parse(None)
"""

BROAD_SOURCE = """\
def value():
    try:
        return int("3")
    except Exception:
        return -1


def other():
    try:
        return int("4")
    except:
        return -1
"""

BROAD_TEST = """\
from broad import other, value


def test_value():
    assert value() == 3


def test_other():
    assert other() == 4
"""

PRICES_SOURCE = """\
PRICES = {"apple": 3, "pear": 5}


def price(item):
    return PRICES[item]
"""

PRICES_TEST = """\
from prices import price


def test_price():
    assert price("apple") == 3
    assert price("pear") == 5
"""

LIST_SOURCE = """\
def toppings():
    return ["soup", "salad"]


def first(items):
    return items[0]
"""

LIST_TEST = """\
from listing import first, toppings


def test_toppings():
    assert toppings() == ["soup", "salad"]


def test_first():
    assert first([9, 8]) == 9
"""

LOOP_SOURCE = """\
def count_down(n):
    total = 0
    while n > 0:
        total += n
        n -= 1
    return total
"""

LOOP_TEST = """\
from counting import count_down


def test_count_down():
    assert count_down(3) == 6
    assert count_down(0) == 0
"""

GUARD_SOURCE = """\
def first_or_none(items):
    if not items:
        return None
    return items[0]
"""

GUARD_TEST = """\
from guard import first_or_none


def test_first_or_none():
    assert first_or_none([]) is None
    assert first_or_none([7]) == 7
"""

PLAIN_IF_SOURCE = """\
def describe(flag):
    if flag:
        return "on"
    return "off"
"""

PLAIN_IF_TEST = """\
from plain import describe


def test_describe():
    assert describe(True) == "on"
    assert describe(False) == "off"
"""

TOTAL_SOURCE = """\
def total(values):
    result = 0
    for value in values:
        result += value
    return result
"""

TOTAL_TEST = """\
from totalling import total


def test_total():
    assert total([1, 2, 3]) == 6
    assert total([]) == 0
"""

LONE_LOOP_SOURCE = """\
for item in [1, 2]:
    print(item)
"""

LONE_LOOP_TEST = """\
import loopy


def test_module_ran():
    assert loopy is not None
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


def test_seed_sixteen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"parsing.py": PARSE_SOURCE, "test_parsing.py": PARSE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-16"] == CAUGHT


def test_seed_sixteen_widens_the_type_and_keeps_the_body():
    source = (
        "def safe_div(a, b):\n"
        "    try:\n"
        "        return a / b\n"
        "    except ZeroDivisionError:\n"
        "        raise ValueError('bad division')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-16"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except Exception" in rendered
    assert "raise ValueError('bad division')" in rendered
    assert "ZeroDivisionError" not in rendered


def test_seed_sixteen_not_applicable_without_a_typed_handler(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"broad.py": BROAD_SOURCE, "test_broad.py": BROAD_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-16"] == NOT_APPLICABLE


def test_seed_seventeen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"prices.py": PRICES_SOURCE, "test_prices.py": PRICES_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-17"] == CAUGHT


def test_seed_seventeen_swaps_every_keyed_entry():
    tree = ast.parse('DATA = {"a": 1, "b": {"c": 2}}\n')
    assert SEEDS_BY_ID["seed-17"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "1: 'a'" in rendered
    assert "2: 'c'" in rendered
    assert "'a': 1" not in rendered


def test_seed_seventeen_leaves_an_unpacking_entry_standing():
    source = 'BASE = {"a": 1}\nMERGED = {**BASE, "x": 9}\n'
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-17"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "**BASE" in rendered
    assert "9: 'x'" in rendered
    assert "'x': 9" not in rendered


def test_seed_seventeen_not_applicable_without_a_dictionary(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"listing.py": LIST_SOURCE, "test_listing.py": LIST_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-17"] == NOT_APPLICABLE


def test_seed_eighteen_plants_and_is_caught_on_a_loop(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"counting.py": LOOP_SOURCE, "test_counting.py": LOOP_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-18"] == CAUGHT


def test_seed_eighteen_plants_and_is_caught_on_a_negated_guard(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"guard.py": GUARD_SOURCE, "test_guard.py": GUARD_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-18"] == CAUGHT


def test_seed_eighteen_not_applicable_without_its_shapes(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"plain.py": PLAIN_IF_SOURCE, "test_plain.py": PLAIN_IF_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-18"] == NOT_APPLICABLE


def test_seed_nineteen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"totalling.py": TOTAL_SOURCE, "test_totalling.py": TOTAL_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-19"] == CAUGHT


def test_seed_nineteen_points_the_loop_at_a_body_name():
    source = "count = 0\nfor entry in ENTRIES:\n    count += 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-19"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "for count in ENTRIES" in rendered
    assert "for entry in ENTRIES" not in rendered


def test_seed_nineteen_not_applicable_when_no_other_name_is_bound(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"loopy.py": LONE_LOOP_SOURCE, "test_loopy.py": LONE_LOOP_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-19"] == NOT_APPLICABLE


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


def _version_two_data():
    return {
        "format": "canaryfield-seeds/2",
        "seeds": [_entry(seed) for seed in SEEDS[:16]],
    }


def test_version_two_pack_still_loads_the_sixteen(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_two_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:16]
    ]


def test_a_version_three_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][19]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_two_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_two_data()
    data["seeds"][15] = _entry(SEEDS[16])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_three_is_refused(
    tmp_path,
):
    sixteen_under_v3 = _version_two_data()
    sixteen_under_v3["format"] = "canaryfield-seeds/3"
    with pytest.raises(ValueError, match="carries 20"):
        load_seed_pack(_pack_file(tmp_path, sixteen_under_v3))
    twenty_under_v2 = seed_pack_data()
    twenty_under_v2["format"] = "canaryfield-seeds/2"
    with pytest.raises(ValueError, match="carries 16"):
        load_seed_pack(_pack_file(tmp_path, twenty_under_v2))
