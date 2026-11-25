//! Feature slice 5 pins, the registry grown a thirteenth time.
//!
//! The behaviour pins drive the applies directly over parsed
//! sources and compare the rendering the port spells against
//! the spelling the contract's obligations state, and the
//! verdict pins drive the run over synthetic targets whose
//! gates turn on the planted behaviour, in the shape the
//! earlier feature slices set. The pack pins build their
//! documents from the crate's own pack data with one fact
//! changed at a time, so each refusal is read against the
//! single fact that earns it.

use std::fs;
use std::path::PathBuf;

use canaryfield::engine::{run, RunRequest};
use canaryfield::json::{self, JsonValue};
use canaryfield::pyapply::apply_seed;
use canaryfield::pyarena::{parse_tree, render_tree};
use canaryfield::registry::SEEDS;
use canaryfield::seedpack::{load_seed_pack, seed_pack_data, seed_pack_text};

fn applied_render(source: &str, seed: &str) -> (bool, String) {
    let mut tree = parse_tree(source).expect("the source parses");
    let planted = apply_seed(&mut tree, seed).expect("the application stands");
    (
        planted,
        render_tree(&tree).expect("the tree renders").to_string(),
    )
}

fn fresh_dir(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("canaryfield-feature5-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("the directory stands");
    dir
}

fn target_with(module: &str, gate_script: &str, tag: &str) -> PathBuf {
    let dir = fresh_dir(tag);
    fs::write(dir.join("mod.py"), module).expect("the module writes");
    fs::write(dir.join("check.sh"), gate_script).expect("the gate script writes");
    dir
}

fn run_target(target: &PathBuf) -> JsonValue {
    let request = RunRequest {
        target: target.to_string_lossy().into_owned(),
        gates: vec![("tests".to_string(), "sh check.sh".to_string())],
        timeout: 30.0,
        work_root: None,
        keep_scratch: false,
        plant_mode: "copy".to_string(),
    };
    run(&request).expect("the run completes")
}

fn cell_of(result: &JsonValue, seed: &str) -> String {
    let rows = match result.get("rows") {
        Some(JsonValue::Array(items)) => items,
        _ => panic!("the result carries rows"),
    };
    for row in rows {
        if row.get("seed").and_then(|value| value.as_str()) == Some(seed) {
            return row
                .get("gates")
                .and_then(|gates| gates.get("tests"))
                .and_then(|cell| cell.as_str())
                .expect("the row carries the tests cell")
                .to_string();
        }
    }
    panic!("the result carries the {seed} row");
}

const DICT_MODULE: &str = "def check():\n    return {'a': 1, 'b': 2}\n";
const DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'a': 1, 'b': 2\" mod.py\n";
const LIST_MODULE: &str = "def check():\n    return [1, 2, 3]\n";
const LIST_GATE: &str = "#!/bin/sh\ngrep -q \"\\[1, 2, 3\\]\" mod.py\n";
const ASSERT_MODULE: &str = "def check(x):\n    y = x + 1\n    assert y > 0\n    return y\n";
const ASSERT_GATE: &str = "#!/bin/sh\ngrep -q \"assert y > 0\" mod.py\n";
const CONST_MODULE: &str = "def check():\n    return 5\n";
const CONST_GATE: &str = "#!/bin/sh\ngrep -q \"return 5\" mod.py\n";

// The registry pins of F5-1.

#[test]
fn test_registry_has_one_hundred_and_forty_four_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        144,
        "the grown registry holds one hundred and forty four"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 144, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-143"));
    for (index, seed) in SEEDS.iter().enumerate() {
        assert_eq!(
            seed.id,
            format!("seed-{index:02}"),
            "the ids run in order at {index}"
        );
    }
}

#[test]
fn test_seed_zero_is_the_stale_version_string() {
    let first = SEEDS.first().expect("a first seed stands");
    assert_eq!(first.id, "seed-00");
    assert_eq!(first.name, "stale version string");
}

