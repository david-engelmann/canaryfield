"""Tests for the Round 8 review fixes in CONTRACT.md.

Two edge rules of slice 9 gain the discriminating content
level pins the two axis review found missing, the attribute
target of M-2 and the computed index of M-4.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def _rendered(seed_id, source):
    tree = ast.parse(source)
    applied = SEEDS_BY_ID[seed_id].apply(tree)
    return applied, ast.unparse(tree)


def test_seed_twenty_four_plants_an_attribute_target():
    applied, rendered = _rendered("seed-24", "box.total += 1\n")
    assert applied is True
    assert "box.total -= 1" in rendered


def test_seed_twenty_six_leaves_a_computed_index_standing():
    applied, rendered = _rendered(
        "seed-26", "value = items[1 + 0]\n"
    )
    assert applied is False
    assert "items[1 + 0]" in rendered
