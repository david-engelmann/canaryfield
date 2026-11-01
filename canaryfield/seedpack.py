"""The versioned seed pack file.

The pack is the seed registry written out as one JSON document so other
tools in the line can consume the same seed definitions without importing
this package. The format field carries the pack format version and every
entry carries the seed id, name, defect class, and description.

Seven formats are written history. Format canaryfield-seeds/1 carries
the original twelve seeds, format canaryfield-seeds/2 carries the
registry of sixteen, format canaryfield-seeds/3 carries the
registry of twenty, format canaryfield-seeds/4 carries the
registry of twenty four, format canaryfield-seeds/5 carries the
registry of twenty eight, format canaryfield-seeds/6 carries the
registry of thirty two, and format canaryfield-seeds/7 carries the
registry of thirty six. Writing a pack always writes the current format.
The loader accepts all seven formats and holds each one to the entry
count its own version carries, so a pack whose count disagrees with
its declared version is refused. The loader also holds each version to the
registry it names, refusing a pack whose ids are not exactly the ids of
its declared version and a pack whose entries disagree with the
registry on a name or a defect class. The description of an entry is
prose for a human reader and is checked only for presence, never
against the registry wording.
"""

import json
from pathlib import Path

from .seeds import SEEDS

SEED_PACK_FORMAT = "canaryfield-seeds/7"
SEED_PACK_FORMAT_V6 = "canaryfield-seeds/6"
SEED_PACK_FORMAT_V5 = "canaryfield-seeds/5"
SEED_PACK_FORMAT_V4 = "canaryfield-seeds/4"
SEED_PACK_FORMAT_V3 = "canaryfield-seeds/3"
SEED_PACK_FORMAT_V2 = "canaryfield-seeds/2"
SEED_PACK_FORMAT_V1 = "canaryfield-seeds/1"

PACK_ENTRY_COUNTS = {
    SEED_PACK_FORMAT_V1: 12,
    SEED_PACK_FORMAT_V2: 16,
    SEED_PACK_FORMAT_V3: 20,
    SEED_PACK_FORMAT_V4: 24,
    SEED_PACK_FORMAT_V5: 28,
    SEED_PACK_FORMAT_V6: 32,
    SEED_PACK_FORMAT: 36,
}

PACK_REGISTRIES = {
    pack_format: {seed.id: seed for seed in SEEDS[:count]}
    for pack_format, count in PACK_ENTRY_COUNTS.items()
}


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

    Raises ValueError when the document is not a pack of an accepted
    format, or when its entry count disagrees with the count its
    format version carries, or when its ids are not exactly the ids
    its format version carries, or when an entry disagrees with the
    registry on a name or a defect class.
    """
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("format") not in PACK_ENTRY_COUNTS:
        raise ValueError(f"not a canaryfield seed pack: {path}")
    pack_format = data["format"]
    entries = data.get("seeds")
    if not isinstance(entries, list):
        raise ValueError(f"seed pack carries no seed list: {path}")
    if len(entries) != PACK_ENTRY_COUNTS[pack_format]:
        raise ValueError(
            f"seed pack carries {len(entries)} seeds and format "
            f"{pack_format} carries {PACK_ENTRY_COUNTS[pack_format]}: {path}"
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
    registry = PACK_REGISTRIES[pack_format]
    if seen != set(registry):
        raise ValueError(
            f"seed pack ids do not match format {pack_format}: {path}"
        )
    for entry in entries:
        seed = registry[entry["id"]]
        for field in ("name", "defect_class"):
            if entry[field] != getattr(seed, field):
                raise ValueError(
                    f"seed pack entry {entry['id']} disagrees with the "
                    f"registry on {field}: {path}"
                )
    return entries
