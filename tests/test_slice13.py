"""Slice 13 tests. Four new seeds and a ninth seed pack format.

Every fixture here is synthetic and written for this repo. Each new
seed gets one target where a gate exercising the defect catches it
and one target where its shape is absent and its row reads not
applicable, and the scoping rules each obligation states get a
content level pin each, at the edges as well as the core. Each
caught verdict is earned on a compiling planted copy, because the
Round 11 review fixes record that a catch against a tree the
language refuses is no catch at all.
"""

import ast
import json
import shutil

import pytest

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, plant_seed, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

TAIL_SOURCE = """\
def tail(items):
    rest = items[1:]
    return rest
"""

TAIL_TEST = """\
from slicing import tail


def test_tail():
    assert tail([10, 20, 30]) == [20, 30]
"""

COLLECT_SOURCE = """\
def collect(items):
    found = []
    for item in items:
        if item is None:
            continue
        found.append(item)
    return found
"""

COLLECT_TEST = """\
from collecting import collect


def test_collect():
    assert collect([1, None, 2, None, 3]) == [1, 2, 3]
"""

COUNTER_SOURCE = """\
COUNT = 0


def bump():
    global COUNT
    COUNT += 1


def total():
    return COUNT
"""

COUNTER_TEST = """\
from counter import bump, total


def test_bump_moves_the_counter():
    bump()
    assert total() == 1
"""

ENDPOINTS_SOURCE = """\
def endpoints():
    low, high = 1, 10
    return low, high
"""

