"""Synthetic fixture projects written by the tests.

Everything here is invented for this repo. The main fixture is a small
module plus a real pytest suite that acts as its gate. The suite covers
most of the module and deliberately leaves the banner punctuation and
the toppings list uncovered, so two seeds are honestly waved through.
"""

import sys

SAMPLE_SOURCE = '''\
VERSION = "1.4.2"


def add(a, b):
    return a + b


def pick(first, second):
    return first


def is_positive(n):
    return n > 0


def ready():
    return True


def countdown(n):
    return list(range(n))


def describe(flag):
    if flag:
        return "on"
    return "off"


def safe_div(a, b):
    try:
        return a / b
    except ZeroDivisionError:
        raise ValueError("bad division")


def greet(name, punct="!"):
    return name + punct


def banner():
    return greet("hi", "!")


def toppings():
    return ["soup", "salad", "cake"]
'''

TEST_SOURCE = '''\
import pytest

from sample import (
    VERSION,
    add,
    banner,
    countdown,
    describe,
    is_positive,
    pick,
    ready,
    safe_div,
)


def test_version():
    assert VERSION == "1.4.2"


def test_add():
    assert add(2, 3) == 5


def test_pick():
    assert pick("x", "y") == "x"


def test_is_positive():
    assert is_positive(1) is True
    assert is_positive(0) is False


def test_ready():
    assert ready() is True


def test_countdown():
    assert countdown(3) == [0, 1, 2]


def test_describe():
    assert describe(True) == "on"
    assert describe(False) == "off"


def test_safe_div():
    assert safe_div(6, 3) == 2
    with pytest.raises(ValueError):
        safe_div(1, 0)


def test_banner():
    assert banner() == "hi!"
'''

TINY_SOURCE = """\
def answer():
    return 42
"""

TINY_TEST_SOURCE = """\
from tiny import answer


def test_answer():
    assert answer() == 42
"""

GATE = ("tests", f"{sys.executable} -m pytest -q")


def write_main_project(root):
    root.mkdir(parents=True, exist_ok=True)
    (root / "sample.py").write_text(SAMPLE_SOURCE, encoding="utf-8")
    (root / "test_sample.py").write_text(TEST_SOURCE, encoding="utf-8")
    return root


def write_tiny_project(root):
    root.mkdir(parents=True, exist_ok=True)
    (root / "tiny.py").write_text(TINY_SOURCE, encoding="utf-8")
    (root / "test_tiny.py").write_text(TINY_TEST_SOURCE, encoding="utf-8")
    return root
