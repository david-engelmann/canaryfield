"""Recorded conformance generator for canaryfield PORT2.

Runs the landed Python over the corpus and records its answers.
Usage: python3 tools/gen_port2_recordings.py <repo_root> <out_json>.
The repo root is a tree carrying the landed Python package, the
spec the recordings in tests/conformance/port2_cases.json were
taken from before the seed module retired. Recordings are never
hand written.
"""
import ast
import hashlib
import json
import sys
import tempfile
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2])
sys.path.insert(0, str(ROOT))

if sys.implementation.name != "cpython" or sys.version_info[:2] != (3, 12):
    raise SystemExit(
        "the recordings stand under CPython 3.12 only and this "
        f"interpreter is {sys.implementation.name} "
        f"{sys.version_info.major}.{sys.version_info.minor}"
    )

from canaryfield.seeds import SEEDS  # noqa: E402
from canaryfield import engine  # noqa: E402

TEXT_LIMIT = 6000


def spell(text):
    """Record a long text as a digest with its length, a short one whole."""
    if len(text) <= TEXT_LIMIT:
        return {"t": text}
    return {"sha256": hashlib.sha256(text.encode("utf-8")).hexdigest(), "len": len(text)}


def spell_bytes(data):
    if len(data) <= TEXT_LIMIT:
        try:
            return {"t": data.decode("utf-8")}
        except UnicodeDecodeError:
            pass
    return {"b64sha": hashlib.sha256(data).hexdigest(), "len": len(data)}


def surface(fn):
    """One recorded surface, a spelling or the recursion marker."""
    try:
        return spell(fn())
    except RecursionError:
        return "recursion"


def record_application(seed, tree, standing_dump, standing_render):
    """One application of a seed over a parsed tree.

    A surface whose spelling equals the standing one is left
    out of the record, and the battery reads the standing
    spelling it already holds in its place. An application
    the Python raises out of is recorded with the exception
    type it raised, the answer the battery holds the port's
    refusal against.
    """
    try:
        planted = seed.apply(tree)
    except Exception as error:  # noqa: BLE001, the raise is the answer
        return {"apply": "raised", "error": type(error).__name__}, standing_dump, standing_render
    record = {"bool": planted}
    dump = surface(lambda: ast.dump(tree))
    if dump != standing_dump:
        record["dump"] = dump
    render = surface(lambda: ast.unparse(tree))
    if render != standing_render:
        record["render"] = render
    return record, dump, render


def apply_case(name, source):
    case = {"kind": "apply", "name": name, "source": source}
    try:
        tree = ast.parse(source)
    except (SyntaxError, ValueError):
        case["parse"] = "refused"
        return case
    case["parse"] = "ok"
    case["dump0"] = surface(lambda: ast.dump(tree))
    case["render0"] = surface(lambda: ast.unparse(tree))
    seeds = {}
    for seed in SEEDS:
        fresh = ast.parse(source)
        first, dump1, render1 = record_application(
            seed, fresh, case["dump0"], case["render0"]
        )
        entry = {"first": first}
        if "bool" in first:
            second, _d, _r = record_application(seed, fresh, dump1, render1)
            entry["second"] = second
        seeds[seed.id] = entry
    case["seeds"] = seeds
    return case


def harvest_sources():
    """The suite's own inputs, drawn mechanically from its files."""
    found = {}
    refused = []
    test_dir = ROOT / "tests"
    for path in sorted(test_dir.glob("*.py")):
        text = path.read_text(encoding="utf-8")
        try:
            module = ast.parse(text)
        except SyntaxError:
            continue
        for node in ast.walk(module):
            if not (isinstance(node, ast.Constant) and isinstance(node.value, str)):
                continue
            value = node.value
            if len(value) < 8 or len(value) > 20000:
                continue
            try:
                value.encode("utf-8")
            except UnicodeEncodeError:
                continue
            if not value.strip():
                continue
            digest = hashlib.sha256(value.encode("utf-8")).hexdigest()[:12]
            try:
                ast.parse(value)
            except (SyntaxError, ValueError):
                if ("\n" in value or "def " in value) and len(refused) < 40:
                    refused.append((f"refused:{path.name}:{digest}", value))
                continue
            found[f"suite:{path.name}:{digest}"] = value
    return found, refused


def file_sources():
    out = {}
    for folder in ("canaryfield", "tests"):
        for path in sorted((ROOT / folder).glob("*.py")):
            text = path.read_text(encoding="utf-8")
            out[f"file:{folder}/{path.name}"] = text
    sample = ROOT / "examples" / "demo-project" / "sample.py"
    out["file:examples/demo-project/sample.py"] = sample.read_text(encoding="utf-8")
    return out


