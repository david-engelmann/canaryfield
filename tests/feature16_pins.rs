//! Feature slice 16 pins, the registry grown a twenty fourth time.
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
        "canaryfield-feature16-{}-{tag}",
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
fn test_registry_has_one_hundred_and_sixteen_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        116,
        "the grown registry holds one hundred and sixteen"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 116, "the ids stand unique");
    assert_eq!(SEEDS[104].id, "seed-104");
    assert_eq!(SEEDS[115].id, "seed-115");
}

// Seed-104, dropped assert message.

const ASSERT_MODULE: &str =
    "def check(value):\n    assert value, \"the value stands\"\n    return value\n";
const ASSERT_GATE: &str = "#!/bin/sh\ngrep -q \"the value stands\" mod.py\n";

#[test]
fn test_seed_104_plants_on_an_assert_carrying_a_message() {
    let (planted, _) = applied_render(ASSERT_MODULE, "seed-104");
    assert!(planted);
}

#[test]
fn test_seed_104_removes_the_assert_message() {
    let (planted, rendered) = applied_render(ASSERT_MODULE, "seed-104");
    assert!(planted);
    assert_eq!(
        rendered,
        "def check(value):\n    assert value\n    return value"
    );
}

#[test]
fn test_seed_104_keeps_the_test_standing_as_written() {
    let (planted, rendered) = applied_render("assert value > 1, \"too small\"\n", "seed-104");
    assert!(planted);
    assert_eq!(rendered, "assert value > 1");
}

#[test]
fn test_seed_104_removes_every_assert_message_in_a_function() {
    let (planted, rendered) = applied_render(
        "def check(first, second):\n    assert first, \"first stands\"\n    assert second, \"second stands\"\n",
        "seed-104",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check(first, second):\n    assert first\n    assert second"
    );
}

#[test]
fn test_seed_104_leaves_an_assert_without_a_message_standing() {
    let (planted, rendered) = applied_render("assert value\n", "seed-104");
    assert!(!planted);
    assert_eq!(rendered, "assert value");
}

