"""Slice 4 tests. Plant at every matching shape inside one file.

Every fixture here is synthetic and written for this repo. Git fixtures
are throwaway repositories built inside the test temp dirs.
"""

import ast
import subprocess
import sys

from canaryfield.engine import CAUGHT, NOT_APPLICABLE, plant_seed, run_field
from canaryfield.seeds import SEEDS_BY_ID

from fixture_project import GATE, write_tiny_project
from test_slice3 import NOOP_GATE, _git, _write_git_project

TRIPLE_SOURCE = """\
def first():
    return 1


def second():
    return 2


def third():
    return 3
"""

TRIPLE_TEST_SOURCE = """\
from multi import first, second, third


def test_first():
    assert first() == 1


def test_second():
    assert second() == 2


def test_third():
    assert third() == 3
"""

SECOND_SOURCE = """\
def first():
    return 1


def second():
    return 2
"""

SECOND_TEST_SOURCE = """\
from pair import second


def test_second():
    assert second() == 2
"""


def _write_project(root, files):
    root.mkdir(parents=True, exist_ok=True)
    for name, text in files.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


def _rows(result):
    return {row["seed"]: row for row in result["rows"]}


def _gate_exit(directory):
    completed = subprocess.run(
        [sys.executable, "-m", "pytest", "-q"],
        cwd=directory,
        capture_output=True,
        text=True,
    )
    return completed.returncode


def _return_values(tree):
    return [
        node.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Return)
    ]


def test_one_apply_call_plants_every_match():
    seed = SEEDS_BY_ID["seed-01"]
    tree = ast.parse(TRIPLE_SOURCE)
    assert seed.apply(tree) is True
    assert _return_values(tree) == [None, None, None]

    seed = SEEDS_BY_ID["seed-05"]
    tree = ast.parse("A = True\nB = False\n")
    assert seed.apply(tree) is True
    rendered = ast.unparse(tree)
    assert "A = False" in rendered
    assert "B = True" in rendered

    seed = SEEDS_BY_ID["seed-06"]
    tree = ast.parse("def f(x, y):\n    return x * 2 + y\n")
    assert seed.apply(tree) is True
    assert "x + 2 - y" in ast.unparse(tree)

    seed = SEEDS_BY_ID["seed-10"]
    tree = ast.parse("def f():\n    return list(range(3)) + list(range(5))\n")
    assert seed.apply(tree) is True
    calls = [
        node
        for node in ast.walk(tree)
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "range"
    ]
    assert len(calls) == 2
    for call in calls:
        assert isinstance(call.args[-1], ast.BinOp)


def test_three_occurrences_in_one_file_are_all_planted(tmp_path):
    files = {"multi.py": TRIPLE_SOURCE, "test_multi.py": TRIPLE_TEST_SOURCE}
    scratch = _write_project(tmp_path / "scratch", files)
    planted = plant_seed(scratch, SEEDS_BY_ID["seed-01"])
    assert planted == ["multi.py"]
    tree = ast.parse(
        (scratch / "multi.py").read_text(encoding="utf-8")
    )
    values = _return_values(tree)
    assert len(values) == 3
    assert values == [None, None, None]

    target = _write_project(tmp_path / "project", files)
    result = run_field(target, [GATE], timeout=60)
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["multi.py"]
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT
    assert result["target_unchanged"] is True


def test_second_occurrence_only_fixture_flips_with_the_widening(tmp_path):
    files = {"pair.py": SECOND_SOURCE, "test_pair.py": SECOND_TEST_SOURCE}
    old_style = _write_project(tmp_path / "old-style", files)
    pair = old_style / "pair.py"
    text = pair.read_text(encoding="utf-8")
    assert "    return 1\n" in text and "    return 2\n" in text
    pair.write_text(text.replace("    return 1\n", "    return\n", 1), encoding="utf-8")
    assert "    return 2\n" in pair.read_text(encoding="utf-8")
    assert _gate_exit(old_style) == 0

    target = _write_project(tmp_path / "project", files)
    result = run_field(target, [GATE], timeout=60)
    rows = _rows(result)
    assert result["clean_exits"]["tests"] == 0
    assert rows["seed-01"]["planted_files"] == ["pair.py"]
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT


