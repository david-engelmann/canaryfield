"""Pins for the seed registry as data after the Round 15 retirement.

The retirement of PORT2-9 removed canaryfield/seeds.py whole. The
registry data the standing Python machinery reads stands in
canaryfield/registry.py, read from the seed pack document the
package ships. These pins hold the package to that shape, the
package imports cleanly, the registry carries the forty eight
seeds in order with exactly the fields the pack carries, and the
behaviour that retired refuses in words instead of answering.
"""

import json
from dataclasses import fields
from pathlib import Path

import pytest

import canaryfield
import canaryfield.engine
import canaryfield.registry
import canaryfield.seedpack
from canaryfield import SEEDS, SEEDS_BY_ID, Seed

ROOT = Path(__file__).resolve().parents[1]


def test_package_imports_cleanly():
    assert canaryfield.run_field is canaryfield.engine.run_field
    assert canaryfield.load_seed_pack is canaryfield.seedpack.load_seed_pack
    assert canaryfield.write_seed_pack is canaryfield.seedpack.write_seed_pack


def test_registry_carries_the_forty_eight_seeds_in_order():
    assert [seed.id for seed in SEEDS] == [
        f"seed-{index:02d}" for index in range(48)
    ]
    assert list(SEEDS_BY_ID) == [seed.id for seed in SEEDS]


def test_registry_records_carry_exactly_the_pack_fields():
    assert [field.name for field in fields(Seed)] == [
        "id",
        "name",
        "defect_class",
        "description",
    ]
    first = SEEDS_BY_ID["seed-00"]
    assert first.name == "stale version string"
    assert first.defect_class == "stale metadata"
    assert SEEDS_BY_ID["seed-47"].name == "dropped logical negation"


def test_engine_and_seedpack_read_the_same_registry():
    assert canaryfield.engine.SEEDS is SEEDS
    assert canaryfield.seedpack.SEEDS is SEEDS
    registry = canaryfield.seedpack.PACK_REGISTRIES["canaryfield-seeds/1"]
    assert list(registry) == [seed.id for seed in SEEDS[:12]]


def test_pack_data_matches_the_shipped_example(tmp_path):
    example = json.loads(
        (ROOT / "examples" / "seed-pack.json").read_text(encoding="utf-8")
    )
    assert canaryfield.seedpack.seed_pack_data() == example
    written = canaryfield.write_seed_pack(tmp_path / "pack.json")
    assert written.read_bytes() == (
        ROOT / "examples" / "seed-pack.json"
    ).read_bytes()


def test_seed_apply_refuses_in_words():
    with pytest.raises(NotImplementedError) as caught:
        SEEDS[0].apply(None)
    assert "Rust crate" in str(caught.value)


def test_plant_seed_still_refuses_in_words(tmp_path):
    with pytest.raises(NotImplementedError) as caught:
        canaryfield.engine.plant_seed(tmp_path, SEEDS[0])
    assert "Rust crate" in str(caught.value)


def _doctored_pack(tmp_path, monkeypatch, mutate):
    data = json.loads(
        (ROOT / "canaryfield" / "seed-pack.json").read_text(encoding="utf-8")
    )
    mutate(data)
    path = tmp_path / "pack.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    monkeypatch.setattr(canaryfield.registry, "_PACK_PATH", path)
    return path


def test_registry_loader_refuses_a_short_pack(tmp_path, monkeypatch):
    _doctored_pack(
        tmp_path, monkeypatch, lambda data: data["seeds"].pop()
    )
    with pytest.raises(ValueError, match="carries 47 seeds"):
        canaryfield.registry._load_seeds()


def test_registry_loader_refuses_a_grown_pack(tmp_path, monkeypatch):
    def mutate(data):
        extra = dict(data["seeds"][0])
        extra["id"] = "seed-99"
        data["seeds"].append(extra)

    _doctored_pack(tmp_path, monkeypatch, mutate)
    with pytest.raises(ValueError, match="carries 49 seeds"):
        canaryfield.registry._load_seeds()


def test_registry_loader_refuses_a_duplicate_id(tmp_path, monkeypatch):
    def mutate(data):
        data["seeds"][1]["id"] = data["seeds"][0]["id"]

    _doctored_pack(tmp_path, monkeypatch, mutate)
    with pytest.raises(ValueError, match="duplicate id"):
        canaryfield.registry._load_seeds()


def test_registry_loader_refuses_a_non_text_field(tmp_path, monkeypatch):
    def mutate(data):
        data["seeds"][0]["id"] = 7

    _doctored_pack(tmp_path, monkeypatch, mutate)
    with pytest.raises(ValueError, match="non empty piece of text"):
        canaryfield.registry._load_seeds()


def test_registry_loader_refuses_a_non_text_description(
    tmp_path, monkeypatch
):
    def mutate(data):
        data["seeds"][3]["description"] = None

    _doctored_pack(tmp_path, monkeypatch, mutate)
    with pytest.raises(ValueError, match="non empty piece of text"):
        canaryfield.registry._load_seeds()


def test_seed_record_fields_carry_no_defaults():
    import dataclasses

    for field in fields(Seed):
        assert field.default is dataclasses.MISSING
