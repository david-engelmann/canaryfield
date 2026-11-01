import ast

from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import SAMPLE_SOURCE


def test_registry_has_twenty_eight_unique_seeds():
    assert len(SEEDS) == 28
    ids = [seed.id for seed in SEEDS]
    assert len(set(ids)) == 28
    assert ids == [f"seed-{index:02d}" for index in range(28)]


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


def test_main_fixture_applies_the_seeds_it_covers():
    # The main fixture was built for the original twelve seeds and
    # also exercises seed-12 through the division in safe_div and
    # seed-16 through the handler in safe_div and seed-20 through
    # the greet call in banner and seed-21 through its plain string
    # literals. The remaining slice 5 and slice 7 and slice 8 and
    # slice 9 seeds carry their own fixtures in test_slice5.py and
    # test_slice7.py and test_slice8.py and test_slice9.py, because
    # this fixture holds no else branch, no compound condition, no
    # await, no dictionary literal, no while loop or negated test,
    # no for loop, no unary minus, no break statement, no augmented
    # assignment, no identity comparison, no subscript, and no call
    # carrying keyword arguments.
    applied = []
    for seed in SEEDS:
        tree = ast.parse(SAMPLE_SOURCE)
        if seed.apply(tree):
            applied.append(seed.id)
    assert applied == [seed.id for seed in SEEDS[:13]] + ["seed-16", "seed-20", "seed-21"]