def test_a_partly_planted_file_plants_the_rest(tmp_path):
    scratch = _write_project(
        tmp_path / "scratch",
        {"versions.py": 'VERSION = "9.9.9"\n__version__ = "1.2.3"\n'},
    )
    planted = plant_seed(scratch, SEEDS_BY_ID["seed-00"])
    assert planted == ["versions.py"]
    tree = ast.parse(
        (scratch / "versions.py").read_text(encoding="utf-8")
    )
    values = [
        node.value.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Assign)
    ]
    assert values == ["9.9.9", "9.9.9"]


def test_a_fully_planted_file_plants_nothing(tmp_path):
    scratch = _write_project(
        tmp_path / "scratch",
        {"versions.py": 'VERSION = "9.9.9"\n__version__ = "9.9.9"\n'},
    )
    assert plant_seed(scratch, SEEDS_BY_ID["seed-00"]) == []


def test_a_planted_shape_is_not_planted_twice():
    seed = SEEDS_BY_ID["seed-06"]
    tree = ast.parse("def scale(x):\n    return x * 2\n")
    assert seed.apply(tree) is True
    rendered = ast.unparse(tree)
    assert "x + 2" in rendered
    assert "x - 2" not in rendered

    seed = SEEDS_BY_ID["seed-05"]
    tree = ast.parse("A = True\n")
    assert seed.apply(tree) is True
    assert "A = False" in ast.unparse(tree)


def test_negated_conditions_wrap_exactly_once():
    source = (
        "def pick(a, b):\n"
        "    if a:\n"
        "        if b:\n"
        "            return 1\n"
        "    return 0\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-08"].apply(tree) is True
    conditions = [
        node for node in ast.walk(tree) if isinstance(node, ast.If)
    ]
    assert len(conditions) == 2
    for condition in conditions:
        assert isinstance(condition.test, ast.UnaryOp)
        assert isinstance(condition.test.op, ast.Not)
        assert not isinstance(condition.test.operand, ast.UnaryOp)


def test_range_bounds_shift_exactly_once():
    source = "def count():\n    return list(range(3)) + list(range(5))\n"
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-10"].apply(tree) is True
    calls = [
        node
        for node in ast.walk(tree)
        if isinstance(node, ast.Call)
        and isinstance(node.func, ast.Name)
        and node.func.id == "range"
    ]
    assert len(calls) == 2
    bounds = []
    for call in calls:
        bound = call.args[-1]
        assert isinstance(bound, ast.BinOp)
        assert isinstance(bound.left, ast.Constant)
        bounds.append(bound.left.value)
    assert bounds == [3, 5]


def test_a_match_inside_a_swallowed_handler_stays_detached():
    source = (
        "def f():\n"
        "    try:\n"
        "        risky()\n"
        "    except ValueError:\n"
        "        try:\n"
        "            fallback()\n"
        "        except KeyError:\n"
        "            raise\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-04"].apply(tree) is True
    handlers = [
        node for node in ast.walk(tree) if isinstance(node, ast.ExceptHandler)
    ]
    assert len(handlers) == 1
    assert len(handlers[0].body) == 1
    assert isinstance(handlers[0].body[0], ast.Pass)
    rendered = ast.unparse(tree)
    assert "fallback" not in rendered
    assert "raise" not in rendered


def test_single_occurrence_fixture_keeps_its_slice_three_verdicts(tmp_path):
    target = write_tiny_project(tmp_path / "tiny")
    result = run_field(target, [GATE], timeout=60)
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["tiny.py"]
    assert rows["seed-01"]["gates"]["tests"] == CAUGHT
    assert rows["seed-03"]["planted_files"] == ["tiny.py"]
    assert rows["seed-03"]["gates"]["tests"] == CAUGHT
    assert rows["seed-00"]["planted_files"] == []
    assert rows["seed-00"]["gates"]["tests"] == NOT_APPLICABLE


def test_git_mode_branch_commit_carries_the_fully_planted_file(tmp_path):
    target = _write_git_project(
        _write_project(
            tmp_path / "project",
            {"multi.py": TRIPLE_SOURCE, "test_multi.py": TRIPLE_TEST_SOURCE},
        )
    )
    work_root = tmp_path / "work"
    result = run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=work_root,
        keep_scratch=True,
        plant_mode="git-branches",
    )
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["multi.py"]
    assert rows["seed-01"]["branch"] == "canaryfield/seed-01"
    committed = _git(
        work_root / "clone", "show", "canaryfield/seed-01:multi.py"
    )
    values = _return_values(ast.parse(committed))
    assert len(values) == 3
    assert values == [None, None, None]
    assert result["target_unchanged"] is True
