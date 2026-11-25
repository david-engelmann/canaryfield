//! Feature slice 12 pins, the registry grown a twentieth time.
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
        "canaryfield-feature12-{}-{tag}",
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
    assert_eq!(SEEDS[88].id, "seed-88");
    assert_eq!(SEEDS[91].id, "seed-91");
}

// Seed-88, dropped return annotation.

const RETURN_MODULE: &str = "def check() -> int:\n    return 1\n";
const RETURN_GATE: &str = "#!/bin/sh\ngrep -q \"\\-> int\" mod.py\n";

#[test]
fn test_seed_88_plants_and_is_caught() {
    let target = target_with(RETURN_MODULE, RETURN_GATE, "caught88");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-88"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_88_clears_the_return_annotation_and_keeps_the_definition() {
    let (planted, rendered) = applied_render("def check() -> int:\n    return 1\n", "seed-88");
    assert!(planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_88_plants_on_every_annotated_definition() {
    let (planted, rendered) = applied_render(
        "def first() -> int:\n    return 1\nasync def second() -> str:\n    return 'x'\n",
        "seed-88",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def first():\n    return 1\n\nasync def second():\n    return 'x'"
    );
}

#[test]
fn test_seed_88_leaves_an_unannotated_definition_standing() {
    let (planted, rendered) = applied_render("def check():\n    return 1\n", "seed-88");
    assert!(!planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_88_is_not_applicable_without_a_return_annotation() {
    let target = target_with("def check():\n    return 1\n", RETURN_GATE, "na88");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-88"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-89, swapped statement order.

const ORDER_MODULE: &str = "first = 1\nsecond = 2\n";
const ORDER_GATE: &str = "#!/bin/sh\nhead -1 mod.py | grep -q \"first = 1\"\n";

#[test]
fn test_seed_89_plants_and_is_caught() {
    let target = target_with(ORDER_MODULE, ORDER_GATE, "caught89");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-89"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_89_exchanges_the_first_two_statements_and_keeps_the_rest() {
    let (planted, rendered) = applied_render("first = 1\nsecond = 2\nthird = 3\n", "seed-89");
    assert!(planted);
    assert_eq!(rendered, "second = 2\nfirst = 1\nthird = 3");
}

#[test]
fn test_seed_89_plants_in_every_body() {
    let (planted, rendered) = applied_render(
        "def check():\n    first = 1\n    second = 2\n    return first\nvalue = 3\n",
        "seed-89",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "value = 3\n\ndef check():\n    second = 2\n    first = 1\n    return first"
    );
}

#[test]
fn test_seed_89_leaves_a_single_statement_body_standing() {
    let (planted, rendered) = applied_render("def check():\n    return 1\n", "seed-89");
    assert!(!planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_89_is_not_applicable_without_a_two_statement_body() {
    let target = target_with("def check():\n    return 1\n", ORDER_GATE, "na89");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-89"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_89_leaves_a_future_led_body_standing() {
    let (planted, rendered) =
        applied_render("from __future__ import annotations\nx = 1\n", "seed-89");
    assert!(!planted);
    assert_eq!(rendered, "from __future__ import annotations\nx = 1");
}

#[test]
fn test_seed_89_exchanges_two_future_imports() {
    let (planted, rendered) = applied_render(
        "from __future__ import annotations\nfrom __future__ import print_function\nx = 1\n",
        "seed-89",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "from __future__ import print_function\nfrom __future__ import annotations\nx = 1"
    );
}

#[test]
fn test_seed_89_plants_a_future_import_into_the_leading_run() {
    let (planted, rendered) =
        applied_render("x = 1\nfrom __future__ import annotations\n", "seed-89");
    assert!(planted);
    assert_eq!(rendered, "from __future__ import annotations\nx = 1");
}

// Seed-90, swapped comprehension condition order.

const CONDITION_MODULE: &str = "def check(values):\n    return [v for v in values if v if v > 1]\n";
const CONDITION_GATE: &str = "#!/bin/sh\ngrep -q \"if v if v > 1\" mod.py\n";

#[test]
fn test_seed_90_plants_and_is_caught() {
    let target = target_with(CONDITION_MODULE, CONDITION_GATE, "caught90");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-90"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_90_exchanges_the_first_two_conditions_and_keeps_the_rest() {
    let (planted, rendered) = applied_render(
        "kept = [v for v in values if v if v > 1 if v != 2]\n",
        "seed-90",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "kept = [v for v in values if v > 1 if v if v != 2]"
    );
}

#[test]
fn test_seed_90_plants_on_every_clause_carrying_two_conditions() {
    let (planted, rendered) = applied_render(
        "kept = [v for v in values if v if v > 1]\nother = [w for w in words if w if len(w)]\n",
        "seed-90",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "kept = [v for v in values if v > 1 if v]\nother = [w for w in words if len(w) if w]"
    );
}

#[test]
fn test_seed_90_leaves_a_single_condition_clause_standing() {
    let (planted, rendered) = applied_render("kept = [v for v in values if v]\n", "seed-90");
    assert!(!planted);
    assert_eq!(rendered, "kept = [v for v in values if v]");
}

#[test]
fn test_seed_90_is_not_applicable_without_two_conditions() {
    let target = target_with(
        "def check(values):\n    return [v for v in values]\n",
        CONDITION_GATE,
        "na90",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-90"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-91, dropped starred argument.

const STARRED_MODULE: &str = "def check():\n    return combine(*parts)\n";
const STARRED_GATE: &str = "#!/bin/sh\ngrep -q \"\\*parts\" mod.py\n";

#[test]
fn test_seed_91_plants_and_is_caught() {
    let target = target_with(STARRED_MODULE, STARRED_GATE, "caught91");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-91"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_91_removes_the_starred_argument_and_keeps_the_rest() {
    let (planted, rendered) = applied_render("total = combine(first, *parts, last)\n", "seed-91");
    assert!(planted);
    assert_eq!(rendered, "total = combine(first, last)");
}

#[test]
fn test_seed_91_plants_on_every_starred_call() {
    let (planted, rendered) =
        applied_render("total = combine(*parts)\nother = merge(*rest)\n", "seed-91");
    assert!(planted);
    assert_eq!(rendered, "total = combine()\nother = merge()");
}

#[test]
fn test_seed_91_leaves_an_unstarred_call_standing() {
    let (planted, rendered) = applied_render("total = combine(first, **options)\n", "seed-91");
    assert!(!planted);
    assert_eq!(rendered, "total = combine(first, **options)");
}

#[test]
fn test_seed_91_is_not_applicable_without_a_starred_argument() {
    let target = target_with(
        "def check():\n    return combine(first)\n",
        STARRED_GATE,
        "na91",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-91"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F12-6.

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
fn test_version_twenty_pack_still_loads_the_eighty_eight() {
    let dir = fresh_dir("pack20");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(88).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/20", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty pack loads");
    assert_eq!(loaded.len(), 88);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-87".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_one_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[91].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-99".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[91] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/21", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(87).collect();
    entries.push(standing_entries()[88].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/20", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_one_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(88).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/21", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "eighty eight entries under the version twenty one format are refused"
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
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
