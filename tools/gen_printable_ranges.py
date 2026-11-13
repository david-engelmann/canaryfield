#!/usr/bin/env python3
"""Generate src/printable_ranges.txt, the printable table
of the port's string spellings.

The interpreter's repr leaves a character standing as
itself exactly when the character is printable, the
reading str.isprintable states over one character. This
script reads that reading out of the running interpreter
itself and writes the printable codepoints as merged
ranges, one range on each line with its ends in
hexadecimal joined by a hyphen, the range format of the
printable table. Run it with the interpreter the port is
judged against and no other.

Usage: gen_printable_ranges.py OUTPUT
"""

import sys
import unicodedata


def main():
    if sys.implementation.name != "cpython" or sys.version_info[:2] != (3, 12):
        raise SystemExit(
            "the printable table stands under CPython 3.12 only "
            "and this interpreter is "
            f"{sys.implementation.name} "
            f"{sys.version_info.major}.{sys.version_info.minor}"
        )
    if unicodedata.unidata_version != "15.0.0":
        raise SystemExit(
            "the printable table stands over Unicode 15.0.0 only "
            "and this interpreter carries "
            f"{unicodedata.unidata_version}"
        )
    ranges = []
    for codepoint in range(0x110000):
        if not chr(codepoint).isprintable():
            continue
        if ranges and codepoint == ranges[-1][1] + 1:
            ranges[-1][1] = codepoint
        else:
            ranges.append([codepoint, codepoint])
    with open(sys.argv[1], "w", encoding="utf-8") as handle:
        for low, high in ranges:
            handle.write(f"{low:x}-{high:x}\n")
    print(f"printable ranges {len(ranges)}")


if __name__ == "__main__":
    main()
