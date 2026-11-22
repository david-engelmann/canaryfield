//! Feature slice 6 pins, the registry grown a fourteenth time.
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
        std::env::temp_dir().join(format!("canaryfield-feature6-{}-{tag}", std::process::id()));
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

const WITH_MODULE: &str =
    "def check():\n    with open('data.txt') as handle:\n        return handle.read()\n";
const WITH_GATE: &str = "#!/bin/sh\ngrep -q \"as handle\" mod.py\n";
const WALRUS_MODULE: &str =
    "def check(items):\n    if (count := len(items)) > 0:\n        return count\n    return 0\n";
const WALRUS_GATE: &str = "#!/bin/sh\ngrep -q \"count :=\" mod.py\n";
const DELETE_MODULE: &str = "def check(cache):\n    del cache['old']\n    return len(cache)\n";
const DELETE_GATE: &str = "#!/bin/sh\ngrep -q \"del cache\" mod.py\n";
const MATCH_MODULE: &str = "def check(code):\n    match code:\n        case 1:\n            return 'one'\n        case _:\n            return 'other'\n";
const MATCH_GATE: &str = "#!/bin/sh\ngrep -q \"case 1:\" mod.py\n";

// The registry pins of F6-1.

#[test]
fn test_registry_has_eighty_eight_unique_seeds() {
    assert_eq!(SEEDS.len(), 100, "the grown registry holds one hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 100, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-99"));
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
    let grown: Vec<&str> = SEEDS[64..].iter().map(|seed| seed.name).collect();
    assert_eq!(
        grown,
        [
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
            "dropped docstring"
        ]
    );
}

// The seed-64 pins of F6-2.

#[test]
fn test_seed_sixty_four_plants_and_is_caught() {
    let target = target_with(WITH_MODULE, WITH_GATE, "caught64");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-64"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_four_clears_a_with_target() {
    let (planted, rendered) = applied_render(
        "def f():\n    with open('data.txt') as handle:\n        return handle.read()\n",
        "seed-64",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    with open('data.txt'):\n        return handle.read()"
    );
}

#[test]
fn test_seed_sixty_four_clears_an_async_with_target() {
    let (planted, rendered) = applied_render(
        "async def f():\n    async with open('data.txt') as handle:\n        return handle.read()\n",
        "seed-64",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "async def f():\n    async with open('data.txt'):\n        return handle.read()"
    );
}

#[test]
fn test_seed_sixty_four_plants_every_with_item_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f():\n    with open('a') as first, open('b') as second:\n        x = first\n    with open('c') as third:\n        return third\n",
        "seed-64",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    with open('a'), open('b'):\n        x = first\n    with open('c'):\n        return third"
    );
}

#[test]
fn test_seed_sixty_four_clears_a_tuple_target_whole() {
    let (planted, rendered) = applied_render(
        "def f():\n    with open('a') as (first, second):\n        return first\n",
        "seed-64",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    with open('a'):\n        return first"
    );
}

#[test]
fn test_seed_sixty_four_leaves_a_targetless_item_standing() {
    let (planted, rendered) = applied_render(
        "def f():\n    with open('a') as handle:\n        x = handle\n    with open('b'):\n        return 1\n",
        "seed-64",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    with open('a'):\n        x = handle\n    with open('b'):\n        return 1"
    );
}

#[test]
fn test_seed_sixty_four_not_applicable_without_a_with_target() {
    let target = target_with(
        "def check():\n    with open('data.txt'):\n        return 1\n",
        WITH_GATE,
        "na64",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-64"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-65 pins of F6-3.

#[test]
fn test_seed_sixty_five_plants_and_is_caught() {
    let target = target_with(WALRUS_MODULE, WALRUS_GATE, "caught65");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-65"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_five_replaces_an_assignment_expression_in_a_test() {
    let (planted, rendered) = applied_render(
        "def f(items):\n    if (count := len(items)) > 0:\n        return count\n    return 0\n",
        "seed-65",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(items):\n    if len(items) > 0:\n        return count\n    return 0"
    );
}

#[test]
fn test_seed_sixty_five_replaces_an_assignment_expression_in_a_while_test() {
    let (planted, rendered) = applied_render(
        "def f(items):\n    while (item := items.pop()):\n        print(item)\n",
        "seed-65",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(items):\n    while items.pop():\n        print(item)"
    );
}

#[test]
fn test_seed_sixty_five_plants_every_assignment_expression_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(items):\n    x = (first := items[0])\n    if (second := items[1]) > first:\n        return second\n    return first\n",
        "seed-65",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(items):\n    x = items[0]\n    if items[1] > first:\n        return second\n    return first"
    );
}

