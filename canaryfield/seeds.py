"""The seed registry.

Each seed is one defect drawn from a real defect class. A seed knows how to
look for the code shape it needs inside a parsed module and how to change
that shape. When the shape is absent the seed reports that it does not
apply, and the caller records not applicable for that target.

One call to a seed apply function plants at every match in the tree it is
given. The matches are collected from the tree as parsed, before any
change is made, and each matched node is planted exactly once, so a
planting never creates a new match for itself inside the same call and a
planted shape that would itself match is never planted a second time.

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
    targets = []
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
            targets.append(value)
    for value in targets:
        value.value = "9.9.9"
    return bool(targets)


def _apply_dropped_return(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Return) and node.value is not None
    ]
    for node in targets:
        node.value = None
    return bool(targets)


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
    targets = []
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Compare) and node.ops:
            flipped = _COMPARISON_FLIPS.get(type(node.ops[0]))
            if flipped is not None:
                targets.append((node, flipped))
    for node, flipped in targets:
        node.ops[0] = flipped()
    return bool(targets)


def _apply_off_by_one(tree):
    targets = []
    seen = set()
    for node in _sorted_nodes(tree):
        if isinstance(node, (ast.Return, ast.Compare)):
            for inner in ast.walk(node):
                if _is_int_constant(inner) and id(inner) not in seen:
                    seen.add(id(inner))
                    targets.append(inner)
    for target in targets:
        target.value = target.value + 1
    return bool(targets)


def _apply_swallowed_exception(tree):
    targets = []
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.ExceptHandler):
            if len(node.body) == 1 and isinstance(node.body[0], ast.Pass):
                continue
            targets.append(node)
    for node in targets:
        node.body = [ast.Pass()]
    return bool(targets)


def _apply_inverted_boolean(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Constant) and isinstance(node.value, bool)
    ]
    for node in targets:
        node.value = not node.value
    return bool(targets)


def _apply_swapped_arithmetic(tree):
    swaps = {ast.Add: ast.Sub, ast.Sub: ast.Add, ast.Mult: ast.Add}
    targets = [
        (node, swaps[type(node.op)])
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.BinOp) and type(node.op) in swaps
    ]
    for node, replacement in targets:
        node.op = replacement()
    return bool(targets)


def _apply_dropped_call_argument(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Call) and len(node.args) >= 2
    ]
    for node in targets:
        node.args = node.args[:-1]
    return bool(targets)


def _apply_negated_condition(tree):
    targets = [
        node for node in _sorted_nodes(tree) if isinstance(node, ast.If)
    ]
    for node in targets:
        node.test = ast.UnaryOp(op=ast.Not(), operand=node.test)
    return bool(targets)


def _apply_removed_element(tree):
    targets = []
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.Dict) and node.keys:
            targets.append(node)
        elif isinstance(node, ast.List) and node.elts:
            targets.append(node)
    for node in targets:
        if isinstance(node, ast.Dict):
            node.keys = node.keys[1:]
            node.values = node.values[1:]
        else:
            node.elts = node.elts[1:]
    return bool(targets)


def _apply_range_bound_shift(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if (
            isinstance(node, ast.Call)
            and isinstance(node.func, ast.Name)
            and node.func.id == "range"
            and node.args
        )
    ]
    for node in targets:
        last = node.args[-1]
        node.args[-1] = ast.BinOp(left=last, op=ast.Add(), right=ast.Constant(value=1))
    return bool(targets)


def _own_returns(node):
    """The return statements that belong to one function body.

    The walk descends through the statements of the body and
    stops at any nested function or class, because a return
    inside one of those belongs to that nested scope and is
    judged against its own parameter list when the walk of the
    whole tree reaches it.
    """
    found = []
    stack = list(node.body)
    while stack:
        current = stack.pop()
        if isinstance(
            current,
            (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Lambda),
        ):
            continue
        if isinstance(current, ast.Return):
            found.append(current)
        stack.extend(ast.iter_child_nodes(current))
    return found


def _apply_wrong_variable_returned(tree):
    targets = []
    planted = set()
    for node in _sorted_nodes(tree):
        if not isinstance(node, ast.FunctionDef):
            continue
        params = [arg.arg for arg in node.args.args]
        if len(params) < 2:
            continue
        for inner in _own_returns(node):
            if (
                isinstance(inner.value, ast.Name)
                and inner.value.id in params
                and id(inner) not in planted
            ):
                planted.add(id(inner))
                index = params.index(inner.value.id)
                targets.append((inner, params[(index + 1) % len(params)]))
    for inner, name in targets:
        inner.value.id = name
    return bool(targets)


_OPERAND_SWAP_OPS = (ast.Sub, ast.Div, ast.FloorDiv, ast.Mod)


def _apply_swapped_operand_order(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.BinOp) and isinstance(node.op, _OPERAND_SWAP_OPS)
    ]
    for node in targets:
        node.left, node.right = node.right, node.left
    return bool(targets)


def _apply_dropped_else(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.If) and node.orelse
    ]
    for node in targets:
        node.orelse = []
    return bool(targets)


def _apply_swapped_boolean_operator(tree):
    targets = [
        node for node in _sorted_nodes(tree) if isinstance(node, ast.BoolOp)
    ]
    for node in targets:
        node.op = ast.Or() if isinstance(node.op, ast.And) else ast.And()
    return bool(targets)


def _collect_awaits(tree):
    """The await expressions of a tree with their parents.

    Each entry carries the depth of the await, the parent node,
    the field the await sits in, the index inside that field when
    the field is a list, and the await node itself. The entries
    come back deepest first, so replacing an inner await inside
    its parent await never strands the inner planting on a node
    the outer planting already detached from the tree.
    """
    found = []

    def visit(node, depth):
        for field, value in ast.iter_fields(node):
            if isinstance(value, list):
                for index, item in enumerate(value):
                    if isinstance(item, ast.AST):
                        if isinstance(item, ast.Await):
                            found.append(
                                (depth + 1, node, field, index, item)
                            )
                        visit(item, depth + 1)
            elif isinstance(value, ast.AST):
                if isinstance(value, ast.Await):
                    found.append((depth + 1, node, field, None, value))
                visit(value, depth + 1)

    visit(tree, 0)
    found.sort(key=lambda entry: -entry[0])
    return found


def _apply_dropped_await(tree):
    targets = _collect_awaits(tree)
    for _depth, parent, field, index, node in targets:
        if index is None:
            setattr(parent, field, node.value)
        else:
            getattr(parent, field)[index] = node.value
    return bool(targets)


_BROAD_HANDLER_NAMES = ("Exception", "BaseException")


def _handler_type_names(handler_type):
    if isinstance(handler_type, ast.Name):
        return {handler_type.id}
    if isinstance(handler_type, ast.Tuple):
        return {
            member.id
            for member in handler_type.elts
            if isinstance(member, ast.Name)
        }
    return set()


def _apply_widened_exception_handler(tree):
    targets = []
    for node in _sorted_nodes(tree):
        if not isinstance(node, ast.ExceptHandler):
            continue
        if node.type is None:
            continue
        if _handler_type_names(node.type) & set(_BROAD_HANDLER_NAMES):
            continue
        targets.append(node)
    for node in targets:
        node.type = ast.Name(id="Exception", ctx=ast.Load())
    return bool(targets)


def _apply_swapped_dictionary_entry(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Dict)
        and any(key is not None for key in node.keys)
    ]
    for node in targets:
        pairs = [
            (key, value) if key is None else (value, key)
            for key, value in zip(node.keys, node.values)
        ]
        node.keys = [key for key, _value in pairs]
        node.values = [value for _key, value in pairs]
    return bool(targets)


def _is_negation(node):
    return isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.Not)


def _apply_inverted_truth_test(tree):
    targets = []
    for node in _sorted_nodes(tree):
        if isinstance(node, ast.While):
            targets.append(node)
        elif isinstance(node, (ast.If, ast.IfExp)) and _is_negation(
            node.test
        ):
            targets.append(node)
    for node in targets:
        if _is_negation(node.test):
            node.test = node.test.operand
        else:
            node.test = ast.UnaryOp(op=ast.Not(), operand=node.test)
    return bool(targets)


_COMPREHENSIONS = (
    ast.ListComp,
    ast.SetComp,
    ast.DictComp,
    ast.GeneratorExp,
)


def _bound_names_in_scope(scope):
    """The names one scope binds, parameters first, then source order.

    The walk reads the body of the scope and stops at any nested
    function or class, because a name bound in a nested scope is
    not bound in this one, and it never descends into a
    comprehension, because a name bound only inside a comprehension
    is not bound in this scope either. A function or class defined
    in the body binds its own name in this scope.
    """
    found = []

    def note(position, name):
        found.append((position, name))

    if isinstance(scope, (ast.FunctionDef, ast.AsyncFunctionDef)):
        args = scope.args
        params = [
            arg.arg for arg in (*args.posonlyargs, *args.args)
        ]
        if args.vararg is not None:
            params.append(args.vararg.arg)
        params.extend(arg.arg for arg in args.kwonlyargs)
        if args.kwarg is not None:
            params.append(args.kwarg.arg)
        for index, name in enumerate(params):
            note((0, 0, index), name)
    stack = list(scope.body)
    while stack:
        current = stack.pop()
        if isinstance(
            current, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
        ):
            note((current.lineno, current.col_offset, 0), current.name)
            continue
        if isinstance(current, (ast.Lambda, *_COMPREHENSIONS)):
            continue
        if isinstance(current, ast.Name) and isinstance(
            current.ctx, ast.Store
        ):
            note((current.lineno, current.col_offset, 0), current.id)
        elif isinstance(current, ast.Import):
            for index, alias in enumerate(current.names):
                note(
                    (current.lineno, current.col_offset, index),
                    alias.asname or alias.name.split(".")[0],
                )
        elif isinstance(current, ast.ImportFrom):
            for index, alias in enumerate(current.names):
                note(
                    (current.lineno, current.col_offset, index),
                    alias.asname or alias.name,
                )
        elif isinstance(current, ast.ExceptHandler) and current.name:
            note((current.lineno, current.col_offset, 0), current.name)
        stack.extend(ast.iter_child_nodes(current))
    names = []
    for _position, name in sorted(found):
        if name not in names:
            names.append(name)
    return names


def _collect_loop_retargets(tree):
    """The for loops of a tree with the name each one is pointed at.

    Each entry carries the loop node and the first name bound in
    the scope that encloses the loop other than the name the loop
    already targets, and a loop whose scope binds no other name
    earns no entry. The entries are collected from the tree as
    parsed, before any loop is pointed anywhere.
    """
    found = []

    def visit(node, scope):
        if isinstance(
            node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
        ):
            scope = node
        if isinstance(node, (ast.For, ast.AsyncFor)) and isinstance(
            node.target, ast.Name
        ):
            for name in _bound_names_in_scope(scope):
                if name != node.target.id:
                    found.append((node, name))
                    break
        for child in ast.iter_child_nodes(node):
            visit(child, scope)

    visit(tree, tree)
    return found


def _apply_reassigned_loop_variable(tree):
    targets = _collect_loop_retargets(tree)
    for node, name in targets:
        node.target = ast.Name(id=name, ctx=ast.Store())
    return bool(targets)


def _apply_swapped_call_argument_order(tree):
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Call) and len(node.args) >= 2
    ]
    for node in targets:
        node.args[0], node.args[1] = node.args[1], node.args[0]
    return bool(targets)


_VERSION_NAMES = ("VERSION", "__version__", "version")


def _out_of_scope_string_ids(tree):
    """The ids of the string constants seed-21 never plants.

    A docstring is the first statement of a module or function or
    class body when that statement is a bare string expression, and
    it is documentation rather than behaviour. The literal a
    version assignment carries whole belongs to seed-00, which
    replaces it entire. Both shapes are gathered here so the apply
    function can read only the constants that stand anywhere else.
    """
    excluded = set()
    for node in ast.walk(tree):
        if isinstance(
            node,
            (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef),
        ):
            if (
                node.body
                and isinstance(node.body[0], ast.Expr)
                and isinstance(node.body[0].value, ast.Constant)
                and isinstance(node.body[0].value.value, str)
            ):
                excluded.add(id(node.body[0].value))
        value = None
        if isinstance(node, ast.Assign) and isinstance(node.value, ast.Constant):
            if any(
                isinstance(target, ast.Name) and target.id in _VERSION_NAMES
                for target in node.targets
            ):
                value = node.value
        if isinstance(node, ast.AnnAssign) and isinstance(node.value, ast.Constant):
            if isinstance(node.target, ast.Name) and node.target.id in _VERSION_NAMES:
                value = node.value
        if value is not None and isinstance(value.value, str):
            excluded.add(id(value))
    return excluded


def _apply_altered_string_literal(tree):
    excluded = _out_of_scope_string_ids(tree)
    targets = [
        node
        for node in _sorted_nodes(tree)
        if isinstance(node, ast.Constant)
        and isinstance(node.value, str)
        and node.value
        and id(node) not in excluded
    ]
    for node in targets:
        first = node.value[0]
        node.value = ("z" if first == "x" else "x") + node.value[1:]
    return bool(targets)


def _collect_negations(tree):
    """The unary minus expressions of a tree with their parents.

    Each entry carries the depth of the negation, the parent node,
    the field the negation sits in, the index inside that field
    when the field is a list, and the negation node itself. The
    entries come back deepest first, so removing an inner negation
    inside its parent negation never strands the inner planting on
    a node the outer planting already detached from the tree. The
    collection mirrors the one the dropped await seed uses.
    """
    found = []

    def is_negation(node):
        return isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub)

    def visit(node, depth):
        for field, value in ast.iter_fields(node):
            if isinstance(value, list):
                for index, item in enumerate(value):
                    if isinstance(item, ast.AST):
                        if is_negation(item):
                            found.append((depth + 1, node, field, index, item))
                        visit(item, depth + 1)
            elif isinstance(value, ast.AST):
                if is_negation(value):
                    found.append((depth + 1, node, field, None, value))
                visit(value, depth + 1)

    visit(tree, 0)
    found.sort(key=lambda entry: -entry[0])
    return found


def _apply_dropped_numeric_negation(tree):
    targets = _collect_negations(tree)
    for _depth, parent, field, index, node in targets:
        if index is None:
            setattr(parent, field, node.operand)
        else:
            getattr(parent, field)[index] = node.operand
    return bool(targets)


def _collect_loop_breaks(tree):
    """The break statements standing in the body of a loop.

    Each entry carries the parent node, the field the break sits
    in, the index inside that field when the field is a list, and
    the break node itself. The walk turns its inside a loop
    reading on for the body of a for or while loop and off again
    for the else clause of the loop, and it never carries the
    reading across a function or class or lambda boundary, because
    a break inside one of those does not belong to the loop
    around it.
    """
    found = []

    def visit(node, parent, field, index, inside_loop_body):
        if isinstance(
            node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef, ast.Lambda)
        ):
            inside_loop_body = False
        if isinstance(node, ast.Break) and inside_loop_body:
            found.append((parent, field, index, node))
        if isinstance(node, (ast.For, ast.AsyncFor, ast.While)):
            for position, statement in enumerate(node.body):
                visit(statement, node, "body", position, True)
            for position, statement in enumerate(node.orelse):
                visit(statement, node, "orelse", position, False)
            return
        for child_field, value in ast.iter_fields(node):
            if isinstance(value, list):
                for position, item in enumerate(value):
                    if isinstance(item, ast.AST):
                        visit(item, node, child_field, position, inside_loop_body)
            elif isinstance(value, ast.AST):
                visit(value, node, child_field, None, inside_loop_body)

    visit(tree, None, None, None, False)
    return found


def _apply_swapped_loop_control(tree):
    targets = _collect_loop_breaks(tree)
    for parent, field, index, _node in targets:
        replacement = ast.Continue()
        if index is None:
            setattr(parent, field, replacement)
        else:
            getattr(parent, field)[index] = replacement
    return bool(targets)


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
    Seed("seed-12", "swapped operand order", "wrong computation", _apply_swapped_operand_order, "Swaps the operands of a subtraction, division, floor division, or modulo, where the order of the operands decides the result"),
    Seed("seed-13", "dropped else branch", "missing fallback", _apply_dropped_else, "Drops the else branch of a conditional so the fallback path never runs"),
    Seed("seed-14", "swapped boolean operator", "wrong predicate", _apply_swapped_boolean_operator, "Swaps a boolean and for an or, or an or for an and, in a compound condition"),
    Seed("seed-15", "dropped await", "missing wait", _apply_dropped_await, "Removes an await so the caller holds a coroutine instead of the awaited result"),
    Seed("seed-16", "widened exception handler", "hidden failure", _apply_widened_exception_handler, "Widens an exception handler so it catches the broad base error type"),
    Seed("seed-17", "swapped dictionary entry", "wrong data", _apply_swapped_dictionary_entry, "Swaps the key and the value of a dictionary entry so the mapping answers backwards"),
    Seed("seed-18", "inverted truth test", "wrong branch", _apply_inverted_truth_test, "Toggles the negation on a condition test so its sense reads backwards on the shapes no other seed plants"),
    Seed("seed-19", "reassigned loop variable", "wrong variable", _apply_reassigned_loop_variable, "Points a for loop at a different name already bound in the same scope so the loop writes into the wrong variable"),
    Seed("seed-20", "swapped call argument order", "wrong input", _apply_swapped_call_argument_order, "Swaps the first two positional arguments of a call so each value reaches the wrong parameter"),
    Seed("seed-21", "altered string literal", "wrong text", _apply_altered_string_literal, "Changes the first character of a string literal so the text the code compares or returns is no longer the text it wrote"),
    Seed("seed-22", "dropped numeric negation", "wrong sign", _apply_dropped_numeric_negation, "Removes a unary minus so a value carries the opposite sign from the one the code wrote"),
    Seed("seed-23", "swapped loop control", "wrong flow", _apply_swapped_loop_control, "Turns a break inside a loop into a continue so the loop runs on instead of ending"),
]

SEEDS_BY_ID = {seed.id: seed for seed in SEEDS}
