"""Slice 10 tests. Four new seeds and a sixth seed pack format.

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

RAISING_SOURCE = """\
def divide(a, b):
    try:
        return a / b
    except ZeroDivisionError:
        raise ValueError("bad division")
"""

RAISING_TEST = """\
import pytest

from raising import divide


def test_divide():
    assert divide(6, 3) == 2
    with pytest.raises(ValueError):
        divide(1, 0)
"""

HEAD_SOURCE = """\
def head(items):
    return items[:2]
"""

HEAD_TEST = """\
from slicing import head


def test_head():
    assert head(["a", "b", "c"]) == ["a", "b"]
    assert head([1]) == [1]
"""

PAGE_SOURCE = """\
def page(items, size=2):
    return items[:size]
"""

PAGE_TEST = """\
from paging import page


def test_page():
    assert page([1, 2, 3, 4]) == [1, 2]
    assert page([1, 2, 3, 4], 3) == [1, 2, 3]
"""

BASKET_SOURCE = """\
class Basket:
    def __init__(self, items, limit):
        self.items = items
        self.limit = limit

    def summary(self):
        return (len(self.items), self.limit)
"""

BASKET_TEST = """\
from basket import Basket


def test_summary():
    assert Basket(["a", "b"], 5).summary() == (2, 5)
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


def _version_five_data():
    return {
        "format": "canaryfield-seeds/5",
        "seeds": [_entry(seed) for seed in SEEDS[:28]],
    }


def test_seed_twenty_eight_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"raising.py": RAISING_SOURCE, "test_raising.py": RAISING_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-28"] == CAUGHT


