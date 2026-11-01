"""Slice 11 tests. Four new seeds and a seventh seed pack format.

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

CONVERT_SOURCE = """\
def convert(text):
    try:
        value = float(text)
        if value > 100:
            raise OverflowError("too big")
        return value
    except Exception:
        return 0.0
"""

CONVERT_TEST = """\
from converting import convert


def test_convert():
    assert convert("5") == 5.0
    assert convert("oops") == 0.0
    assert convert("200") == 0.0
"""

LABEL_SOURCE = """\
def label(name="anon", punct="!"):
    return name + punct
"""

LABEL_TEST = """\
from labelling import label


def test_label():
    assert label() == "anon!"
    assert label("hi") == "hi!"
    assert label("hi", "?") == "hi?"
"""

FETCH_SOURCE = """\
class Handle:
    def __init__(self):
        self.closed = False

    def read(self):
        return "data"

    def close(self):
        self.closed = True


def fetch(handle):
    try:
        return handle.read()
    finally:
        handle.close()
"""

FETCH_TEST = """\
from fetching import Handle, fetch


def test_fetch():
    handle = Handle()
    assert fetch(handle) == "data"
    assert handle.closed is True
"""

GREETING_SOURCE = """\
def shout(fn):
    def wrapper():
        return fn().upper()

    return wrapper


@shout
def greeting():
    return "hi"
"""

GREETING_TEST = """\
from greeting import greeting


def test_greeting():
    assert greeting() == "HI"
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


def _version_six_data():
    return {
        "format": "canaryfield-seeds/6",
        "seeds": [_entry(seed) for seed in SEEDS[:32]],
    }


def test_seed_thirty_two_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"converting.py": CONVERT_SOURCE, "test_converting.py": CONVERT_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-32"] == CAUGHT


def test_seed_thirty_two_narrows_the_handler_to_the_first_type_the_try_body_raises():
    tree = ast.parse(CONVERT_SOURCE)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except OverflowError:" in rendered


def test_seed_thirty_two_narrows_a_base_exception_handler():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        return parse(raw)\n"
        "    except BaseException:\n"
        "        raise TypeError('bad input')\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise TypeError('bad input')\n"
        "    except BaseException:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except TypeError:" in rendered


def test_seed_thirty_two_narrows_a_tuple_handler_as_a_whole():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise ValueError('bad input')\n"
        "    except (KeyError, Exception):\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except ValueError:" in rendered
    assert "KeyError" not in rendered


def test_seed_thirty_two_gathers_a_raise_from_a_try_nested_in_the_body():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        try:\n"
        "            raise KeyError('missing')\n"
        "        except KeyError:\n"
        "            pass\n"
        "        return raw\n"
        "    except Exception:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except KeyError:\n        return None" in rendered


