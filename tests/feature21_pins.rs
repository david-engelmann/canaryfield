//! Feature slice 21 pins, the registry grown a twenty ninth time.
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
        "canaryfield-feature21-{}-{tag}",
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
fn test_registry_has_one_hundred_and_forty_four_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        144,
        "the grown registry holds one hundred and forty four"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 144, "the ids stand unique");
    assert_eq!(SEEDS[124].id, "seed-124");
    assert_eq!(SEEDS[127].id, "seed-127");
}

// Seed-124, dropped tuple element.

const TUPLE_MODULE: &str = "value = (alpha, beta)\n";
const TUPLE_GATE: &str = "#!/bin/sh\ngrep -q '(alpha, beta)' mod.py\n";

#[test]
fn test_seed_124_plants_on_a_tuple_display() {
    let (planted, _) = applied_render(TUPLE_MODULE, "seed-124");
    assert!(planted, "a tuple display carrying two elements plants");
}

#[test]
fn test_seed_124_removes_the_first_element() {
    let (_, render) = applied_render("value = (alpha, beta, gamma)\n", "seed-124");
    assert_eq!(render, "value = (beta, gamma)");
}

#[test]
fn test_seed_124_keeps_the_remaining_elements_standing() {
    let (_, render) = applied_render("value = (prefix + alpha, beta)\n", "seed-124");
    assert_eq!(render, "value = (beta,)");
}

#[test]
fn test_seed_124_removes_the_first_element_of_every_tuple_display() {
    let (_, render) = applied_render("value = (alpha, beta) + (gamma, delta)\n", "seed-124");
    assert_eq!(render, "value = (beta,) + (delta,)");
}

#[test]
fn test_seed_124_leaves_a_one_element_tuple_standing() {
    let (planted, render) = applied_render("value = (alpha,)\n", "seed-124");
    assert!(!planted, "a tuple carrying a single element does not plant");
    assert_eq!(render, "value = (alpha,)");
}

#[test]
fn test_seed_124_leaves_an_unpacking_target_standing() {
    let (planted, render) = applied_render("alpha, beta = pair\n", "seed-124");
    assert!(!planted, "an unpacking target is no tuple display");
    assert_eq!(render, "alpha, beta = pair");
}

