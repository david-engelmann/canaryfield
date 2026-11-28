//! Feature slice 32 pins, the registry grown a fortieth time.
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
        "canaryfield-feature32-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 224, "the registry stands at 172 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 224, "every seed id stands unique");
    assert_eq!(SEEDS[223].id, "seed-223", "the last seed stands at 179");
}

// Seed-168, dropped last lambda body tuple element.

const LAST_BODY_MODULE: &str = "pick = lambda: (alpha, beta)\nkeep = gamma\n";
const LAST_BODY_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_168_plants_on_a_lambda_whose_body_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_BODY_MODULE, "seed-168");
    assert!(planted, "a lambda whose body is a two element tuple plants");
}

#[test]
fn test_seed_168_removes_the_last_lambda_body_tuple_element() {
    let (_, render) = applied_render(LAST_BODY_MODULE, "seed-168");
    assert_eq!(render, "pick = lambda: (alpha,)\nkeep = gamma");
}

#[test]
fn test_seed_168_keeps_the_earlier_body_elements_standing() {
    let (_, render) = applied_render("pick = lambda: (alpha, beta, gamma)\n", "seed-168");
    assert_eq!(render, "pick = lambda: (alpha, beta)");
}

#[test]
fn test_seed_168_removes_the_last_element_of_every_lambda_body() {
    let source = "pick = lambda: (alpha, beta)\nchoose = lambda: (one, two)\n";
    let (_, render) = applied_render(source, "seed-168");
    assert_eq!(render, "pick = lambda: (alpha,)\nchoose = lambda: (one,)");
}

#[test]
fn test_seed_168_leaves_a_single_element_lambda_body_standing() {
    let (planted, render) = applied_render("pick = lambda: (alpha,)\n", "seed-168");
    assert!(
        !planted,
        "a lambda whose body is a one element tuple does not plant"
    );
    assert_eq!(render, "pick = lambda: (alpha,)");
}

#[test]
fn test_seed_168_leaves_a_function_return_tuple_standing() {
    let source = "def pick():\n    return (alpha, beta)\n";
    let (planted, render) = applied_render(source, "seed-168");
    assert!(!planted, "a tuple a function returns is not a lambda body");
    assert_eq!(render, "def pick():\n    return (alpha, beta)");
}

#[test]
fn test_seed_168_verdict_is_caught() {
    let target = target_with(LAST_BODY_MODULE, LAST_BODY_GATE, "caught168");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-168"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-169, dropped first lambda body tuple element.

const FIRST_BODY_MODULE: &str = "pick = lambda: (alpha, beta)\nkeep = gamma\n";
const FIRST_BODY_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_169_plants_on_a_lambda_whose_body_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_BODY_MODULE, "seed-169");
    assert!(planted, "a lambda whose body is a two element tuple plants");
}

#[test]
fn test_seed_169_removes_the_first_lambda_body_tuple_element() {
    let (_, render) = applied_render(FIRST_BODY_MODULE, "seed-169");
    assert_eq!(render, "pick = lambda: (beta,)\nkeep = gamma");
}

#[test]
fn test_seed_169_keeps_the_later_body_elements_standing() {
    let (_, render) = applied_render("pick = lambda: (alpha, beta, gamma)\n", "seed-169");
    assert_eq!(render, "pick = lambda: (beta, gamma)");
}

#[test]
fn test_seed_169_removes_the_first_element_of_every_lambda_body() {
    let source = "pick = lambda: (alpha, beta)\nchoose = lambda: (one, two)\n";
    let (_, render) = applied_render(source, "seed-169");
    assert_eq!(render, "pick = lambda: (beta,)\nchoose = lambda: (two,)");
}

#[test]
fn test_seed_169_leaves_a_single_element_lambda_body_standing() {
    let (planted, render) = applied_render("pick = lambda: (alpha,)\n", "seed-169");
    assert!(
        !planted,
        "a lambda whose body is a one element tuple does not plant"
    );
    assert_eq!(render, "pick = lambda: (alpha,)");
}

#[test]
fn test_seed_169_leaves_a_function_return_tuple_standing() {
    let source = "def pick():\n    return (alpha, beta)\n";
    let (planted, render) = applied_render(source, "seed-169");
    assert!(!planted, "a tuple a function returns is not a lambda body");
    assert_eq!(render, "def pick():\n    return (alpha, beta)");
}

