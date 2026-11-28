//! Feature slice 38 pins, the registry grown a forty sixth time.
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
        "canaryfield-feature38-{}-{tag}",
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
fn test_registry_has_two_hundred_and_twenty_unique_seeds() {
    assert_eq!(SEEDS.len(), 220, "the registry stands at 196 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "every seed id stands unique");
    assert_eq!(SEEDS[219].id, "seed-219", "the last seed stands at 195");
}

// Seed-192, dropped last nested list element.

const LAST_NESTED_MODULE: &str = "rows = [[alpha, beta], gamma]\nkeep = delta\n";
const LAST_NESTED_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_192_plants_on_a_nested_list_of_two_elements() {
    let (planted, _) = applied_render(LAST_NESTED_MODULE, "seed-192");
    assert!(planted, "a nested list plants");
}

#[test]
fn test_seed_192_removes_the_last_nested_list_element() {
    let (_, render) = applied_render(LAST_NESTED_MODULE, "seed-192");
    assert_eq!(render, "rows = [[alpha], gamma]\nkeep = delta");
}

#[test]
fn test_seed_192_keeps_the_earlier_nested_elements_standing() {
    let source = "rows = [[alpha, beta, gamma], delta]\n";
    let (_, render) = applied_render(source, "seed-192");
    assert_eq!(render, "rows = [[alpha, beta], delta]");
}

#[test]
fn test_seed_192_removes_the_last_element_of_every_nested_list() {
    let source = "rows = [[alpha, beta], [one, two]]\n";
    let (_, render) = applied_render(source, "seed-192");
    assert_eq!(render, "rows = [[alpha], [one]]");
}

#[test]
fn test_seed_192_leaves_a_single_element_nested_list_standing() {
    let source = "rows = [[alpha], gamma]\n";
    let (planted, render) = applied_render(source, "seed-192");
    assert!(!planted, "a one element nested list does not plant");
    assert_eq!(render, "rows = [[alpha], gamma]");
}

#[test]
fn test_seed_192_leaves_a_list_standing_outside_another_display() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-192");
    assert!(
        !planted,
        "a list standing outside another display is not nested"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_192_verdict_is_caught() {
    let target = target_with(LAST_NESTED_MODULE, LAST_NESTED_GATE, "caught192");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-192"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-193, dropped first nested list element.

const FIRST_NESTED_MODULE: &str = "rows = [[alpha, beta], gamma]\nkeep = delta\n";
const FIRST_NESTED_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_193_plants_on_a_nested_list_of_two_elements() {
    let (planted, _) = applied_render(FIRST_NESTED_MODULE, "seed-193");
    assert!(planted, "a nested list plants");
}

#[test]
fn test_seed_193_removes_the_first_nested_list_element() {
    let (_, render) = applied_render(FIRST_NESTED_MODULE, "seed-193");
    assert_eq!(render, "rows = [[beta], gamma]\nkeep = delta");
}

#[test]
fn test_seed_193_keeps_the_later_nested_elements_standing() {
    let source = "rows = [[alpha, beta, gamma], delta]\n";
    let (_, render) = applied_render(source, "seed-193");
    assert_eq!(render, "rows = [[beta, gamma], delta]");
}

#[test]
fn test_seed_193_removes_the_first_element_of_every_nested_list() {
    let source = "rows = [[alpha, beta], [one, two]]\n";
    let (_, render) = applied_render(source, "seed-193");
    assert_eq!(render, "rows = [[beta], [two]]");
}

#[test]
fn test_seed_193_leaves_a_single_element_nested_list_standing() {
    let source = "rows = [[alpha], gamma]\n";
    let (planted, render) = applied_render(source, "seed-193");
    assert!(!planted, "a one element nested list does not plant");
    assert_eq!(render, "rows = [[alpha], gamma]");
}

#[test]
fn test_seed_193_leaves_a_list_standing_outside_another_display() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-193");
    assert!(
        !planted,
        "a list standing outside another display is not nested"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_193_verdict_is_caught() {
    let target = target_with(FIRST_NESTED_MODULE, FIRST_NESTED_GATE, "caught193");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-193"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-194, dropped last for loop list element.

const LAST_FOR_MODULE: &str = "for item in [alpha, beta]:\n    total = item\n";
const LAST_FOR_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_194_plants_on_a_for_iterable_that_is_a_two_element_list() {
    let (planted, _) = applied_render(LAST_FOR_MODULE, "seed-194");
    assert!(planted, "a for loop list plants");
}

#[test]
fn test_seed_194_removes_the_last_for_loop_list_element() {
    let (_, render) = applied_render(LAST_FOR_MODULE, "seed-194");
    assert_eq!(render, "for item in [alpha]:\n    total = item");
}

#[test]
fn test_seed_194_keeps_the_earlier_iterable_elements_standing() {
    let source = "for item in [alpha, beta, gamma]:\n    total = item\n";
    let (_, render) = applied_render(source, "seed-194");
    assert_eq!(render, "for item in [alpha, beta]:\n    total = item");
}

#[test]
fn test_seed_194_removes_the_last_element_of_every_for_loop_list() {
    let source = "for item in [alpha, beta]:\n    total = item\nfor entry in [one, two]:\n    total = entry\n";
    let (_, render) = applied_render(source, "seed-194");
    assert_eq!(
        render,
        "for item in [alpha]:\n    total = item\nfor entry in [one]:\n    total = entry"
    );
}

#[test]
fn test_seed_194_leaves_a_single_element_for_loop_list_standing() {
    let source = "for item in [alpha]:\n    total = item\n";
    let (planted, render) = applied_render(source, "seed-194");
    assert!(!planted, "a one element for loop list does not plant");
    assert_eq!(render, "for item in [alpha]:\n    total = item");
}

#[test]
fn test_seed_194_leaves_a_list_standing_outside_a_for_iterable() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-194");
    assert!(
        !planted,
        "a list standing outside a for iterable is not walked by a loop"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_194_verdict_is_caught() {
    let target = target_with(LAST_FOR_MODULE, LAST_FOR_GATE, "caught194");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-194"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-195, dropped first for loop list element.

const FIRST_FOR_MODULE: &str = "for item in [alpha, beta]:\n    total = item\n";
const FIRST_FOR_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_195_plants_on_a_for_iterable_that_is_a_two_element_list() {
    let (planted, _) = applied_render(FIRST_FOR_MODULE, "seed-195");
    assert!(planted, "a for loop list plants");
}

#[test]
fn test_seed_195_removes_the_first_for_loop_list_element() {
    let (_, render) = applied_render(FIRST_FOR_MODULE, "seed-195");
    assert_eq!(render, "for item in [beta]:\n    total = item");
}

#[test]
fn test_seed_195_keeps_the_later_iterable_elements_standing() {
    let source = "for item in [alpha, beta, gamma]:\n    total = item\n";
    let (_, render) = applied_render(source, "seed-195");
    assert_eq!(render, "for item in [beta, gamma]:\n    total = item");
}

#[test]
fn test_seed_195_leaves_a_single_element_for_loop_list_standing() {
    let source = "for item in [alpha]:\n    total = item\n";
    let (planted, render) = applied_render(source, "seed-195");
    assert!(!planted, "a one element for loop list does not plant");
    assert_eq!(render, "for item in [alpha]:\n    total = item");
}

#[test]
fn test_seed_195_leaves_a_list_standing_outside_a_for_iterable() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-195");
    assert!(
        !planted,
        "a list standing outside a for iterable is not walked by a loop"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_195_verdict_is_caught() {
    let target = target_with(FIRST_FOR_MODULE, FIRST_FOR_GATE, "caught195");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-195"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty seventh format.

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
fn test_version_forty_six_pack_still_loads_the_one_hundred_and_ninety_two() {
    let dir = fresh_dir("pack46");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(192).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/46", entries));
    let loaded = load_seed_pack(&path).expect("the version forty six pack loads");
    assert_eq!(loaded.len(), 192);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_seven_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[195].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-196".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[195] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/47", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_six_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(191).collect();
    entries.push(standing_entries()[192].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/46", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty six pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_seven_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(192).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/47", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and ninety two entries under the version forty seventh format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 220);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[219], "seed-219");
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
        written.contains("\"canaryfield-seeds/53\""),
        "the written pack carries the forty seventh format"
    );
    let _ = fs::remove_dir_all(&dir);
}
