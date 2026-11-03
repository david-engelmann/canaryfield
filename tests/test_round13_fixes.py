"""Round 13 review fix pins for the slice 14 obligations.

The red team returned one Major and two Minors against the new
seeds. Seed-47 planted a negation standing in the guard of a
match case, ground R-5 gives to the seeds that own condition
tests, and seed-46 negated two zero spellings of a range step
into themselves where R-4 excludes a zero step for changing
nothing. The pins below hold the readings the Round 13 review
fixes section of CONTRACT.md states. Every fixture here is
synthetic.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def _planted(seed_id, source):
    tree = ast.parse(source)
    applied = SEEDS_BY_ID[seed_id].apply(tree)
    return applied, tree


def test_seed_forty_seven_never_plants_a_negation_in_a_match_guard():
    source = (
        "def f(item):\n"
        "    match item:\n"
        "        case _ if not item.ready:\n"
        "            return 1\n"
        "        case _:\n"
        "            return 0\n"
    )
    applied, tree = _planted("seed-47", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_seven_never_plants_a_negation_in_a_nested_match_guard():
    source = (
        "def f(item):\n"
        "    match item:\n"
        "        case [inner]:\n"
        "            match inner:\n"
        "                case _ if not inner.ready:\n"
        "                    return 1\n"
        "    return 0\n"
    )
    applied, tree = _planted("seed-47", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_seven_plants_outside_a_match_while_its_guard_negation_stands():
    source = (
        "def f(item):\n"
        "    match item:\n"
        "        case _ if not item.ready:\n"
        "            return 1\n"
        "    return not item.done\n"
    )
    applied, tree = _planted("seed-47", source)
    assert applied
    rendered = ast.unparse(tree)
    assert "not item.ready" in rendered
    assert "return item.done" in rendered


def test_seed_forty_six_never_plants_a_zero_step_in_any_literal_spelling():
    for step in ("0.0", "+0", "+0.0", "0j"):
        source = f"def f():\n    return range(0, 10, {step})\n"
        applied, tree = _planted("seed-46", source)
        assert not applied, step
        assert ast.dump(tree) == ast.dump(ast.parse(source)), step


def test_seed_forty_six_still_plants_a_nonzero_float_step():
    source = "def f():\n    return range(0, 10, 0.5)\n"
    applied, tree = _planted("seed-46", source)
    assert applied
    assert "-0.5" in ast.unparse(tree)
