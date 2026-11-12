#!/usr/bin/env python3
"""Generate the character name data of src/pydata.rs.

The interpreter resolves a character name three ways. Most
names stand in its table exactly as spelled, capitals,
single spaces, and hyphens all standing, with the formal
aliases of NameAliases.txt resolving beside them. The
Hangul syllables and the CJK unified ideographs are formed
by rule instead, under exact prefixes. This script reads
the first kind out of the running interpreter's database
and the alias file of the system Unicode data, and writes
src/unicode_names.txt with one name and codepoint per
line, the stored names in codepoint order and the aliases
after them. The unified ideograph ranges the rule checks
against go to src/unified_ranges.txt in the range format
of the printable table.

Usage: gen_unicode_names.py REPO_ROOT [ALIASES_TXT]
"""

import sys
import unicodedata
from pathlib import Path


def main():
    root = Path(sys.argv[1])
    aliases_path = (
        Path(sys.argv[2])
        if len(sys.argv) > 2
        else Path("/usr/share/unicode/NameAliases.txt")
    )
    names = []
    unified = []
    for codepoint in range(0x110000):
        name = unicodedata.name(chr(codepoint), None)
        if name is None:
            continue
        if name.startswith("HANGUL SYLLABLE "):
            continue
        if name.startswith("CJK UNIFIED IDEOGRAPH-"):
            unified.append(codepoint)
            continue
        names.append((name, codepoint))
    aliases = []
    version = "unknown"
    for line in aliases_path.read_text(encoding="utf-8").splitlines():
        if line.startswith("# NameAliases"):
            version = line.lstrip("# ").strip()
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        fields = line.split(";")
        aliases.append((fields[1], int(fields[0], 16)))
    with open(root / "src" / "unicode_names.txt", "w", encoding="utf-8") as handle:
        for name, codepoint in names + aliases:
            handle.write(f"{name} {codepoint:x}\n")
    ranges = []
    for codepoint in unified:
        if ranges and codepoint == ranges[-1][1] + 1:
            ranges[-1][1] = codepoint
        else:
            ranges.append([codepoint, codepoint])
    with open(root / "src" / "unified_ranges.txt", "w", encoding="utf-8") as handle:
        for low, high in ranges:
            handle.write(f"{low:x}-{high:x}\n")
    print(
        f"stored names {len(names)}, aliases {len(aliases)} ({version}), "
        f"total {len(names) + len(aliases)}, unified ranges {len(ranges)}"
    )


if __name__ == "__main__":
    main()
