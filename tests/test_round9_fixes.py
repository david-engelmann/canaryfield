"""Tests for the Round 9 review fixes in CONTRACT.md.

A nested definition's header expressions stand in the
enclosing scope and are judged there, and a store or a load
standing inside a comprehension counts in the scope that
holds the comprehension.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def test_seed_thirty_one_reads_a_nested_default_in_the_enclosing_scope():
    source = (
        "def outer(cfg):\n"
        "    first = cfg.alpha\n"
        "    def inner(value=cfg.beta):\n"
        "        return value\n"
        "    return first\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "first = cfg.beta" in rendered


def test_seed_thirty_one_never_plants_on_nested_scope_evidence():
    source = (
        "def outer():\n"
        "    def inner(value=cfg.alpha):\n"
        "        cfg = build()\n"
        "        return cfg.beta\n"
        "    return inner\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_a_comprehension_store_disqualifies_its_base():
    source = (
        "config = build()\n"
        "first = config.path\n"
        "second = config.mode\n"
        "seen = [config for config in items]\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is False
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_thirty_one_a_comprehension_load_counts_in_its_scope():
    source = (
        "config = build()\n"
        "first = config.path\n"
        "vals = [config.mode for item in items]\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-31"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "first = config.mode" in rendered
