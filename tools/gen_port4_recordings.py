"""Recorded conformance generator for canaryfield PORT4.

Runs the landed Python report module over synthetic result
documents and over the two committed example tables and records
its answers. Usage: python3 tools/gen_port4_recordings.py
<repo_root> <out_json>. The repo root is a tree carrying the
landed Python package while canaryfield/report.py still stands.
Recordings are never hand written and never taken from the port.
"""

import hashlib
import json
import sys
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

from canaryfield import report  # noqa: E402

TEXT_LIMIT = 6000


def spell(text):
    """Record a long text as a digest with its length, a short one whole."""
    if len(text) <= TEXT_LIMIT:
        return {"t": text}
    return {"sha256": hashlib.sha256(text.encode("utf-8")).hexdigest(), "len": len(text)}


def roundtrip(document):
    """The document exactly as the report door receives it.

    Every case document passes through the JSON spelling and
    back, so the recorded input is the document the port reads,
    with no Python only shape hiding inside it.
    """
    return json.loads(json.dumps(document))


def crash_of(error):
    record = {"class": type(error).__name__, "message": str(error)}
    if isinstance(error, KeyError) and error.args:
        named = error.args[0]
        if isinstance(named, (str, int, float, bool)) or named is None:
            record["name"] = named
        else:
            record["name"] = repr(named)
    return record


def record_render(name, document):
    document = roundtrip(document)
    case = {"kind": "render", "name": name, "input": document}
    case["text"] = spell(report.to_text(document))
    case["json"] = spell(report.to_json(document))
    return case


def record_refusal(name, document):
    """A shape the port refuses.

    The crash is recorded where the landed Python meets one in
    the text form, and the JSON form's own answer is recorded
    beside it, a rendering where the encoder renders and a
    crash where it does not. A refusal answering no crash
    records the rendering the Python gives and no crash,
    because none exists.
    """
    document = roundtrip(document)
    case = {"kind": "refusal", "name": name, "input": document}
    try:
        rendered = report.to_text(document)
    except Exception as error:  # noqa: BLE001, the raise is the answer
        case["crash"] = crash_of(error)
    else:
        case["text_rendered"] = spell(rendered)
    try:
        case["json"] = spell(report.to_json(document))
    except RecursionError as error:
        case["json_crash"] = crash_of(error)
    return case


def base():
    return {
        "gates": ["alpha"],
        "target": "demo-target",
        "target_unchanged": True,
        "clean_exits": {"alpha": 0},
        "rows": [
            {
                "seed": "seed-01",
                "name": "first seed",
                "planted_file": "mod.py",
                "gates": {"alpha": "caught"},
                "planted_files": ["mod.py"],
            }
        ],
        "unparseable_files": [],
        "summary": {
            "alpha": {
                "caught": 1,
                "waved-through": 0,
                "not-applicable": 0,
                "gate-broken": 0,
            }
        },
    }


def mutated(name, change):
    document = base()
    change(document)
    return record_refusal(name, document)


def rendered(name, change):
    document = base()
    change(document)
    return record_render(name, document)


CASES = []


def add(case):
    CASES.append(case)


# The two committed example tables are cases of record. Each
# parsed document renders through the landed Python to the
# committed bytes standing beside it, asserted here as the
# generator runs, on PORT4-1.
for label, directory in (("example-one-gate", "examples"), ("example-two-gates", "examples/two-gates")):
    folder = ROOT / directory
    document = json.loads((folder / "catch-table.json").read_text(encoding="utf-8"))
    text_bytes = (folder / "catch-table.txt").read_bytes()
    json_bytes = (folder / "catch-table.json").read_bytes()
    assert report.to_text(document).encode("utf-8") == text_bytes, label
    assert report.to_json(document).encode("utf-8") == json_bytes, label
    case = {"kind": "render", "name": label, "input": document}
    case["text"] = spell(report.to_text(document))
    case["json"] = spell(report.to_json(document))
    add(case)

# Rendering cases over the broad surface.
add(record_render("one-gate-minimal", base()))


def four_gates(document):
    gates = ["compile", "tests", "lint", "types"]
    statuses = ["caught", "waved-through", "not-applicable", "gate-broken"]
    document["gates"] = gates
    document["clean_exits"] = {"compile": 0, "tests": 1, "lint": 0, "types": -1}
    rows = []
    for index in range(48):
        planted = [f"file-{index}.py"] if index % 3 else [f"a-{index}.py", f"b-{index}.py"]
        rows.append(
            {
                "seed": f"seed-{index + 1:02d}",
                "name": f"seed number {index + 1}",
                "planted_file": planted[0],
                "gates": {gate: statuses[(index + offset) % 4] for offset, gate in enumerate(gates)},
                "planted_files": planted,
            }
        )
    document["rows"] = rows
    document["unparseable_files"] = ["broken-1.py", "broken-2.py"]
    document["summary"] = {
        gate: {"caught": 12, "waved-through": 11, "not-applicable": 13, "gate-broken": 12}
        for gate in gates
    }


