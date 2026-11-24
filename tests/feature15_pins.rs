//! Feature slice 15 pins, the registry grown a twenty third time.
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
        "canaryfield-feature15-{}-{tag}",
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
fn test_registry_has_one_hundred_and_twelve_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        112,
        "the grown registry holds one hundred and twelve"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 112, "the ids stand unique");
    assert_eq!(SEEDS[100].id, "seed-100");
    assert_eq!(SEEDS[103].id, "seed-103");
}

// Seed-100, dropped parameter.

const PARAMETER_MODULE: &str = "def pick(first, second):\n    return first\n";
const PARAMETER_GATE: &str = "#!/bin/sh\ngrep -q \"def pick(first, second)\" mod.py\n";

#[test]
fn test_seed_100_plants_on_a_definition_with_parameters() {
    let (planted, _) = applied_render(PARAMETER_MODULE, "seed-100");
    assert!(planted);
}

#[test]
fn test_seed_100_removes_the_first_parameter() {
    let (planted, rendered) = applied_render(PARAMETER_MODULE, "seed-100");
    assert!(planted);
    assert_eq!(rendered, "def pick(second):\n    return first");
}

#[test]
fn test_seed_100_removes_the_default_that_stood_as_the_parameters_own() {
    let (planted, rendered) = applied_render(
        "def pick(first=1, second=2):\n    return first\n",
        "seed-100",
    );
    assert!(planted);
    assert_eq!(rendered, "def pick(second=2):\n    return first");
}

#[test]
fn test_seed_100_keeps_the_defaults_of_the_remaining_parameters() {
    let (planted, rendered) = applied_render(
        "def pick(first, second=2, third=3):\n    return first\n",
        "seed-100",
    );
    assert!(planted);
    assert_eq!(rendered, "def pick(second=2, third=3):\n    return first");
}

#[test]
fn test_seed_100_removes_a_positional_only_parameter() {
    let (planted, rendered) = applied_render(
        "def pick(first, /, second=1):\n    return first\n",
        "seed-100",
    );
    assert!(planted);
    assert_eq!(rendered, "def pick(second=1):\n    return first");
}

#[test]
fn test_seed_100_removes_a_lambdas_first_parameter() {
    let (planted, rendered) = applied_render("pick = lambda first, second: first\n", "seed-100");
    assert!(planted);
    assert_eq!(rendered, "pick = lambda second: first");
}

#[test]
fn test_seed_100_leaves_a_definition_without_parameters_standing() {
    let (planted, rendered) = applied_render("def pick():\n    return 1\n", "seed-100");
    assert!(!planted);
    assert_eq!(rendered, "def pick():\n    return 1");
}

