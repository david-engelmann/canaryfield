//! Feature slice 19 pins, the registry grown a twenty seventh time.
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
        "canaryfield-feature19-{}-{tag}",
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
fn test_registry_has_one_hundred_and_forty_eight_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        148,
        "the grown registry holds one hundred and forty eight"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 148, "the ids stand unique");
    assert_eq!(SEEDS[116].id, "seed-116");
    assert_eq!(SEEDS[119].id, "seed-119");
}

// Seed-116, dropped set element.

const SET_ELEMENT_MODULE: &str = "values = {10, 20, 30}\n";
const SET_ELEMENT_GATE: &str = "#!/bin/sh\ngrep -q '10, 20' mod.py\n";

#[test]
fn test_seed_116_plants_on_a_set_carrying_two_elements() {
    let (planted, _) = applied_render(SET_ELEMENT_MODULE, "seed-116");
    assert!(planted, "a set carrying at least two elements plants");
}

#[test]
fn test_seed_116_removes_the_first_element() {
    let (_, render) = applied_render(SET_ELEMENT_MODULE, "seed-116");
    assert_eq!(render, "values = {20, 30}");
}

#[test]
fn test_seed_116_keeps_the_remaining_elements_standing() {
    let (_, render) = applied_render(SET_ELEMENT_MODULE, "seed-116");
    assert!(
        render.contains("{20, 30}"),
        "the remaining elements stand as written: {render}"
    );
}

#[test]
fn test_seed_116_removes_the_first_element_of_every_set() {
    let (_, render) = applied_render("a = {1, 2}\nb = {3, 4, 5}\n", "seed-116");
    assert_eq!(render, "a = {2}\nb = {4, 5}");
}

#[test]
fn test_seed_116_leaves_a_single_element_set_standing() {
    let (planted, render) = applied_render("values = {10}\n", "seed-116");
    assert!(!planted, "a set carrying one element does not plant");
    assert_eq!(render, "values = {10}");
}

