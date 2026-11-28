//! Feature slice 20 pins, the registry grown a twenty eighth time.
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
        "canaryfield-feature20-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 220, "the grown registry holds two hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "the ids stand unique");
    assert_eq!(SEEDS[120].id, "seed-120");
    assert_eq!(SEEDS[123].id, "seed-123");
}

// Seed-120, dropped subscript.

const SUBSCRIPT_MODULE: &str = "value = items[0]\n";
const SUBSCRIPT_GATE: &str = "#!/bin/sh\ngrep -q 'items\\[0\\]' mod.py\n";

#[test]
fn test_seed_120_plants_on_a_subscript() {
    let (planted, _) = applied_render(SUBSCRIPT_MODULE, "seed-120");
    assert!(planted, "a subscript plants");
}

#[test]
fn test_seed_120_removes_the_subscript() {
    let (_, render) = applied_render(SUBSCRIPT_MODULE, "seed-120");
    assert_eq!(render, "value = items");
}

#[test]
fn test_seed_120_keeps_the_subscripted_object_standing() {
    let (_, render) = applied_render("value = items[index + 1]\n", "seed-120");
    assert!(
        render.contains("value = items"),
        "the subscripted object stands as written: {render}"
    );
}

#[test]
fn test_seed_120_removes_every_subscript_in_an_expression() {
    let (_, render) = applied_render("value = left[0] + right[1]\n", "seed-120");
    assert_eq!(render, "value = left + right");
}

#[test]
fn test_seed_120_leaves_a_plain_name_standing() {
    let (planted, render) = applied_render("value = items\n", "seed-120");
    assert!(!planted, "a plain name does not plant");
    assert_eq!(render, "value = items");
}

#[test]
fn test_seed_120_verdict_is_caught() {
    let target = target_with(SUBSCRIPT_MODULE, SUBSCRIPT_GATE, "caught120");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-120"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-121, dropped arithmetic operand.

const ARITHMETIC_MODULE: &str = "value = base + extra\n";
const ARITHMETIC_GATE: &str = "#!/bin/sh\ngrep -q 'base + extra' mod.py\n";

#[test]
fn test_seed_121_plants_on_a_binary_operation() {
    let (planted, _) = applied_render(ARITHMETIC_MODULE, "seed-121");
    assert!(planted, "a binary operation plants");
}

#[test]
fn test_seed_121_removes_the_operation_and_its_right_operand() {
    let (_, render) = applied_render(ARITHMETIC_MODULE, "seed-121");
    assert_eq!(render, "value = base");
}

#[test]
fn test_seed_121_keeps_the_left_operand_standing() {
    let (_, render) = applied_render("value = base + extra - tax\n", "seed-121");
    assert!(
        render.contains("value = base"),
        "the left operand stands as written: {render}"
    );
}

#[test]
fn test_seed_121_removes_every_operation_in_an_expression() {
    let (_, render) = applied_render("value = first + second * third\n", "seed-121");
    assert_eq!(render, "value = first");
}

#[test]
fn test_seed_121_leaves_a_plain_name_standing() {
    let (planted, render) = applied_render("value = total\n", "seed-121");
    assert!(!planted, "a plain name does not plant");
    assert_eq!(render, "value = total");
}

#[test]
fn test_seed_121_verdict_is_caught() {
    let target = target_with(ARITHMETIC_MODULE, ARITHMETIC_GATE, "caught121");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-121"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-122, dropped first call argument.

const FIRST_ARGUMENT_MODULE: &str = "value = combine(first, second)\n";
const FIRST_ARGUMENT_GATE: &str = "#!/bin/sh\ngrep -q 'combine(first, second)' mod.py\n";

#[test]
fn test_seed_122_plants_on_a_call_carrying_two_arguments() {
    let (planted, _) = applied_render(FIRST_ARGUMENT_MODULE, "seed-122");
    assert!(planted, "a call carrying at least two arguments plants");
}

#[test]
fn test_seed_122_removes_the_first_argument() {
    let (_, render) = applied_render(FIRST_ARGUMENT_MODULE, "seed-122");
    assert_eq!(render, "value = combine(second)");
}

#[test]
fn test_seed_122_keeps_the_remaining_arguments_standing() {
    let (_, render) = applied_render("value = combine(first, second + 1, third)\n", "seed-122");
    assert_eq!(render, "value = combine(second + 1, third)");
}

#[test]
fn test_seed_122_removes_the_first_argument_of_every_call() {
    let (_, render) = applied_render("value = combine(a, b) + merge(c, d)\n", "seed-122");
    assert_eq!(render, "value = combine(b) + merge(d)");
}

#[test]
fn test_seed_122_leaves_a_single_argument_call_standing() {
    let (planted, render) = applied_render("value = combine(only)\n", "seed-122");
    assert!(!planted, "a call carrying a single argument does not plant");
    assert_eq!(render, "value = combine(only)");
}

#[test]
fn test_seed_122_verdict_is_caught() {
    let target = target_with(FIRST_ARGUMENT_MODULE, FIRST_ARGUMENT_GATE, "caught122");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-122"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-123, dropped first condition term.

const FIRST_TERM_MODULE: &str = "value = ready and willing\n";
const FIRST_TERM_GATE: &str = "#!/bin/sh\ngrep -q 'ready and willing' mod.py\n";

#[test]
fn test_seed_123_plants_on_a_boolean_operation_carrying_two_values() {
    let (planted, _) = applied_render(FIRST_TERM_MODULE, "seed-123");
    assert!(
        planted,
        "a boolean operation carrying at least two values plants"
    );
}

#[test]
fn test_seed_123_removes_the_first_value() {
    let (_, render) = applied_render(FIRST_TERM_MODULE, "seed-123");
    assert_eq!(render, "value = willing");
}

#[test]
fn test_seed_123_keeps_the_remaining_values_standing() {
    let (_, render) = applied_render("value = first or second or third\n", "seed-123");
    assert_eq!(render, "value = second or third");
}

#[test]
fn test_seed_123_removes_the_first_value_of_every_boolean_operation() {
    let (_, render) = applied_render(
        "value = (alpha and beta) or (gamma and delta)\n",
        "seed-123",
    );
    assert_eq!(render, "value = delta");
}

#[test]
fn test_seed_123_leaves_a_comparison_standing() {
    let (planted, render) = applied_render("value = left == right\n", "seed-123");
    assert!(!planted, "a comparison does not plant");
    assert_eq!(render, "value = left == right");
}

#[test]
fn test_seed_123_verdict_is_caught() {
    let target = target_with(FIRST_TERM_MODULE, FIRST_TERM_GATE, "caught123");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-123"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F20-6.

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
fn test_version_twenty_eight_pack_still_loads_the_one_hundred_and_twenty() {
    let dir = fresh_dir("pack28");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(120).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/28", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty eight pack loads");
    assert_eq!(loaded.len(), 120);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_nine_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[123].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-127".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[123] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/29", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_eight_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(119).collect();
    entries.push(standing_entries()[120].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/28", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_nine_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(120).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/29", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and twenty entries under the version twenty nine format are refused"
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
        written.contains("\"canaryfield-seeds/53\""),
        "the written pack carries the twenty ninth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
