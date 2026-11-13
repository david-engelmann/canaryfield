import re, unicodedata
from pathlib import Path

ROOT = Path('/dev/shm/cf16/tree')

# Copy A: static NON_PRINTABLE_RANGES in src/json.rs
text = (ROOT/'src'/'json.rs').read_text()
block = text.split('static NON_PRINTABLE_RANGES')[1].split('];')[0]
a_nonprint = [(int(a,16), int(b,16)) for a,b in re.findall(r'\(0x([0-9a-f]+), 0x([0-9a-f]+)\)', block)]
print("copy A ranges:", len(a_nonprint))

# Copy B: src/printable_ranges.txt
b_print = []
for line in (ROOT/'src'/'printable_ranges.txt').read_text().splitlines():
    if '-' in line:
        lo, hi = line.split('-'); b_print.append((int(lo,16), int(hi,16)))
print("copy B ranges:", len(b_print))

def in_ranges(ranges, cp):
    import bisect
    starts = [r[0] for r in ranges]
    i = bisect.bisect_right(starts, cp) - 1
    return i >= 0 and ranges[i][0] <= cp <= ranges[i][1]

diff_ab = []
diff_a_py = []
diff_b_py = []
for cp in range(0x110000):
    a_printable = not in_ranges(a_nonprint, cp)
    b_printable = in_ranges(b_print, cp)
    if cp < 0xD800 or cp > 0xDFFF:
        py_printable = chr(cp).isprintable()
    else:
        py_printable = False  # category Cs reading: never printable
    if a_printable != b_printable: diff_ab.append(cp)
    if a_printable != py_printable: diff_a_py.append(cp)
    if b_printable != py_printable: diff_b_py.append(cp)

def summarise(name, diffs):
    print(f"{name}: {len(diffs)} codepoints", [hex(c) for c in diffs[:20]])
summarise("A vs B", diff_ab)
summarise("A vs Python isprintable", diff_a_py)
summarise("B vs Python isprintable", diff_b_py)
