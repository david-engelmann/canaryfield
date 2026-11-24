//! Feature slice 7 pins, the registry grown a fifteenth time.
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
        std::env::temp_dir().join(format!("canaryfield-feature7-{}-{tag}", std::process::id()));
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

#[test]
fn test_registry_has_one_hundred_and_twenty_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        120,
        "the grown registry holds one hundred and twenty"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 120, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-123"));
    for (index, seed) in SEEDS.iter().enumerate() {
        assert_eq!(
            seed.id,
            format!("seed-{index:02}"),
            "the ids run in order at {index}"
        );
    }
    assert_eq!(SEEDS[68].name, "swapped slice bounds");
    assert_eq!(SEEDS[69].name, "swapped branch bodies");
    assert_eq!(SEEDS[70].name, "dropped match guard");
    assert_eq!(SEEDS[71].name, "dropped yield-from delegation");
}

// Seed-68, swapped slice bounds.

const SLICE_MODULE: &str = "def check(items):\n    return items[1:3]\n";
const SLICE_GATE: &str = "#!/bin/sh\ngrep -q \"items\\[1:3\\]\" mod.py\n";

#[test]
fn test_seed_sixty_eight_plants_and_is_caught() {
    let target = target_with(SLICE_MODULE, SLICE_GATE, "caught68");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-68"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_eight_exchanges_a_slice_bounds() {
    let (planted, rendered) = applied_render("def f(items):\n    return items[1:3]\n", "seed-68");
    assert!(planted);
    assert_eq!(rendered, "def f(items):\n    return items[3:1]");
}

#[test]
fn test_seed_sixty_eight_keeps_a_step_standing() {
    let (planted, rendered) = applied_render("def f(items):\n    return items[1:6:2]\n", "seed-68");
    assert!(planted);
    assert_eq!(rendered, "def f(items):\n    return items[6:1:2]");
}

#[test]
fn test_seed_sixty_eight_plants_every_slice_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(items):\n    return items[0:2]\n\ndef g(items):\n    return items[3:5]\n",
        "seed-68",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(items):\n    return items[2:0]\n\ndef g(items):\n    return items[5:3]"
    );
}

#[test]
fn test_seed_sixty_eight_leaves_an_open_bound_standing() {
    let (planted, rendered) = applied_render("def f(items):\n    return items[:3]\n", "seed-68");
    assert!(!planted);
    assert_eq!(rendered, "def f(items):\n    return items[:3]");
    let (planted, rendered) = applied_render("def f(items):\n    return items[1:]\n", "seed-68");
    assert!(!planted);
    assert_eq!(rendered, "def f(items):\n    return items[1:]");
}

#[test]
fn test_seed_sixty_eight_not_applicable_without_two_bounds() {
    let target = target_with("def check(x):\n    return x + 1\n", SLICE_GATE, "na68");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-68"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-69, swapped branch bodies.

const BRANCH_MODULE: &str =
    "def check(flag):\n    if flag:\n        return 'up'\n    else:\n        return 'down'\n";
const BRANCH_GATE: &str = "#!/bin/sh\ngrep -A1 \"if flag:\" mod.py | grep -q \"return 'up'\"\n";

#[test]
fn test_seed_sixty_nine_plants_and_is_caught() {
    let target = target_with(BRANCH_MODULE, BRANCH_GATE, "caught69");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-69"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_sixty_nine_exchanges_branch_bodies() {
    let (planted, rendered) = applied_render(
        "def f(flag):\n    if flag:\n        return 'up'\n    else:\n        return 'down'\n",
        "seed-69",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(flag):\n    if flag:\n        return 'down'\n    else:\n        return 'up'"
    );
}

#[test]
fn test_seed_sixty_nine_exchanges_an_elif_chain_as_a_body() {
    let (planted, rendered) = applied_render(
        "def f(a, b):\n    if a:\n        return 'a'\n    elif b:\n        return 'b'\n",
        "seed-69",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(a, b):\n    if a:\n        if b:\n            return 'b'\n    else:\n        return 'a'"
    );
}

#[test]
fn test_seed_sixty_nine_plants_every_if_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(a):\n    if a:\n        return 1\n    else:\n        return 2\n\ndef g(b):\n    if b:\n        return 3\n    else:\n        return 4\n",
        "seed-69",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(a):\n    if a:\n        return 2\n    else:\n        return 1\n\ndef g(b):\n    if b:\n        return 4\n    else:\n        return 3"
    );
}

