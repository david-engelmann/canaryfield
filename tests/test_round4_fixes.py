"""Tests for the Round 4 review fix obligations in CONTRACT.md.

Every fixture here is synthetic and written for this test file.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def test_seed_fourteen_plants_its_swapped_operators_in_the_content():
    tree = ast.parse(
        "def admit(age, member):\n"
        "    return age >= 18 and member\n"
        "def either(flag, other):\n"
        "    return flag or other\n"
    )
    assert SEEDS_BY_ID["seed-14"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "age >= 18 or member" in rendered
    assert "flag and other" in rendered
