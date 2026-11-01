"""Tests for the slice 6 obligations in CONTRACT.md.

Slice 6 pins each seed pack version to the registry it names. The
loader refuses a pack whose ids are not exactly the id set of its
declared format version and a pack whose entries disagree with the
registry on a name or a defect class, while the description of an
entry stays unverified prose.

Every fixture here is synthetic and written for this test file.
"""

import json

import pytest

from canaryfield.seedpack import load_seed_pack, seed_pack_data
from canaryfield.seeds import SEEDS


def _pack_file(tmp_path, data):
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def _entry(seed):
    return {
        "id": seed.id,
        "name": seed.name,
        "defect_class": seed.defect_class,
        "description": seed.description,
    }


def _version_one_data():
    return {
        "format": "canaryfield-seeds/1",
        "seeds": [_entry(seed) for seed in SEEDS[:12]],
    }


def _version_two_data():
    return {
        "format": "canaryfield-seeds/2",
        "seeds": [_entry(seed) for seed in SEEDS[:16]],
    }


def test_a_version_one_pack_carrying_a_grown_registry_id_is_refused(
    tmp_path,
):
    data = _version_one_data()
    data["seeds"][11] = _entry(SEEDS[12])
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_one_pack_with_an_invented_id_is_refused(tmp_path):
    data = _version_one_data()
    data["seeds"][0]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_version_two_pack_with_an_invented_id_is_refused(tmp_path):
    data = _version_two_data()
    data["seeds"][15]["id"] = "seed-99"
    with pytest.raises(ValueError, match="ids do not match"):
        load_seed_pack(_pack_file(tmp_path, data))


def test_pack_entry_order_does_not_matter_for_acceptance(tmp_path):
    data = seed_pack_data()
    data["seeds"] = list(reversed(data["seeds"]))
    entries = load_seed_pack(_pack_file(tmp_path, data))
    assert len(entries) == 28
    assert {entry["id"] for entry in entries} == {
        seed.id for seed in SEEDS
    }


def test_a_pack_entry_whose_defect_class_disagrees_with_the_registry_is_refused(
    tmp_path,
):
    data = seed_pack_data()
    data["seeds"][3]["defect_class"] = "wrong predicate"
    with pytest.raises(
        ValueError,
        match="seed pack entry seed-03 disagrees with the registry"
        " on defect_class",
    ):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_entry_whose_name_disagrees_with_the_registry_is_refused(
    tmp_path,
):
    data = _version_one_data()
    data["seeds"][0]["name"] = "a renamed seed"
    with pytest.raises(
        ValueError,
        match="seed pack entry seed-00 disagrees with the registry"
        " on name",
    ):
        load_seed_pack(_pack_file(tmp_path, data))


def test_a_pack_entry_description_is_prose_and_is_not_verified(tmp_path):
    data = seed_pack_data()
    data["seeds"][5]["description"] = (
        "A reworded description of the same planted defect"
    )
    entries = load_seed_pack(_pack_file(tmp_path, data))
    assert len(entries) == 28
    assert entries[5]["description"] == (
        "A reworded description of the same planted defect"
    )