#[test]
fn test_seed_100_verdict_is_caught() {
    let target = target_with(PARAMETER_MODULE, PARAMETER_GATE, "caught100");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-100"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-101, dropped elif branch.

const ELIF_MODULE: &str = "if a:\n    x()\nelif b:\n    y()\n";
const ELIF_GATE: &str = "#!/bin/sh\ngrep -q \"elif b\" mod.py\n";

#[test]
fn test_seed_101_plants_on_an_elif_branch() {
    let (planted, _) = applied_render(ELIF_MODULE, "seed-101");
    assert!(planted);
}

#[test]
fn test_seed_101_removes_the_elif_link() {
    let (planted, rendered) = applied_render(ELIF_MODULE, "seed-101");
    assert!(planted);
    assert_eq!(rendered, "if a:\n    x()");
}

#[test]
fn test_seed_101_collapses_a_chain_of_elif_branches() {
    let (planted, rendered) = applied_render(
        "if a:\n    x()\nelif b:\n    y()\nelif c:\n    w()\nelse:\n    z()\n",
        "seed-101",
    );
    assert!(planted);
    assert_eq!(rendered, "if a:\n    x()\nelse:\n    z()");
}

#[test]
fn test_seed_101_keeps_the_final_else_clause() {
    let (planted, rendered) = applied_render(
        "if a:\n    x()\nelif b:\n    y()\nelse:\n    z()\n",
        "seed-101",
    );
    assert!(planted);
    assert_eq!(rendered, "if a:\n    x()\nelse:\n    z()");
}

#[test]
fn test_seed_101_leaves_an_if_with_a_plain_else_standing() {
    let (planted, rendered) = applied_render("if a:\n    x()\nelse:\n    z()\n", "seed-101");
    assert!(!planted);
    assert_eq!(rendered, "if a:\n    x()\nelse:\n    z()");
}

#[test]
fn test_seed_101_leaves_an_if_without_an_else_standing() {
    let (planted, rendered) = applied_render("if a:\n    x()\n", "seed-101");
    assert!(!planted);
    assert_eq!(rendered, "if a:\n    x()");
}

#[test]
fn test_seed_101_verdict_is_caught() {
    let target = target_with(ELIF_MODULE, ELIF_GATE, "caught101");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-101"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-102, dropped import alias.

const ALIAS_MODULE: &str = "import json as js\n";
const ALIAS_GATE: &str = "#!/bin/sh\ngrep -q \"as js\" mod.py\n";

#[test]
fn test_seed_102_plants_on_an_aliased_import() {
    let (planted, _) = applied_render(ALIAS_MODULE, "seed-102");
    assert!(planted);
}

#[test]
fn test_seed_102_removes_the_alias_name() {
    let (planted, rendered) = applied_render(ALIAS_MODULE, "seed-102");
    assert!(planted);
    assert_eq!(rendered, "import json");
}

#[test]
fn test_seed_102_removes_the_first_alias_only() {
    let (planted, rendered) = applied_render("import json as js, sys as system\n", "seed-102");
    assert!(planted);
    assert_eq!(rendered, "import json, sys as system");
}

#[test]
fn test_seed_102_plants_on_a_from_import_alias() {
    let (planted, rendered) = applied_render("from json import dumps as dump\n", "seed-102");
    assert!(planted);
    assert_eq!(rendered, "from json import dumps");
}

#[test]
fn test_seed_102_leaves_an_import_without_an_alias_standing() {
    let (planted, rendered) = applied_render("import json\n", "seed-102");
    assert!(!planted);
    assert_eq!(rendered, "import json");
}

#[test]
fn test_seed_102_verdict_is_caught() {
    let target = target_with(ALIAS_MODULE, ALIAS_GATE, "caught102");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-102"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-103, dropped yield value.

const YIELD_MODULE: &str = "def gen():\n    yield 1\n";
const YIELD_GATE: &str = "#!/bin/sh\ngrep -q \"yield 1\" mod.py\n";

#[test]
fn test_seed_103_plants_on_a_yield_carrying_a_value() {
    let (planted, _) = applied_render(YIELD_MODULE, "seed-103");
    assert!(planted);
}

#[test]
fn test_seed_103_removes_the_yielded_value() {
    let (planted, rendered) = applied_render(YIELD_MODULE, "seed-103");
    assert!(planted);
    assert_eq!(rendered, "def gen():\n    yield");
}

#[test]
fn test_seed_103_removes_every_yielded_value_in_a_function() {
    let (planted, rendered) = applied_render("def gen():\n    yield 1\n    yield 2\n", "seed-103");
    assert!(planted);
    assert_eq!(rendered, "def gen():\n    yield\n    yield");
}

#[test]
fn test_seed_103_leaves_a_bare_yield_standing() {
    let (planted, rendered) = applied_render("def gen():\n    yield\n", "seed-103");
    assert!(!planted);
    assert_eq!(rendered, "def gen():\n    yield");
}

#[test]
fn test_seed_103_leaves_a_function_without_a_yield_standing() {
    let (planted, rendered) = applied_render("def pick():\n    return 1\n", "seed-103");
    assert!(!planted);
    assert_eq!(rendered, "def pick():\n    return 1");
}

#[test]
fn test_seed_103_verdict_is_caught() {
    let target = target_with(YIELD_MODULE, YIELD_GATE, "caught103");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-103"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F15-6.

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
fn test_version_twenty_three_pack_still_loads_the_one_hundred() {
    let dir = fresh_dir("pack23");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(100).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/23", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty three pack loads");
    assert_eq!(loaded.len(), 100);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-99".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_four_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[103].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-108".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[103] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/24", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_three_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(99).collect();
    entries.push(standing_entries()[100].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/23", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_four_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(100).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/24", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred entries under the version twenty four format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 112);
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
        written.contains("\"canaryfield-seeds/26\""),
        "the written pack carries the twenty fifth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