def test_seed_thirty_two_leaves_a_raise_inside_a_nested_function_out_of_the_reading():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        def build():\n"
        "            raise ValueError('bad input')\n"
        "        return build\n"
        "    except Exception:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_two_leaves_a_handler_that_names_no_broad_base_standing():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise ValueError('bad input')\n"
        "    except ValueError:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_two_leaves_a_bare_handler_standing():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise ValueError('bad input')\n"
        "    except:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_two_leaves_a_try_that_raises_only_the_broad_base_standing():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise Exception('bad input')\n"
        "    except Exception:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_two_narrows_only_the_broad_handler_of_a_try():
    source = (
        "def load(raw):\n"
        "    try:\n"
        "        raise TypeError('bad input')\n"
        "    except ValueError:\n"
        "        return 1\n"
        "    except Exception:\n"
        "        return 2\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except ValueError:" in rendered
    assert "except TypeError:" in rendered


def test_seed_thirty_two_not_applicable_without_a_broad_handler(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-32"] == NOT_APPLICABLE


def test_seed_thirty_three_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"labelling.py": LABEL_SOURCE, "test_labelling.py": LABEL_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-33"] == CAUGHT


def test_seed_thirty_three_swaps_the_first_two_positional_defaults():
    tree = ast.parse("def scale(a=1, b=2):\n    return (a, b)\n")
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def scale(a=2, b=1):" in rendered


def test_seed_thirty_three_swaps_a_positional_default_with_a_keyword_only_default():
    tree = ast.parse("def scale(a=1, *, b=2):\n    return (a, b)\n")
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def scale(a=2, *, b=1):" in rendered


def test_seed_thirty_three_keeps_the_third_default_standing():
    tree = ast.parse("def scale(a=1, b=2, c=3):\n    return (a, b, c)\n")
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def scale(a=2, b=1, c=3):" in rendered


def test_seed_thirty_three_swaps_two_keyword_only_defaults():
    source = "def scale(*, a=1, b=2):\n    return (a, b)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def scale(*, a=2, b=1):" in rendered
    source = "def scale(a, *, key, b=2, c=3):\n    return (b, c)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "def scale(a, *, key, b=3, c=2):" in rendered


def test_seed_thirty_three_plants_the_defaults_of_a_lambda():
    tree = ast.parse("fn = lambda a=1, b=2: (a, b)\n")
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "fn = lambda a=2, b=1: (a, b)" in rendered


def test_seed_thirty_three_identical_defaults_exchange_into_themselves():
    source = "def scale(a=1, b=1):\n    return (a, b)\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-33"].apply(tree) is True
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_three_leaves_a_single_default_standing():
    source = "def scale(a=1):\n    return a\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-33"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_three_not_applicable_without_two_defaults(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-33"] == NOT_APPLICABLE


def test_seed_thirty_four_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"fetching.py": FETCH_SOURCE, "test_fetching.py": FETCH_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-34"] == CAUGHT


def test_seed_thirty_four_removes_the_finally_block_and_keeps_the_try():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except OSError:\n"
        "        return ''\n"
        "    finally:\n"
        "        handle.close()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-34"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except OSError:" in rendered
    assert "return handle.read()" in rendered
    assert "finally" not in rendered
    assert "handle.close()" not in rendered


def test_seed_thirty_four_plants_on_a_try_star_statement():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except* OSError:\n"
        "        return ''\n"
        "    finally:\n"
        "        handle.close()\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-34"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except* OSError:" in rendered
    assert "finally" not in rendered
    assert "handle.close()" not in rendered


def test_seed_thirty_four_leaves_a_try_without_a_finally_standing():
    source = (
        "def fetch(handle):\n"
        "    try:\n"
        "        return handle.read()\n"
        "    except OSError:\n"
        "        return ''\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-34"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_four_not_applicable_without_a_finally_block(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-34"] == NOT_APPLICABLE


def test_seed_thirty_five_plants_and_is_caught(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"greeting.py": GREETING_SOURCE, "test_greeting.py": GREETING_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-35"] == CAUGHT


def test_seed_thirty_five_removes_the_first_decorator_and_keeps_the_rest():
    source = "@alpha\n@beta\ndef render():\n    return 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-35"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "@beta" in rendered
    assert "alpha" not in rendered
    assert "def render():" in rendered


def test_seed_thirty_five_plants_on_a_class_definition():
    source = "@register\nclass Thing:\n    pass\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-35"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "class Thing:" in rendered
    assert "register" not in rendered


def test_seed_thirty_five_plants_on_an_async_definition():
    source = "@deco\nasync def fetch():\n    return 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-35"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "async def fetch():" in rendered
    assert "deco" not in rendered


def test_seed_thirty_five_leaves_an_undecorated_definition_standing():
    source = "def render():\n    return 1\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-35"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_five_not_applicable_without_a_decorator(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"single.py": SINGLE_SOURCE, "test_single.py": SINGLE_TEST},
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-35"] == NOT_APPLICABLE


def test_version_six_pack_still_loads_the_thirty_two(tmp_path):
    entries = load_seed_pack(_pack_file(tmp_path, _version_six_data()))
    assert [entry["id"] for entry in entries] == [
        seed.id for seed in SEEDS[:32]
    ]


def test_a_version_seven_pack_with_an_invented_id_is_refused(tmp_path):
    data = seed_pack_data()
    data["seeds"][35]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_six_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_six_data()
    data["seeds"][31] = _entry(SEEDS[32])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_whose_count_disagrees_with_version_seven_is_refused(
    tmp_path,
):
    thirty_two_under_v7 = _version_six_data()
    thirty_two_under_v7["format"] = "canaryfield-seeds/7"
    with pytest.raises(ValueError, match="carries 36"):
        load_seed_pack(_pack_file(tmp_path, thirty_two_under_v7))
    current_under_v6 = seed_pack_data()
    current_under_v6["format"] = "canaryfield-seeds/6"
    with pytest.raises(ValueError, match="carries 32"):
        load_seed_pack(_pack_file(tmp_path, current_under_v6))
