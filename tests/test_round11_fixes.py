"""Round 11 review fix tests.

The red team rejected slice 12 on two Criticals of one kind.
A planting must never leave a tree the language refuses,
because the engine writes planted trees with no compile
check, so a broken planting scores as a catch against an
innocent gate. Seed-36 planted that breakage at the lone
handler standing beside both an else clause and a finally
block, and seed-34, landed in slice 11, already planted it
on a try and finally with no handler, the shape of its own
caught fixture, whose catch could not tell the defect from
the breakage. These tests pin the repairs and the invariant
behind them, that every seed's planting on the repo's own
corpus compiles.

Every fixture here is synthetic and written for this repo.
"""

import ast
import shutil
from pathlib import Path

from canaryfield.engine import CAUGHT, plant_seed, run_field
from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import GATE

ROOT = Path(__file__).resolve().parents[1]

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


def test_fetch_closes_the_handle():
    handle = Handle()
    assert fetch(handle) == "data"
    assert handle.closed is True
"""


def _write_project(root, files):
    root.mkdir(parents=True)
    for name, text in files.items():
        (root / name).write_text(text, encoding="utf-8")
    return root


def _cells(result):
    return {
        row["seed"]: row["gates"]["tests"]
        for row in result["rows"]
    }


def test_seed_thirty_six_refuses_a_lone_handler_beside_an_else_and_a_finally():
    source = (
        "def parse(text):\n"
        "    try:\n"
        "        return int(text)\n"
        "    except ValueError:\n"
        "        return -1\n"
        "    else:\n"
        "        return 0\n"
        "    finally:\n"
        "        pass\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_six_refuses_the_star_form_of_the_conjunction():
    source = (
        "def parse(text):\n"
        "    try:\n"
        "        return int(text)\n"
        "    except* ValueError:\n"
        "        return -1\n"
        "    else:\n"
        "        return 0\n"
        "    finally:\n"
        "        pass\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-36"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_four_plants_a_handlerless_finally_as_a_pass_body():
    tree = ast.parse(FETCH_SOURCE)
    assert SEEDS_BY_ID["seed-34"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "finally:" in rendered
    assert "handle.close()" not in rendered


def test_seed_thirty_four_still_removes_the_block_beside_handlers():
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
    assert SEEDS_BY_ID["seed-34"].apply(tree) is True
    rendered = ast.unparse(tree)
    compile(rendered, "planted", "exec")
    assert "finally:" not in rendered
    assert "except OSError:" in rendered


def test_seed_thirty_four_catch_is_earned_on_a_compiling_copy(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"fetching.py": FETCH_SOURCE, "test_fetching.py": FETCH_TEST},
    )
    planted = tmp_path / "planted"
    shutil.copytree(target, planted)
    changed = plant_seed(planted, SEEDS_BY_ID["seed-34"])
    assert changed == ["fetching.py"]
    compile(
        (planted / "fetching.py").read_text(encoding="utf-8"),
        "fetching.py",
        "exec",
    )
    result = run_field(target, [GATE], timeout=60)
    assert result["clean_exits"]["tests"] == 0
    assert _cells(result)["seed-34"] == CAUGHT


NONLOCAL_SHAPES = {
    "seed-34": (
        "def outer():\n"
        "    def inner():\n"
        "        nonlocal x\n"
        "        return x\n"
        "    try:\n"
        "        pass\n"
        "    finally:\n"
        "        x = 1\n"
        "    return inner\n"
    ),
    "seed-36": (
        "def outer():\n"
        "    def inner():\n"
        "        nonlocal x\n"
        "        return x\n"
        "    try:\n"
        "        pass\n"
        "    except ValueError:\n"
        "        x = 1\n"
        "    except TypeError:\n"
        "        pass\n"
        "    return inner\n"
    ),
    "seed-38": (
        "def outer():\n"
        "    def inner():\n"
        "        nonlocal x\n"
        "        return x\n"
        "    ok = flag and (x := 3)\n"
        "    return inner\n"
    ),
    "seed-39": (
        "def outer():\n"
        "    def make():\n"
        "        nonlocal x\n"
        "        return x\n"
        "    class C((x := Base)):\n"
        "        pass\n"
        "    return make\n"
    ),
}


def test_a_planting_that_would_break_a_nonlocal_binding_is_never_written(
    tmp_path,
):
    # Each shape holds the only binding for a nested nonlocal inside
    # the part its seed removes, so the direct planting does not
    # compile. The engine never writes a refused tree, so the file
    # stands unplanted and unchanged for each of the four seeds.
    for seed_id, source in NONLOCAL_SHAPES.items():
        project = tmp_path / seed_id
        project.mkdir()
        target = project / "mod.py"
        target.write_text(source, encoding="utf-8")
        planted = plant_seed(project, SEEDS_BY_ID[seed_id])
        assert planted == [], seed_id
        assert target.read_text(encoding="utf-8") == source, seed_id
        compile(source, "mod.py", "exec")


def test_every_seed_plants_only_compiling_trees_on_the_repo_corpus():
    corpus = sorted(
        path
        for path in ROOT.rglob("*.py")
        if "__pycache__" not in path.parts
    )
    assert len(corpus) >= 30
    failures = []
    for seed in SEEDS:
        for path in corpus:
            tree = ast.parse(path.read_text(encoding="utf-8"))
            if seed.apply(tree):
                rendered = ast.unparse(tree)
                try:
                    compile(rendered, str(path), "exec")
                except SyntaxError:
                    failures.append((seed.id, path.name))
    assert failures == []
