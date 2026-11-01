"""Command line entry point for canaryfield."""

import argparse
import sys
from pathlib import Path

from .engine import run_field
from .report import to_json, to_text
from .seedpack import SEED_PACK_FORMAT, write_seed_pack


def _parse_gate(text):
    name, separator, command = text.partition("=")
    if not separator or not name.strip() or not command.strip():
        raise argparse.ArgumentTypeError(
            f"gate must be written as NAME=COMMAND, got {text!r}"
        )
    return name.strip(), command.strip()


def build_parser():
    parser = argparse.ArgumentParser(
        prog="canaryfield",
        description="Plant seeded defects into scratch copies of a target "
        "tree and score which gates catch each one.",
    )
    parser.add_argument("--target", help="target tree to copy and plant into")
    parser.add_argument(
        "--gate",
        action="append",
        type=_parse_gate,
        metavar="NAME=COMMAND",
        help="a named gate command, repeatable",
    )
    parser.add_argument(
        "--plant",
        choices=("copy", "git-branches"),
        default="copy",
        help="planting mode, copy for scratch copies or git-branches for a scratch clone",
    )
    parser.add_argument(
        "--write-seed-pack",
        metavar="PATH",
        help=f"write the {SEED_PACK_FORMAT} seed pack to PATH and exit",
    )
    parser.add_argument("--out", help="directory to write catch-table.txt and catch-table.json into")
    parser.add_argument("--json", action="store_true", help="print the JSON report to stdout")
    parser.add_argument("--timeout", type=int, default=120, help="per gate run timeout in seconds")
    parser.add_argument("--work-root", help="scratch directory root, defaults to a temp directory")
    parser.add_argument("--keep-scratch", action="store_true", help="keep the scratch copies after the run")
    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)
    if args.write_seed_pack:
        write_seed_pack(args.write_seed_pack)
        print(f"wrote {SEED_PACK_FORMAT} seed pack to {args.write_seed_pack}")
        return 0
    if not args.target or not args.gate:
        parser.error("--target and at least one --gate are required unless --write-seed-pack is used")
    try:
        result = run_field(
            args.target,
            args.gate,
            timeout=args.timeout,
            work_root=args.work_root,
            keep_scratch=args.keep_scratch,
            plant_mode=args.plant,
        )
    except ValueError as error:
        print(f"error, {error}", file=sys.stderr)
        return 2
    if not result["target_unchanged"]:
        print("error, the target tree changed during the run", file=sys.stderr)
        return 3
    if args.out:
        out = Path(args.out)
        out.mkdir(parents=True, exist_ok=True)
        (out / "catch-table.txt").write_text(to_text(result), encoding="utf-8")
        (out / "catch-table.json").write_text(to_json(result), encoding="utf-8")
    print(to_json(result) if args.json else to_text(result), end="")
    return 0