SYNTHETIC = {
    "empty": "",
    "only-pass": "pass\n",
    "version-forms": (
        "VERSION = '1.2.3'\n__version__ = \"0.1\"\nversion: str = '2.0'\n"
        "other = 'x'\nVERSION = 3\nversion = VERSION\n"
    ),
    "already-planted-version": "VERSION = '9.9.9'\n",
    "u-version": "VERSION = u'1.0'\n",
    "returns": (
        "def f(a, b):\n    if a:\n        return a\n    return b\n"
        "def g():\n    return\n"
        "async def h(x, y):\n    return x\n"
    ),
    "compares": (
        "a = x == y\na = x != y\na = x < y <= z\na = x is y\na = x is not y\n"
        "a = x in y\na = x not in y\na = -1 < b\ndef f():\n    return a < b < c\n"
    ),
    "bool-consts": "a = True\nb = False\nc = [True, False]\nd = x if True else False\n",
    "arith": (
        "a = x + y\na = x - y\na = x * y\na = x / y\na = x // y\na = x % y\n"
        "a = x ** y\na = x @ y\na += 1\na -= 1\na *= 2\na /= 2\n"
    ),
    "calls": (
        "f(a)\nf(a, b)\nf(a, b, c)\nf(a, *rest)\nf(key=1)\nf(a, key=1, other=2)\n"
        "f(**kw)\nf(a, **kw)\nobj.method(a, b)\nrange(10)\nrange(1, 10)\n"
        "range(1, 10, 2)\nrange(1, 10, step)\nrange(1, 10, -1)\nrange(1, 10, 0)\n"
        "range(1, 10, 0.0)\nrange(1, 10, True)\nrange(1, 10, +0)\nrange(1, 10, *s)\n"
    ),
    "if-shapes": (
        "if a:\n    pass\nif not a:\n    pass\nelse:\n    pass\n"
        "if a:\n    pass\nelif not b:\n    pass\nelse:\n    pass\n"
        "while a:\n    pass\nwhile not a:\n    pass\n"
        "x = a if not b else c\ny = a if b else c\n"
    ),
    "collections": (
        "a = [1, 2, 3]\nb = {1: 'a', 2: 'b'}\nc = {**d}\nd = {}\ne = []\n"
        "f = {None: 1}\ng = {'k': v, **rest, 'j': w}\nh = (1, 2)\n"
    ),
    "dict-selfmap-17": "d = {key: key}\n",
    "dict-swap-17": "d = {key: key}\ne = {a: b, c: d}\n",
    "handlers": (
        "try:\n    risky()\nexcept ValueError:\n    raise\n"
        "try:\n    risky()\nexcept (KeyError, IndexError) as e:\n    handle(e)\n"
    ),
    "try-shapes": (
        "try:\n    raise ValueError('x')\nexcept ValueError:\n    pass\n"
        "try:\n    pass\nexcept Exception:\n    pass\nfinally:\n    cleanup()\n"
        "try:\n    pass\nexcept* ValueError:\n    pass\nfinally:\n    pass\n"
        "try:\n    raise SpecificError()\nexcept Exception:\n    pass\n"
        "try:\n    raise SpecificError()\nexcept BaseException:\n    pass\n"
        "try:\n    pass\nexcept mod.Error:\n    pass\n"
        "try:\n    pass\nexcept:\n    pass\n"
        "try:\n    pass\nexcept Exception:\n    pass\nexcept ValueError:\n    pass\nelse:\n    pass\n"
        "try:\n    pass\nexcept Exception:\n    pass\nexcept ValueError:\n    pass\nfinally:\n    pass\n"
        "def outer():\n    try:\n        def inner():\n            raise HiddenError()\n"
        "    except Exception:\n        pass\n"
    ),
    "raise-shapes": (
        "def f():\n    raise ValueError('a')\n"
        "try:\n    pass\nexcept KeyError:\n    pass\n"
        "def g():\n    raise lower_case()\n    raise _Private()\n    raise mod.Error()\n"
    ),
    "loops": (
        "for i in items:\n    total += i\nelse:\n    done()\n"
        "for i in items:\n    if i:\n        break\n    continue\n"
        "while x:\n    break\nelse:\n    pass\n"
        "async def f():\n    async for row in rows:\n        pass\n"
        "for a, b in pairs:\n    pass\n"
        "class C:\n    for i in items:\n        pass\n"
        "total = 0\nfor i in items:\n    pass\n"
        "def f(items):\n    for i in items:\n        pass\n"
        "def g(items, other):\n    for i in items:\n        pass\n"
    ),
    "loop-scope-19": (
        "def f(a, *rest, key, **kw):\n    for i in a:\n        pass\n"
        "def g():\n    import os.path\n    from mod import thing as t\n"
        "    for i in x:\n        pass\n"
        "def h():\n    try:\n        pass\n    except ValueError as err:\n"
        "        for i in x:\n            pass\n"
        "def k():\n    comp = [i for i in y]\n    for i in x:\n        pass\n"
        "def m():\n    def inner():\n        bound = 1\n    for i in x:\n        pass\n"
        "def n():\n    if (w := 5):\n        pass\n    for i in x:\n        pass\n"
    ),
    "strings-21": (
        "a = 'hello'\nb = 'xray'\nc = ''\nd = b'bytes'\ne = 'x'\n"
        "def f():\n    '''docstring'''\n    s = 'inner'\n    return 'ret'\n"
        "class C:\n    'class doc'\n    x = 'attr'\n"
        "s = f'abc {v} def'\nt = f'{v}'\nVERSION = '1.0'\n"
        "module_text = 'standalone'\n"
    ),
    "negations": (
        "a = -5\nb = -x\nc = --y\nd = not flag\ne = not not flag\n"
        "if not a:\n    pass\nx = -1\ny = ---z\n"
    ),
    "negation-tests-47": (
        "x = not a\nif not b:\n    pass\nwhile not c:\n    pass\n"
        "y = a if not d else e\nif check(not f):\n    pass\n"
        "match v:\n    case _ if not g:\n        pass\n"
        "def f():\n    return not h\nz = [not i]\n"
    ),
    "defaults-30-33": (
        "def f(a=1, b=2, c='x'):\n    pass\n"
        "def g(a, *, b=None, c=2, d):\n    pass\n"
        "def h(a=1.5, b=True, c=-3):\n    pass\n"
        "lam = lambda a=1, *, b=None, c=2: a\n"
        "def k(a=(1, 2), b=[3]):\n    pass\n"
    ),
    "siblings-31": (
        "def f(obj):\n    a = obj.right\n    b = obj.left\n    return obj.right\n"
        "def g(obj):\n    obj = reset()\n    return obj.right + obj.left\n"
        "def h():\n    cfg = load()\n    x = cfg.alpha\n    y = cfg.beta\n    z = cfg.alpha\n"
        "class C:\n    def m(self):\n        return self.first + self.second\n"
        "def outer(obj):\n    def inner(p):\n        return p.aa + p.bb\n    return obj.xx + obj.yy\n"
        "def withstore(obj):\n    obj.x = 1\n    return obj.aa + obj.bb\n"
    ),
    "declarations-42": (
        "def f():\n    global g\n    g = 1\n"
        "def f2():\n    global g\n"
        "def outer():\n    x = 1\n    def inner():\n        nonlocal x\n        x = 2\n"
        "def f3():\n    global a\n    global b\n    x = 1\n"
        "def f4():\n    global a\n    x = 1\n    global b\n"
    ),
    "subscripts-slices": (
        "a = items[1]\nb = items[-1]\nc = items[0]\nd = items[1:2]\n"
        "e = items[:5]\nf = items[2:]\ng = items[::2]\nh = items[x]\n"
        "i = items[True]\nj = items[1:9:2]\nk = d['key']\n"
    ),
    "decorators-bases": (
        "@ deco\n@mod.deco(arg)\ndef f():\n    pass\n"
        "class C(Base, Other):\n    pass\nclass D:\n    pass\n"
        "@deco\nclass E(Base):\n    pass\n"
    ),
    "yields-awaits": (
        "def gen():\n    yield 1\n    yield\n    x = yield 2\n    yield from other()\n"
        "async def agen():\n    await ready()\n    x = await fetch(a, await fetch(b))\n"
        "async def nested():\n    return await (await inner())\n"
    ),
    "assign-tuples-43": (
        "a, b = 1, 2\nx = (1, 2, 3)\ny: tuple = (4, 5)\nz = (1,)\nw = 5\n"
    ),
    "trystar": (
        "try:\n    pass\nexcept* ValueError:\n    pass\n"
        "try:\n    raise SpecificError()\nexcept* Exception:\n    pass\nfinally:\n    pass\n"
        "try:\n    pass\nexcept* ValueError:\n    pass\nexcept* KeyError:\n    pass\nfinally:\n    pass\n"
    ),
    "match-shapes": (
        "match v:\n    case 1:\n        pass\n    case [a, *rest] if not flag:\n        pass\n"
        "    case {'k': x, **rest}:\n        pass\n    case Point(x=0) | None:\n        pass\n"
        "    case _:\n        pass\n"
    ),
    "match-star-paren": (
        "match v:\n    case (first, *rest):\n        pass\n"
    ),
    "match-star-open": (
        "match v:\n    case first, *rest:\n        pass\n"
    ),
    "match-star-value": (
        "match v:\n    case (1, *rest):\n        pass\n"
    ),
    "match-star-bracket": (
        "match v:\n    case [first, *rest]:\n        pass\n"
    ),
    "double-await": (
        "async def f():\n    await await g()\n"
    ),
    "big-ints": (
        "x = 123456789012345678901234567890\n"
        "def f(a=99999999999999999999999999):\n    return 18446744073709551616\n"
    ),
    "fstring-parts-21": (
        "a = f'xxx {v}'\nb = f'{v} tail'\nc = f'pre {v} post'\n"
        "d = f'{v:{w}}'\ne = f'{u!r}'\n"
    ),
    "walrus": (
        "if (n := len(a)) > 2:\n    pass\nwhile (line := read()):\n    pass\n"
        "vals = [y for x in xs if (y := check(x))]\n"
    ),
    "comprehensions": (
        "a = [x for x in xs]\nb = {k: v for k, v in pairs if k}\n"
        "c = (x async for x in agen())\nd = {x for x in xs if x if x}\n"
    ),
    "one-position-ties": (
        "x = f(a)\ny = obj.attr\nz = a and b\nw = -a\nv = a[0]\n"
        "def f():\n    return check(a) == other(b)\n"
    ),
    "nested-try-32": (
        "try:\n    try:\n        raise InnerError()\n    except Exception:\n        pass\n"
        "except Exception:\n    pass\n"
    ),
}

