import sys
from pathlib import Path

ROOT = Path('/dev/shm/cf16/tree')

def code_lines(path, comment_prefixes):
    n = 0
    for line in path.read_text(encoding='utf-8').splitlines():
        s = line.strip()
        if not s: continue
        if any(s.startswith(p) for p in comment_prefixes): continue
        n += 1
    return n

def count(paths, prefixes):
    return sum(code_lines(p, prefixes) for p in paths)

rust_src = sorted((ROOT/'src').glob('*.rs'))
rust_tests = sorted((ROOT/'tests').glob('*.rs'))
rust_all = [p for p in rust_src + rust_tests if p.name != 'xid_tables.rs']
print("Rust (src+tests, xid excluded):", count(rust_all, ['//']))
print("Rust incl xid:", count(rust_src + rust_tests, ['//']))
py_pkg = sorted((ROOT/'canaryfield').glob('*.py'))
py_tests = sorted((ROOT/'tests').glob('*.py'))
print("Python (package+tests):", count(py_pkg + py_tests, ['#']))
py_demo = sorted((ROOT/'examples'/'demo-project').glob('*.py'))
print("Python (package+tests+examples/demo-project):", count(py_pkg + py_tests + py_demo, ['#']))
tools = sorted((ROOT/'tools').glob('*.py'))
print("tools (neither side):", count(tools, ['#']))
print("report.py by method:", code_lines(ROOT/'canaryfield'/'report.py', ['#']))
print("gen_port2_recordings.py by method:", code_lines(ROOT/'tools'/'gen_port2_recordings.py', ['#']))
for p in py_pkg + py_tests:
    print("  py", p.relative_to(ROOT), code_lines(p, ['#']))
for p in rust_all:
    print("  rs", p.relative_to(ROOT), code_lines(p, ['//']))
