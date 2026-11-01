"""Tests for the Round 10 review fixes in CONTRACT.md.

The replacement a narrowed handler takes is the first
counted type the try body raises in source order, pinned
here over a body raising two counted types.
"""

import ast

from canaryfield.seeds import SEEDS_BY_ID


def test_a_narrowed_handler_takes_the_first_of_two_raised_types():
    source = (
        "def load(flag):\n"
        "    try:\n"
        "        if flag:\n"
        "            raise KeyError('missing')\n"
        "        raise ValueError('bad')\n"
        "    except Exception:\n"
        "        return None\n"
    )
    tree = ast.parse(source)
    assert SEEDS_BY_ID["seed-32"].apply(tree) is True
    rendered = ast.unparse(tree)
    assert "except KeyError:" in rendered
    assert "except ValueError:" not in rendered