add(rendered("four-gates-forty-eight-rows", four_gates))


def zero_gates(document):
    document["gates"] = []
    document["clean_exits"] = {}
    document["rows"] = [
        {"seed": "seed-01", "name": "lonely seed", "planted_file": None, "gates": {}}
    ]
    document["summary"] = {}


add(rendered("zero-gates", zero_gates))


def planted_variants(document):
    document["rows"] = [
        {"seed": "seed-01", "name": "no list", "planted_file": None, "gates": {"alpha": "caught"}},
        {"seed": "seed-02", "name": "none list", "planted_file": "", "gates": {"alpha": "caught"}, "planted_files": None},
        {"seed": "seed-03", "name": "zero list", "planted_file": "x.py", "gates": {"alpha": "waved-through"}, "planted_files": 0},
        {"seed": "seed-04", "name": "false list", "planted_file": "x.py", "gates": {"alpha": "caught"}, "planted_files": False},
        {"seed": "seed-05", "name": "empty text list", "planted_file": "x.py", "gates": {"alpha": "caught"}, "planted_files": ""},
        {"seed": "seed-06", "name": "single", "planted_file": "only.py", "gates": {"alpha": "caught"}, "planted_files": ["only.py"]},
        {"seed": "seed-07", "name": "triple", "planted_file": "one.py", "gates": {"alpha": "gate-broken"}, "planted_files": ["one.py", "two.py", "three.py"]},
    ]


add(rendered("planted-variants", planted_variants))


def unparseable_populated(document):
    document["unparseable_files"] = ["bad.py", "worse.py"]


add(rendered("unparseable-populated", unparseable_populated))


def unparseable_falsy(document):
    document["unparseable_files"] = 0


add(rendered("unparseable-falsy-zero", unparseable_falsy))


def unicode_positions(document):
    document["gates"] = ["gαte", "élan", "🔥gate"]
    document["target"] = "targét-🔥"
    document["clean_exits"] = {"gαte": 0, "élan": 2, "🔥gate": -1}
    document["rows"] = [
        {
            "seed": "séed-🔥",
            "name": "naïve café",
            "planted_file": "füü.py",
            "gates": {"gαte": "caught", "élan": "waved-through", "🔥gate": "caught"},
            "planted_files": ["füü.py", "bär.py"],
        }
    ]
    document["summary"] = {
        gate: {"caught": 1, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}
        for gate in document["gates"]
    }


add(rendered("unicode-positions", unicode_positions))


def long_values(document):
    document["rows"][0]["name"] = "long " + "x" * 7000
    document["target"] = "t" * 300


add(rendered("long-values", long_values))


def git_mode(document):
    document["branch"] = "canaryfield/seed-01"
    document["plant_mode"] = "git-branches"
    document["rows"][0]["defect_class"] = "boolean-flip"
    document["rows"][0]["planted_exits"] = {"mod.py": 0}


add(rendered("git-mode-extras", git_mode))


def copy_mode_extras(document):
    document["extra_top"] = {"nested": [1, 2, {"deep": True}]}
    document["rows"][0]["extra_row"] = "ignored"
    document["summary"]["alpha"]["extra_count"] = 99


add(rendered("copy-mode-extras", copy_mode_extras))


def float_spellings(document):
    document["measurements"] = {
        "big": 1e20,
        "small": 1e-07,
        "negative_zero": -0.0,
        "tenth": 0.1,
        "whole_float": 1.0,
        "limit": float("inf"),
        "below_limit": float("-inf"),
        "missing_reading": float("nan"),
        "huge_int": 123456789012345678901234567890,
        "negative_int": -42,
    }


add(rendered("float-and-int-spellings", float_spellings))


def key_sort(document):
    document["unsorted"] = {"b": 1, "A": 2, "a": 3, "é": 4, "aa": 5, "z": {"y": 1, "b": 2}}


add(rendered("json-key-sort", key_sort))


def escapes(document):
    document["target"] = 'quo"te\\back\nline\ttab'
    document["notes"] = "de\x7fl é 😀"


add(rendered("escapes", escapes))


def negative_and_large(document):
    document["clean_exits"] = {"alpha": -1}
    document["summary"]["alpha"] = {
        "caught": 1000000,
        "waved-through": 999999,
        "not-applicable": 3,
        "gate-broken": 0,
    }


add(rendered("negative-exit-large-counts", negative_and_large))


def unchanged_false(document):
    document["target_unchanged"] = False