#[test]
fn test_seed_124_verdict_is_caught() {
    let target = target_with(TUPLE_MODULE, TUPLE_GATE, "caught124");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-124"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-125, dropped conditional test.

const CONDITIONAL_MODULE: &str = "value = alpha if ready else beta\n";
const CONDITIONAL_GATE: &str = "#!/bin/sh\ngrep -q 'alpha if ready else beta' mod.py\n";

#[test]
fn test_seed_125_plants_on_a_conditional_expression() {
    let (planted, _) = applied_render(CONDITIONAL_MODULE, "seed-125");
    assert!(planted, "a conditional expression plants");
}

#[test]
fn test_seed_125_removes_the_test() {
    let (_, render) = applied_render(CONDITIONAL_MODULE, "seed-125");
    assert_eq!(render, "value = alpha");
}

#[test]
fn test_seed_125_leaves_the_guarded_value_standing() {
    let (_, render) = applied_render("value = combine(alpha) if ready else beta\n", "seed-125");
    assert_eq!(render, "value = combine(alpha)");
}

#[test]
fn test_seed_125_removes_the_test_of_every_conditional_expression() {
    let (_, render) = applied_render(
        "value = (alpha if ready else beta) + (gamma if set else delta)\n",
        "seed-125",
    );
    assert_eq!(render, "value = alpha + gamma");
}

#[test]
fn test_seed_125_leaves_a_plain_name_standing() {
    let (planted, render) = applied_render("value = alpha\n", "seed-125");
    assert!(!planted, "a plain name does not plant");
    assert_eq!(render, "value = alpha");
}

#[test]
fn test_seed_125_verdict_is_caught() {
    let target = target_with(CONDITIONAL_MODULE, CONDITIONAL_GATE, "caught125");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-125"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-126, dropped comparison head.

const COMPARISON_MODULE: &str = "value = alpha < beta < gamma\n";
const COMPARISON_GATE: &str = "#!/bin/sh\ngrep -q 'alpha < beta < gamma' mod.py\n";

#[test]
fn test_seed_126_plants_on_a_chained_comparison() {
    let (planted, _) = applied_render(COMPARISON_MODULE, "seed-126");
    assert!(planted, "a comparison carrying two operators plants");
}

#[test]
fn test_seed_126_removes_the_head() {
    let (_, render) = applied_render(COMPARISON_MODULE, "seed-126");
    assert_eq!(render, "value = beta < gamma");
}

#[test]
fn test_seed_126_keeps_the_remaining_comparison_standing() {
    let (_, render) = applied_render("value = alpha < beta <= gamma < delta\n", "seed-126");
    assert_eq!(render, "value = beta <= gamma < delta");
}

#[test]
fn test_seed_126_removes_the_head_of_every_chained_comparison() {
    let (_, render) = applied_render(
        "value = (alpha < beta < gamma) and (delta < epsilon < zeta)\n",
        "seed-126",
    );
    assert_eq!(render, "value = beta < gamma and epsilon < zeta");
}

#[test]
fn test_seed_126_leaves_a_single_comparison_standing() {
    let (planted, render) = applied_render("value = alpha < beta\n", "seed-126");
    assert!(
        !planted,
        "a comparison carrying one operator does not plant"
    );
    assert_eq!(render, "value = alpha < beta");
}

#[test]
fn test_seed_126_verdict_is_caught() {
    let target = target_with(COMPARISON_MODULE, COMPARISON_GATE, "caught126");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-126"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-127, dropped assignment.

const ASSIGNMENT_MODULE: &str = "value = alpha\nkeep = beta\n";
const ASSIGNMENT_GATE: &str = "#!/bin/sh\ngrep -q 'value = alpha' mod.py\n";

#[test]
fn test_seed_127_plants_on_an_assignment() {
    let (planted, _) = applied_render(
        "def f():\n    value = alpha\n    return value\n",
        "seed-127",
    );
    assert!(planted, "an assignment plants");
}

#[test]
fn test_seed_127_removes_the_assignment() {
    let (_, render) = applied_render(
        "def f():\n    value = alpha\n    return value\n",
        "seed-127",
    );
    assert_eq!(render, "def f():\n    return value");
}

#[test]
fn test_seed_127_keeps_the_neighbouring_statements_standing() {
    let (_, render) = applied_render(
        "def f():\n    value = alpha\n    print(value)\n    return value\n",
        "seed-127",
    );
    assert_eq!(render, "def f():\n    print(value)\n    return value");
}

#[test]
fn test_seed_127_removes_every_assignment_in_a_body() {
    let (_, render) = applied_render(
        "def f():\n    value = alpha\n    other = beta\n    return other\n",
        "seed-127",
    );
    assert_eq!(render, "def f():\n    return other");
}

#[test]
fn test_seed_127_never_empties_a_body() {
    let (planted, render) = applied_render("def f():\n    value = alpha\n", "seed-127");
    assert!(!planted, "the only statement of a body is never planted");
    assert_eq!(render, "def f():\n    value = alpha");
}

#[test]
fn test_seed_127_leaves_an_augmented_assignment_standing() {
    let (_, render) = applied_render(
        "def f():\n    value = alpha\n    value += beta\n    return value\n",
        "seed-127",
    );
    assert_eq!(render, "def f():\n    value += beta\n    return value");
}

#[test]
fn test_seed_127_verdict_is_caught() {
    let target = target_with(ASSIGNMENT_MODULE, ASSIGNMENT_GATE, "caught127");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-127"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F21-6.

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
fn test_version_twenty_nine_pack_still_loads_the_one_hundred_and_twenty_four() {
    let dir = fresh_dir("pack29");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(124).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/29", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty nine pack loads");
    assert_eq!(loaded.len(), 124);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[127].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-131".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[127] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/30", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_nine_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(123).collect();
    entries.push(standing_entries()[124].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/29", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(124).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/30", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and twenty four entries under the version thirty format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 144);
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
        written.contains("\"canaryfield-seeds/34\""),
        "the written pack carries the thirtieth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
