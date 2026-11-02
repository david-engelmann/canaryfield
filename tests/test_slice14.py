"""Slice 14 tests. Four new seeds and a tenth seed pack format.

Every fixture here is synthetic and written for this repo. Each new
seed gets one target where a gate exercising the defect catches it
and one target where its shape is absent and its row reads not
applicable, and the scoping rules each obligation states get a
content level pin each, at the edges as well as the core. Each
caught verdict is earned on a compiling planted copy, because the
Round 11 review fixes record that a catch against a tree the
language refuses is no catch at all. The two seeds whose removal
can strand a nonlocal declaration are also pinned at the engine
write path, where a planting the language refuses is never written.
"""

import ast
import json
import shutil

import pytest

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, plant_seed, run_field
from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

SEARCH_SOURCE = """\
def first_big(items):
    for item in items:
        if item > 10:
            break
    else:
        return "none"
    return item
"""

SEARCH_TEST = """\
from searching import first_big


def test_first_big():
    assert first_big([1, 2]) == "none"
    assert first_big([20, 30]) == 20
"""

GREET_SOURCE = """\
def greet(name, punct="!"):
    return name + punct


def banner():
    return greet("hi", punct="?")
"""

GREET_TEST = """\
from greeting import banner


def test_banner():
    assert banner() == "hi?"
"""

EVENS_SOURCE = """\
def evens():
    return list(range(0, 10, 2))
"""

EVENS_TEST = """\
from evens import evens


def test_evens():
    assert evens() == [0, 2, 4, 6, 8]
"""

BLOCK_SOURCE = """\
def blocked(flag):
    return not flag
"""

BLOCK_TEST = """\
from blocking import blocked


def test_blocked():
    assert blocked(False) is True
    assert blocked(True) is False
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

LOOP_ELSE_BINDING_SOURCE = """\
def outer():
    def inner():
        nonlocal x
        return x
    for i in range(3):
        pass
    else:
        x = 1
    return inner
"""

KEYWORD_BINDING_SOURCE = """\
def outer():
    def inner():
        nonlocal x
        return x
    return combine(value=(x := 3)), inner
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


def _version_nine_data():
    return {
        "format": "canaryfield-seeds/9",
        "seeds": [_entry(seed) for seed in SEEDS[:44]],
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


def test_seed_forty_four_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-44",
        {"searching.py": SEARCH_SOURCE, "test_searching.py": SEARCH_TEST},
        "searching.py",
    )


def test_seed_forty_four_drops_the_else_clause_of_a_for_loop():
    tree = ast.parse(SEARCH_SOURCE)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "else" not in rendered
    assert "break" in rendered
    namespace = {}
    exec(rendered, namespace)
    assert namespace["first_big"]([1, 2]) == 2
    assert namespace["first_big"]([20, 30]) == 20


