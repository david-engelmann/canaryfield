"""The versioned seed pack file.

The pack is the seed registry written out as one JSON document so other
tools in the line can consume the same seed definitions without importing
this package. The format field carries the pack format version and every
entry carries the seed id, name, defect class, and description.
"""

import json
from pathlib import Path

from .seeds import SEEDS

SEED_PACK_FORMAT = "canaryfield-seeds/1"


def seed_pack_data():
    """Return the pack as a plain dict shaped like the JSON document."""
    return {
        "format": SEED_PACK_FORMAT,
        "seeds": [
            {
                "id": seed.id,
                "name": seed.name,
                "defect_class": seed.defect_class,
                "description": seed.description,
            }
            for seed in SEEDS
        ],
    }


def write_seed_pack(path):
    """Write the seed pack to path as JSON and return the path."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(seed_pack_data(), indent=2) + "\n", encoding="utf-8")
    return path


def load_seed_pack(path):
    """Load a seed pack and return its list of seed entries.

    Raises ValueError when the document is not a pack of this format.
    """
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("format") != SEED_PACK_FORMAT:
        raise ValueError(f"not a {SEED_PACK_FORMAT} seed pack: {path}")
    entries = data.get("seeds")
    if not isinstance(entries, list):
        raise ValueError(f"seed pack carries no seed list: {path}")
    if len(entries) != len(SEEDS):
        raise ValueError(
            f"seed pack carries {len(entries)} seeds and format "
            f"{SEED_PACK_FORMAT} carries {len(SEEDS)}: {path}"
        )
    seen = set()
    for entry in entries:
        if not isinstance(entry, dict) or not {"id", "name", "defect_class", "description"} <= set(entry):
            raise ValueError(f"seed pack entry is missing fields: {entry!r}")
        for field in ("id", "name", "defect_class", "description"):
            if not isinstance(entry[field], str) or not entry[field].strip():
                raise ValueError(
                    "seed pack entry field is not a non empty piece "
                    f"of text, {field} in {entry!r}"
                )
        if entry["id"] in seen:
            raise ValueError(f"seed pack carries a duplicate id: {entry['id']}")
        seen.add(entry["id"])
    return entries