#[test]
fn test_seed_sixty_nine_leaves_an_if_without_an_else_standing() {
    let (planted, rendered) = applied_render(
        "def f(a):\n    if a:\n        return 1\n    return 0\n",
        "seed-69",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(a):\n    if a:\n        return 1\n    return 0"
    );
}

#[test]
fn test_seed_sixty_nine_not_applicable_without_an_else() {
    let target = target_with("def check(x):\n    return x + 1\n", BRANCH_GATE, "na69");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-69"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-70, dropped match guard.

const GUARD_MODULE: &str = "def check(code):\n    match code:\n        case x if x > 10:\n            return 'big'\n        case _:\n            return 'other'\n";
const GUARD_GATE: &str = "#!/bin/sh\ngrep -q \"if x > 10\" mod.py\n";

#[test]
fn test_seed_seventy_plants_and_is_caught() {
    let target = target_with(GUARD_MODULE, GUARD_GATE, "caught70");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-70"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_clears_a_match_guard() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case x if x > 10:\n            return 'big'\n        case _:\n            return 'other'\n",
        "seed-70",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case x:\n            return 'big'\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_seventy_clears_every_guard_in_one_match() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case x if x > 10:\n            return 'big'\n        case y if y > 0:\n            return 'some'\n        case _:\n            return 'other'\n",
        "seed-70",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case x:\n            return 'big'\n        case y:\n            return 'some'\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_seventy_leaves_an_unguarded_case_standing() {
    let (planted, rendered) = applied_render(
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n        case _:\n            return 'other'\n",
        "seed-70",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(code):\n    match code:\n        case 1:\n            return 'one'\n        case _:\n            return 'other'"
    );
}

#[test]
fn test_seed_seventy_not_applicable_without_a_guard() {
    let target = target_with("def check(x):\n    return x + 1\n", GUARD_GATE, "na70");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-70"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-71, dropped yield-from delegation.

const YIELDFROM_MODULE: &str = "def check(gen):\n    yield from gen\n";
const YIELDFROM_GATE: &str = "#!/bin/sh\ngrep -q \"yield from\" mod.py\n";

#[test]
fn test_seed_seventy_one_plants_and_is_caught() {
    let target = target_with(YIELDFROM_MODULE, YIELDFROM_GATE, "caught71");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-71"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_one_stands_a_yield_from_as_a_yield() {
    let (planted, rendered) = applied_render("def f(gen):\n    yield from gen\n", "seed-71");
    assert!(planted);
    assert_eq!(rendered, "def f(gen):\n    yield gen");
}

#[test]
fn test_seed_seventy_one_plants_every_yield_from_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(gen):\n    yield from gen\n\ndef g(gen):\n    yield from gen\n",
        "seed-71",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(gen):\n    yield gen\n\ndef g(gen):\n    yield gen"
    );
}

#[test]
fn test_seed_seventy_one_leaves_a_plain_yield_standing() {
    let (planted, rendered) = applied_render("def f(items):\n    yield items\n", "seed-71");
    assert!(!planted);
    assert_eq!(rendered, "def f(items):\n    yield items");
}

#[test]
fn test_seed_seventy_one_not_applicable_without_a_yield_from() {
    let target = target_with("def check(x):\n    return x + 1\n", YIELDFROM_GATE, "na71");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-71"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F7-6.

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
fn test_version_fifteen_pack_still_loads_the_sixty_eight() {
    let dir = fresh_dir("pack15");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(68).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/15", entries));
    let loaded = load_seed_pack(&path).expect("the version fifteen pack loads");
    assert_eq!(loaded.len(), 68);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-67".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_sixteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[71].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-99".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[71] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/16", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version sixteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(67).collect();
    entries.push(standing_entries()[68].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/15", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version fifteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_sixteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(68).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/16", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "sixty eight entries under the version sixteen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}