#[test]
fn test_seed_metadata_is_complete() {
    for seed in SEEDS {
        assert!(!seed.name.is_empty(), "{} carries a name", seed.id);
        assert!(
            !seed.defect_class.is_empty(),
            "{} carries a defect class",
            seed.id
        );
        assert!(
            !seed.description.is_empty(),
            "{} carries a description",
            seed.id
        );
    }
    let grown: Vec<&str> = SEEDS[60..].iter().map(|seed| seed.name).collect();
    assert_eq!(
        grown,
        [
            "swapped dictionary entry values",
            "swapped sequence literal elements",
            "dropped assert statement",
            "added numeric negation",
            "dropped with target",
            "dropped assignment expression",
            "dropped delete statement",
            "dropped match case",
            "swapped slice bounds",
            "swapped branch bodies",
            "dropped match guard",
            "dropped yield-from delegation",
            "swapped comprehension generator order",
            "dropped argument unpacking",
            "swapped formatted string values",
            "dropped call statement",
            "swapped decorator order",
            "dropped parameter default",
            "swapped base class order",
            "swapped match case order",
            "swapped with items",
            "dropped asynchronous modifier",
            "swapped keyword argument order",
            "swapped import names",
            "dropped with item",
            "swapped try handler order",
            "dropped argument annotation",
            "swapped assignment target order",
            "dropped return annotation",
            "swapped statement order",
            "swapped comprehension condition order",
            "dropped starred argument",
            "dropped raise statement",
            "dropped starred display element",
            "dropped loop control statement",
            "swapped try else bodies",
            "dropped class keyword argument",
            "swapped loop else bodies",
            "dropped starred parameter",
            "dropped docstring",
            "dropped parameter",
            "dropped elif branch",
            "dropped import alias",
            "dropped yield value",
            "dropped assert message",
            "dropped formatted value conversion",
            "swapped loop target elements",
            "dropped keyword-only parameter",
            "dropped format spec",
            "dropped comparison tail",
            "dropped imported name",
            "dropped keyword-only default",
            "dropped slice step",
            "dropped annotation value",
            "dropped formatted value",
            "dropped raised error message",
            "dropped set element",
            "dropped decorator call",
            "dropped attribute",
            "dropped exception type",
            "dropped subscript",
            "dropped arithmetic operand",
            "dropped first call argument",
            "dropped first condition term",
            "dropped tuple element",
            "dropped conditional test",
            "dropped comparison head",
            "dropped assignment",
            "dropped annotated assignment",
            "dropped function definition",
            "dropped class definition",
            "dropped exception handler name",
            "dropped assignment target",
            "dropped last tuple element",
            "dropped last base class",
            "dropped decorator argument",
            "dropped unpacking target element",
            "dropped last unpacking target element",
            "dropped keyword unpacking",
            "dropped last dictionary entry",
            "dropped last keyword argument",
            "dropped last list element",
            "dropped first imported name",
            "dropped first boolean operand"
        ]
    );
}

// The seed-60 pins of F5-2.

#[test]
fn test_seed_sixty_plants_and_is_caught() {
    let target = target_with(DICT_MODULE, DICT_GATE, "caught60");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-60"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_exchanges_the_values_of_the_first_two_keyed_entries() {
    let (planted, rendered) = applied_render("def f():\n    return {'a': 1, 'b': 2}\n", "seed-60");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return {'a': 2, 'b': 1}");
}

#[test]
fn test_seed_sixty_plants_every_dictionary_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f():\n    x = {'a': 1, 'b': 2}\n    return {'c': 3, 'd': 4}\n",
        "seed-60",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    x = {'a': 2, 'b': 1}\n    return {'c': 4, 'd': 3}"
    );
}

#[test]
fn test_seed_sixty_keeps_the_keys_and_the_later_entries_standing() {
    let (planted, rendered) =
        applied_render("def f():\n    return {'a': 1, 'b': 2, 'c': 3}\n", "seed-60");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return {'a': 2, 'b': 1, 'c': 3}");
}

#[test]
fn test_seed_sixty_leaves_an_unkeyed_entry_standing_in_place() {
    let (planted, rendered) = applied_render(
        "def f(base):\n    return {**base, 'a': 1, 'b': 2}\n",
        "seed-60",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(base):\n    return {**base, 'a': 2, 'b': 1}"
    );
}

#[test]
fn test_seed_sixty_not_applicable_without_two_keyed_entries() {
    let target = target_with("def check():\n    return {'a': 1}\n", DICT_GATE, "na60");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-60"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-61 pins of F5-3.

#[test]
fn test_seed_sixty_one_plants_and_is_caught() {
    let target = target_with(LIST_MODULE, LIST_GATE, "caught61");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-61"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_one_exchanges_the_first_two_elements_of_a_list() {
    let (planted, rendered) = applied_render("def f():\n    return [1, 2, 3]\n", "seed-61");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return [2, 1, 3]");
}

#[test]
fn test_seed_sixty_one_exchanges_the_first_two_elements_of_a_tuple() {
    let (planted, rendered) = applied_render("def f():\n    return (1, 2)\n", "seed-61");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return (2, 1)");
}

#[test]
fn test_seed_sixty_one_plants_every_literal_in_one_file() {
    let (planted, rendered) =
        applied_render("def f():\n    x = [1, 2]\n    return (3, 4)\n", "seed-61");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    x = [2, 1]\n    return (4, 3)");
}

#[test]
fn test_seed_sixty_one_identical_elements_exchange_into_themselves() {
    let (planted, rendered) = applied_render("def f(x):\n    return [x, x]\n", "seed-61");
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    return [x, x]");
}

#[test]
fn test_seed_sixty_one_not_applicable_without_two_elements() {
    let target = target_with("def check():\n    return [1]\n", LIST_GATE, "na61");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-61"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-62 pins of F5-4.

#[test]
fn test_seed_sixty_two_plants_and_is_caught() {
    let target = target_with(ASSERT_MODULE, ASSERT_GATE, "caught62");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-62"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_two_removes_an_assert_statement() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    y = x + 1\n    assert y > 0\n    return y\n",
        "seed-62",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    y = x + 1\n    return y");
}

#[test]
fn test_seed_sixty_two_plants_every_assert_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    assert x > 0\n    y = x + 1\n    assert y > 1\n    return y\n",
        "seed-62",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    y = x + 1\n    return y");
}

#[test]
fn test_seed_sixty_two_removes_an_assert_with_a_message() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    y = x + 1\n    assert y > 0, 'too small'\n    return y\n",
        "seed-62",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    y = x + 1\n    return y");
}

