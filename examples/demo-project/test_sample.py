# Synthetic example gate suite written for canaryfield.
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
