//! Feature slice 31 pins, the registry grown a thirty ninth time.
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
        "canaryfield-feature31-{}-{tag}",
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
fn test_registry_has_two_hundred_and_four_unique_seeds() {
    assert_eq!(SEEDS.len(), 204, "the registry stands at 168 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 204, "every seed id stands unique");
    assert_eq!(SEEDS[203].id, "seed-203", "the last seed stands at 179");
}

// Seed-164, dropped last annotated assignment value element.

const LAST_VALUE_MODULE: &str = "total: tuple = (alpha, beta)\nkeep = gamma\n";
const LAST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_164_plants_on_an_annotated_assignment_carrying_a_two_element_tuple_value() {
    let (planted, _) = applied_render(LAST_VALUE_MODULE, "seed-164");
    assert!(
        planted,
        "an annotated assignment carrying a two element tuple value plants"
    );
}

#[test]
fn test_seed_164_removes_the_last_annotated_assignment_value_element() {
    let (_, render) = applied_render(LAST_VALUE_MODULE, "seed-164");
    assert_eq!(render, "total: tuple = (alpha,)\nkeep = gamma");
}

#[test]
fn test_seed_164_keeps_the_earlier_value_elements_standing() {
    let (_, render) = applied_render("total: tuple = (alpha, beta, gamma)\n", "seed-164");
    assert_eq!(render, "total: tuple = (alpha, beta)");
}

#[test]
fn test_seed_164_removes_the_last_element_of_every_annotated_assignment_value() {
    let source = "first: tuple = (alpha, beta)\nsecond: tuple = (one, two)\n";
    let (_, render) = applied_render(source, "seed-164");
    assert_eq!(render, "first: tuple = (alpha,)\nsecond: tuple = (one,)");
}

#[test]
fn test_seed_164_leaves_a_single_element_tuple_value_standing() {
    let (planted, render) = applied_render("total: tuple = (alpha,)\n", "seed-164");
    assert!(
        !planted,
        "an annotated assignment carrying a one element tuple value does not plant"
    );
    assert_eq!(render, "total: tuple = (alpha,)");
}

#[test]
fn test_seed_164_leaves_a_plain_assignment_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-164");
    assert!(
        !planted,
        "a tuple a plain assignment hands over is not an annotated assignment value"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_164_verdict_is_caught() {
    let target = target_with(LAST_VALUE_MODULE, LAST_VALUE_GATE, "caught164");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-164"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-165, dropped first annotated assignment value element.

const FIRST_VALUE_MODULE: &str = "total: tuple = (alpha, beta)\nkeep = gamma\n";
const FIRST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_165_plants_on_an_annotated_assignment_carrying_a_two_element_tuple_value() {
    let (planted, _) = applied_render(FIRST_VALUE_MODULE, "seed-165");
    assert!(
        planted,
        "an annotated assignment carrying a two element tuple value plants"
    );
}

#[test]
fn test_seed_165_removes_the_first_annotated_assignment_value_element() {
    let (_, render) = applied_render(FIRST_VALUE_MODULE, "seed-165");
    assert_eq!(render, "total: tuple = (beta,)\nkeep = gamma");
}

#[test]
fn test_seed_165_keeps_the_later_value_elements_standing() {
    let (_, render) = applied_render("total: tuple = (alpha, beta, gamma)\n", "seed-165");
    assert_eq!(render, "total: tuple = (beta, gamma)");
}

#[test]
fn test_seed_165_removes_the_first_element_of_every_annotated_assignment_value() {
    let source = "first: tuple = (alpha, beta)\nsecond: tuple = (one, two)\n";
    let (_, render) = applied_render(source, "seed-165");
    assert_eq!(render, "first: tuple = (beta,)\nsecond: tuple = (two,)");
}

#[test]
fn test_seed_165_leaves_a_single_element_tuple_value_standing() {
    let (planted, render) = applied_render("total: tuple = (alpha,)\n", "seed-165");
    assert!(
        !planted,
        "an annotated assignment carrying a one element tuple value does not plant"
    );
    assert_eq!(render, "total: tuple = (alpha,)");
}

#[test]
fn test_seed_165_leaves_a_plain_assignment_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-165");
    assert!(
        !planted,
        "a tuple a plain assignment hands over is not an annotated assignment value"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_165_verdict_is_caught() {
    let target = target_with(FIRST_VALUE_MODULE, FIRST_VALUE_GATE, "caught165");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-165"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-166, dropped last comprehension tuple element.

const LAST_ELEMENT_MODULE: &str = "pairs = [(alpha, beta) for item in items]\nkeep = gamma\n";
const LAST_ELEMENT_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_166_plants_on_a_comprehension_carrying_a_two_element_tuple_element() {
    let (planted, _) = applied_render(LAST_ELEMENT_MODULE, "seed-166");
    assert!(
        planted,
        "a comprehension carrying a two element tuple element plants"
    );
}

#[test]
fn test_seed_166_removes_the_last_comprehension_tuple_element() {
    let (_, render) = applied_render(LAST_ELEMENT_MODULE, "seed-166");
    assert_eq!(render, "pairs = [(alpha,) for item in items]\nkeep = gamma");
}

#[test]
fn test_seed_166_keeps_the_earlier_tuple_elements_standing() {
    let source = "pairs = [(alpha, beta, gamma) for item in items]\n";
    let (_, render) = applied_render(source, "seed-166");
    assert_eq!(render, "pairs = [(alpha, beta) for item in items]");
}

#[test]
fn test_seed_166_removes_the_last_element_of_every_comprehension_tuple() {
    let source =
        "pairs = [(alpha, beta) for item in items]\nnames = [(one, two) for part in parts]\n";
    let (_, render) = applied_render(source, "seed-166");
    assert_eq!(
        render,
        "pairs = [(alpha,) for item in items]\nnames = [(one,) for part in parts]"
    );
}

#[test]
fn test_seed_166_leaves_a_single_element_comprehension_tuple_standing() {
    let source = "pairs = [(alpha,) for item in items]\n";
    let (planted, render) = applied_render(source, "seed-166");
    assert!(
        !planted,
        "a comprehension carrying a one element tuple element does not plant"
    );
    assert_eq!(render, "pairs = [(alpha,) for item in items]");
}

#[test]
fn test_seed_166_leaves_a_dictionary_comprehension_tuple_standing() {
    let source = "table = {(alpha, beta): item for item in items}\n";
    let (planted, render) = applied_render(source, "seed-166");
    assert!(
        !planted,
        "a tuple standing as a dictionary comprehension key is not an element expression"
    );
    assert_eq!(render, "table = {(alpha, beta): item for item in items}");
}

#[test]
fn test_seed_166_verdict_is_caught() {
    let target = target_with(LAST_ELEMENT_MODULE, LAST_ELEMENT_GATE, "caught166");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-166"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-167, dropped first comprehension tuple element.

const FIRST_ELEMENT_MODULE: &str = "pairs = [(alpha, beta) for item in items]\nkeep = gamma\n";
const FIRST_ELEMENT_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_167_plants_on_a_comprehension_carrying_a_two_element_tuple_element() {
    let (planted, _) = applied_render(FIRST_ELEMENT_MODULE, "seed-167");
    assert!(
        planted,
        "a comprehension carrying a two element tuple element plants"
    );
}

#[test]
fn test_seed_167_removes_the_first_comprehension_tuple_element() {
    let (_, render) = applied_render(FIRST_ELEMENT_MODULE, "seed-167");
    assert_eq!(render, "pairs = [(beta,) for item in items]\nkeep = gamma");
}

#[test]
fn test_seed_167_keeps_the_later_tuple_elements_standing() {
    let source = "pairs = [(alpha, beta, gamma) for item in items]\n";
    let (_, render) = applied_render(source, "seed-167");
    assert_eq!(render, "pairs = [(beta, gamma) for item in items]");
}

#[test]
fn test_seed_167_leaves_a_single_element_comprehension_tuple_standing() {
    let source = "pairs = [(alpha,) for item in items]\n";
    let (planted, render) = applied_render(source, "seed-167");
    assert!(
        !planted,
        "a comprehension carrying a one element tuple element does not plant"
    );
    assert_eq!(render, "pairs = [(alpha,) for item in items]");
}

#[test]
fn test_seed_167_leaves_a_comprehension_over_a_name_standing() {
    let source = "pairs = [pair for pair in items]\n";
    let (planted, render) = applied_render(source, "seed-167");
    assert!(
        !planted,
        "a comprehension whose element is a name does not plant"
    );
    assert_eq!(render, "pairs = [pair for pair in items]");
}

#[test]
fn test_seed_167_verdict_is_caught() {
    let target = target_with(FIRST_ELEMENT_MODULE, FIRST_ELEMENT_GATE, "caught167");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-167"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the fortieth format.

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
fn test_version_thirty_nine_pack_still_loads_the_one_hundred_and_sixty_four() {
    let dir = fresh_dir("pack39");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(164).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/39", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty nine pack loads");
    assert_eq!(loaded.len(), 164);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[167].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-168".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[167] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/40", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_nine_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(163).collect();
    entries.push(standing_entries()[164].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/39", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(164).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/40", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and sixty four entries under the version forty format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 204);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[203], "seed-203");
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
        written.contains("\"canaryfield-seeds/49\""),
        "the written pack carries the fortieth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