DEPTH_LADDER = [100, 300, 320, 400, 900, 985, 995, 1100, 2000]


def depth_cases():
    cases = {}
    for n in DEPTH_LADDER:
        cases[f"depth-not-{n}"] = "x = " + "not " * n + "y\n"
        cases[f"depth-neg-{n}"] = "x = " + "-" * n + "y\n"
    for n in (300, 400, 900, 1100):
        cases[f"depth-bin-{n}"] = "x = " + "a + " * n + "b\n"
        cases[f"depth-attr-{n}"] = "x = a" + ".b" * n + "\n"
    cases["depth-dict-400"] = "d = {'k': " + "not " * 400 + "y}\n"
    cases["depth-try-100"] = (
        "try:\n    " + "x = " + "not " * 100 + "y\n"
        "except Exception:\n    pass\n"
    )
    return cases


def plant_tree_spec(files):
    return files


def plant_cases():
    """Staged directory shapes for plant_seed, as file maps.

    A value is file text, or a ("bytes", b64) pair through the
    bytes field, or a symlink through the link field.
    """
    demo = (ROOT / "examples" / "demo-project" / "sample.py").read_text(encoding="utf-8")
    demo_test = (ROOT / "examples" / "demo-project" / "test_sample.py").read_text(encoding="utf-8")
    sets = {
        "demo": {"sample.py": demo, "test_sample.py": demo_test},
        "candidates": {
            "mod.py": "x = -1\nVERSION = '1.0'\n",
            "contest.py": "def broken(:\n",
            "test_mod.py": "y = -2\n",
            "mod_test.py": "y = -2\n",
            "conftest.py": "y = -2\n",
            "test.py": "y = -2\n",
            "tests/helper.py": "y = -2\n",
            "testing/helper2.py": "y = -2\n",
            "stem.py": "def broken(:\n",
            "stem/inner.py": "z = not flag\n",
            ".git/hidden.py": "w = -1\n",
            "__pycache__/cached.py": "w = -1\n",
            "sub/deep.py": "a = [1, 2, 3]\n",
        },
        "broken-and-odd": {
            "good.py": "x = -5\n",
            "bad.py": "def broken(:\n",
            "empty.py": "",
            "bom.py": "﻿x = 1\n",
            "cookie.py": "# -*- coding: latin-1 -*-\nx = 'caf\\xe9'\n",
        },
        "nonutf8": {
            "good.py": "x = -5\n",
            "raw.py": {"bytes": b"x = -1\n# caf\xff\xfe bytes\n".hex()},
        },
        "dict-nochange-17": {"only.py": "d = {key: key}\n"},
        "nonlocal-34": {
            "mod.py": (
                "def outer():\n    def inner():\n        nonlocal x\n"
                "        return x\n    try:\n        pass\n    finally:\n"
                "        x = 1\n    return inner\n"
            ),
        },
        "nonlocal-36": {
            "mod.py": (
                "def outer():\n    def inner():\n        nonlocal x\n"
                "        return x\n    try:\n        pass\n"
                "    except ValueError:\n        x = 1\n"
                "    except TypeError:\n        pass\n    return inner\n"
            ),
        },
        "nonlocal-38": {
            "mod.py": (
                "def outer():\n    def inner():\n        nonlocal x\n"
                "        return x\n    ok = flag and (x := 3)\n"
                "    return inner\n"
            ),
        },
        "nonlocal-39": {
            "mod.py": (
                "def outer():\n    def make():\n        nonlocal x\n"
                "        return x\n    class C((x := Base)):\n"
                "        pass\n    return make\n"
            ),
        },
        "depth-250": {"deep.py": "x = " + "not " * 250 + "y\n"},
        "depth-450": {"deep.py": "x = " + "not " * 450 + "y\n"},
        "depth-2000": {"deep.py": "x = " + "not " * 2000 + "y\n"},
        "dot-py": {
            ".py": "x = -1\n",
            "mod.py": "y = -2\n",
        },
        "dirlink": {
            "real/inside.py": "x = -1\nVERSION = '1.0'\n",
            "dirlink": {"link": "real"},
        },
        "dirlink-double-flip": {
            "real/flag.py": "flag = True\n",
            "dirlink": {"link": "real"},
        },
        "symlink-inside": {
            "real.py": "x = -1\n",
            "alias.py": {"link": "real.py"},
        },
        "symlink-escape": {
            "inner.py": "x = -1\n",
            "escape.py": {"link": "../outside.py"},
            "__outside__": "z = -9\n",
        },
    }
    return sets