#[test]
fn test_seed_116_verdict_is_caught() {
    let target = target_with(SET_ELEMENT_MODULE, SET_ELEMENT_GATE, "caught116");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-116"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-117, dropped decorator call.

const DECORATOR_CALL_MODULE: &str = "@register('main')\ndef handler():\n    return 1\n";
const DECORATOR_CALL_GATE: &str = "#!/bin/sh\ngrep -q \"'main'\" mod.py\n";

#[test]
fn test_seed_117_plants_on_a_call_decorator() {
    let (planted, _) = applied_render(DECORATOR_CALL_MODULE, "seed-117");
    assert!(planted, "a decorator standing as a call plants");
}

#[test]
fn test_seed_117_removes_the_call() {
    let (_, render) = applied_render(DECORATOR_CALL_MODULE, "seed-117");
    assert_eq!(render, "@register\ndef handler():\n    return 1");
}

#[test]
fn test_seed_117_keeps_the_decorated_definition_standing() {
    let (_, render) = applied_render(DECORATOR_CALL_MODULE, "seed-117");
    assert!(
        render.contains("def handler():"),
        "the decorated definition stands as written: {render}"
    );
}

#[test]
fn test_seed_117_removes_every_decorator_call_in_a_list() {
    let (_, render) = applied_render(
        "@first(1)\n@second\n@third(3)\ndef handler():\n    return 1\n",
        "seed-117",
    );
    assert_eq!(
        render,
        "@first\n@second\n@third\ndef handler():\n    return 1"
    );
}

#[test]
fn test_seed_117_leaves_a_plain_decorator_standing() {
    let (planted, render) = applied_render("@register\ndef handler():\n    return 1\n", "seed-117");
    assert!(
        !planted,
        "a decorator standing as a plain name does not plant"
    );
    assert_eq!(render, "@register\ndef handler():\n    return 1");
}

#[test]
fn test_seed_117_verdict_is_caught() {
    let target = target_with(DECORATOR_CALL_MODULE, DECORATOR_CALL_GATE, "caught117");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-117"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-118, dropped attribute.

const ATTRIBUTE_MODULE: &str = "value = config.settings\n";
const ATTRIBUTE_GATE: &str = "#!/bin/sh\ngrep -q 'settings' mod.py\n";

#[test]
fn test_seed_118_plants_on_an_attribute_access() {
    let (planted, _) = applied_render(ATTRIBUTE_MODULE, "seed-118");
    assert!(planted, "an attribute access plants");
}

#[test]
fn test_seed_118_removes_the_attribute() {
    let (_, render) = applied_render(ATTRIBUTE_MODULE, "seed-118");
    assert_eq!(render, "value = config");
}

#[test]
fn test_seed_118_keeps_the_object_standing() {
    let (_, render) = applied_render(ATTRIBUTE_MODULE, "seed-118");
    assert!(
        render.contains("value = config"),
        "the object stands as written: {render}"
    );
}

#[test]
fn test_seed_118_removes_every_attribute_in_an_expression() {
    let (_, render) = applied_render(
        "value = config.settings.timeout\nother = config.settings\n",
        "seed-118",
    );
    assert_eq!(render, "value = config\nother = config");
}

#[test]
fn test_seed_118_leaves_a_plain_name_standing() {
    let (planted, render) = applied_render("value = config\n", "seed-118");
    assert!(!planted, "a plain name does not plant");
    assert_eq!(render, "value = config");
}

#[test]
fn test_seed_118_verdict_is_caught() {
    let target = target_with(ATTRIBUTE_MODULE, ATTRIBUTE_GATE, "caught118");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-118"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-119, dropped exception type.

const EXCEPTION_TYPE_MODULE: &str = "try:\n    run()\nexcept ValueError:\n    recover()\n";
const EXCEPTION_TYPE_GATE: &str = "#!/bin/sh\ngrep -q 'ValueError' mod.py\n";

#[test]
fn test_seed_119_plants_on_a_typed_handler() {
    let (planted, _) = applied_render(EXCEPTION_TYPE_MODULE, "seed-119");
    assert!(planted, "a handler carrying a type plants");
}

#[test]
fn test_seed_119_removes_the_type() {
    let (_, render) = applied_render(EXCEPTION_TYPE_MODULE, "seed-119");
    assert_eq!(render, "try:\n    run()\nexcept:\n    recover()");
}

#[test]
fn test_seed_119_keeps_the_handler_body_standing() {
    let (_, render) = applied_render(EXCEPTION_TYPE_MODULE, "seed-119");
    assert!(
        render.contains("recover()"),
        "the handler body stands as written: {render}"
    );
}

#[test]
fn test_seed_119_removes_every_handler_type_in_a_try() {
    let (_, render) = applied_render(
        "try:\n    run()\nexcept ValueError:\n    recover()\nexcept KeyError:\n    reset()\n",
        "seed-119",
    );
    assert_eq!(
        render,
        "try:\n    run()\nexcept:\n    recover()\nexcept:\n    reset()"
    );
}

#[test]
fn test_seed_119_leaves_a_bare_handler_standing() {
    let (planted, render) = applied_render("try:\n    run()\nexcept:\n    recover()\n", "seed-119");
    assert!(!planted, "a bare handler does not plant");
    assert_eq!(render, "try:\n    run()\nexcept:\n    recover()");
}

#[test]
fn test_seed_119_verdict_is_caught() {
    let target = target_with(EXCEPTION_TYPE_MODULE, EXCEPTION_TYPE_GATE, "caught119");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-119"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F19-6.

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
fn test_version_twenty_seven_pack_still_loads_the_one_hundred_and_sixteen() {
    let dir = fresh_dir("pack27");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(116).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/27", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty seven pack loads");
    assert_eq!(loaded.len(), 116);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_eight_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[119].clone();
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
    entries[119] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/28", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_seven_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(115).collect();
    entries.push(standing_entries()[116].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/27", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_eight_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(116).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/28", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and sixteen entries under the version twenty eight format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 148);
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
        "the written pack carries the twenty eighth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
