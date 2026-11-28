//! Feature slice 42 pins, the registry grown a fiftieth time.
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
        "canaryfield-feature42-{}-{tag}",
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
fn test_registry_has_two_hundred_and_twelve_unique_seeds() {
    assert_eq!(SEEDS.len(), 212, "the registry stands at 212 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 212, "every seed id stands unique");
    assert_eq!(SEEDS[211].id, "seed-211", "the last seed stands at 211");
}

// Seed-208, dropped last binop operand list element.

const LAST_BINOP_LIST_MODULE: &str = "total = [alpha, beta] + rest\nkeep = gamma\n";
const LAST_BINOP_LIST_GATE: &str = "#!/bin/sh\ngrep -q \"beta\" mod.py\n";

#[test]
fn test_seed_208_plants_on_a_binop_operand_that_is_a_two_element_list() {
    let (planted, _) = applied_render(LAST_BINOP_LIST_MODULE, "seed-208");
    assert!(planted, "a binop operand list plants");
}

#[test]
fn test_seed_208_removes_the_last_binop_operand_list_element() {
    let (_, render) = applied_render(LAST_BINOP_LIST_MODULE, "seed-208");
    assert_eq!(render, "total = [alpha] + rest\nkeep = gamma");
}

#[test]
fn test_seed_208_keeps_the_earlier_operand_elements_standing() {
    let source = "total = [alpha, beta, gamma] + rest\n";
    let (_, render) = applied_render(source, "seed-208");
    assert_eq!(render, "total = [alpha, beta] + rest");
}

#[test]
fn test_seed_208_removes_the_last_element_of_every_binop_operand_list() {
    let source = "total = [alpha, beta] + [one, two]\n";
    let (_, render) = applied_render(source, "seed-208");
    assert_eq!(render, "total = [alpha] + [one]");
}

#[test]
fn test_seed_208_leaves_a_single_element_binop_operand_list_standing() {
    let source = "total = [alpha] + rest\n";
    let (planted, render) = applied_render(source, "seed-208");
    assert!(!planted, "a one element binop operand list does not plant");
    assert_eq!(render, "total = [alpha] + rest");
}

#[test]
fn test_seed_208_leaves_a_list_standing_outside_a_binop_operand() {
    let (planted, render) = applied_render("table = [alpha, beta]\n", "seed-208");
    assert!(
        !planted,
        "a list standing outside a binop operand is not an operand"
    );
    assert_eq!(render, "table = [alpha, beta]");
}

#[test]
fn test_seed_208_verdict_is_caught() {
    let target = target_with(LAST_BINOP_LIST_MODULE, LAST_BINOP_LIST_GATE, "caught208");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-208"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-209, dropped first binop operand list element.

const FIRST_BINOP_LIST_MODULE: &str = "total = [alpha, beta] + rest\nkeep = gamma\n";
const FIRST_BINOP_LIST_GATE: &str = "#!/bin/sh\ngrep -q \"alpha\" mod.py\n";

#[test]
fn test_seed_209_plants_on_a_binop_operand_that_is_a_two_element_list() {
    let (planted, _) = applied_render(FIRST_BINOP_LIST_MODULE, "seed-209");
    assert!(planted, "a binop operand list plants");
}

#[test]
fn test_seed_209_removes_the_first_binop_operand_list_element() {
    let (_, render) = applied_render(FIRST_BINOP_LIST_MODULE, "seed-209");
    assert_eq!(render, "total = [beta] + rest\nkeep = gamma");
}

#[test]
fn test_seed_209_keeps_the_later_operand_elements_standing() {
    let source = "total = [alpha, beta, gamma] + rest\n";
    let (_, render) = applied_render(source, "seed-209");
    assert_eq!(render, "total = [beta, gamma] + rest");
}

#[test]
fn test_seed_209_removes_the_first_element_of_every_binop_operand_list() {
    let source = "total = [alpha, beta] + [one, two]\n";
    let (_, render) = applied_render(source, "seed-209");
    assert_eq!(render, "total = [beta] + [two]");
}

#[test]
fn test_seed_209_leaves_a_single_element_binop_operand_list_standing() {
    let source = "total = [alpha] + rest\n";
    let (planted, render) = applied_render(source, "seed-209");
    assert!(!planted, "a one element binop operand list does not plant");
    assert_eq!(render, "total = [alpha] + rest");
}

#[test]
fn test_seed_209_leaves_a_list_standing_outside_a_binop_operand() {
    let (planted, render) = applied_render("table = [alpha, beta]\n", "seed-209");
    assert!(
        !planted,
        "a list standing outside a binop operand is not an operand"
    );
    assert_eq!(render, "table = [alpha, beta]");
}

#[test]
fn test_seed_209_verdict_is_caught() {
    let target = target_with(FIRST_BINOP_LIST_MODULE, FIRST_BINOP_LIST_GATE, "caught209");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-209"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-210, dropped last augmented assignment value tuple element.

const LAST_AUGASSIGN_TUPLE_MODULE: &str = "total += (alpha, beta)\nkeep = gamma\n";
const LAST_AUGASSIGN_TUPLE_GATE: &str = "#!/bin/sh\ngrep -q \"beta\" mod.py\n";

#[test]
fn test_seed_210_plants_on_an_augmented_assignment_value_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_AUGASSIGN_TUPLE_MODULE, "seed-210");
    assert!(planted, "an augmented assignment value tuple plants");
}

#[test]
fn test_seed_210_removes_the_last_augmented_assignment_value_tuple_element() {
    let (_, render) = applied_render(LAST_AUGASSIGN_TUPLE_MODULE, "seed-210");
    assert_eq!(render, "total += (alpha,)\nkeep = gamma");
}

#[test]
fn test_seed_210_keeps_the_earlier_value_elements_standing() {
    let source = "total += (alpha, beta, gamma)\n";
    let (_, render) = applied_render(source, "seed-210");
    assert_eq!(render, "total += (alpha, beta)");
}

#[test]
fn test_seed_210_removes_the_last_element_of_every_augmented_assignment_value_tuple() {
    let source = "total += (alpha, beta)\nother += (one, two)\n";
    let (_, render) = applied_render(source, "seed-210");
    assert_eq!(render, "total += (alpha,)\nother += (one,)");
}

#[test]
fn test_seed_210_leaves_a_single_element_augmented_assignment_value_tuple_standing() {
    let source = "total += (alpha,)\n";
    let (planted, render) = applied_render(source, "seed-210");
    assert!(
        !planted,
        "a one element augmented assignment value tuple does not plant"
    );
    assert_eq!(render, "total += (alpha,)");
}

#[test]
fn test_seed_210_leaves_a_tuple_standing_outside_an_augmented_assignment_value() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-210");
    assert!(
        !planted,
        "a tuple standing outside an augmented assignment value is not handed over"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_210_verdict_is_caught() {
    let target = target_with(
        LAST_AUGASSIGN_TUPLE_MODULE,
        LAST_AUGASSIGN_TUPLE_GATE,
        "caught210",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-210"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-211, dropped first augmented assignment value tuple element.

const FIRST_AUGASSIGN_TUPLE_MODULE: &str = "total += (alpha, beta)\nkeep = gamma\n";
const FIRST_AUGASSIGN_TUPLE_GATE: &str = "#!/bin/sh\ngrep -q \"alpha\" mod.py\n";

#[test]
fn test_seed_211_plants_on_an_augmented_assignment_value_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_AUGASSIGN_TUPLE_MODULE, "seed-211");
    assert!(planted, "an augmented assignment value tuple plants");
}

#[test]
fn test_seed_211_removes_the_first_augmented_assignment_value_tuple_element() {
    let (_, render) = applied_render(FIRST_AUGASSIGN_TUPLE_MODULE, "seed-211");
    assert_eq!(render, "total += (beta,)\nkeep = gamma");
}

#[test]
fn test_seed_211_keeps_the_later_value_elements_standing() {
    let source = "total += (alpha, beta, gamma)\n";
    let (_, render) = applied_render(source, "seed-211");
    assert_eq!(render, "total += (beta, gamma)");
}

#[test]
fn test_seed_211_removes_the_first_element_of_every_augmented_assignment_value_tuple() {
    let source = "total += (alpha, beta)\nother += (one, two)\n";
    let (_, render) = applied_render(source, "seed-211");
    assert_eq!(render, "total += (beta,)\nother += (two,)");
}

#[test]
fn test_seed_211_leaves_a_single_element_augmented_assignment_value_tuple_standing() {
    let source = "total += (alpha,)\n";
    let (planted, render) = applied_render(source, "seed-211");
    assert!(
        !planted,
        "a one element augmented assignment value tuple does not plant"
    );
    assert_eq!(render, "total += (alpha,)");
}

#[test]
fn test_seed_211_leaves_a_tuple_standing_outside_an_augmented_assignment_value() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-211");
    assert!(
        !planted,
        "a tuple standing outside an augmented assignment value is not handed over"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_211_verdict_is_caught() {
    let target = target_with(
        FIRST_AUGASSIGN_TUPLE_MODULE,
        FIRST_AUGASSIGN_TUPLE_GATE,
        "caught211",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-211"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the fifty first format.

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
fn test_version_fifty_pack_still_loads_the_two_hundred_and_eight() {
    let dir = fresh_dir("pack50");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(208).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/50", entries));
    let loaded = load_seed_pack(&path).expect("the version fifty pack loads");
    assert_eq!(loaded.len(), 208);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_one_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[211].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-212".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[211] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/51", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fifty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(207).collect();
    entries.push(standing_entries()[208].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/50", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version fifty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fifty_one_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(208).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/51", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "two hundred and eight entries under the version fifty first format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 212);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[211], "seed-211");
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
        written.contains("\"canaryfield-seeds/51\""),
        "the written pack carries the fifty first format"
    );
    let _ = fs::remove_dir_all(&dir);
}