def test_seed_forty_four_drops_the_else_clause_of_a_while_loop():
    source = (
        "def drain(queue):\n"
        "    found = []\n"
        "    while queue:\n"
        "        item = queue.pop()\n"
        "        if item is None:\n"
        "            break\n"
        "        found.append(item)\n"
        "    else:\n"
        "        return found, 'drained'\n"
        "    return found, 'stopped'\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "else" not in rendered
    namespace = {}
    exec(rendered, namespace)
    assert namespace["drain"]([1, 2]) == ([2, 1], "stopped")


def test_seed_forty_four_drops_the_else_clause_of_an_async_for_loop():
    source = (
        "async def collect(stream):\n"
        "    found = []\n"
        "    async for item in stream:\n"
        "        found.append(item)\n"
        "    else:\n"
        "        found.append('done')\n"
        "    return found\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "else" not in rendered
    assert "async for" in rendered


def test_seed_forty_four_plants_every_loop_else_in_one_file():
    source = (
        "def scan(items, queue):\n"
        "    for item in items:\n"
        "        pass\n"
        "    else:\n"
        "        first = 1\n"
        "    while queue:\n"
        "        break\n"
        "    else:\n"
        "        second = 2\n"
        "    return first, second\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "else" not in rendered


def test_seed_forty_four_shows_the_outer_planting_where_an_else_clause_holds_a_loop():
    source = (
        "def scan(rows):\n"
        "    for row in rows:\n"
        "        pass\n"
        "    else:\n"
        "        while rows:\n"
        "            break\n"
        "        else:\n"
        "            marker = 1\n"
        "    return rows\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "while" not in rendered
    assert "marker" not in rendered
    assert "else" not in rendered


def test_seed_forty_four_plants_a_loop_else_standing_inside_a_try_finally():
    source = (
        "def search(items, log):\n"
        "    try:\n"
        "        for item in items:\n"
        "            if item is None:\n"
        "                break\n"
        "        else:\n"
        "            log.append('exhausted')\n"
        "    finally:\n"
        "        log.append('closed')\n"
        "    return log\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "else" not in rendered
    assert "finally" in rendered
    namespace = {}
    exec(rendered, namespace)
    log = []
    namespace["search"]([1], log)
    assert log == ["closed"]


def test_seed_forty_four_planting_that_would_break_a_nonlocal_binding_is_never_written(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project", {"binding.py": LOOP_ELSE_BINDING_SOURCE}
    )
    planted = tmp_path / "planted"
    shutil.copytree(target, planted)
    assert plant_seed(planted, SEEDS_BY_ID["seed-44"]) == []
    assert (
        planted / "binding.py"
    ).read_text(encoding="utf-8") == LOOP_ELSE_BINDING_SOURCE


def test_seed_forty_four_leaves_a_loop_without_an_else_clause_standing():
    source = (
        "def f(items):\n"
        "    for item in items:\n"
        "        pass\n"
        "    while items:\n"
        "        break\n"
        "    return items\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-44"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_four_not_applicable_without_a_loop_else(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-44"] == NOT_APPLICABLE


def test_seed_forty_five_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-45",
        {"greeting.py": GREET_SOURCE, "test_greeting.py": GREET_TEST},
        "greeting.py",
    )


def test_seed_forty_five_drops_the_first_keyword_argument_and_keeps_the_rest():
    source = "def f():\n    return combine(1, left=2, right=3)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "combine(1, right=3)" in rendered
    assert "left" not in rendered


def test_seed_forty_five_keeps_the_positional_arguments_standing():
    source = "def f():\n    return combine(1, 2, left=3)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    rendered = ast.unparse(tree)
    call = ast.parse(rendered).body[0].body[0].value
    assert [arg.value for arg in call.args] == [1, 2]
    assert call.keywords == []


def test_seed_forty_five_drops_a_named_argument_standing_after_an_unnamed_one():
    source = "def f(opts):\n    return greet('hi', **opts, punct='?')\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "punct" not in rendered
    call = ast.parse(rendered).body[0].body[0].value
    assert len(call.keywords) == 1
    assert call.keywords[0].arg is None


def test_seed_forty_five_leaves_a_call_carrying_only_an_unnamed_keyword_standing():
    source = "def f(opts):\n    return greet('hi', **opts)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_five_plants_every_call_in_one_file():
    source = (
        "def f():\n"
        "    first = combine(left=1, right=2)\n"
        "    second = combine(top=3, bottom=4)\n"
        "    return first, second\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "combine(right=2)" in rendered
    assert "combine(bottom=4)" in rendered


def test_seed_forty_five_shows_the_outer_planting_where_a_keyword_value_holds_a_call():
    source = "def f():\n    return combine(left=inner(key=1), right=2)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "combine(right=2)" in rendered
    assert "inner" not in rendered


def test_seed_forty_five_planting_that_would_break_a_nonlocal_binding_is_never_written(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project", {"binding.py": KEYWORD_BINDING_SOURCE}
    )
    planted = tmp_path / "planted"
    shutil.copytree(target, planted)
    assert plant_seed(planted, SEEDS_BY_ID["seed-45"]) == []
    assert (
        planted / "binding.py"
    ).read_text(encoding="utf-8") == KEYWORD_BINDING_SOURCE


def test_seed_forty_five_and_the_other_call_argument_seeds_hold_disjoint_ground():
    source = "def f():\n    return combine(1, left=2, right=3)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-07"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-20"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-27"].apply(tree) is True
    swapped = ast.unparse(tree)
    assert "combine(1, left=3, right=2)" in swapped
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is True
    dropped = ast.unparse(tree)
    assert "combine(1, right=3)" in dropped
    assert swapped != dropped
    plain = "def f():\n    return combine(1, 2)\n"
    tree = ast.parse(plain)
    assert SEEDS_BY_ID["seed-45"].apply(tree) is False
    tree = ast.parse(plain)
    assert SEEDS_BY_ID["seed-07"].apply(tree) is True


def test_seed_forty_five_not_applicable_without_a_keyword_argument(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-45"] == NOT_APPLICABLE


def test_seed_forty_six_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-46",
        {"evens.py": EVENS_SOURCE, "test_evens.py": EVENS_TEST},
        "evens.py",
    )


def test_seed_forty_six_negates_a_constant_step():
    tree = ast.parse(EVENS_SOURCE)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "range(0, 10, -2)" in rendered
    namespace = {}
    exec(rendered, namespace)
    assert namespace["evens"]() == []


def test_seed_forty_six_negates_a_computed_step():
    source = "def f(step):\n    return list(range(0, 10, step))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    call = ast.parse(rendered).body[0].body[0].value.args[0]
    step = call.args[2]
    assert isinstance(step, ast.UnaryOp)
    assert isinstance(step.op, ast.USub)
    assert isinstance(step.operand, ast.Name)
    source = "def f(k):\n    return list(range(0, 10, k + 1))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "-(k + 1)" in rendered


def test_seed_forty_six_plants_every_range_call_in_one_file():
    source = (
        "def f():\n"
        "    first = list(range(0, 10, 2))\n"
        "    second = list(range(0, 20, 5))\n"
        "    return first, second\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "range(0, 10, -2)" in rendered
    assert "range(0, 20, -5)" in rendered


def test_seed_forty_six_and_seed_ten_plant_the_same_step_differently():
    tree = ast.parse(EVENS_SOURCE)
    assert SEEDS_BY_ID["seed-10"].apply(tree) is True
    raised = ast.unparse(tree)
    assert "range(0, 10, 2 + 1)" in raised
    tree = ast.parse(EVENS_SOURCE)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is True
    negated = ast.unparse(tree)
    assert "range(0, 10, -2)" in negated
    assert raised != negated


def test_seed_forty_six_leaves_a_two_argument_range_standing():
    source = "def f():\n    return list(range(1, 5))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-10"].apply(tree) is True


def test_seed_forty_six_leaves_a_step_carrying_a_written_minus_standing():
    source = "def f():\n    return list(range(10, 0, -1))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-22"].apply(tree) is True
    assert "range(10, 0, 1)" in ast.unparse(tree)


def test_seed_forty_six_leaves_a_zero_step_standing():
    source = "def f():\n    return list(range(0, 10, 0))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_six_leaves_a_boolean_step_standing():
    source = "def f():\n    return list(range(0, 10, True))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_six_leaves_a_starred_step_standing():
    source = "def f(steps):\n    return list(range(0, 10, *steps))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-46"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_six_not_applicable_without_a_three_argument_range(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-46"] == NOT_APPLICABLE


def test_seed_forty_seven_plants_and_is_caught(tmp_path):
    _assert_caught_on_a_compiling_copy(
        tmp_path,
        "seed-47",
        {"blocking.py": BLOCK_SOURCE, "test_blocking.py": BLOCK_TEST},
        "blocking.py",
    )


def test_seed_forty_seven_removes_a_negation_from_a_return_value():
    tree = ast.parse(BLOCK_SOURCE)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "return flag" in rendered
    assert "not" not in rendered
    namespace = {}
    exec(rendered, namespace)
    assert namespace["blocked"](False) is False


def test_seed_forty_seven_removes_a_negation_standing_as_a_term_of_a_value():
    source = "def f(a, b):\n    return a and not b\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "a and b" in rendered
    assert "not" not in rendered


def test_seed_forty_seven_removes_a_negation_from_the_test_of_an_assert():
    source = "def f(a):\n    assert not a\n    return 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "assert a" in rendered
    assert "not" not in rendered


def test_seed_forty_seven_removes_a_negation_from_a_comprehension_condition():
    source = (
        "def f(items):\n"
        "    return [item for item in items if not item.hidden]\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "if item.hidden" in rendered
    assert "not" not in rendered


def test_seed_forty_seven_plants_a_negation_in_the_body_of_a_conditional_expression():
    source = "def f(a, b):\n    return not a if b else a\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "a if b else a" in rendered
    source = "def f(a, b):\n    return a if b else not a\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "a if b else a" in rendered


def test_seed_forty_seven_drops_a_doubled_negation_at_both_levels():
    source = "def f(a):\n    return not (not a)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "return a" in rendered
    assert "not" not in rendered


def test_seed_forty_seven_leaves_a_whole_test_negation_standing():
    sources = (
        "def f(a):\n    if not a:\n        return 1\n    return 0\n",
        "def f(a):\n    while not a:\n        break\n    return 0\n",
        "def f(a):\n    return 1 if not a else 2\n",
    )
    for source in sources:
        tree = ast.parse(source)
        assert SEEDS_BY_ID["seed-47"].apply(tree) is False
        assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_seven_leaves_a_negation_inside_a_test_standing():
    sources = (
        "def f(a, b):\n    if a and not b:\n        return 1\n    return 0\n",
        "def f(a, b):\n    while a and not b:\n        break\n    return 0\n",
        "def f(a):\n    if (lambda: not a)():\n        return 1\n    return 0\n",
    )
    for source in sources:
        tree = ast.parse(source)
        assert SEEDS_BY_ID["seed-47"].apply(tree) is False
        assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_seven_and_seed_eighteen_hold_disjoint_negations():
    source = (
        "def f(a, b):\n"
        "    if not a:\n"
        "        return 1\n"
        "    return not b\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-18"].apply(tree) is True
    toggled = ast.unparse(tree)
    assert "if a:" in toggled
    assert "return not b" in toggled
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-47"].apply(tree) is True
    dropped = ast.unparse(tree)
    assert "if not a:" in dropped
    assert "return b" in dropped
    assert toggled != dropped


def test_seed_forty_seven_not_applicable_without_a_negation_outside_a_test(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-47"] == NOT_APPLICABLE


def test_version_nine_pack_still_loads_the_forty_four(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_nine_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:44]
    ]


def test_a_version_ten_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][47]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_nine_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_nine_data()
    data["seeds"][43] = _entry(SEEDS[44])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_ten_is_refused(
    tmp_path,
):
    forty_four_under_v10 = _version_nine_data()
    forty_four_under_v10["format"] = "canaryfield-seeds/10"
    with pytest.raises(ValueError, match="carries 48"):
        load_seed_pack(_pack_file(tmp_path, forty_four_under_v10))
    current_under_v9 = seed_pack_data()
    current_under_v9["format"] = "canaryfield-seeds/9"
    with pytest.raises(ValueError, match="carries 44"):
        load_seed_pack(_pack_file(tmp_path, current_under_v9))
