"""Command line entry point for canaryfield."""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from .engine import run_field
from .seedpack import SEED_PACK_FORMAT, write_seed_pack

_CHECKOUT_ROOT = Path(__file__).resolve().parent.parent


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


def _find_report_binary():
    """Find the crate binary the report bridge drives.

    The discovery is the engine bridge's discovery, the name
    in CANARYFIELD_BINARY first and a name that stands for no
    file refused in words that name it, then the binary on
    the path, then the checkout's own build under target in
    debug or release shape. Where no binary stands at all the
    refusal names the ground the renderings stand on, so a
    caller holding only the Python meets the boundary in
    words instead of a second rendering.
    """
    named = os.environ.get("CANARYFIELD_BINARY")
    if named:
        candidate = Path(named)
        if candidate.is_file():
            return candidate
        raise NotImplementedError(
            f"the named canaryfield binary does not stand: {named}"
        )
    on_path = shutil.which("canaryfield")
    if on_path:
        return Path(on_path)
    for candidate in (
        _CHECKOUT_ROOT / "target" / "debug" / "canaryfield",
        _CHECKOUT_ROOT / "target" / "release" / "canaryfield",
    ):
        if candidate.is_file():
            return candidate
    raise NotImplementedError(
        "the report renderings stand in the Rust crate under PORT4"
    )


def _check_carryable(value, active):
    """Refuse a result the JSON rendering cannot carry.

    A value of no JSON kind, a key that is not text, and a
    reference cycle can stand in a Python result and no JSON
    document can hold any of them, so the report door never
    meets them and this bridge refuses them in the port's own
    wording before it drives the door, as PORT4-6 states.
    """
    if value is None or isinstance(value, (bool, int, float, str)):
        return
    if isinstance(value, (list, tuple)):
        marker = id(value)
        if marker in active:
            raise ValueError("the result holds a reference cycle")
        active.add(marker)
        for item in value:
            _check_carryable(item, active)
        active.discard(marker)
        return
    if isinstance(value, dict):
        marker = id(value)
        if marker in active:
            raise ValueError("the result holds a reference cycle")
        active.add(marker)
        for key, item in value.items():
            if not isinstance(key, str):
                raise ValueError("the result holds a key that is not text")
            _check_carryable(item, active)
        active.discard(marker)
        return
    raise ValueError("the result holds a value with no JSON kind")


def _render_report(result, form):
    """Render one report form through the crate binary.

    The renderings stand in the Rust crate under PORT4, proven
    against the recordings the Python report module made
    before it retired. This body is the bridge to those
    renderings. It writes the result as a JSON document and
    drives the binary's report door over it once, carrying
    the rendering back whole, and a refusal of the door is
    raised as an error carrying the door's words.
    """
    _check_carryable(result, set())
    binary = _find_report_binary()
    staged = tempfile.NamedTemporaryFile(
        "w", suffix=".json", encoding="utf-8", delete=False
    )
    try:
        with staged:
            json.dump(result, staged)
        completed = subprocess.run(
            [str(binary), "--render-report", staged.name, form],
            capture_output=True,
            text=True,
            encoding="utf-8",
        )
    finally:
        os.unlink(staged.name)
    if completed.returncode != 0:
        raise ValueError(
            "the canaryfield binary refused the rendering: "
            f"{completed.stderr.strip()}"
        )
    return completed.stdout


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
        (out / "catch-table.txt").write_text(_render_report(result, "text"), encoding="utf-8")
        (out / "catch-table.json").write_text(_render_report(result, "json"), encoding="utf-8")
    print(_render_report(result, "json") if args.json else _render_report(result, "text"), end="")
    return 0
