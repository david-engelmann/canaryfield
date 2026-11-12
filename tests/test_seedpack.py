"""Pins for the seed pack loader refusals, restored at Round 15.

These pins stood in tests/test_slice6.py and
tests/test_round1_fixes.py at the base tree and retired with
the apply surface under PORT2-9, but the machinery they pin,
the seed pack loader of canaryfield/seedpack.py, stands
untouched and answers in Python still. The Round 15 fix pass
restores them here, re-pointed at canaryfield.registry where
they named the retired canaryfield.seeds module. The report
module pins that retired in the same files are not restored
here, because they belong to the slice that ports report.py.

Every fixture here is synthetic and written for this test file.
"""

import json

import pytest

from canaryfield.registry import SEEDS
from canaryfield.seedpack import load_seed_pack, seed_pack_data


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
    assert len(entries) == 48
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
    assert len(entries) == 48
    assert entries[5]["description"] == (
        "A reworded description of the same planted defect"
    )


def _pack_with(tmp_path, mutate):
    data = seed_pack_data()
    mutate(data)
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_seed_pack_rejects_a_duplicate_id(tmp_path):
    def mutate(data):
        data["seeds"][1]["id"] = data["seeds"][0]["id"]

    with pytest.raises(ValueError, match="duplicate"):
        load_seed_pack(_pack_with(tmp_path, mutate))


def test_seed_pack_rejects_an_extra_seed(tmp_path):
    def mutate(data):
        extra = dict(data["seeds"][0])
        extra["id"] = "seed-99"
        data["seeds"].append(extra)

    with pytest.raises(ValueError, match="carries 49"):
        load_seed_pack(_pack_with(tmp_path, mutate))


def test_seed_pack_rejects_a_numeric_id(tmp_path):
    def mutate(data):
        data["seeds"][0]["id"] = 7

    with pytest.raises(ValueError, match="non empty piece of text"):
        load_seed_pack(_pack_with(tmp_path, mutate))
