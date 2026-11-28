//! Feature slice 18 pins, the registry grown a twenty sixth time.
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
        "canaryfield-feature18-{}-{tag}",
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
fn test_registry_has_two_hundred_and_twenty_four_unique_seeds() {
    assert_eq!(SEEDS.len(), 224, "the grown registry holds two hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 224, "the ids stand unique");
    assert_eq!(SEEDS[112].id, "seed-112");
    assert_eq!(SEEDS[115].id, "seed-115");
}

// Seed-112, dropped slice step.

const SLICE_STEP_MODULE: &str = "value = letters[0:8:2]\n";
const SLICE_STEP_GATE: &str = "#!/bin/sh\ngrep -q '0:8:2' mod.py\n";

#[test]
fn test_seed_112_plants_on_a_slice_carrying_a_step() {
    let (planted, _) = applied_render(SLICE_STEP_MODULE, "seed-112");
    assert!(planted, "a slice carrying a step plants");
}

#[test]
fn test_seed_112_removes_the_step() {
    let (_, render) = applied_render(SLICE_STEP_MODULE, "seed-112");
    assert_eq!(render, "value = letters[0:8]");
}

#[test]
fn test_seed_112_keeps_the_bounds_standing() {
    let (_, render) = applied_render(SLICE_STEP_MODULE, "seed-112");
    assert!(
        render.contains("letters[0:8]"),
        "the bounds stand as written: {render}"
    );
}

#[test]
fn test_seed_112_removes_every_step_in_an_expression() {
    let (_, render) = applied_render(
        "value = letters[0:8:2]\nother = letters[9:3:-1]\n",
        "seed-112",
    );
    assert_eq!(render, "value = letters[0:8]\nother = letters[9:3]");
}

#[test]
fn test_seed_112_leaves_a_slice_without_a_step_standing() {
    let (planted, render) = applied_render("value = letters[1:4]\n", "seed-112");
    assert!(!planted, "a slice without a step does not plant");
    assert_eq!(render, "value = letters[1:4]");
}

#[test]
fn test_seed_112_verdict_is_caught() {
    let target = target_with(SLICE_STEP_MODULE, SLICE_STEP_GATE, "caught112");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-112"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-113, dropped annotation value.

const ANNOTATION_VALUE_MODULE: &str = "total: int = 0\n";
const ANNOTATION_VALUE_GATE: &str = "#!/bin/sh\ngrep -q '= 0' mod.py\n";

#[test]
fn test_seed_113_plants_on_an_annotated_assignment_carrying_a_value() {
    let (planted, _) = applied_render(ANNOTATION_VALUE_MODULE, "seed-113");
    assert!(planted, "an annotated assignment carrying a value plants");
}

#[test]
fn test_seed_113_removes_the_value() {
    let (_, render) = applied_render(ANNOTATION_VALUE_MODULE, "seed-113");
    assert_eq!(render, "total: int");
}

#[test]
fn test_seed_113_keeps_the_annotation_standing() {
    let (_, render) = applied_render(ANNOTATION_VALUE_MODULE, "seed-113");
    assert!(
        render.contains("total: int"),
        "the annotation stands as written: {render}"
    );
}

#[test]
fn test_seed_113_removes_every_annotation_value_in_a_body() {
    let (_, render) = applied_render("first: int = 1\nsecond: str = 'x'\n", "seed-113");
    assert_eq!(render, "first: int\nsecond: str");
}

#[test]
fn test_seed_113_leaves_a_bare_annotation_standing() {
    let (planted, render) = applied_render("total: int\n", "seed-113");
    assert!(!planted, "a bare annotation does not plant");
    assert_eq!(render, "total: int");
}

#[test]
fn test_seed_113_verdict_is_caught() {
    let target = target_with(ANNOTATION_VALUE_MODULE, ANNOTATION_VALUE_GATE, "caught113");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-113"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-114, dropped formatted value.

const FORMATTED_VALUE_MODULE: &str = "label = f'{name} is {age} years old'\n";
const FORMATTED_VALUE_GATE: &str = "#!/bin/sh\ngrep -q '{age}' mod.py\n";

#[test]
fn test_seed_114_plants_on_a_string_carrying_two_formatted_values() {
    let (planted, _) = applied_render(FORMATTED_VALUE_MODULE, "seed-114");
    assert!(planted, "a string carrying two formatted values plants");
}

#[test]
fn test_seed_114_removes_the_last_formatted_value() {
    let (_, render) = applied_render(FORMATTED_VALUE_MODULE, "seed-114");
    assert_eq!(render, "label = f'{name} is  years old'");
}

#[test]
fn test_seed_114_keeps_the_remaining_values_standing() {
    let (_, render) = applied_render(FORMATTED_VALUE_MODULE, "seed-114");
    assert!(
        render.contains("{name}"),
        "the remaining value stands as written: {render}"
    );
}

#[test]
fn test_seed_114_removes_the_last_value_of_every_string() {
    let (_, render) = applied_render("a = f'{x} and {y}'\nb = f'{p} or {q}'\n", "seed-114");
    assert_eq!(render, "a = f'{x} and '\nb = f'{p} or '");
}

#[test]
fn test_seed_114_leaves_a_single_value_string_standing() {
    let (planted, render) = applied_render("label = f'only {name}'\n", "seed-114");
    assert!(
        !planted,
        "a string carrying one formatted value does not plant"
    );
    assert_eq!(render, "label = f'only {name}'");
}

#[test]
fn test_seed_114_verdict_is_caught() {
    let target = target_with(FORMATTED_VALUE_MODULE, FORMATTED_VALUE_GATE, "caught114");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-114"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-115, dropped raised error message.

const RAISED_MESSAGE_MODULE: &str = "raise ValueError('the count is short')\n";
const RAISED_MESSAGE_GATE: &str = "#!/bin/sh\ngrep -q 'the count is short' mod.py\n";

#[test]
fn test_seed_115_plants_on_a_raise_carrying_an_argument() {
    let (planted, _) = applied_render(RAISED_MESSAGE_MODULE, "seed-115");
    assert!(
        planted,
        "a raise whose exception call carries an argument plants"
    );
}

#[test]
fn test_seed_115_removes_the_error_message() {
    let (_, render) = applied_render(RAISED_MESSAGE_MODULE, "seed-115");
    assert_eq!(render, "raise ValueError()");
}

#[test]
fn test_seed_115_removes_every_argument_of_the_call() {
    let (_, render) = applied_render("raise ValueError('short', code)\n", "seed-115");
    assert_eq!(render, "raise ValueError()");
}

#[test]
fn test_seed_115_keeps_the_exception_type_standing() {
    let (_, render) = applied_render(RAISED_MESSAGE_MODULE, "seed-115");
    assert!(
        render.contains("ValueError"),
        "the exception type stands as written: {render}"
    );
}

#[test]
fn test_seed_115_leaves_a_bare_raise_standing() {
    let (planted, render) = applied_render("raise ValueError\n", "seed-115");
    assert!(!planted, "a raise of a bare exception type does not plant");
    assert_eq!(render, "raise ValueError");
}

#[test]
fn test_seed_115_leaves_a_raise_of_a_plain_name_standing() {
    let (planted, render) = applied_render("raise problem\n", "seed-115");
    assert!(!planted, "a raise of a plain name does not plant");
    assert_eq!(render, "raise problem");
}

#[test]
fn test_seed_115_verdict_is_caught() {
    let target = target_with(RAISED_MESSAGE_MODULE, RAISED_MESSAGE_GATE, "caught115");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-115"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F18-6.

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
fn test_version_twenty_six_pack_still_loads_the_one_hundred_and_twelve() {
    let dir = fresh_dir("pack26");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(112).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/26", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty six pack loads");
    assert_eq!(loaded.len(), 112);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_seven_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(116).collect();
    let last = entries[115].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-124".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[115] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/27", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_six_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(111).collect();
    entries.push(standing_entries()[112].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/26", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty six pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_seven_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(112).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/27", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and twelve entries under the version twenty seven format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 224);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("an entry carries its id: {other:?}"),
        })
        .collect();
    let standing: Vec<String> = SEEDS.iter().map(|seed| seed.id.to_string()).collect();
    assert_eq!(ids, standing, "the round trip holds the registry in order");
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
        written.contains("\"canaryfield-seeds/54\""),
        "the written pack carries the twenty eighth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
