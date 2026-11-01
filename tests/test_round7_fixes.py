"""Tests for the Round 7 review fixes in CONTRACT.md.

Four edge rules of slice 8 gain the discriminating content
level pins the two axis review found missing, the first two
arguments only rule of L-2 past two arguments, the empty
string and bytes exclusions of L-3, and the positive half of
the corrected nested function sentence of L-5.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def _rendered(seed_id, source):
    tree = ast.parse(source)
    applied = SEEDS_BY_ID[seed_id].apply(tree)
    return applied, ast.unparse(tree)


def test_seed_twenty_leaves_a_third_argument_standing():
    applied, rendered = _rendered(
        "seed-20", "result = combine(one, two, three)\n"
    )
    assert applied is True
    assert "combine(two, one, three)" in rendered


def test_seed_twenty_one_leaves_an_empty_string_standing():
    applied, rendered = _rendered(
        "seed-21", "a = ''\nb = 'ready'\n"
    )
    assert applied is True
    assert "''" in rendered
    assert "'xeady'" in rendered


def test_seed_twenty_one_leaves_a_bytes_literal_standing():
    applied, rendered = _rendered(
        "seed-21", "a = b'ready'\nb = 'ready'\n"
    )
    assert applied is True
    assert "b'ready'" in rendered
    assert "'xeady'" in rendered


def test_seed_twenty_three_plants_a_break_in_a_nested_functions_loop():
    source = (
        "for outer in items:\n"
        "    def helper():\n"
        "        for inner in items:\n"
        "            break\n"
        "    helper()\n"
    )
    applied, rendered = _rendered("seed-23", source)
    assert applied is True
    assert "continue" in rendered
    assert "break" not in rendered
