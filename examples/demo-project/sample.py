# Synthetic example target written for canaryfield.
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
