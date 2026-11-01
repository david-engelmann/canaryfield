import ast

from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import SAMPLE_SOURCE


def test_registry_has_twelve_unique_seeds():
    assert len(SEEDS) == 12
    ids = [seed.id for seed in SEEDS]
    assert len(set(ids)) == 12
    assert ids == [f"seed-{index:02d}" for index in range(12)]


def test_seed_zero_is_the_stale_version_string():
    seed = SEEDS_BY_ID["seed-00"]
    tree = ast.parse(SAMPLE_SOURCE)
    assert seed.apply(tree) is True
    rendered = ast.unparse(tree)
    assert '"9.9.9"' in rendered or "'9.9.9'" in rendered
    assert "1.4.2" not in rendered


def test_seed_metadata_is_complete():
    for seed in SEEDS:
        assert seed.name.strip(), seed.id
        assert seed.defect_class.strip(), seed.id
        tree = ast.parse(SAMPLE_SOURCE)
        assert isinstance(seed.apply(tree), bool), seed.id


def test_every_seed_applies_to_the_main_fixture():
    applied = []
    for seed in SEEDS:
        tree = ast.parse(SAMPLE_SOURCE)
        if seed.apply(tree):
            applied.append(seed.id)
    assert applied == [seed.id for seed in SEEDS]
