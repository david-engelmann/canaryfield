//! Feature slice 43 pins, the registry grown a fifty first time.
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
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-feature43-{}-{tag}",
        std::process::id()
    ));
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
fn test_registry_has_two_hundred_and_sixteen_unique_seeds() {
    assert_eq!(SEEDS.len(), 216, "the registry stands at 216 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 216, "every seed id stands unique");
    assert_eq!(SEEDS[215].id, "seed-215", "the last seed stands at 215");
}

// Seed-212, flipped first comparison operator.

const FIRST_COMPARISON_MODULE: &str = "ready = alpha < beta\nkeep = gamma\n";
const FIRST_COMPARISON_GATE: &str = "#!/bin/sh\ngrep -q \"alpha > beta\" mod.py\n";

#[test]
fn test_seed_212_plants_on_a_text_carrying_a_comparison() {
    let (planted, _) = applied_render(FIRST_COMPARISON_MODULE, "seed-212");
    assert!(planted, "a comparison plants");
}

#[test]
fn test_seed_212_flips_the_first_comparison_operator() {
    let (_, render) = applied_render(FIRST_COMPARISON_MODULE, "seed-212");
    assert_eq!(render, "ready = alpha > beta\nkeep = gamma");
}

#[test]
fn test_seed_212_leaves_the_later_operators_of_a_chain_standing() {
    let source = "ready = alpha < beta < delta\n";
    let (_, render) = applied_render(source, "seed-212");
    assert_eq!(render, "ready = alpha > beta < delta");
}

#[test]
fn test_seed_212_flips_the_first_operator_of_every_comparison() {
    let source = "ready = alpha < beta\nsteady = one == two\n";
    let (_, render) = applied_render(source, "seed-212");
    assert_eq!(render, "ready = alpha > beta\nsteady = one != two");
}

#[test]
fn test_seed_212_flips_an_equality_operator_to_its_opposite() {
    let source = "ready = alpha == beta\n";
    let (_, render) = applied_render(source, "seed-212");
    assert_eq!(render, "ready = alpha != beta");
}

#[test]
fn test_seed_212_leaves_a_text_without_a_comparison_standing() {
    let (planted, render) = applied_render("total = alpha + beta\n", "seed-212");
    assert!(!planted, "a text without a comparison does not plant");
    assert_eq!(render, "total = alpha + beta");
}

#[test]
fn test_seed_212_verdict_is_caught() {
    let target = target_with(FIRST_COMPARISON_MODULE, FIRST_COMPARISON_GATE, "caught212");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-212"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-213, flipped first boolean operator.

const FIRST_BOOLEAN_MODULE: &str = "ready = alpha and beta\nkeep = gamma\n";
const FIRST_BOOLEAN_GATE: &str = "#!/bin/sh\ngrep -q \"alpha or beta\" mod.py\n";

#[test]
fn test_seed_213_plants_on_a_text_carrying_a_boolean_operation() {
    let (planted, _) = applied_render(FIRST_BOOLEAN_MODULE, "seed-213");
    assert!(planted, "a boolean operation plants");
}

#[test]
fn test_seed_213_flips_the_first_boolean_operator() {
    let (_, render) = applied_render(FIRST_BOOLEAN_MODULE, "seed-213");
    assert_eq!(render, "ready = alpha or beta\nkeep = gamma");
}

#[test]
fn test_seed_213_flips_an_or_to_an_and() {
    let source = "ready = alpha or beta\n";
    let (_, render) = applied_render(source, "seed-213");
    assert_eq!(render, "ready = alpha and beta");
}

#[test]
fn test_seed_213_leaves_the_later_boolean_operations_standing() {
    let source = "ready = alpha and beta\nsteady = one or two\n";
    let (_, render) = applied_render(source, "seed-213");
    assert_eq!(render, "ready = alpha or beta\nsteady = one or two");
}

#[test]
fn test_seed_213_flips_the_first_of_two_operators_on_one_line() {
    let source = "ready = alpha and beta\nsteady = one and two\n";
    let (_, render) = applied_render(source, "seed-213");
    assert_eq!(render, "ready = alpha or beta\nsteady = one and two");
}

#[test]
fn test_seed_213_leaves_a_text_without_a_boolean_operation_standing() {
    let (planted, render) = applied_render("total = alpha + beta\n", "seed-213");
    assert!(!planted, "a text without a boolean operation does not plant");
    assert_eq!(render, "total = alpha + beta");
}

#[test]
fn test_seed_213_verdict_is_caught() {
    let target = target_with(FIRST_BOOLEAN_MODULE, FIRST_BOOLEAN_GATE, "caught213");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-213"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-214, flipped first arithmetic operator.

const FIRST_ARITHMETIC_MODULE: &str = "total = alpha + beta\nkeep = gamma\n";
const FIRST_ARITHMETIC_GATE: &str = "#!/bin/sh\ngrep -q \"alpha - beta\" mod.py\n";

#[test]
fn test_seed_214_plants_on_a_text_carrying_an_arithmetic_operation() {
    let (planted, _) = applied_render(FIRST_ARITHMETIC_MODULE, "seed-214");
    assert!(planted, "an arithmetic operation plants");
}

#[test]
fn test_seed_214_swaps_the_first_arithmetic_operator() {
    let (_, render) = applied_render(FIRST_ARITHMETIC_MODULE, "seed-214");
    assert_eq!(render, "total = alpha - beta\nkeep = gamma");
}

#[test]
fn test_seed_214_swaps_a_multiplication_to_an_addition() {
    let source = "total = alpha * beta\n";
    let (_, render) = applied_render(source, "seed-214");
    assert_eq!(render, "total = alpha + beta");
}

#[test]
fn test_seed_214_leaves_the_later_arithmetic_operations_standing() {
    let source = "total = alpha + beta\nother = one * two\n";
    let (_, render) = applied_render(source, "seed-214");
    assert_eq!(render, "total = alpha - beta\nother = one * two");
}

#[test]
fn test_seed_214_skips_an_operator_the_swap_table_does_not_carry() {
    let source = "total = alpha / beta\nother = one + two\n";
    let (_, render) = applied_render(source, "seed-214");
    assert_eq!(render, "total = alpha / beta\nother = one - two");
}

#[test]
fn test_seed_214_leaves_a_text_without_a_swappable_operator_standing() {
    let (planted, render) = applied_render("total = alpha / beta\n", "seed-214");
    assert!(
        !planted,
        "a text without a swappable arithmetic operator does not plant"
    );
    assert_eq!(render, "total = alpha / beta");
}

#[test]
fn test_seed_214_verdict_is_caught() {
    let target = target_with(FIRST_ARITHMETIC_MODULE, FIRST_ARITHMETIC_GATE, "caught214");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-214"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-215, flipped first augmented assignment operator.

const FIRST_AUGASSIGN_MODULE: &str = "total += alpha\nkeep = gamma\n";
const FIRST_AUGASSIGN_GATE: &str = "#!/bin/sh\ngrep -q \"total -= alpha\" mod.py\n";

#[test]
fn test_seed_215_plants_on_a_text_carrying_an_augmented_assignment() {
    let (planted, _) = applied_render(FIRST_AUGASSIGN_MODULE, "seed-215");
    assert!(planted, "an augmented assignment plants");
}

#[test]
fn test_seed_215_swaps_the_first_augmented_assignment_operator() {
    let (_, render) = applied_render(FIRST_AUGASSIGN_MODULE, "seed-215");
    assert_eq!(render, "total -= alpha\nkeep = gamma");
}

#[test]
fn test_seed_215_swaps_a_multiplication_to_an_addition() {
    let source = "total *= alpha\n";
    let (_, render) = applied_render(source, "seed-215");
    assert_eq!(render, "total += alpha");
}

#[test]
fn test_seed_215_leaves_the_later_augmented_assignments_standing() {
    let source = "total += alpha\nother *= beta\n";
    let (_, render) = applied_render(source, "seed-215");
    assert_eq!(render, "total -= alpha\nother *= beta");
}

#[test]
fn test_seed_215_skips_an_operator_the_swap_table_does_not_carry() {
    let source = "total /= alpha\nother += beta\n";
    let (_, render) = applied_render(source, "seed-215");
    assert_eq!(render, "total /= alpha\nother -= beta");
}

#[test]
fn test_seed_215_leaves_a_text_without_a_swappable_operator_standing() {
    let (planted, render) = applied_render("total /= alpha\n", "seed-215");
    assert!(
        !planted,
        "a text without a swappable augmented operator does not plant"
    );
    assert_eq!(render, "total /= alpha");
}

#[test]
fn test_seed_215_verdict_is_caught() {
    let target = target_with(FIRST_AUGASSIGN_MODULE, FIRST_AUGASSIGN_GATE, "caught215");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-215"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the fifty second format.

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
fn test_version_fifty_one_pack_still_loads_the_two_hundred_and_twelve() {
    let dir = fresh_dir("pack51");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(212).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/51", entries));
    let loaded = load_seed_pack(&path).expect("the version fifty one pack loads");
    assert_eq!(loaded.len(), 212);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_two_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[215].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-216".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[215] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/52", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fifty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_one_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(211).collect();
    entries.push(standing_entries()[212].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/51", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version fifty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fifty_two_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(212).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/52", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "two hundred and twelve entries under the version fifty second format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 216);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[215], "seed-215");
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
        .expect("the door runs");
    assert!(output.status.success(), "the pack door exits clean");
    let written = fs::read_to_string(&path).expect("the written pack reads");
    assert_eq!(written, seed_pack_text());
    assert!(
        written.contains("\"canaryfield-seeds/52\""),
        "the written pack carries the fifty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