ENDPOINTS_TEST = """\
from endpoints import endpoints


def test_endpoints():
    assert endpoints() == (1, 10)
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


def _version_eight_data():
    return {
        "format": "canaryfield-seeds/8",
        "seeds": [_entry(seed) for seed in SEEDS[:40]],
    }


def _assert_caught_on_a_compiling_copy(
    tmp_path, seed_id, files, module_name
):
    target = _write_project(tmp_path / "project", files)
    planted = tmp_path / "planted"
    shutil.copytree(target, planted)
    changed = plant_seed(planted, SEEDS_BY_ID[seed_id])
    assert changed == [module_name]
    compile(
        (planted / module_name).read_text(encoding="utf-8"),
        module_name,
        "exec",
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)[seed_id] == CAUGHT


def test_seed_forty_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-40",
        {"slicing.py": TAIL_SOURCE, "test_slicing.py": TAIL_TEST},
        "slicing.py",
    )


def test_seed_forty_shifts_the_start_by_one():
    tree = ast.parse(TAIL_SOURCE)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "items[2:]" in rendered
    assert "items[1:]" not in rendered


def test_seed_forty_leaves_the_stop_and_the_step_standing():
    source = "def f(items):\n    return items[1:9:2]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is True
    rendered = ast.unparse(tree)
    subscript = ast.parse(rendered).body[0].body[0].value
    assert subscript.slice.lower.value == 2
    assert subscript.slice.upper.value == 9
    assert subscript.slice.step.value == 2


def test_seed_forty_plants_a_slice_inside_a_tuple_index():
    source = "def f(grid):\n    return grid[1:2, 0]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "grid[2:2, 0]" in rendered


def test_seed_forty_leaves_a_negative_start_standing():
    source = "def f(items):\n    return items[-1:]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_leaves_a_computed_start_standing():
    source = (
        "def f(items, n):\n"
        "    head = items[n:]\n"
        "    rest = items[n + 1:]\n"
        "    return head, rest\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_leaves_a_boolean_start_standing():
    source = "def f(items):\n    return items[True:]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_plants_the_start_standing_in_an_assignment():
    tree = ast.parse(TAIL_SOURCE)
    assert SEEDS_BY_ID["seed-03"].apply(tree) is False
    tree = ast.parse(TAIL_SOURCE)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is True
    source = "def answer():\n    return 42\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is False
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-03"].apply(tree) is True


def test_seed_forty_and_seed_twenty_nine_plant_opposite_bounds_of_one_slice():
    source = "def f(items):\n    part = items[1:3]\n    return part\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-40"].apply(tree) is True
    start_rendered = ast.unparse(tree)
    assert "items[2:3]" in start_rendered
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-29"].apply(tree) is True
    stop_rendered = ast.unparse(tree)
    assert "items[1:4]" in stop_rendered
    assert start_rendered != stop_rendered


def test_seed_forty_not_applicable_without_a_constant_start(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-40"] == NOT_APPLICABLE


def test_seed_forty_one_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-41",
        {"collecting.py": COLLECT_SOURCE, "test_collecting.py": COLLECT_TEST},
        "collecting.py",
    )


def test_seed_forty_one_turns_a_continue_into_a_break():
    tree = ast.parse(COLLECT_SOURCE)
    assert SEEDS_BY_ID["seed-41"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "break" in rendered
    assert "continue" not in rendered
    source = (
        "def drain(queue):\n"
        "    while queue:\n"
        "        item = queue.pop()\n"
        "        if item is None:\n"
        "            continue\n"
        "        print(item)\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-41"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "break" in rendered
    assert "continue" not in rendered


def test_seed_forty_one_plants_the_inner_loop_continue_on_its_own_loop():
    source = (
        "def scan(rows):\n"
        "    total = 0\n"
        "    for row in rows:\n"
        "        for cell in row:\n"
        "            if cell is None:\n"
        "                continue\n"
        "            total += cell\n"
        "    return total\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-41"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "continue" not in rendered
    assert "break" in rendered
    namespace = {}
    exec(rendered, namespace)
    assert namespace["scan"]([[1, None, 2], [3]]) == 4


def test_seed_forty_one_and_seed_twenty_three_hold_disjoint_statements():
    break_source = (
        "def f(items):\n"
        "    for item in items:\n"
        "        if item is None:\n"
        "            break\n"
        "    return items\n"
    )
    tree = ast.parse(break_source)
    assert SEEDS_BY_ID["seed-41"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(break_source))
    tree = ast.parse(break_source)
    assert SEEDS_BY_ID["seed-23"].apply(tree) is True
    tree = ast.parse(COLLECT_SOURCE)
    assert SEEDS_BY_ID["seed-23"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(COLLECT_SOURCE))


def test_seed_forty_one_plants_a_continue_in_a_loop_inside_a_nested_function():
    source = (
        "def outer(items):\n"
        "    for item in items:\n"
        "        def helper(values):\n"
        "            kept = []\n"
        "            for value in values:\n"
        "                if value is None:\n"
        "                    continue\n"
        "                kept.append(value)\n"
        "            return kept\n"
        "    return helper\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-41"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "continue" not in rendered
    assert "break" in rendered


def test_seed_forty_one_not_applicable_without_a_continue(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-41"] == NOT_APPLICABLE


def test_seed_forty_two_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-42",
        {"counter.py": COUNTER_SOURCE, "test_counter.py": COUNTER_TEST},
        "counter.py",
    )


def test_seed_forty_two_removes_a_global_declaration():
    tree = ast.parse(COUNTER_SOURCE)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "global" not in rendered
    assert "COUNT += 1" in rendered


def test_seed_forty_two_removes_a_nonlocal_declaration():
    source = (
        "def outer():\n"
        "    total = 0\n"
        "    def inner():\n"
        "        nonlocal total\n"
        "        total += 1\n"
        "    return inner\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "nonlocal" not in rendered
    assert "total += 1" in rendered


def test_seed_forty_two_removes_every_declaration_standing_in_one_body():
    source = (
        "def f():\n"
        "    global FIRST\n"
        "    global SECOND\n"
        "    return 1\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "global" not in rendered
    assert "return 1" in rendered


def test_seed_forty_two_plants_a_declaration_standing_in_a_nested_list():
    source = (
        "def f(flag):\n"
        "    if flag:\n"
        "        global NESTED\n"
        "        NESTED = 1\n"
        "    return 0\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "global" not in rendered
    assert "NESTED = 1" in rendered


def test_seed_forty_two_leaves_a_declaration_standing_alone_in_its_list():
    source = "def f():\n    global ONLY\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = (
        "def outer():\n"
        "    total = 0\n"
        "    def inner(flag):\n"
        "        if flag:\n"
        "            nonlocal total\n"
        "        return total\n"
        "    return inner\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_two_leaves_a_list_holding_only_declarations_standing():
    source = "def f():\n    global FIRST\n    global SECOND\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-42"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_two_not_applicable_without_a_declaration(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-42"] == NOT_APPLICABLE


def test_seed_forty_three_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-43",
        {"endpoints.py": ENDPOINTS_SOURCE, "test_endpoints.py": ENDPOINTS_TEST},
        "endpoints.py",
    )


def test_seed_forty_three_swaps_the_first_two_elements_and_keeps_the_rest():
    source = "def f():\n    values = (1, 2, 3)\n    return values\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "(2, 1, 3)" in rendered


def test_seed_forty_three_plants_an_annotated_assignment():
    source = "pair: tuple = (1, 2)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "(2, 1)" in rendered


def test_seed_forty_three_trades_a_starred_element_like_any_other():
    source = "def f(first, rest):\n    head, tail = (first, *rest)\n    return head\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "(*rest, first)" in rendered


def test_seed_forty_three_plants_a_chained_assignment_once():
    source = "a = b = (1, 2)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert rendered.count("(2, 1)") == 1


def test_seed_forty_three_identical_elements_exchange_into_themselves():
    source = "same = (7, 7)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_three_and_the_call_and_definition_exchanges_hold_disjoint_ground():
    tree = ast.parse(ENDPOINTS_SOURCE)
    assert SEEDS_BY_ID["seed-20"].apply(tree) is False
    tree = ast.parse(ENDPOINTS_SOURCE)
    assert SEEDS_BY_ID["seed-27"].apply(tree) is False
    tree = ast.parse(ENDPOINTS_SOURCE)
    assert SEEDS_BY_ID["seed-33"].apply(tree) is False
    tree = ast.parse(ENDPOINTS_SOURCE)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is True
    source = (
        "def f(a=1, b=2):\n"
        "    return combine(a, b, left=a, right=b)\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_three_leaves_a_single_element_tuple_standing():
    source = "pair = (1,)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "pair = 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-43"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_three_not_applicable_without_a_tuple_assignment(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-43"] == NOT_APPLICABLE


def test_version_eight_pack_still_loads_the_forty(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_eight_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:40]
    ]


def test_a_version_nine_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][43]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_eight_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_eight_data()
    data["seeds"][39] = _entry(SEEDS[40])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_nine_is_refused(
    tmp_path,
):
    forty_under_v9 = _version_eight_data()
    forty_under_v9["format"] = "canaryfield-seeds/9"
    with pytest.raises(ValueError, match="carries 44"):
        load_seed_pack(_pack_file(tmp_path, forty_under_v9))
    current_under_v8 = seed_pack_data()
    current_under_v8["format"] = "canaryfield-seeds/8"
    with pytest.raises(ValueError, match="carries 40"):
        load_seed_pack(_pack_file(tmp_path, current_under_v8))