#[test]
fn test_seed_sixty_five_collapses_a_nested_assignment_expression() {
    let (planted, rendered) = applied_render(
        "def f():\n    value = (outer := (inner := 5))\n    return value\n",
        "seed-65",
    );
    assert!(planted);
    assert_eq!(rendered, "def f():\n    value = 5\n    return value");
}

#[test]
fn test_seed_sixty_five_not_applicable_without_an_assignment_expression() {
    let target = target_with("def check():\n    return 1\n", WALRUS_GATE, "na65");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-65"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-66 pins of F6-4.

#[test]
fn test_seed_sixty_six_plants_and_is_caught() {
    let target = target_with(DELETE_MODULE, DELETE_GATE, "caught66");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-66"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_six_removes_a_delete_statement() {
    let (planted, rendered) = applied_render(
        "def f(cache):\n    del cache['old']\n    return cache\n",
        "seed-66",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(cache):\n    return cache");
}

#[test]
fn test_seed_sixty_six_plants_every_delete_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(cache):\n    del cache['a']\n    x = 1\n    del cache['b']\n    return x\n",
        "seed-66",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(cache):\n    x = 1\n    return x");
}

#[test]
fn test_seed_sixty_six_removes_a_delete_of_several_targets_whole() {
    let (planted, rendered) = applied_render(
        "def f():\n    first = 1\n    second = 2\n    del first, second\n    return 0\n",
        "seed-66",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    first = 1\n    second = 2\n    return 0"
    );
}

#[test]
fn test_seed_sixty_six_never_empties_a_body() {
    let (planted, rendered) = applied_render("def f(cache):\n    del cache\n", "seed-66");
    assert!(!planted);
    assert_eq!(rendered, "def f(cache):\n    del cache");
}

#[test]
fn test_seed_sixty_six_not_applicable_without_a_delete() {
    let target = target_with("def check(x):\n    return x + 1\n", DELETE_GATE, "na66");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-66"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-67 pins of F6-5.

#[test]
fn test_seed_sixty_seven_plants_and_is_caught() {
    let target = target_with(MATCH_MODULE, MATCH_GATE, "caught67");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-67"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_seven_removes_the_first_case() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'\n",
        "seed-67",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_sixty_seven_plants_every_match_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n        case _:\n            return 'other'\n\ndef g(code):\n    match code:\n        case 2:\n            return 'two'\n        case _:\n            return 'other'\n",
        "seed-67",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case _:\n            return 'other'\n\ndef g(code):\n    match code:\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_sixty_seven_keeps_the_later_cases_in_their_order() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n        case x if x > 10:\n            return 'big'\n        case _:\n            return 'other'\n",
        "seed-67",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case x if x > 10:\n            return 'big'\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_sixty_seven_never_empties_a_match() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n",
        "seed-67",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case 1:\n            return 'one'"
    );
}

#[test]
fn test_seed_sixty_seven_not_applicable_without_a_match() {
    let target = target_with("def check(x):\n    return x + 1\n", MATCH_GATE, "na67");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-67"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F6-6.

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
fn test_version_fourteen_pack_still_loads_the_sixty_four() {
    let dir = fresh_dir("pack14");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(64).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/14", entries));
    let loaded = load_seed_pack(&path).expect("the version fourteen pack loads");
    assert_eq!(loaded.len(), 64);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-63".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    entries[67] = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Str("seed-99".to_string())),
        (
            "name".to_string(),
            JsonValue::Str("dropped match case".to_string()),
        ),
        (
            "defect_class".to_string(),
            JsonValue::Str("wrong branch".to_string()),
        ),
        (
            "description".to_string(),
            JsonValue::Str(
                "Removes the first case of a match statement so a value the case would have taken falls to a later case or past the match altogether"
                    .to_string(),
            ),
        ),
    ]);
    let path = write_document(&dir, &pack_document("canaryfield-seeds/15", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fifteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fourteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(63).collect();
    entries.push(standing_entries()[64].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/14", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version fourteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fifteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(64).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/15", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "sixty four entries under the version fifteen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 100);
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
    assert_eq!(loaded.len(), 100);
    let _ = fs::remove_dir_all(&dir);
}
