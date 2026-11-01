"""Tests for the Round 2 review fix obligations in CONTRACT.md.

Every fixture here is synthetic and written for this test file.
"""

import os
from pathlib import Path

import pytest

from canaryfield.engine import NOT_APPLICABLE, _is_test_file, run_field
from canaryfield.seedpack import load_seed_pack
from canaryfield.seeds import SEEDS_BY_ID

from test_slice3 import NOOP_GATE, _git, _write_git_project

needs_symlink = pytest.mark.skipif(
    not hasattr(os, "symlink"), reason="platform cannot create symlinks"
)

MODULE = (
    "def value():\n"
    "    return 7\n"
)

TEST_FOR_MODULE = (
    "from mod import value\n"
    "\n"
    "\n"
    "def test_value():\n"
    "    assert value() == 7\n"
)

PYTEST_GATE = ("tests", "python3 -m pytest -q")


def _rows(result):
    return {row["seed"]: row for row in result["rows"]}


def _write_project(root, files):
    root.mkdir(parents=True)
    for name, text in files.items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


@needs_symlink
def test_git_mode_never_writes_through_a_committed_symlink(tmp_path):
    outside = tmp_path / "outside.py"
    outside.write_text(MODULE, encoding="utf-8")
    target = _write_project(
        tmp_path / "project",
        {"mod.py": MODULE, "test_mod.py": TEST_FOR_MODULE},
    )
    os.symlink(outside, target / "escape.py")
    _write_git_project(target)
    result = run_field(
        target,
        [PYTEST_GATE],
        timeout=60,
        work_root=tmp_path / "work",
        plant_mode="git-branches",
    )
    assert outside.read_text(encoding="utf-8") == MODULE
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["mod.py"]
    assert result["target_unchanged"] is True


@needs_symlink
def test_a_relative_symlink_is_not_a_second_planting(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {"real.py": MODULE, "test_mod.py": TEST_FOR_MODULE},
    )
    (target / "test_mod.py").write_text(
        "from real import value\n"
        "\n"
        "\n"
        "def test_value():\n"
        "    assert value() == 7\n",
        encoding="utf-8",
    )
    os.symlink("real.py", target / "linked.py")
    _write_git_project(target)
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
    assert rows["seed-01"]["planted_files"] == ["real.py"]
    committed = _git(
        work_root / "clone",
        "show",
        "--name-only",
        "--format=",
        "canaryfield/seed-01",
    )
    assert committed.splitlines() == ["real.py"]


def test_an_identity_rewrite_is_not_a_planting(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {
            "version.py": "VERSION = '9.9.9'\n",
            "test_version.py": (
                "from version import VERSION\n"
                "\n"
                "\n"
                "def test_version():\n"
                "    assert VERSION == '9.9.9'\n"
            ),
        },
    )
    result = run_field(target, [PYTEST_GATE], timeout=60)
    rows = _rows(result)
    assert rows["seed-00"]["planted_files"] == []
    assert rows["seed-00"]["gates"]["tests"] == NOT_APPLICABLE


def test_git_mode_survives_an_identity_only_target(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {
            "version.py": "VERSION = '9.9.9'\n",
            "test_version.py": (
                "from version import VERSION\n"
                "\n"
                "\n"
                "def test_version():\n"
                "    assert VERSION == '9.9.9'\n"
            ),
        },
    )
    _write_git_project(target)
    result = run_field(
        target,
        [NOOP_GATE],
        timeout=30,
        work_root=tmp_path / "work",
        plant_mode="git-branches",
    )
    rows = _rows(result)
    assert rows["seed-00"]["planted_files"] == []
    assert rows["seed-00"]["branch"] is None


def test_test_infrastructure_is_never_planted(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {
            "mod.py": "def ready():\n    return 'yes'\n",
            "conftest.py": (
                "import pytest\n"
                "\n"
                "FLAG = True\n"
                "\n"
                "\n"
                "@pytest.fixture\n"
                "def flag():\n"
                "    return FLAG\n"
            ),
            "test_mod.py": (
                "from mod import ready\n"
                "\n"
                "\n"
                "def test_ready(flag):\n"
                "    assert ready() == 'yes'\n"
                "    assert flag is True\n"
            ),
        },
    )
    result = run_field(target, [PYTEST_GATE], timeout=60)
    rows = _rows(result)
    assert rows["seed-05"]["planted_files"] == []
    assert rows["seed-05"]["gates"]["tests"] == NOT_APPLICABLE


def test_the_test_file_test_covers_the_wider_surface():
    assert _is_test_file(Path("conftest.py"))
    assert _is_test_file(Path("test.py"))
    assert _is_test_file(Path("test/helper.py"))
    assert _is_test_file(Path("testing/helper.py"))
    assert not _is_test_file(Path("src/helper.py"))


def test_one_unparseable_file_costs_no_run(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {
            "mod.py": MODULE,
            "broken.py": "def broken(:\n    return 1\n",
            "test_mod.py": TEST_FOR_MODULE,
        },
    )
    result = run_field(target, [PYTEST_GATE], timeout=60)
    assert result["unparseable_files"] == ["broken.py"]
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["mod.py"]
    assert rows["seed-01"]["gates"]["tests"] == "caught"


def test_planted_files_sort_as_rendered_strings(tmp_path):
    target = _write_project(
        tmp_path / "project",
        {
            "mod.py": MODULE,
            "mod/x.py": MODULE,
            "test_mod.py": TEST_FOR_MODULE,
        },
    )
    result = run_field(target, [NOOP_GATE], timeout=30)
    rows = _rows(result)
    assert rows["seed-01"]["planted_files"] == ["mod.py", "mod/x.py"]


def test_a_whitespace_only_pack_field_is_refused(tmp_path):
    entries = []
    for seed in sorted(SEEDS_BY_ID.values(), key=lambda item: item.id)[:12]:
        entries.append(
            {
                "id": seed.id,
                "name": seed.name,
                "defect_class": seed.defect_class,
                "description": seed.description,
            }
        )
    entries[0]["description"] = "   "
    pack = tmp_path / "pack.json"
    import json

    pack.write_text(
        json.dumps({"format": "canaryfield-seeds/1", "seeds": entries}),
        encoding="utf-8",
    )
    with pytest.raises(ValueError):
        load_seed_pack(pack)
