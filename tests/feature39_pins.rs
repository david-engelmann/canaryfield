//! Feature slice 39 pins, the registry grown a forty seventh time.
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
        "canaryfield-feature39-{}-{tag}",
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
fn test_registry_has_two_hundred_unique_seeds() {
    assert_eq!(SEEDS.len(), 200, "the registry stands at 200 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 200, "every seed id stands unique");
    assert_eq!(SEEDS[199].id, "seed-199", "the last seed stands at 199");
}

// Seed-196, dropped last assignment value set element.

const LAST_ASSIGN_SET_MODULE: &str = "values = {alpha, beta}\nkeep = delta\n";
const LAST_ASSIGN_SET_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_196_plants_on_an_assignment_value_that_is_a_two_element_set() {
    let (planted, _) = applied_render(LAST_ASSIGN_SET_MODULE, "seed-196");
    assert!(planted, "an assignment value set plants");
}

#[test]
fn test_seed_196_removes_the_last_assignment_value_set_element() {
    let (_, render) = applied_render(LAST_ASSIGN_SET_MODULE, "seed-196");
    assert_eq!(render, "values = {alpha}\nkeep = delta");
}

#[test]
fn test_seed_196_keeps_the_earlier_value_elements_standing() {
    let source = "values = {alpha, beta, gamma}\n";
    let (_, render) = applied_render(source, "seed-196");
    assert_eq!(render, "values = {alpha, beta}");
}

#[test]
fn test_seed_196_removes_the_last_element_of_every_assignment_value_set() {
    let source = "values = {alpha, beta}\nothers = {one, two}\n";
    let (_, render) = applied_render(source, "seed-196");
    assert_eq!(render, "values = {alpha}\nothers = {one}");
}

#[test]
fn test_seed_196_leaves_a_single_element_assignment_value_set_standing() {
    let source = "values = {alpha}\n";
    let (planted, render) = applied_render(source, "seed-196");
    assert!(
        !planted,
        "a one element assignment value set does not plant"
    );
    assert_eq!(render, "values = {alpha}");
}

#[test]
fn test_seed_196_leaves_a_set_standing_outside_an_assignment_value() {
    let (planted, render) = applied_render("pick({alpha, beta})\n", "seed-196");
    assert!(
        !planted,
        "a set standing outside an assignment value is not handed to a target"
    );
    assert_eq!(render, "pick({alpha, beta})");
}

#[test]
fn test_seed_196_verdict_is_caught() {
    let target = target_with(LAST_ASSIGN_SET_MODULE, LAST_ASSIGN_SET_GATE, "caught196");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-196"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-197, dropped first assignment value set element.

const FIRST_ASSIGN_SET_MODULE: &str = "values = {alpha, beta}\nkeep = delta\n";
const FIRST_ASSIGN_SET_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_197_plants_on_an_assignment_value_that_is_a_two_element_set() {
    let (planted, _) = applied_render(FIRST_ASSIGN_SET_MODULE, "seed-197");
    assert!(planted, "an assignment value set plants");
}

#[test]
fn test_seed_197_removes_the_first_assignment_value_set_element() {
    let (_, render) = applied_render(FIRST_ASSIGN_SET_MODULE, "seed-197");
    assert_eq!(render, "values = {beta}\nkeep = delta");
}

#[test]
fn test_seed_197_keeps_the_later_value_elements_standing() {
    let source = "values = {alpha, beta, gamma}\n";
    let (_, render) = applied_render(source, "seed-197");
    assert_eq!(render, "values = {beta, gamma}");
}

#[test]
fn test_seed_197_removes_the_first_element_of_every_assignment_value_set() {
    let source = "values = {alpha, beta}\nothers = {one, two}\n";
    let (_, render) = applied_render(source, "seed-197");
    assert_eq!(render, "values = {beta}\nothers = {two}");
}

#[test]
fn test_seed_197_leaves_a_single_element_assignment_value_set_standing() {
    let source = "values = {alpha}\n";
    let (planted, render) = applied_render(source, "seed-197");
    assert!(
        !planted,
        "a one element assignment value set does not plant"
    );
    assert_eq!(render, "values = {alpha}");
}

#[test]
fn test_seed_197_leaves_a_set_standing_outside_an_assignment_value() {
    let (planted, render) = applied_render("pick({alpha, beta})\n", "seed-197");
    assert!(
        !planted,
        "a set standing outside an assignment value is not handed to a target"
    );
    assert_eq!(render, "pick({alpha, beta})");
}

#[test]
fn test_seed_197_verdict_is_caught() {
    let target = target_with(FIRST_ASSIGN_SET_MODULE, FIRST_ASSIGN_SET_GATE, "caught197");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-197"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-198, dropped last comparison operand set element.

const LAST_COMPARE_SET_MODULE: &str = "check = {alpha, beta} == probe\n";
const LAST_COMPARE_SET_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_198_plants_on_a_comparison_operand_that_is_a_two_element_set() {
    let (planted, _) = applied_render(LAST_COMPARE_SET_MODULE, "seed-198");
    assert!(planted, "a comparison operand set plants");
}

#[test]
fn test_seed_198_removes_the_last_comparison_operand_set_element() {
    let (_, render) = applied_render(LAST_COMPARE_SET_MODULE, "seed-198");
    assert_eq!(render, "check = {alpha} == probe");
}

#[test]
fn test_seed_198_keeps_the_earlier_operand_elements_standing() {
    let source = "check = {alpha, beta, gamma} == probe\n";
    let (_, render) = applied_render(source, "seed-198");
    assert_eq!(render, "check = {alpha, beta} == probe");
}

#[test]
fn test_seed_198_removes_the_last_element_of_every_comparison_operand_set() {
    let source = "check = {alpha, beta} == {one, two}\n";
    let (_, render) = applied_render(source, "seed-198");
    assert_eq!(render, "check = {alpha} == {one}");
}

#[test]
fn test_seed_198_leaves_a_single_element_comparison_operand_set_standing() {
    let source = "check = {alpha} == probe\n";
    let (planted, render) = applied_render(source, "seed-198");
    assert!(
        !planted,
        "a one element comparison operand set does not plant"
    );
    assert_eq!(render, "check = {alpha} == probe");
}

#[test]
fn test_seed_198_leaves_a_set_standing_outside_a_comparison() {
    let (planted, render) = applied_render("values = {alpha, beta}\n", "seed-198");
    assert!(
        !planted,
        "a set standing outside a comparison is not an operand"
    );
    assert_eq!(render, "values = {alpha, beta}");
}

#[test]
fn test_seed_198_verdict_is_caught() {
    let target = target_with(LAST_COMPARE_SET_MODULE, LAST_COMPARE_SET_GATE, "caught198");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-198"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-199, dropped first comparison operand set element.

const FIRST_COMPARE_SET_MODULE: &str = "check = {alpha, beta} == probe\n";
const FIRST_COMPARE_SET_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_199_plants_on_a_comparison_operand_that_is_a_two_element_set() {
    let (planted, _) = applied_render(FIRST_COMPARE_SET_MODULE, "seed-199");
    assert!(planted, "a comparison operand set plants");
}

#[test]
fn test_seed_199_removes_the_first_comparison_operand_set_element() {
    let (_, render) = applied_render(FIRST_COMPARE_SET_MODULE, "seed-199");
    assert_eq!(render, "check = {beta} == probe");
}

#[test]
fn test_seed_199_keeps_the_later_operand_elements_standing() {
    let source = "check = {alpha, beta, gamma} == probe\n";
    let (_, render) = applied_render(source, "seed-199");
    assert_eq!(render, "check = {beta, gamma} == probe");
}

#[test]
fn test_seed_199_removes_the_first_element_of_every_comparison_operand_set() {
    let source = "check = {alpha, beta} == {one, two}\n";
    let (_, render) = applied_render(source, "seed-199");
    assert_eq!(render, "check = {beta} == {two}");
}

#[test]
fn test_seed_199_leaves_a_single_element_comparison_operand_set_standing() {
    let source = "check = {alpha} == probe\n";
    let (planted, render) = applied_render(source, "seed-199");
    assert!(
        !planted,
        "a one element comparison operand set does not plant"
    );
    assert_eq!(render, "check = {alpha} == probe");
}

#[test]
fn test_seed_199_leaves_a_set_standing_outside_a_comparison() {
    let (planted, render) = applied_render("values = {alpha, beta}\n", "seed-199");
    assert!(
        !planted,
        "a set standing outside a comparison is not an operand"
    );
    assert_eq!(render, "values = {alpha, beta}");
}

#[test]
fn test_seed_199_verdict_is_caught() {
    let target = target_with(
        FIRST_COMPARE_SET_MODULE,
        FIRST_COMPARE_SET_GATE,
        "caught199",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-199"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty eighth format.

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
fn test_version_forty_seven_pack_still_loads_the_one_hundred_and_ninety_six() {
    let dir = fresh_dir("pack47");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(196).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/47", entries));
    let loaded = load_seed_pack(&path).expect("the version forty seven pack loads");
    assert_eq!(loaded.len(), 196);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_eight_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[199].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-200".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[199] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/48", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_seven_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(195).collect();
    entries.push(standing_entries()[196].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/47", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_eight_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(196).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/48", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and ninety six entries under the version forty eighth format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 200);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[199], "seed-199");
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
        written.contains("\"canaryfield-seeds/48\""),
        "the written pack carries the forty eighth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