#[test]
fn test_seed_104_verdict_is_caught() {
    let target = target_with(ASSERT_MODULE, ASSERT_GATE, "caught104");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-104"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-105, dropped formatted value conversion.

const CONVERSION_MODULE: &str = "label = f'{name!r}'\n";
const CONVERSION_GATE: &str = "#!/bin/sh\ngrep -q '!r' mod.py\n";

#[test]
fn test_seed_105_plants_on_a_formatted_value_carrying_a_conversion() {
    let (planted, _) = applied_render(CONVERSION_MODULE, "seed-105");
    assert!(planted);
}

#[test]
fn test_seed_105_removes_the_conversion() {
    let (planted, rendered) = applied_render(CONVERSION_MODULE, "seed-105");
    assert!(planted);
    assert_eq!(rendered, "label = f'{name}'");
}

#[test]
fn test_seed_105_keeps_a_format_spec_standing() {
    let (planted, rendered) = applied_render("label = f'{name!r:>10}'\n", "seed-105");
    assert!(planted);
    assert_eq!(rendered, "label = f'{name:>10}'");
}

#[test]
fn test_seed_105_removes_every_conversion_in_a_string() {
    let (planted, rendered) = applied_render("pair = f'{first!s} and {second!a}'\n", "seed-105");
    assert!(planted);
    assert_eq!(rendered, "pair = f'{first} and {second}'");
}

#[test]
fn test_seed_105_leaves_a_formatted_value_without_a_conversion_standing() {
    let (planted, rendered) = applied_render("label = f'{name}'\n", "seed-105");
    assert!(!planted);
    assert_eq!(rendered, "label = f'{name}'");
}

#[test]
fn test_seed_105_verdict_is_caught() {
    let target = target_with(CONVERSION_MODULE, CONVERSION_GATE, "caught105");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-105"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-106, swapped loop target elements.

const LOOP_TARGET_MODULE: &str = "for first, second in pairs:\n    total = first\n";
const LOOP_TARGET_GATE: &str = "#!/bin/sh\ngrep -qF \"for first, second in pairs\" mod.py\n";

#[test]
fn test_seed_106_plants_on_a_loop_sequence_target() {
    let (planted, _) = applied_render(LOOP_TARGET_MODULE, "seed-106");
    assert!(planted);
}

#[test]
fn test_seed_106_exchanges_the_first_two_elements() {
    let (planted, rendered) = applied_render(LOOP_TARGET_MODULE, "seed-106");
    assert!(planted);
    assert_eq!(rendered, "for second, first in pairs:\n    total = first");
}

#[test]
fn test_seed_106_plants_on_a_list_target() {
    let (planted, rendered) = applied_render(
        "for [first, second] in pairs:\n    total = first\n",
        "seed-106",
    );
    assert!(planted);
    assert_eq!(rendered, "for [second, first] in pairs:\n    total = first");
}

#[test]
fn test_seed_106_plants_on_an_asynchronous_loop() {
    let (planted, rendered) = applied_render(
        "async def run():\n    async for first, second in stream:\n        total = first\n",
        "seed-106",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "async def run():\n    async for second, first in stream:\n        total = first"
    );
}

#[test]
fn test_seed_106_leaves_a_single_name_target_standing() {
    let (planted, rendered) = applied_render("for item in pairs:\n    total = item\n", "seed-106");
    assert!(!planted);
    assert_eq!(rendered, "for item in pairs:\n    total = item");
}

#[test]
fn test_seed_106_leaves_a_one_element_sequence_standing() {
    let (planted, rendered) =
        applied_render("for [first] in pairs:\n    total = first\n", "seed-106");
    assert!(!planted);
    assert_eq!(rendered, "for [first] in pairs:\n    total = first");
}

#[test]
fn test_seed_106_verdict_is_caught() {
    let target = target_with(LOOP_TARGET_MODULE, LOOP_TARGET_GATE, "caught106");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-106"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-107, dropped keyword-only parameter.

const KEYWORD_ONLY_MODULE: &str = "def pick(first, *, key):\n    return key\n";
const KEYWORD_ONLY_GATE: &str = "#!/bin/sh\ngrep -qF \"def pick(first, *, key)\" mod.py\n";

#[test]
fn test_seed_107_plants_on_a_keyword_only_parameter() {
    let (planted, _) = applied_render(KEYWORD_ONLY_MODULE, "seed-107");
    assert!(planted);
}

#[test]
fn test_seed_107_removes_the_first_keyword_only_parameter() {
    let (planted, rendered) = applied_render(KEYWORD_ONLY_MODULE, "seed-107");
    assert!(planted);
    assert_eq!(rendered, "def pick(first):\n    return key");
}

#[test]
fn test_seed_107_removes_the_default_entry_that_stood_as_its_own() {
    let (planted, rendered) =
        applied_render("def pick(*, key=1, other=2):\n    return key\n", "seed-107");
    assert!(planted);
    assert_eq!(rendered, "def pick(*, other=2):\n    return key");
}

#[test]
fn test_seed_107_keeps_the_remaining_keyword_only_parameters_aligned() {
    let (planted, rendered) =
        applied_render("def pick(*, key, other=2):\n    return key\n", "seed-107");
    assert!(planted);
    assert_eq!(rendered, "def pick(*, other=2):\n    return key");
}

#[test]
fn test_seed_107_removes_a_lambdas_keyword_only_parameter() {
    let (planted, rendered) = applied_render("pick = lambda *, key: key\n", "seed-107");
    assert!(planted);
    assert_eq!(rendered, "pick = lambda: key");
}

#[test]
fn test_seed_107_leaves_a_definition_without_keyword_only_parameters_standing() {
    let (planted, rendered) =
        applied_render("def pick(first, second):\n    return first\n", "seed-107");
    assert!(!planted);
    assert_eq!(rendered, "def pick(first, second):\n    return first");
}

#[test]
fn test_seed_107_verdict_is_caught() {
    let target = target_with(KEYWORD_ONLY_MODULE, KEYWORD_ONLY_GATE, "caught107");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-107"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F16-6.

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
fn test_version_twenty_four_pack_still_loads_the_one_hundred_and_four() {
    let dir = fresh_dir("pack24");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(104).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/24", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty four pack loads");
    assert_eq!(loaded.len(), 104);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-103".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_five_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[107].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-112".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[107] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/25", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty five pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_four_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(103).collect();
    entries.push(standing_entries()[104].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/24", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_five_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(104).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/25", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and four entries under the version twenty five format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 116);
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
        written.contains("\"canaryfield-seeds/27\""),
        "the written pack carries the twenty fifth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
