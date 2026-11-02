"""Round 12 review fix pins for the slice 13 obligations.

The two axis review and the red team returned the same three
record gaps at the edges of the new seeds, and two further
edges with no pin. The behaviour at every one of them already
conformed under the reviewers own probes, and the pins below
hold the readings the Round 12 review fixes section of
CONTRACT.md states. Every fixture here is synthetic.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def _planted(seed_id, source):
    tree = ast.parse(source)
    applied = SEEDS_BY_ID[seed_id].apply(tree)
    return applied, tree


def test_seed_forty_and_seed_three_plant_the_same_tree_on_a_returned_start_only_slice():
    source = "def f(items):\n    return items[1:]\n"
    applied40, tree40 = _planted("seed-40", source)
    applied03, tree03 = _planted("seed-03", source)
    assert applied40 and applied03
    assert ast.dump(tree40) == ast.dump(tree03)
    assert "items[2:]" in ast.unparse(tree40)


def test_seed_forty_and_seed_three_plant_the_same_tree_on_a_compared_start_only_slice():
    source = "def f(items):\n    return items[1:] == probe\n"
    applied40, tree40 = _planted("seed-40", source)
    applied03, tree03 = _planted("seed-03", source)
    assert applied40 and applied03
    assert ast.dump(tree40) == ast.dump(tree03)


def test_seed_forty_and_seed_three_diverge_where_a_stop_stands():
    source = "def f(items):\n    return items[1:4]\n"
    _, tree40 = _planted("seed-40", source)
    _, tree03 = _planted("seed-03", source)
    assert "items[2:4]" in ast.unparse(tree40)
    assert "items[2:5]" in ast.unparse(tree03)
    assert ast.dump(tree40) != ast.dump(tree03)


def test_seed_forty_one_never_plants_a_continue_in_a_loop_else_clause():
    source = (
        "def f(items):\n"
        "    for item in items:\n"
        "        pass\n"
        "    else:\n"
        "        continue\n"
    )
    applied, tree = _planted("seed-41", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_one_never_plants_a_continue_in_an_inner_loops_else_clause():
    source = (
        "def f(items):\n"
        "    for item in items:\n"
        "        for other in items:\n"
        "            pass\n"
        "        else:\n"
        "            continue\n"
    )
    applied, tree = _planted("seed-41", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_three_never_plants_an_augmented_assignment():
    source = "pair += (1, 2)\n"
    applied, tree = _planted("seed-43", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_two_refuses_a_declaration_only_handler_body():
    source = (
        "def f():\n"
        "    try:\n"
        "        pass\n"
        "    except ValueError:\n"
        "        global x\n"
    )
    applied, tree = _planted("seed-42", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_two_plants_a_declaration_beside_a_statement_in_a_handler_body():
    source = (
        "def f():\n"
        "    try:\n"
        "        pass\n"
        "    except ValueError:\n"
        "        global x\n"
        "        y = 1\n"
    )
    applied, tree = _planted("seed-42", source)
    assert applied
    rendered = ast.unparse(tree)
    assert "global x" not in rendered
    compile(rendered, "planted", "exec")


def test_seed_forty_two_refuses_a_declaration_only_finally_body():
    source = (
        "def f():\n"
        "    try:\n"
        "        pass\n"
        "    finally:\n"
        "        global x\n"
    )
    applied, tree = _planted("seed-42", source)
    assert not applied
    assert ast.dump(tree) == ast.dump(ast.parse(source))


def test_seed_forty_three_swaps_a_named_expression_standing_first():
    source = "pair = ((x := 1), 2)\n"
    applied, tree = _planted("seed-43", source)
    assert applied
    rendered = ast.unparse(tree)
    assert rendered == "pair = (2, (x := 1))"
    compile(rendered, "planted", "exec")


def test_seed_forty_three_swaps_a_named_expression_standing_second():
    source = "pair = (2, (x := 1))\n"
    applied, tree = _planted("seed-43", source)
    assert applied
    rendered = ast.unparse(tree)
    assert rendered == "pair = ((x := 1), 2)"
    compile(rendered, "planted", "exec")
