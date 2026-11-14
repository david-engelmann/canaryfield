"""Planting and gate running, bridged to the crate.

A run works on scratch material only. In copy mode it copies the target
tree into scratch copies, one clean copy and one planted copy per
applicable seed. Inside a planted copy or branch a seed plants into
every file where it applies, never just the first one, and at every
matching shape inside each of those files. In git branch
mode it clones the target into a scratch clone and commits each seed on
its own throwaway branch there, so git
mode judges the committed HEAD of the target while copy mode judges its
working tree. Gates run inside the scratch material only. The target
tree itself is opened read only, and the run proves that with a hash
walk taken before and after. A supplied work root must start empty and
a run removes only what it created.

The run itself stands in the Rust crate under PORT5, and the planting
stands in the crate under PORT2-1 and PORT2-7. This module is the
bridge to both doors, and the specification of the run from PORT5 on
is the recordings at tests/conformance/port5_cases.json.
"""

import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path

from .registry import SEEDS

CAUGHT = "caught"
WAVED = "waved-through"
NOT_APPLICABLE = "not-applicable"
GATE_BROKEN = "gate-broken"

_CHECKOUT_ROOT = Path(__file__).resolve().parent.parent


def _find_binary(standing_words="plant_seed stands in the Rust crate under PORT2-7"):
    """Find the crate binary the bridges drive.

    The name the caller hands over in CANARYFIELD_BINARY
    stands first, and a name that stands for no file is
    refused in words that name it. Then the binary on the
    path, then the checkout's own build under target in
    debug or release shape. Where no binary stands at all
    the standing refusal in words is the answer, so a caller
    holding only the Python meets the boundary in words
    instead of a second planting.
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
    raise NotImplementedError(standing_words)


def plant_seed(source_root, seed, skipped=None):
    """Plant one seed everywhere it applies in the copy at source_root.

    The planting stands in the Rust crate under PORT2-1 and
    PORT2-7, proven against the recordings this function's own
    behaviour made before it retired. This body is the bridge
    to that planting. It drives the crate binary over the
    copy and carries the answer back, the planted names as
    the return and the skipped names into the list the caller
    hands over, so the engine runs whole again with the one
    planting standing where the crate put it.
    """
    binary = _find_binary()
    completed = subprocess.run(
        [str(binary), "--plant-seed", str(source_root), seed.id],
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise ValueError(
            "the canaryfield binary refused the planting: "
            f"{completed.stderr.strip()}"
        )
    answer = json.loads(completed.stdout)
    if skipped is not None:
        for name in answer["skipped"]:
            if name not in skipped:
                skipped.append(name)
    return answer["planted"]


def run_field(
    target, gates, timeout=120, work_root=None, keep_scratch=False, plant_mode="copy"
):
    """Run every gate against the clean copy and every planted copy.

    gates is a list of (name, command) pairs. The result is a plain dict
    shaped like the JSON report. plant_mode is copy for the slice 1
    scratch copies or git-branches for a scratch clone where each seed is
    committed on its own throwaway branch.

    The run stands in the Rust crate under PORT5, proven against
    the recordings this function's own behaviour made before
    the machinery retired. This body is the bridge to that
    run. It writes the request document, drives the crate
    binary's run door once, and carries the answer back, a
    refusal of the door raised as the ValueError carrying the
    door's words, as the Python raises them itself.
    """
    binary = _find_binary("run_field stands in the Rust crate under PORT5")
    request = {
        "target": str(target),
        "gates": [[name, command] for name, command in gates],
        "timeout": timeout,
        "work_root": None if work_root is None else str(work_root),
        "keep_scratch": keep_scratch,
        "plant_mode": plant_mode,
    }
    staged = tempfile.NamedTemporaryFile(
        "w", suffix=".json", delete=False, encoding="utf-8"
    )
    try:
        json.dump(request, staged)
        staged.close()
        completed = subprocess.run(
            [str(binary), "--run-field", staged.name],
            capture_output=True,
            text=True,
        )
    finally:
        Path(staged.name).unlink(missing_ok=True)
    if completed.returncode != 0:
        raise ValueError(completed.stderr.strip())
    return json.loads(completed.stdout)
