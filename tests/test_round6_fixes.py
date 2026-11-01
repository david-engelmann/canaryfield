"""Tests for the Round 6 review fixes in CONTRACT.md.

FIX6-1 keeps seed-16 off a handler whose type names a broad
base in any member, bare or inside a tuple. FIX6-2 counts
seed-19 parameters in signature declaration order. The scope
rules of K-5 and the conditional expression shape of K-4 gain
the discriminating pins the two axis review found missing.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def _rendered(seed_id, source):
    tree = ast.parse(source)
    applied = SEEDS_BY_ID[seed_id].apply(tree)
    return applied, ast.unparse(tree)


def test_seed_sixteen_skips_a_tuple_naming_exception():
    source = (
        "def f():\n"
        "    try:\n"
        "        return 1\n"
        "    except (ValueError, Exception):\n"
        "        return -1\n"
    )
    applied, _rendered_text = _rendered("seed-16", source)
    assert applied is False


def test_seed_sixteen_skips_a_tuple_naming_base_exception():
    source = (
        "def f():\n"
        "    try:\n"
        "        return 1\n"
        "    except (BaseException, ValueError):\n"
        "        return -1\n"
    )
    applied, _rendered_text = _rendered("seed-16", source)
    assert applied is False


def test_seed_sixteen_widens_a_tuple_of_narrow_types():
    source = (
        "def f():\n"
        "    try:\n"
        "        return 1\n"
        "    except (ValueError, TypeError):\n"
        "        return -1\n"
    )
    applied, rendered = _rendered("seed-16", source)
    assert applied is True
    assert "except Exception" in rendered
    assert "ValueError" not in rendered


def test_seed_nineteen_parameters_follow_signature_order():
    source = (
        "def run(first, *rest, flag):\n"
        "    for first in items:\n"
        "        pass\n"
    )
    applied, rendered = _rendered("seed-19", source)
    assert applied is True
    assert "for rest in items" in rendered


def test_seed_nineteen_stays_inside_the_inner_scope():
    source = (
        "def outer(outer_name):\n"
        "    def inner():\n"
        "        total = 0\n"
        "        for item in items:\n"
        "            total += item\n"
        "        return total\n"
        "    return inner\n"
    )
    applied, rendered = _rendered("seed-19", source)
    assert applied is True
    assert "for total in items" in rendered
    assert "for outer_name in items" not in rendered


def test_seed_nineteen_parameters_come_before_body_names():
    source = (
        "def f(param, items):\n"
        "    helper = 0\n"
        "    for item in items:\n"
        "        pass\n"
    )
    applied, rendered = _rendered("seed-19", source)
    assert applied is True
    assert "for param in items" in rendered


def test_seed_nineteen_ignores_a_comprehension_target():
    source = "for item in [y for y in range(3)]:\n    pass\n"
    applied, _rendered_text = _rendered("seed-19", source)
    assert applied is False


def test_seed_nineteen_ignores_a_walrus_inside_a_comprehension():
    source = "for item in [(w := v) for v in range(3)]:\n    pass\n"
    applied, _rendered_text = _rendered("seed-19", source)
    assert applied is False


def test_seed_eighteen_strips_a_negated_conditional_expression():
    source = "def pick(flag):\n    return 1 if not flag else 2\n"
    applied, rendered = _rendered("seed-18", source)
    assert applied is True
    assert "1 if flag else 2" in rendered
    assert "not flag" not in rendered