#[test]
fn test_seed_sixty_two_never_empties_a_body() {
    let (planted, rendered) = applied_render("def f(x):\n    assert x > 0\n", "seed-62");
    assert!(!planted);
    assert_eq!(rendered, "def f(x):\n    assert x > 0");
}

#[test]
fn test_seed_sixty_two_not_applicable_without_an_assert() {
    let target = target_with("def check(x):\n    return x + 1\n", ASSERT_GATE, "na62");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-62"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-63 pins of F5-5.

#[test]
fn test_seed_sixty_three_plants_and_is_caught() {
    let target = target_with(CONST_MODULE, CONST_GATE, "caught63");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-63"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_three_negates_an_integer_constant() {
    let (planted, rendered) = applied_render("def f():\n    return 5\n", "seed-63");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return -5");
}

#[test]
fn test_seed_sixty_three_negates_a_float_constant() {
    let (planted, rendered) = applied_render("def f():\n    return 1.5\n", "seed-63");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return -1.5");
}

#[test]
fn test_seed_sixty_three_plants_every_constant_in_one_file() {
    let (planted, rendered) = applied_render("def f():\n    x = 5\n    return x + 2\n", "seed-63");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    x = -5\n    return x + -2");
}

#[test]
fn test_seed_sixty_three_leaves_a_negated_constant_standing() {
    let (planted, rendered) = applied_render("def f():\n    return -5\n", "seed-63");
    assert!(!planted);
    assert_eq!(rendered, "def f():\n    return -5");
}

#[test]
fn test_seed_sixty_three_leaves_zero_and_a_boolean_standing() {
    let (planted, rendered) = applied_render("def f():\n    x = 0\n    return True\n", "seed-63");
    assert!(!planted);
    assert_eq!(rendered, "def f():\n    x = 0\n    return True");
}

#[test]
fn test_seed_sixty_three_not_applicable_without_a_numeric_constant() {
    let target = target_with("def check():\n    return 'five'\n", CONST_GATE, "na63");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-63"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F5-6.

fn pack_document(format: &str, entries: Vec<JsonValue>) -> JsonValue {
    JsonValue::Object(vec![
        ("format".to_string(), JsonValue::Str(format.to_string())),
        ("seeds".to_string(), JsonValue::Array(entries)),
    ])
}

fn standing_entries() -> Vec<JsonValue> {
    match seed_pack_data().get("seeds") {
        Some(JsonValue::Array(items)) => items.clone(),
        other => panic!("the pack data carries its seeds: {other:?}"),
    }
}

fn write_document(dir: &PathBuf, document: &JsonValue) -> PathBuf {
    let path = dir.join("pack.json");
    fs::write(&path, json::dumps_indent2(document)).expect("the document writes");
    path
}

#[test]
fn test_version_thirteen_pack_still_loads_the_sixty() {
    let dir = fresh_dir("pack13");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(60).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/13", entries));
    let loaded = load_seed_pack(&path).expect("the version thirteen pack loads");
    assert_eq!(loaded.len(), 60);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-59".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fourteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    entries[63] = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Str("seed-99".to_string())),
        (
            "name".to_string(),
            JsonValue::Str("added numeric negation".to_string()),
        ),
        (
            "defect_class".to_string(),
            JsonValue::Str("wrong sign".to_string()),
        ),
        (
            "description".to_string(),
            JsonValue::Str(
                "Wraps an integer or float constant in a unary minus so a value carries the opposite sign from the one the code wrote"
                    .to_string(),
            ),
        ),
    ]);
    let path = write_document(&dir, &pack_document("canaryfield-seeds/14", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fourteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(59).collect();
    entries.push(standing_entries()[60].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/13", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fourteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(60).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/14", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "sixty entries under the version fourteen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 144);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_via_cli() {
    let dir = fresh_dir("packcli");
    let path = dir.join("cli-pack.json");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_canaryfield"))
        .arg("--write-seed-pack")
        .arg(&path)
        .output()
        .expect("the command runs");
    assert!(
        output.status.success(),
        "the write seed pack command stands"
    );
    let loaded = load_seed_pack(&path).expect("the cli pack loads");
    assert_eq!(loaded.len(), 144);
    let _ = fs::remove_dir_all(&dir);
}
