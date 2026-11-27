//! Feature slice 33 pins, the registry grown a forty first time.
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
        "canaryfield-feature33-{}-{tag}",
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
fn test_registry_has_one_hundred_and_eighty_four_unique_seeds() {
    assert_eq!(SEEDS.len(), 184, "the registry stands at 176 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 184, "every seed id stands unique");
    assert_eq!(SEEDS[183].id, "seed-183", "the last seed stands at 179");
}

// Seed-172, dropped last parameter default tuple element.

const LAST_DEFAULT_MODULE: &str =
    "def pick(scale=(alpha, beta)):\n    return scale\nkeep = gamma\n";
const LAST_DEFAULT_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_172_plants_on_a_default_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_DEFAULT_MODULE, "seed-172");
    assert!(planted, "a default that is a two element tuple plants");
}

#[test]
fn test_seed_172_removes_the_last_parameter_default_tuple_element() {
    let (_, render) = applied_render(LAST_DEFAULT_MODULE, "seed-172");
    assert_eq!(
        render,
        "def pick(scale=(alpha,)):\n    return scale\nkeep = gamma"
    );
}

#[test]
fn test_seed_172_keeps_the_earlier_default_elements_standing() {
    let source = "def pick(scale=(alpha, beta, gamma)):\n    return scale\n";
    let (_, render) = applied_render(source, "seed-172");
    assert_eq!(render, "def pick(scale=(alpha, beta)):\n    return scale");
}

#[test]
fn test_seed_172_removes_the_last_element_of_every_tuple_default() {
    let source = "def pick(scale=(alpha, beta), shift=(one, two)):\n    return scale\n";
    let (_, render) = applied_render(source, "seed-172");
    assert_eq!(
        render,
        "def pick(scale=(alpha,), shift=(one,)):\n    return scale"
    );
}

#[test]
fn test_seed_172_leaves_a_single_element_tuple_default_standing() {
    let source = "def pick(scale=(alpha,)):\n    return scale\n";
    let (planted, render) = applied_render(source, "seed-172");
    assert!(!planted, "a one element tuple default does not plant");
    assert_eq!(render, "def pick(scale=(alpha,)):\n    return scale");
}

#[test]
fn test_seed_172_leaves_a_tuple_standing_outside_a_default() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-172");
    assert!(
        !planted,
        "a tuple standing as a plain value is not a default"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_172_verdict_is_caught() {
    let target = target_with(LAST_DEFAULT_MODULE, LAST_DEFAULT_GATE, "caught172");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-172"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-173, dropped first parameter default tuple element.

const FIRST_DEFAULT_MODULE: &str =
    "def pick(scale=(alpha, beta)):\n    return scale\nkeep = gamma\n";
const FIRST_DEFAULT_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_173_plants_on_a_default_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_DEFAULT_MODULE, "seed-173");
    assert!(planted, "a default that is a two element tuple plants");
}

#[test]
fn test_seed_173_removes_the_first_parameter_default_tuple_element() {
    let (_, render) = applied_render(FIRST_DEFAULT_MODULE, "seed-173");
    assert_eq!(
        render,
        "def pick(scale=(beta,)):\n    return scale\nkeep = gamma"
    );
}

#[test]
fn test_seed_173_keeps_the_later_default_elements_standing() {
    let source = "def pick(scale=(alpha, beta, gamma)):\n    return scale\n";
    let (_, render) = applied_render(source, "seed-173");
    assert_eq!(render, "def pick(scale=(beta, gamma)):\n    return scale");
}

#[test]
fn test_seed_173_removes_the_first_element_of_every_tuple_default() {
    let source = "def pick(scale=(alpha, beta), shift=(one, two)):\n    return scale\n";
    let (_, render) = applied_render(source, "seed-173");
    assert_eq!(
        render,
        "def pick(scale=(beta,), shift=(two,)):\n    return scale"
    );
}

#[test]
fn test_seed_173_leaves_a_single_element_tuple_default_standing() {
    let source = "def pick(scale=(alpha,)):\n    return scale\n";
    let (planted, render) = applied_render(source, "seed-173");
    assert!(!planted, "a one element tuple default does not plant");
    assert_eq!(render, "def pick(scale=(alpha,)):\n    return scale");
}

