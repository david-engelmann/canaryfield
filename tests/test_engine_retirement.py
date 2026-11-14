"""Pins for the retirement of PORT5-8.

The machinery below the bridges stands retired, and its
names stand absent from the engine module. The standing
surface is the four status constants, the seed table
import, the discovery, the planting bridge, and the run
bridge with its signature whole.
"""

import inspect

from canaryfield import engine
from canaryfield.registry import SEEDS

RETIRED = [
    "hash_tree",
    "_copy_tree",
    "_run_gate",
    "_summary",
    "_git",
    "_clean_caches",
    "_restore_checkout",
    "_run_git_mode",
    "_GIT_IDENTITY",
    "IGNORED_PARTS",
]


def test_the_retired_names_stand_absent():
    for name in RETIRED:
        assert not hasattr(engine, name), name


def test_the_standing_surface_stands():
    assert engine.CAUGHT == "caught"
    assert engine.WAVED == "waved-through"
    assert engine.NOT_APPLICABLE == "not-applicable"
    assert engine.GATE_BROKEN == "gate-broken"
    assert engine.SEEDS is SEEDS
    assert callable(engine.plant_seed)
    assert callable(engine.run_field)


def test_the_run_signature_stands_whole():
    signature = inspect.signature(engine.run_field)
    defaults = {
        name: parameter.default
        for name, parameter in signature.parameters.items()
    }
    assert defaults == {
        "target": inspect.Parameter.empty,
        "gates": inspect.Parameter.empty,
        "timeout": 120,
        "work_root": None,
        "keep_scratch": False,
        "plant_mode": "copy",
    }