add(rendered("target-unchanged-false", unchanged_false))


def empty_cells(document):
    document["gates"] = [""]
    document["clean_exits"] = {"": 3}
    document["rows"] = [
        {"seed": "", "name": "", "planted_file": "", "gates": {"": "caught"}}
    ]
    document["summary"] = {
        "": {"caught": 0, "waved-through": 0, "not-applicable": 1, "gate-broken": 0}
    }


add(rendered("empty-cells", empty_cells))


def nested(depth):
    value = 1
    for _ in range(depth):
        value = [value]
    return value


def nested_900(document):
    document["deep"] = nested(900)


add(rendered("nested-900-renders", nested_900))

# Refusal cases, one fault each over the standing base.

add(mutated("missing-gates", lambda d: d.pop("gates")))
add(mutated("missing-target", lambda d: d.pop("target")))
add(mutated("missing-target-unchanged", lambda d: d.pop("target_unchanged")))
add(mutated("missing-clean-exits", lambda d: d.pop("clean_exits")))
add(mutated("missing-clean-exit-for-gate", lambda d: d["clean_exits"].pop("alpha")))
add(mutated("missing-rows", lambda d: d.pop("rows")))
add(mutated("missing-row-seed", lambda d: d["rows"][0].pop("seed")))
add(mutated("missing-row-name", lambda d: d["rows"][0].pop("name")))
add(mutated("missing-row-planted-file", lambda d: d["rows"][0].pop("planted_file")))
add(mutated("missing-row-gates", lambda d: d["rows"][0].pop("gates")))
add(mutated("missing-row-gate-cell", lambda d: d["rows"][0]["gates"].pop("alpha")))
add(mutated("missing-summary", lambda d: d.pop("summary")))
add(mutated("missing-summary-for-gate", lambda d: d["summary"].pop("alpha")))
add(mutated("missing-count-caught", lambda d: d["summary"]["alpha"].pop("caught")))
add(mutated("missing-count-waved-through", lambda d: d["summary"]["alpha"].pop("waved-through")))
add(mutated("missing-count-not-applicable", lambda d: d["summary"]["alpha"].pop("not-applicable")))
add(mutated("missing-count-gate-broken", lambda d: d["summary"]["alpha"].pop("gate-broken")))
add(mutated("empty-rows", lambda d: d.update(rows=[])))

add(mutated("seed-number", lambda d: d["rows"][0].update(seed=7)))
add(mutated("seed-list", lambda d: d["rows"][0].update(seed=["seed-01"])))
add(mutated("name-number", lambda d: d["rows"][0].update(name=3.5)))
add(mutated("name-object", lambda d: d["rows"][0].update(name={"n": 1})))
add(mutated("planted-file-number", lambda d: d["rows"][0].update(planted_file=7)))
add(mutated("planted-file-list", lambda d: d["rows"][0].update(planted_file=["mod.py"])))
add(mutated("gate-cell-number", lambda d: d["rows"][0]["gates"].update(alpha=0)))
add(mutated("gate-cell-list", lambda d: d["rows"][0]["gates"].update(alpha=["caught"])))


def gate_name_number(document):
    document["gates"] = [7]
    document["rows"][0]["gates"] = {7: "caught"}
    document["summary"] = {
        7: {"caught": 1, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}
    }


add(mutated("gate-name-number", gate_name_number))


def gate_name_list(document):
    document["gates"] = [["alpha"]]
    document["clean_exits"] = {}
    document["rows"][0]["gates"] = {"alpha": "caught"}
    document["summary"] = {}


add(mutated("gate-name-unhashable", gate_name_list))

# General readings the Python renders and the port refuses on
# the shape ground, recorded with the rendering the Python
# gives and no crash.
add(mutated("target-number", lambda d: d.update(target=5)))
add(mutated("target-unchanged-word", lambda d: d.update(target_unchanged="yes")))
add(mutated("clean-exit-word", lambda d: d["clean_exits"].update(alpha="zero")))
add(mutated("clean-exit-float", lambda d: d["clean_exits"].update(alpha=0.0)))
add(mutated("count-word", lambda d: d["summary"]["alpha"].update(caught="one")))
add(mutated("count-float", lambda d: d["summary"]["alpha"].update(caught=1.5)))
add(mutated("planted-file-zero", lambda d: d["rows"][0].update(planted_file=0)))
add(mutated("planted-file-false", lambda d: d["rows"][0].update(planted_file=False)))
add(mutated("planted-files-object", lambda d: d["rows"][0].update(planted_files={"a.py": 1, "b.py": 2})))
add(mutated("unparseable-object", lambda d: d.update(unparseable_files={"a.py": 1, "b.py": 2})))