#[test]
fn test_seed_173_leaves_a_tuple_standing_outside_a_default() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-173");
    assert!(
        !planted,
        "a tuple standing as a plain value is not a default"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_173_verdict_is_caught() {
    let target = target_with(FIRST_DEFAULT_MODULE, FIRST_DEFAULT_GATE, "caught173");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-173"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-174, dropped last conditional expression tuple element.

const LAST_ARM_MODULE: &str = "pick = (alpha, beta) if flag else fallback\nkeep = gamma\n";
const LAST_ARM_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_174_plants_on_a_conditional_expression_carrying_a_tuple_arm() {
    let (planted, _) = applied_render(LAST_ARM_MODULE, "seed-174");
    assert!(
        planted,
        "a conditional expression carrying a two element tuple arm plants"
    );
}

#[test]
fn test_seed_174_removes_the_last_conditional_expression_tuple_element() {
    let (_, render) = applied_render(LAST_ARM_MODULE, "seed-174");
    assert_eq!(
        render,
        "pick = (alpha,) if flag else fallback\nkeep = gamma"
    );
}

#[test]
fn test_seed_174_keeps_the_earlier_arm_elements_standing() {
    let source = "pick = (alpha, beta, gamma) if flag else fallback\n";
    let (_, render) = applied_render(source, "seed-174");
    assert_eq!(render, "pick = (alpha, beta) if flag else fallback");
}

#[test]
fn test_seed_174_removes_the_last_element_of_every_arm_tuple() {
    let source = "pick = (alpha, beta) if flag else (one, two)\n";
    let (_, render) = applied_render(source, "seed-174");
    assert_eq!(render, "pick = (alpha,) if flag else (one,)");
}

#[test]
fn test_seed_174_leaves_a_single_element_arm_tuple_standing() {
    let source = "pick = (alpha,) if flag else fallback\n";
    let (planted, render) = applied_render(source, "seed-174");
    assert!(!planted, "a one element tuple arm does not plant");
    assert_eq!(render, "pick = (alpha,) if flag else fallback");
}

#[test]
fn test_seed_174_leaves_a_tuple_standing_as_the_test() {
    let source = "pick = value if (alpha, beta) else fallback\n";
    let (planted, render) = applied_render(source, "seed-174");
    assert!(!planted, "a tuple standing as the test is not an arm");
    assert_eq!(render, "pick = value if (alpha, beta) else fallback");
}

#[test]
fn test_seed_174_verdict_is_caught() {
    let target = target_with(LAST_ARM_MODULE, LAST_ARM_GATE, "caught174");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-174"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-175, dropped first conditional expression tuple element.

const FIRST_ARM_MODULE: &str = "pick = (alpha, beta) if flag else fallback\nkeep = gamma\n";
const FIRST_ARM_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_175_plants_on_a_conditional_expression_carrying_a_tuple_arm() {
    let (planted, _) = applied_render(FIRST_ARM_MODULE, "seed-175");
    assert!(
        planted,
        "a conditional expression carrying a two element tuple arm plants"
    );
}

#[test]
fn test_seed_175_removes_the_first_conditional_expression_tuple_element() {
    let (_, render) = applied_render(FIRST_ARM_MODULE, "seed-175");
    assert_eq!(render, "pick = (beta,) if flag else fallback\nkeep = gamma");
}

#[test]
fn test_seed_175_keeps_the_later_arm_elements_standing() {
    let source = "pick = (alpha, beta, gamma) if flag else fallback\n";
    let (_, render) = applied_render(source, "seed-175");
    assert_eq!(render, "pick = (beta, gamma) if flag else fallback");
}

#[test]
fn test_seed_175_leaves_a_single_element_arm_tuple_standing() {
    let source = "pick = (alpha,) if flag else fallback\n";
    let (planted, render) = applied_render(source, "seed-175");
    assert!(!planted, "a one element tuple arm does not plant");
    assert_eq!(render, "pick = (alpha,) if flag else fallback");
}

#[test]
fn test_seed_175_leaves_a_tuple_standing_as_the_test() {
    let source = "pick = value if (alpha, beta) else fallback\n";
    let (planted, render) = applied_render(source, "seed-175");
    assert!(!planted, "a tuple standing as the test is not an arm");
    assert_eq!(render, "pick = value if (alpha, beta) else fallback");
}

#[test]
fn test_seed_175_verdict_is_caught() {
    let target = target_with(FIRST_ARM_MODULE, FIRST_ARM_GATE, "caught175");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-175"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty second format.

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
fn test_version_forty_one_pack_still_loads_the_one_hundred_and_seventy_two() {
    let dir = fresh_dir("pack41");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(172).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/41", entries));
    let loaded = load_seed_pack(&path).expect("the version forty one pack loads");
    assert_eq!(loaded.len(), 172);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_two_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[175].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-176".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[175] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/42", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_one_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(171).collect();
    entries.push(standing_entries()[172].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/41", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_two_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(172).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/42", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and seventy two entries under the version forty two format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 184);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[183], "seed-183");
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
        written.contains("\"canaryfield-seeds/44\""),
        "the written pack carries the forty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
