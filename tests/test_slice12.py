"""Slice 12 tests. Four new seeds and an eighth seed pack format.

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

PARSE_SOURCE = """\
def parse(text):
    try:
        return int(text)
    except ValueError:
        return -1
    except TypeError:
        return -2
"""

PARSE_TEST = """\
from parsing import parse


def test_parse():
    assert parse("42") == 42
    assert parse("oops") == -1
    assert parse(None) == -2
"""

ITEMS_SOURCE = """\
def first_items():
    yield "a"
    yield "b"
"""

ITEMS_TEST = """\
from items import first_items


def test_first_items():
    assert list(first_items()) == ["a", "b"]
"""

ADMISSION_SOURCE = """\
def admitted(age, member):
    return age >= 18 and member
"""

ADMISSION_TEST = """\
from admission import admitted


def test_admitted():
    assert admitted(20, True) is True
    assert admitted(20, False) is False
    assert admitted(10, True) is False
"""

SHAPES_SOURCE = """\
class Named:
    def label(self):
        return "named"


class Widget(Named):
    pass
"""

SHAPES_TEST = """\
from shapes import Widget


def test_widget():
    assert Widget().label() == "named"
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


def _version_seven_data():
    return {
        "format": "canaryfield-seeds/7",
        "seeds": [_entry(seed) for seed in SEEDS[:36]],
    }


def test_seed_thirty_six_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"parsing.py": PARSE_SOURCE, "test_parsing.py": PARSE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-36"] == CAUGHT


def test_seed_thirty_six_drops_the_first_handler_and_keeps_the_rest():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except ValueError:\n"
        "        return 'bad'\n"
        "    except OSError:\n"
        "        return 'gone'\n"
        "    else:\n"
        "        return 'ok'\n"
        "    finally:\n"
        "        handle.close()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except ValueError:" not in rendered
    assert "return 'bad'" not in rendered
    assert "except OSError:" in rendered
    assert "return handle.read()" in rendered
    assert "finally:" in rendered
    assert "handle.close()" in rendered


def test_seed_thirty_six_plants_a_lone_handler_beside_a_finally_block():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except OSError:\n"
        "        return 'gone'\n"
        "    finally:\n"
        "        handle.close()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except OSError:" not in rendered
    assert "finally:" in rendered
    assert "handle.close()" in rendered


def test_seed_thirty_six_plants_on_a_try_star_statement():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except* ValueError:\n"
        "        return 'bad'\n"
        "    except* OSError:\n"
        "        return 'gone'\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except* ValueError:" not in rendered
    assert "except* OSError:" in rendered


def test_seed_thirty_six_leaves_a_lone_handler_without_a_finally_standing():
    source = (
        "def parse(text):\n"
        "    try:\n"
        "        return int(text)\n"
        "    except ValueError:\n"
        "        return -1\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_six_leaves_a_lone_handler_with_an_else_clause_standing():
    source = (
        "def parse(text):\n"
        "    try:\n"
        "        return int(text)\n"
        "    except ValueError:\n"
        "        return -1\n"
        "    else:\n"
        "        return 0\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_six_leaves_a_handlerless_try_standing():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    finally:\n"
        "        handle.close()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_six_not_applicable_without_a_plantable_try(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-36"] == NOT_APPLICABLE


def test_seed_thirty_seven_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"items.py": ITEMS_SOURCE, "test_items.py": ITEMS_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-37"] == CAUGHT


def test_seed_thirty_seven_replaces_a_yield_with_the_value_it_yields():
    tree = ast.parse(ITEMS_SOURCE)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "yield" not in rendered
    reparsed = ast.parse(rendered)
    function = reparsed.body[0]
    values = [
        statement.value.value
        for statement in function.body
        if isinstance(statement, ast.Expr)
        and isinstance(statement.value, ast.Constant)
    ]
    assert values == ["a", "b"]


def test_seed_thirty_seven_replaces_a_bare_yield_with_no_value():
    source = "def spin():\n    yield\n    yield 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "yield" not in rendered
    assert "None" in rendered


def test_seed_thirty_seven_plants_a_yield_whose_value_feeds_an_assignment():
    source = "def pump():\n    sent = yield 5\n    yield sent\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "yield" not in rendered
    assert "sent = 5" in rendered


def test_seed_thirty_seven_drops_a_nested_yield_once_at_each_level():
    source = "def gen():\n    yield (yield 3)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "yield" not in rendered
    assert "3" in rendered


def test_seed_thirty_seven_leaves_a_yield_from_standing():
    source = "def gen(items):\n    yield from items\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = "def gen(items):\n    yield from items\n    yield 0\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-37"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "yield from items" in rendered


def test_seed_thirty_seven_not_applicable_without_a_yield(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-37"] == NOT_APPLICABLE


def test_seed_thirty_eight_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"admission.py": ADMISSION_SOURCE, "test_admission.py": ADMISSION_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-38"] == CAUGHT


def test_seed_thirty_eight_drops_the_last_term_and_keeps_the_rest_in_order():
    source = "def check(first, second, third):\n    return first and second and third\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-38"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "first and second" in rendered
    assert "third" not in rendered.split("return", 1)[1]


def test_seed_thirty_eight_plants_every_boolean_operation_in_one_file():
    source = (
        "def check(a, b, c, d):\n"
        "    if a and b:\n"
        "        return 1\n"
        "    return c or d\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-38"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "if a:" in rendered
    assert "return c" in rendered
    assert "c or d" not in rendered


def test_seed_thirty_eight_plants_a_boolean_operation_standing_as_a_value():
    source = "flag = ready and willing\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-38"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "flag = ready" in rendered
    assert "willing" not in rendered


def test_seed_thirty_eight_shows_the_outer_planting_where_a_term_holds_an_operation():
    source = "def check(x, y, z):\n    return x and (y or z)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-38"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "return x" in rendered
    assert "y or z" not in rendered


def test_seed_thirty_eight_not_applicable_without_a_boolean_operation(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-38"] == NOT_APPLICABLE


def test_seed_thirty_nine_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"shapes.py": SHAPES_SOURCE, "test_shapes.py": SHAPES_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-39"] == CAUGHT


def test_seed_thirty_nine_drops_the_first_base_and_keeps_the_rest():
    source = "class Child(First, Second):\n    pass\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-39"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "class Child(Second):" in rendered
    assert "First" not in rendered


def test_seed_thirty_nine_drops_a_lone_base_and_keeps_the_header_keywords():
    source = "class Child(First, metaclass=Meta):\n    pass\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-39"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "class Child(metaclass=Meta):" in rendered
    assert "First" not in rendered


def test_seed_thirty_nine_leaves_a_baseless_class_standing():
    source = "class Plain:\n    pass\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-39"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_nine_not_applicable_without_a_based_class(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-39"] == NOT_APPLICABLE


def test_version_seven_pack_still_loads_the_thirty_six(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_seven_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:36]
    ]


def test_a_version_eight_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][39]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_seven_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_seven_data()
    data["seeds"][35] = _entry(SEEDS[36])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_eight_is_refused(
    tmp_path,
):
    thirty_six_under_v8 = _version_seven_data()
    thirty_six_under_v8["format"] = "canaryfield-seeds/8"
    with pytest.raises(ValueError, match="carries 40"):
        load_seed_pack(_pack_file(tmp_path, thirty_six_under_v8))
    current_under_v7 = seed_pack_data()
    current_under_v7["format"] = "canaryfield-seeds/7"
    with pytest.raises(ValueError, match="carries 36"):
        load_seed_pack(_pack_file(tmp_path, current_under_v7))