# Container positions in the wrong kind.
add(record_refusal("result-list", [1, 2]))
add(record_refusal("result-string", "result"))
add(record_refusal("result-number", 42))
add(record_refusal("result-none", None))
add(mutated("clean-exits-list", lambda d: d.update(clean_exits=[0])))
add(mutated("clean-exits-string", lambda d: d.update(clean_exits="alpha")))
add(mutated("clean-exits-number", lambda d: d.update(clean_exits=0)))
add(mutated("clean-exits-none", lambda d: d.update(clean_exits=None)))
add(mutated("gates-number", lambda d: d.update(gates=2)))
add(mutated("gates-none", lambda d: d.update(gates=None)))
add(mutated("gates-object", lambda d: d.update(gates={"alpha": 0})))
add(mutated("gates-string", lambda d: d.update(gates="alpha")))
add(mutated("rows-number", lambda d: d.update(rows=2)))
add(mutated("rows-none", lambda d: d.update(rows=None)))
add(mutated("rows-string", lambda d: d.update(rows="row")))
add(mutated("rows-object", lambda d: d.update(rows={"r": 1})))
add(mutated("row-none", lambda d: d.update(rows=[None])))
add(mutated("row-string", lambda d: d.update(rows=["row"])))
add(mutated("row-number", lambda d: d.update(rows=[7])))
add(mutated("row-list", lambda d: d.update(rows=[["seed-01"]])))
add(mutated("row-gates-none", lambda d: d["rows"][0].update(gates=None)))
add(mutated("row-gates-list", lambda d: d["rows"][0].update(gates=["caught"])))
add(mutated("row-gates-string", lambda d: d["rows"][0].update(gates="caught")))
add(mutated("row-gates-number", lambda d: d["rows"][0].update(gates=0)))
add(mutated("planted-files-number", lambda d: d["rows"][0].update(planted_files=3)))
add(mutated("planted-files-true", lambda d: d["rows"][0].update(planted_files=True)))
add(mutated("unparseable-number", lambda d: d.update(unparseable_files=3)))
add(mutated("unparseable-true", lambda d: d.update(unparseable_files=True)))
add(mutated("summary-none", lambda d: d.update(summary=None)))
add(mutated("summary-list", lambda d: d.update(summary=[1])))
add(mutated("summary-string", lambda d: d.update(summary="alpha")))
add(mutated("summary-number", lambda d: d.update(summary=1)))
add(mutated("summary-entry-none", lambda d: d["summary"].update(alpha=None)))
add(mutated("summary-entry-list", lambda d: d["summary"].update(alpha=[1, 2, 3, 4])))
add(mutated("summary-entry-string", lambda d: d["summary"].update(alpha="counts")))
add(mutated("summary-entry-number", lambda d: d["summary"].update(alpha=4)))

# Entries of the wrong kind and strings standing as lists.
add(mutated("planted-entry-multi-number", lambda d: d["rows"][0].update(planted_files=[7, "a.py"])))
add(mutated("planted-entry-single-number", lambda d: d["rows"][0].update(planted_files=[7])))
add(mutated("planted-entry-multi-list", lambda d: d["rows"][0].update(planted_files=[["a.py"], "b.py"])))
add(mutated("unparseable-entry-single-number", lambda d: d.update(unparseable_files=[7])))
add(mutated("unparseable-entry-none-in-list", lambda d: d.update(unparseable_files=[None, "a.py"])))
add(mutated("planted-files-string-long", lambda d: d["rows"][0].update(planted_files="ab")))
add(mutated("planted-files-string-single", lambda d: d["rows"][0].update(planted_files="a")))
add(mutated("unparseable-string", lambda d: d.update(unparseable_files="ab")))

# The stated divergence. Nested at 1000 levels under a key the
# text form ignores, the Python encoder meets its recursion
# error and the port renders within its standing bound.
document = base()
document["deep"] = nested(1000)
case = {"kind": "divergence", "name": "nested-1000-divergence", "input": roundtrip(document)}
case["text"] = spell(report.to_text(document))
try:
    report.to_json(document)
except RecursionError as error:
    case["json_crash"] = crash_of(error)
else:  # the contract's probe says this never stands
    raise SystemExit("the 1000 level case rendered in the Python encoder")
add(case)

names = [case["name"] for case in CASES]
assert len(names) == len(set(names)), "case names stand unique"
# Every recording above stands taken under the standing
# recursion limit, the divergence case among them. The battery
# write itself is bookkeeping, and spelling the deepest inputs
# whole needs room the recording never asked for, so the limit
# rises only here, after the last recording stands.
sys.setrecursionlimit(20000)
OUT.write_text(json.dumps({"cases": CASES}, indent=1) + "\n", encoding="utf-8")
print(f"recorded {len(CASES)} cases to {OUT}")
