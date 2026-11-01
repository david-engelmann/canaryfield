"""The slice 1 seed registry.

Each seed is one defect drawn from a real defect class. A seed knows how to
look for the code shape it needs inside a parsed module and how to change
that shape. When the shape is absent the seed reports that it does not
apply, and the caller records not applicable for that target.

Transforms only ever run on scratch copies. Nothing in this module writes
to a file.
"""

import ast
from dataclasses import dataclass
from typing import Callable


def _sorted_nodes(tree):
    nodes = [node for node in ast.walk(tree) if hasattr(node, "lineno")]
    return sorted(nodes, key=lambda n: (n.lineno, n.col_offset, type(n).__name__))


def _is_int_constant(node):
    return (
        isinstance(node, ast.Constant)
        and isinstance(node.value, int)
        and not isinstance(node.value, bool)
    )


def _apply_stale_version(tree):
    for node in _sorted_nodes(tree):
        value = None
        if isinstance(node, ast.Assign) and isinstance(node.value, ast.Constant):
            if any(
                isinstance(target, ast.Name)
                and target.id in ("VERSION", "__version__", "version")
                for target in node.targets
            ):
                value = node.value
        if isinstance(node, ast.AnnAssign) and isinstance(node.value, ast.Constant):
            if isinstance(node.target, ast.Name) and node.target.id in (
                "VERSION",
                "__version__",
                "version",
            ):
                value = node.value
        if value is not None and isinstance(value.value, str):
            value.value = "9.9.9"
            return True
    return False


def _apply_dropped_return(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Return) and node.value is not None:
            node.value = None
            return True
    return False


_COMPARISON_FLIPS = {
    ast.Eq: ast.NotEq,
    ast.NotEq: ast.Eq,
    ast.Lt: ast.Gt,
    ast.Gt: ast.Lt,
    ast.LtE: ast.GtE,
    ast.GtE: ast.LtE,
    ast.In: ast.NotIn,
    ast.NotIn: ast.In,
    ast.Is: ast.IsNot,
    ast.IsNot: ast.Is,
}


def _apply_inverted_comparison(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Compare) and node.ops:
            flipped = _COMPARISON_FLIPS.get(type(node.ops[0]))
            if flipped is not None:
                node.ops[0] = flipped()
                return True
    return False


def _apply_off_by_one(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, (ast.Return, ast.Compare)):
            for inner in ast.walk(node):
                if _is_int_constant(inner):
                    inner.value = inner.value + 1
                    return True
    return False


def _apply_swallowed_exception(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.ExceptHandler):
            if len(node.body) == 1 and isinstance(node.body[0], ast.Pass):
                return False
            node.body = [ast.Pass()]
            return True
    return False


def _apply_inverted_boolean(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, bool):
            node.value = not node.value
            return True
    return False


def _apply_swapped_arithmetic(tree):
    swaps = {ast.Add: ast.Sub, ast.Sub: ast.Add, ast.Mult: ast.Add}
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.BinOp) and type(node.op) in swaps:
            node.op = swaps[type(node.op)]()
            return True
    return False


def _apply_dropped_call_argument(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Call) and len(node.args) >= 2:
            node.args = node.args[:-1]
            return True
    return False


def _apply_negated_condition(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.If):
            node.test = ast.UnaryOp(op=ast.Not(), operand=node.test)
            return True
    return False


def _apply_removed_element(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Dict) and node.keys:
            node.keys = node.keys[1:]
            node.values = node.values[1:]
            return True
        if isinstance(node, ast.List) and node.elts:
            node.elts = node.elts[1:]
            return True
    return False


def _apply_range_bound_shift(tree):
    for node in _sorted_nodes(tree):
        if (
            isinstance(node, ast.Call)
            and isinstance(node.func, ast.Name)
            and node.func.id == "range"
            and node.args
        ):
            last = node.args[-1]
            node.args[-1] = ast.BinOp(left=last, op=ast.Add(), right=ast.Constant(value=1))
            return True
    return False


def _apply_wrong_variable_returned(tree):
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.FunctionDef):
            params = [arg.arg for arg in node.args.args]
            if len(params) < 2:
                continue
            for inner in _sorted_nodes(node):
                if (
                    isinstance(inner, ast.Return)
                    and isinstance(inner.value, ast.Name)
                    and inner.value.id in params
                ):
                    index = params.index(inner.value.id)
                    inner.value.id = params[(index + 1) % len(params)]
                    return True
    return False


@dataclass(frozen=True)
class Seed:
    id: str
    name: str
    defect_class: str
    apply: Callable[[ast.AST], bool]
    description: str = ""


SEEDS = [
    Seed("seed-00", "stale version string", "stale metadata", _apply_stale_version, "Replaces a version literal with the stale string 9.9.9"),
    Seed("seed-01", "dropped return value", "missing result", _apply_dropped_return, "Drops the value from a return so the caller gets nothing"),
    Seed("seed-02", "inverted comparison", "wrong predicate", _apply_inverted_comparison, "Flips a comparison operator to its opposite"),
    Seed("seed-03", "off by one constant", "boundary error", _apply_off_by_one, "Adds one to an integer constant in a return or comparison"),
    Seed("seed-04", "swallowed exception", "hidden failure", _apply_swallowed_exception, "Replaces an exception handler body with a silent pass"),
    Seed("seed-05", "inverted boolean literal", "wrong flag", _apply_inverted_boolean, "Flips a boolean literal to its opposite"),
    Seed("seed-06", "swapped arithmetic operator", "wrong computation", _apply_swapped_arithmetic, "Swaps an arithmetic operator for a different one"),
    Seed("seed-07", "dropped call argument", "missing input", _apply_dropped_call_argument, "Drops the last argument from a call that takes at least two"),
    Seed("seed-08", "negated condition", "wrong branch", _apply_negated_condition, "Wraps an if condition in a negation"),
    Seed("seed-09", "removed collection element", "missing data", _apply_removed_element, "Removes the first element from a list or dictionary literal"),
    Seed("seed-10", "range bound shifted by one", "boundary error", _apply_range_bound_shift, "Raises the last bound of a range call by one"),
    Seed("seed-11", "wrong variable returned", "wrong result", _apply_wrong_variable_returned, "Returns the next parameter instead of the one the code named"),
]

SEEDS_BY_ID = {seed.id: seed for seed in SEEDS}