def record_plant_case(name, spec):
    case = {"kind": "plant", "name": name, "files": {}}
    for rel, value in spec.items():
        if isinstance(value, dict) and "link" in value:
            case["files"][rel] = {"link": value["link"]}
        elif isinstance(value, dict) and "bytes" in value:
            import base64

            case["files"][rel] = {
                "bytes_b64": base64.b64encode(bytes.fromhex(value["bytes"])).decode("ascii")
            }
        else:
            case["files"][rel] = {"t": value}
    seeds = {}
    for seed in SEEDS:
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = base / "proj"
            root.mkdir()
            outside = None
            for rel, value in spec.items():
                if rel == "__outside__":
                    outside = base / "outside.py"
                    outside.write_text(value, encoding="utf-8")
                    continue
                path = root / rel
                path.parent.mkdir(parents=True, exist_ok=True)
                if isinstance(value, dict) and "link" in value:
                    path.symlink_to(value["link"])
                elif isinstance(value, dict) and "bytes" in value:
                    path.write_bytes(bytes.fromhex(value["bytes"]))
                else:
                    path.write_text(value, encoding="utf-8")
            skipped = []
            entry = {}
            try:
                planted = engine.plant_seed(root, seed, skipped)
                entry["planted"] = [str(p) for p in planted]
            except Exception as error:  # noqa: BLE001, the raise is the answer
                entry["raised"] = type(error).__name__
            entry["skipped"] = skipped
            after = {}
            for path in sorted(root.rglob("*")):
                rel = str(path.relative_to(root))
                if path.is_symlink():
                    after[rel] = {"link": str(path.readlink())}
                elif path.is_file():
                    after[rel] = spell_bytes(path.read_bytes())
            if outside is not None:
                entry["outside_after"] = spell_bytes(outside.read_bytes())
            entry["after"] = after
            seeds[seed.id] = entry
    case["seeds"] = seeds
    return case


def main():
    cases = []
    sources, refused = harvest_sources()
    for name, source in sorted(sources.items()):
        cases.append(apply_case(name, source))
    for name, source in refused:
        cases.append(apply_case(name, source))
    for name, source in sorted(file_sources().items()):
        cases.append(apply_case(name, source))
    for name, source in sorted(SYNTHETIC.items()):
        cases.append(apply_case(f"synthetic:{name}", source))
    for name, source in sorted(depth_cases().items()):
        cases.append(apply_case(f"synthetic:{name}", source))
    for name, spec in sorted(plant_cases().items()):
        cases.append(record_plant_case(f"plant:{name}", spec))
    stamp = (
        f"CPython {sys.version_info.major}."
        f"{sys.version_info.minor}.{sys.version_info.micro}"
    )
    OUT.write_text(
        json.dumps({"interpreter": stamp, "cases": cases}, indent=1),
        encoding="utf-8",
    )
    print(f"{len(cases)} cases recorded")


if __name__ == "__main__":
    main()