#[test]
fn test_seed_169_verdict_is_caught() {
    let target = target_with(FIRST_BODY_MODULE, FIRST_BODY_GATE, "caught169");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-169"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-170, dropped last comparison operand tuple element.

const LAST_OPERAND_MODULE: &str = "check = (alpha, beta) == limit\nkeep = gamma\n";
const LAST_OPERAND_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_170_plants_on_a_comparison_carrying_a_two_element_tuple_operand() {
    let (planted, _) = applied_render(LAST_OPERAND_MODULE, "seed-170");
    assert!(
        planted,
        "a comparison carrying a two element tuple operand plants"
    );
}

#[test]
fn test_seed_170_removes_the_last_comparison_operand_tuple_element() {
    let (_, render) = applied_render(LAST_OPERAND_MODULE, "seed-170");
    assert_eq!(render, "check = (alpha,) == limit\nkeep = gamma");
}

#[test]
fn test_seed_170_keeps_the_earlier_operand_elements_standing() {
    let (_, render) = applied_render("check = (alpha, beta, gamma) == limit\n", "seed-170");
    assert_eq!(render, "check = (alpha, beta) == limit");
}

#[test]
fn test_seed_170_removes_the_last_element_of_every_operand_tuple() {
    let source = "check = (alpha, beta) == (one, two)\n";
    let (_, render) = applied_render(source, "seed-170");
    assert_eq!(render, "check = (alpha,) == (one,)");
}

#[test]
fn test_seed_170_leaves_a_single_element_operand_tuple_standing() {
    let (planted, render) = applied_render("check = (alpha,) == limit\n", "seed-170");
    assert!(
        !planted,
        "a comparison carrying a one element tuple operand does not plant"
    );
    assert_eq!(render, "check = (alpha,) == limit");
}

#[test]
fn test_seed_170_leaves_a_tuple_outside_a_comparison_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-170");
    assert!(
        !planted,
        "a tuple standing outside a comparison is not an operand"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_170_verdict_is_caught() {
    let target = target_with(LAST_OPERAND_MODULE, LAST_OPERAND_GATE, "caught170");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-170"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-171, dropped first comparison operand tuple element.

const FIRST_OPERAND_MODULE: &str = "check = (alpha, beta) == limit\nkeep = gamma\n";
const FIRST_OPERAND_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_171_plants_on_a_comparison_carrying_a_two_element_tuple_operand() {
    let (planted, _) = applied_render(FIRST_OPERAND_MODULE, "seed-171");
    assert!(
        planted,
        "a comparison carrying a two element tuple operand plants"
    );
}

#[test]
fn test_seed_171_removes_the_first_comparison_operand_tuple_element() {
    let (_, render) = applied_render(FIRST_OPERAND_MODULE, "seed-171");
    assert_eq!(render, "check = (beta,) == limit\nkeep = gamma");
}

#[test]
fn test_seed_171_keeps_the_later_operand_elements_standing() {
    let (_, render) = applied_render("check = (alpha, beta, gamma) == limit\n", "seed-171");
    assert_eq!(render, "check = (beta, gamma) == limit");
}

#[test]
fn test_seed_171_leaves_a_single_element_operand_tuple_standing() {
    let (planted, render) = applied_render("check = (alpha,) == limit\n", "seed-171");
    assert!(
        !planted,
        "a comparison carrying a one element tuple operand does not plant"
    );
    assert_eq!(render, "check = (alpha,) == limit");
}

#[test]
fn test_seed_171_leaves_a_tuple_outside_a_comparison_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-171");
    assert!(
        !planted,
        "a tuple standing outside a comparison is not an operand"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_171_verdict_is_caught() {
    let target = target_with(FIRST_OPERAND_MODULE, FIRST_OPERAND_GATE, "caught171");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-171"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty first format.

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
fn test_version_forty_pack_still_loads_the_one_hundred_and_sixty_eight() {
    let dir = fresh_dir("pack40");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(168).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/40", entries));
    let loaded = load_seed_pack(&path).expect("the version forty pack loads");
    assert_eq!(loaded.len(), 168);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_one_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[171].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-172".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[171] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/41", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(167).collect();
    entries.push(standing_entries()[168].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/40", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_one_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(168).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/41", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and sixty eight entries under the version forty one format are refused"
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
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[223], "seed-223");
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
        "the written pack carries the forty first format"
    );
    let _ = fs::remove_dir_all(&dir);
}