def test_seed_twenty_eight_swaps_the_raised_type_for_the_first_other_name():
    source = (
        "def divide(a, b):\n"
        "    try:\n"
        "        return a / b\n"
        "    except ZeroDivisionError:\n"
        "        raise ValueError('bad division')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "raise ZeroDivisionError('bad division')" in rendered


def test_seed_twenty_eight_plants_a_raise_toward_the_head_of_the_order():
    source = (
        "def first():\n"
        "    raise TypeError('one')\n"
        "\n"
        "\n"
        "def second():\n"
        "    raise ValueError('two')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "raise ValueError('one')" in rendered
    assert "raise TypeError('two')" in rendered


def test_seed_twenty_eight_reads_a_handler_name_as_a_replacement():
    source = (
        "def convert(text):\n"
        "    try:\n"
        "        return int(text)\n"
        "    except TypeError:\n"
        "        return 0\n"
        "\n"
        "\n"
        "def require(value):\n"
        "    if value is None:\n"
        "        raise ValueError('missing')\n"
        "    return value\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "raise TypeError('missing')" in rendered


def test_seed_twenty_eight_reads_tuple_handler_names_in_written_order():
    source = (
        "def lookup(table, key):\n"
        "    try:\n"
        "        return table[key]\n"
        "    except (KeyError, TypeError):\n"
        "        raise ValueError('missing')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "raise KeyError('missing')" in rendered


def test_seed_twenty_eight_never_plants_when_the_module_names_only_the_raised_type():
    source = "def fail():\n    raise ValueError('only')\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_eight_leaves_a_bare_raise_and_a_bare_name_raise_standing():
    source = (
        "def rerun(task):\n"
        "    try:\n"
        "        return task()\n"
        "    except ValueError:\n"
        "        raise\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "def fail(error):\n    raise error\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_eight_leaves_a_raise_of_an_unnamed_call_standing():
    source = (
        "def fail():\n"
        "    try:\n"
        "        return run()\n"
        "    except ValueError:\n"
        "        raise make_error()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = (
        "def fail():\n"
        "    try:\n"
        "        return run()\n"
        "    except ValueError:\n"
        "        raise errors.BadInput('x')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_eight_keeps_the_arguments_and_the_cause_standing():
    source = (
        "def wrap(error):\n"
        "    try:\n"
        "        return run()\n"
        "    except TypeError:\n"
        "        raise ValueError('bad input') from error\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-28"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "raise TypeError('bad input') from error" in rendered


def test_seed_twenty_eight_not_applicable_without_a_second_named_type(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-28"] == NOT_APPLICABLE


def test_seed_twenty_nine_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"slicing.py": HEAD_SOURCE, "test_slicing.py": HEAD_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-29"] == CAUGHT


def test_seed_twenty_nine_shifts_the_stop_by_one():
    tree = ast.parse("first = items[:2]\n")
    assert SEEDS_BY_ID["seed-29"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "items[:3]" in rendered


def test_seed_twenty_nine_leaves_the_start_and_the_step_standing():
    tree = ast.parse("part = items[1:3:2]\n")
    assert SEEDS_BY_ID["seed-29"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "items[1:4:2]" in rendered


def test_seed_twenty_nine_plants_a_slice_inside_a_tuple_index():
    tree = ast.parse("cell = items[0:2, 1]\n")
    assert SEEDS_BY_ID["seed-29"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "items[0:3, 1]" in rendered


def test_seed_twenty_nine_leaves_a_negative_stop_standing():
    source = "rest = items[:-1]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-29"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_nine_leaves_a_computed_stop_standing():
    source = "part = items[:limit]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-29"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "part = items[:count + 1]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-29"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_nine_leaves_a_boolean_stop_standing():
    source = "part = items[:True]\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-29"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_twenty_nine_not_applicable_without_a_constant_stop(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-29"] == NOT_APPLICABLE


def test_seed_thirty_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"paging.py": PAGE_SOURCE, "test_paging.py": PAGE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-30"] == CAUGHT


def test_seed_thirty_shifts_a_positional_default_by_one():
    tree = ast.parse("def take(limit=10):\n    return limit\n")
    assert SEEDS_BY_ID["seed-30"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def take(limit=11)" in rendered


def test_seed_thirty_shifts_a_keyword_only_default():
    tree = ast.parse("def take(item, *, key, size=3):\n    return size\n")
    assert SEEDS_BY_ID["seed-30"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def take(item, *, key, size=4)" in rendered


def test_seed_thirty_plants_a_lambda_default():
    tree = ast.parse("take = lambda count=4: count\n")
    assert SEEDS_BY_ID["seed-30"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "lambda count=5" in rendered


def test_seed_thirty_leaves_a_boolean_default_standing():
    source = "def take(flag=True):\n    return flag\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-30"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_leaves_a_negative_default_standing():
    source = "def take(delta=-2):\n    return delta\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-30"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_leaves_a_string_default_standing():
    source = "def take(label='x'):\n    return label\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-30"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_not_applicable_without_a_numeric_default(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-30"] == NOT_APPLICABLE


def test_seed_thirty_one_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"basket.py": BASKET_SOURCE, "test_basket.py": BASKET_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-31"] == CAUGHT


def test_seed_thirty_one_swaps_the_first_loaded_attribute_for_the_second():
    tree = ast.parse(BASKET_SOURCE)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "len(self.limit)" in rendered
    assert "self.items = items" in rendered


def test_seed_thirty_one_plants_in_a_module_scope_bound_once():
    source = "config = build()\nfirst = config.path\nsecond = config.mode\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "first = config.mode" in rendered
    assert "second = config.mode" in rendered


def test_seed_thirty_one_leaves_a_rebound_base_standing():
    source = (
        "config = build()\n"
        "config = other()\n"
        "first = config.path\n"
        "second = config.mode\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_leaves_a_rebound_parameter_standing():
    source = (
        "def render(box):\n"
        "    box = normalize(box)\n"
        "    return box.width + box.height\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_leaves_a_single_attribute_base_standing():
    source = "def total(data):\n    return data.total\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_never_plants_an_attribute_store():
    source = (
        "class Meter:\n"
        "    def fill(self, value):\n"
        "        self.items = value\n"
        "        return len(self.items) + self.limit\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "self.items = value" in rendered
    assert "len(self.limit)" in rendered
    source = (
        "class Meter:\n"
        "    def fill(self, value):\n"
        "        self.items = value\n"
        "        self.limit = value\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_judges_a_nested_scope_on_its_own_statements():
    source = (
        "def outer(box):\n"
        "    def inner():\n"
        "        return box.width\n"
        "    return box.width + box.height\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return box.width\n" in rendered
    assert "box.height + box.height" in rendered


def test_seed_thirty_one_not_applicable_without_two_attributes_on_one_base(
    tmp_path,
):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-31"] == NOT_APPLICABLE


def test_version_five_pack_still_loads_the_twenty_eight(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_five_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:28]
    ]


def test_a_version_six_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][31]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_five_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_five_data()
    data["seeds"][27] = _entry(SEEDS[28])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_six_is_refused(
    tmp_path,
):
    twenty_eight_under_v6 = _version_five_data()
    twenty_eight_under_v6["format"] = "canaryfield-seeds/6"
    with pytest.raises(ValueError, match="carries 32"):
        load_seed_pack(_pack_file(tmp_path, twenty_eight_under_v6))
    current_under_v5 = seed_pack_data()
    current_under_v5["format"] = "canaryfield-seeds/5"
    with pytest.raises(ValueError, match="carries 28"):
        load_seed_pack(_pack_file(tmp_path, current_under_v5))
