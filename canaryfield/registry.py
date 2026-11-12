"""The seed registry as data.

The registry the standing Python machinery reads stands here as
data. The records are read from the seed pack document this
package ships beside this module, seed-pack.json, written in the
canaryfield-seeds/10 format, so the registry the Python reads is
the registry the pack writer writes and the one the crate
carries as data under PORT-2.

Each record carries exactly the four fields the pack carries,
the id and the name and the defect class and the description.
The apply each seed once carried lived in canaryfield/seeds.py,
and that module retired whole in Round 15 under PORT2-9, and
the applies stand in the Rust crate now. A record therefore
carries no planting behaviour, and its apply refuses in words
rather than planting or pretending to plant.
"""

import json
from dataclasses import dataclass
from pathlib import Path

_PACK_PATH = Path(__file__).with_name("seed-pack.json")
_PACK_FORMAT = "canaryfield-seeds/10"
_PACK_FIELDS = ("id", "name", "defect_class", "description")


@dataclass(frozen=True)
class Seed:
    """One seed of the registry, metadata only.

    The record carries the four fields of its seed pack entry.
    The apply the seed once carried stands in the Rust crate,
    and calling it here refuses in words.
    """

    id: str
    name: str
    defect_class: str
    description: str = ""

    def apply(self, tree):
        raise NotImplementedError(
            "the seed applies stand in the Rust crate under PORT2-5"
        )


def _load_seeds():
    data = json.loads(_PACK_PATH.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("format") != _PACK_FORMAT:
        raise ValueError(f"not a {_PACK_FORMAT} seed pack: {_PACK_PATH}")
    entries = data.get("seeds")
    if not isinstance(entries, list):
        raise ValueError(f"seed pack carries no seed list: {_PACK_PATH}")
    seeds = []
    for entry in entries:
        if not isinstance(entry, dict) or not set(_PACK_FIELDS) <= set(entry):
            raise ValueError(f"seed pack entry is missing fields: {entry!r}")
        for field in _PACK_FIELDS:
            if not isinstance(entry[field], str) or not entry[field].strip():
                raise ValueError(
                    "seed pack entry field is not a non empty piece "
                    f"of text, {field} in {entry!r}"
                )
        seeds.append(Seed(**{field: entry[field] for field in _PACK_FIELDS}))
    return seeds


SEEDS = _load_seeds()

SEEDS_BY_ID = {seed.id: seed for seed in SEEDS}
