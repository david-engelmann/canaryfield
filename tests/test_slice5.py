"""Slice 5 tests. Four new seeds and a versioned seed pack.

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

SUB_SOURCE = """\
def difference(a, b):
    return a - b


def ratio(a, b):
    return a / b
"""

SUB_TEST = """\
from calc import difference, ratio


def test_difference():
    assert difference(10, 4) == 6


def test_ratio():
    assert ratio(12, 3) == 4
"""

COMMUTATIVE_SOURCE = """\
def combine(a, b):
    return a + b * 2
"""

COMMUTATIVE_TEST = """\
from mix import combine


def test_combine():
    assert combine(1, 2) == 5
"""

ELSE_SOURCE = """\
def label(flag):
    if flag:
        return "on"
    else:
        return "off"
"""

ELSE_TEST = """\
from label import label


def test_label():
    assert label(True) == "on"
    assert label(False) == "off"
"""

NO_ELSE_SOURCE = """\
def label(flag):
    if flag:
        return "on"
    return "off"
"""

BOOL_SOURCE = """\
def admitted(age, member):
    return age >= 18 and member
"""

BOOL_TEST = """\
from gate import admitted


def test_admitted():
    assert admitted(20, True) is True
    assert admitted(20, False) is False
    assert admitted(10, True) is False
"""

SINGLE_CONDITION_SOURCE = """\
def admitted(age):
    return age >= 18
"""

SINGLE_CONDITION_TEST = """\
from gate import admitted


def test_admitted():
    assert admitted(20) is True
    assert admitted(10) is False
"""

AWAIT_SOURCE = """\
import asyncio


async def fetch():
    return 42


async def get():
    return await fetch()


def value():
    return asyncio.run(get())
"""

AWAIT_TEST = """\
from fetch import value


def test_value():
    assert value() == 42
"""

SYNC_SOURCE = """\
def value():
    return 42
"""

SYNC_TEST = """\
from syncmod import value


def test_value():
    assert value() == 42
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


def test_seed_twelve_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"calc.py": SUB_SOURCE, "test_calc.py": SUB_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-12"] == CAUGHT


def test_seed_twelve_plants_at_two_occurrences_in_one_file():
    tree = ast.parse(SUB_SOURCE)
    assert SEEDS_BY_ID["seed-12"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return b - a" in rendered
    assert "return b / a" in rendered


def test_seed_twelve_plants_a_nested_operation_once_at_each_level():
    tree = ast.parse("def f(a, b, c):\n    return a - b - c\n")
    assert SEEDS_BY_ID["seed-12"].apply(tree) is True
    assert "c - (b - a)" in ast.unparse(tree)


def test_seed_twelve_not_applicable_without_a_noncommutative_operator(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"mix.py": COMMUTATIVE_SOURCE, "test_mix.py": COMMUTATIVE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-12"] == NOT_APPLICABLE


def test_seed_thirteen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"label.py": ELSE_SOURCE, "test_label.py": ELSE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-13"] == CAUGHT


def test_seed_thirteen_drops_an_elif_chain_with_the_else():
    source = (
        "def label(n):\n"
        "    if n == 1:\n"
        "        return 'one'\n"
        "    elif n == 2:\n"
        "        return 'two'\n"
        "    else:\n"
        "        return 'other'\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-13"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return 'one'" in rendered
    assert "return 'two'" not in rendered
    assert "return 'other'" not in rendered


def test_seed_thirteen_not_applicable_without_an_else_branch(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"label.py": NO_ELSE_SOURCE, "test_label.py": ELSE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-13"] == NOT_APPLICABLE


def test_seed_fourteen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"gate.py": BOOL_SOURCE, "test_gate.py": BOOL_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-14"] == CAUGHT


def test_seed_fourteen_not_applicable_without_a_compound_condition(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {
            "gate.py": SINGLE_CONDITION_SOURCE,
            "test_gate.py": SINGLE_CONDITION_TEST,
        },
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-14"] == NOT_APPLICABLE


def test_seed_fifteen_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"fetch.py": AWAIT_SOURCE, "test_fetch.py": AWAIT_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-15"] == CAUGHT


def test_seed_fifteen_plants_an_await_nested_inside_an_await():
    source = (
        "async def get():\n"
        "    return await fetch(await inner())\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-15"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return fetch(inner())" in rendered
    assert "await" not in rendered


def test_seed_fifteen_not_applicable_without_an_await(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"syncmod.py": SYNC_SOURCE, "test_syncmod.py": SYNC_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert _cells(result)["seed-15"] == NOT_APPLICABLE


def _pack_file(tmp_path, data):
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def _version_one_data():
    return {
        "format": "canaryfield-seeds/1",
        "seeds": [
            {
                "id": seed.id,
                "name": seed.name,
                "defect_class": seed.defect_class,
                "description": seed.description,
            }
            for seed in SEEDS[:12]
        ],
    }


def test_version_one_pack_still_loads_the_original_twelve(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_one_data()))
    assert [entry["id"] for entry in entries] == [
        f"seed-{index:02d}" for index in range(12)
    ]
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:12]
    ]


def test_a_pack_whose_count_disagrees_with_its_version_is_refused(
    tmp_path,
):
    current_under_v1 = seed_pack_data()
    current_under_v1["format"] = "canaryfield-seeds/1"
    with pytest.raises(ValueError, match="carries 12"):
        load_seed_pack(_pack_file(tmp_path, current_under_v1))
    twelve_under_v2 = _version_one_data()
    twelve_under_v2["format"] = "canaryfield-seeds/2"
    with pytest.raises(ValueError, match="carries 16"):
        load_seed_pack(_pack_file(tmp_path, twelve_under_v2))
