import copy
import json
import sys
from pathlib import Path

sys.path.insert(0, "/dev/shm/cf16/tree")
from canaryfield.report import to_json, to_text

EX = Path("/dev/shm/cf16/tree/examples")

# The standing specimen in examples/ is a full result rendered both ways.
result = json.loads((EX / "catch-table.json").read_text())
print("specimen to_text == examples/catch-table.txt:", to_text(result) == (EX / "catch-table.txt").read_text())
print("specimen to_json round-trips the file:", to_json(result) == (EX / "catch-table.json").read_text())


def base(**over):
    r = {
        "target": "/tmp/target",
        "gates": ["lint", "tests"],
        "clean_exits": {"lint": 0, "tests": 1},
        "target_unchanged": True,
        "unparseable_files": [],
        "rows": [],
        "summary": {
            "lint": {"caught": 0, "waved-through": 0, "not-applicable": 0, "gate-broken": 0},
            "tests": {"caught": 0, "waved-through": 0, "not-applicable": 0, "gate-broken": 0},
        },
    }
    r.update(over)
    return r


ROW = {
    "seed": "seed-00", "name": "stale version string", "defect_class": "stale metadata",
    "planted_file": None, "planted_files": [], "planted_exits": {},
    "gates": {"lint": "not-applicable", "tests": "not-applicable"},
}


def show(label, fn):
    print(f"--- {label} ---")
    try:
        print(fn(), end="")
    except Exception as error:
        print(f"RAISES {type(error).__name__}: {error}")


show("zero rows (the width line reads max over an empty star)", lambda: to_text(base()))
show("zero gates, one row", lambda: to_text(base(gates=[], clean_exits={}, summary={}, rows=[{**ROW, "gates": {}}])))
show("one row, planted_file None spells none", lambda: to_text(base(rows=[copy.deepcopy(ROW)])))
row2 = {**copy.deepcopy(ROW), "planted_file": "a.py", "planted_files": ["a.py", "b.py"],
        "gates": {"lint": "caught", "tests": "waved-through"}}
show("two planted files and unparseable files", lambda: to_text(base(rows=[row2], unparseable_files=["bad.py", "odd.py"])))
row1 = {**copy.deepcopy(ROW), "planted_file": "a.py", "planted_files": ["a.py"],
        "gates": {"lint": "caught", "tests": "waved-through"}}
print("exactly one planted file adds no planted-in line:", "planted in" not in to_text(base(rows=[row1])))
row0 = {k: v for k, v in copy.deepcopy(ROW).items() if k != "planted_files"}
row0["planted_file"] = "a.py"
print("row with no planted_files key renders (get default):", "a.py" in to_text(base(rows=[row0])))


def missing_summary():
    r = base(rows=[copy.deepcopy(ROW)]); del r["summary"]["tests"]; return to_text(r)
def missing_clean():
    r = base(rows=[copy.deepcopy(ROW)]); del r["clean_exits"]["tests"]; return to_text(r)
def missing_cell():
    r = base(rows=[copy.deepcopy(ROW)]); del r["rows"][0]["gates"]["lint"]; return to_text(r)
def missing_gates():
    r = base(rows=[copy.deepcopy(ROW)]); del r["gates"]; return to_text(r)
show("summary missing a gate", missing_summary)
show("clean_exits missing a gate", missing_clean)
show("row missing a gate cell", missing_cell)
show("result missing the gates key", missing_gates)
print("target_unchanged False spells:", [l for l in to_text(base(rows=[copy.deepcopy(ROW)], target_unchanged=False)).splitlines() if "unchanged" in l])
print("--- to_json: sorted keys, indent 2, one trailing newline ---")
print(to_json({"b": 1, "a": [1, 2]}), end="")
