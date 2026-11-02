import ast

from canaryfield.seeds import SEEDS, SEEDS_BY_ID

from fixture_project import SAMPLE_SOURCE


def test_registry_has_forty_eight_unique_seeds():
    assert len(SEEDS) == 48
    ids = [seed.id for seed in SEEDS]
    assert len(set(ids)) == 48
    assert ids == [f"seed-{index:02d}" for index in range(48)]


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
    # literals and seed-28 through the raise in safe_div. The
    # remaining slice 5 and slice 7 and slice 8 and slice 9 and
    # slice 10 and slice 11 seeds carry their own fixtures in
    # test_slice5.py and test_slice7.py and test_slice8.py and
    # test_slice9.py and test_slice10.py and test_slice11.py,
    # because this fixture holds no else branch, no compound
    # condition, no await, no dictionary literal, no while loop or
    # negated test, no for loop, no unary minus, no break statement,
    # no augmented assignment, no identity comparison, no subscript,
    # no numeric default, no attribute load, no call carrying
    # keyword arguments, no broad exception handler, no finally
    # block, no decorator, and no definition carrying a second
    # argument default. The slice 12 seeds carry their own
    # fixtures in test_slice12.py, because this fixture holds no
    # try statement a dropped handler could plant on, no yield,
    # and no class definition. The slice 13 seeds carry their own
    # fixtures in test_slice13.py, because this fixture holds no
    # slice, no loop a reversed control could plant on, no scope
    # declaration, and no tuple valued assignment. The slice 14
    # seeds carry their own fixtures in test_slice14.py, because
    # this fixture holds no loop else clause, no call carrying a
    # keyword argument, no three argument range call, and no
    # negation standing outside a condition test.
    applied = []
    for seed in SEEDS:
        tree = ast.parse(SAMPLE_SOURCE)
        if seed.apply(tree):
            applied.append(seed.id)
    assert applied == [seed.id for seed in SEEDS[:13]] + ["seed-16", "seed-20", "seed-21", "seed-28"]
