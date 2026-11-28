//! Feature slice 41 pins, the registry grown a forty ninth time.
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
        "canaryfield-feature41-{}-{tag}",
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
fn test_registry_has_two_hundred_and_eight_unique_seeds() {
    assert_eq!(SEEDS.len(), 208, "the registry stands at 208 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 208, "every seed id stands unique");
    assert_eq!(SEEDS[207].id, "seed-207", "the last seed stands at 207");
}

// Seed-204, dropped last return value dict entry.

const LAST_RETURN_DICT_MODULE: &str = "def pick():\n    return {'a': alpha, 'b': beta}\n";
const LAST_RETURN_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'b': beta\" mod.py\n";

#[test]
fn test_seed_204_plants_on_a_return_value_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(LAST_RETURN_DICT_MODULE, "seed-204");
    assert!(planted, "a return value dictionary plants");
}

#[test]
fn test_seed_204_removes_the_last_return_value_dict_entry() {
    let (_, render) = applied_render(LAST_RETURN_DICT_MODULE, "seed-204");
    assert_eq!(render, "def pick():\n    return {'a': alpha}");
}

#[test]
fn test_seed_204_keeps_the_earlier_value_entries_standing() {
    let source = "def pick():\n    return {'a': alpha, 'b': beta, 'c': gamma}\n";
    let (_, render) = applied_render(source, "seed-204");
    assert_eq!(render, "def pick():\n    return {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_204_removes_the_last_entry_of_every_return_value_dictionary() {
    let source = "def pick():\n    return {'a': alpha, 'b': beta}\n\ndef choose():\n    return {'x': one, 'y': two}\n";
    let (_, render) = applied_render(source, "seed-204");
    assert_eq!(
        render,
        "def pick():\n    return {'a': alpha}\n\ndef choose():\n    return {'x': one}"
    );
}

#[test]
fn test_seed_204_leaves_a_single_entry_return_value_dictionary_standing() {
    let source = "def pick():\n    return {'a': alpha}\n";
    let (planted, render) = applied_render(source, "seed-204");
    assert!(
        !planted,
        "a one entry return value dictionary does not plant"
    );
    assert_eq!(render, "def pick():\n    return {'a': alpha}");
}

#[test]
fn test_seed_204_leaves_a_dictionary_standing_outside_a_return_value() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-204");
    assert!(
        !planted,
        "a dictionary standing outside a return value is not handed back"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_204_verdict_is_caught() {
    let target = target_with(LAST_RETURN_DICT_MODULE, LAST_RETURN_DICT_GATE, "caught204");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-204"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-205, dropped first return value dict entry.

const FIRST_RETURN_DICT_MODULE: &str = "def pick():\n    return {'a': alpha, 'b': beta}\n";
const FIRST_RETURN_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'a': alpha\" mod.py\n";

#[test]
fn test_seed_205_plants_on_a_return_value_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(FIRST_RETURN_DICT_MODULE, "seed-205");
    assert!(planted, "a return value dictionary plants");
}

#[test]
fn test_seed_205_removes_the_first_return_value_dict_entry() {
    let (_, render) = applied_render(FIRST_RETURN_DICT_MODULE, "seed-205");
    assert_eq!(render, "def pick():\n    return {'b': beta}");
}

#[test]
fn test_seed_205_keeps_the_later_value_entries_standing() {
    let source = "def pick():\n    return {'a': alpha, 'b': beta, 'c': gamma}\n";
    let (_, render) = applied_render(source, "seed-205");
    assert_eq!(render, "def pick():\n    return {'b': beta, 'c': gamma}");
}

#[test]
fn test_seed_205_removes_the_first_entry_of_every_return_value_dictionary() {
    let source = "def pick():\n    return {'a': alpha, 'b': beta}\n\ndef choose():\n    return {'x': one, 'y': two}\n";
    let (_, render) = applied_render(source, "seed-205");
    assert_eq!(
        render,
        "def pick():\n    return {'b': beta}\n\ndef choose():\n    return {'y': two}"
    );
}

#[test]
fn test_seed_205_leaves_a_single_entry_return_value_dictionary_standing() {
    let source = "def pick():\n    return {'a': alpha}\n";
    let (planted, render) = applied_render(source, "seed-205");
    assert!(
        !planted,
        "a one entry return value dictionary does not plant"
    );
    assert_eq!(render, "def pick():\n    return {'a': alpha}");
}

#[test]
fn test_seed_205_leaves_a_dictionary_standing_outside_a_return_value() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-205");
    assert!(
        !planted,
        "a dictionary standing outside a return value is not handed back"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_205_verdict_is_caught() {
    let target = target_with(
        FIRST_RETURN_DICT_MODULE,
        FIRST_RETURN_DICT_GATE,
        "caught205",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-205"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-206, dropped last comprehension element dict entry.

const LAST_ELEMENT_DICT_MODULE: &str =
    "rows = [{'a': alpha, 'b': beta} for item in items]\nkeep = gamma\n";
const LAST_ELEMENT_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'b': beta\" mod.py\n";

#[test]
fn test_seed_206_plants_on_a_comprehension_element_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(LAST_ELEMENT_DICT_MODULE, "seed-206");
    assert!(planted, "a comprehension element dictionary plants");
}

#[test]
fn test_seed_206_removes_the_last_comprehension_element_dict_entry() {
    let (_, render) = applied_render(LAST_ELEMENT_DICT_MODULE, "seed-206");
    assert_eq!(
        render,
        "rows = [{'a': alpha} for item in items]\nkeep = gamma"
    );
}

#[test]
fn test_seed_206_keeps_the_earlier_element_entries_standing() {
    let source = "rows = [{'a': alpha, 'b': beta, 'c': gamma} for item in items]\n";
    let (_, render) = applied_render(source, "seed-206");
    assert_eq!(render, "rows = [{'a': alpha, 'b': beta} for item in items]");
}

#[test]
fn test_seed_206_removes_the_last_entry_of_every_comprehension_element_dictionary() {
    let source = "rows = [{'a': alpha, 'b': beta} for item in items]\nothers = [{'x': one, 'y': two} for entry in entries]\n";
    let (_, render) = applied_render(source, "seed-206");
    assert_eq!(
        render,
        "rows = [{'a': alpha} for item in items]\nothers = [{'x': one} for entry in entries]"
    );
}

#[test]
fn test_seed_206_leaves_a_single_entry_comprehension_element_dictionary_standing() {
    let source = "rows = [{'a': alpha} for item in items]\n";
    let (planted, render) = applied_render(source, "seed-206");
    assert!(
        !planted,
        "a one entry comprehension element dictionary does not plant"
    );
    assert_eq!(render, "rows = [{'a': alpha} for item in items]");
}

#[test]
fn test_seed_206_leaves_a_dictionary_standing_outside_a_comprehension_element() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-206");
    assert!(
        !planted,
        "a dictionary standing outside a comprehension element is not yielded"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_206_verdict_is_caught() {
    let target = target_with(
        LAST_ELEMENT_DICT_MODULE,
        LAST_ELEMENT_DICT_GATE,
        "caught206",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-206"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-207, dropped first comprehension element dict entry.

const FIRST_ELEMENT_DICT_MODULE: &str =
    "rows = [{'a': alpha, 'b': beta} for item in items]\nkeep = gamma\n";
const FIRST_ELEMENT_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'a': alpha\" mod.py\n";

#[test]
fn test_seed_207_plants_on_a_comprehension_element_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(FIRST_ELEMENT_DICT_MODULE, "seed-207");
    assert!(planted, "a comprehension element dictionary plants");
}

#[test]
fn test_seed_207_removes_the_first_comprehension_element_dict_entry() {
    let (_, render) = applied_render(FIRST_ELEMENT_DICT_MODULE, "seed-207");
    assert_eq!(
        render,
        "rows = [{'b': beta} for item in items]\nkeep = gamma"
    );
}

#[test]
fn test_seed_207_keeps_the_later_element_entries_standing() {
    let source = "rows = [{'a': alpha, 'b': beta, 'c': gamma} for item in items]\n";
    let (_, render) = applied_render(source, "seed-207");
    assert_eq!(render, "rows = [{'b': beta, 'c': gamma} for item in items]");
}

#[test]
fn test_seed_207_removes_the_first_entry_of_every_comprehension_element_dictionary() {
    let source = "rows = [{'a': alpha, 'b': beta} for item in items]\nothers = [{'x': one, 'y': two} for entry in entries]\n";
    let (_, render) = applied_render(source, "seed-207");
    assert_eq!(
        render,
        "rows = [{'b': beta} for item in items]\nothers = [{'y': two} for entry in entries]"
    );
}

#[test]
fn test_seed_207_leaves_a_single_entry_comprehension_element_dictionary_standing() {
    let source = "rows = [{'a': alpha} for item in items]\n";
    let (planted, render) = applied_render(source, "seed-207");
    assert!(
        !planted,
        "a one entry comprehension element dictionary does not plant"
    );
    assert_eq!(render, "rows = [{'a': alpha} for item in items]");
}

#[test]
fn test_seed_207_leaves_a_dictionary_standing_outside_a_comprehension_element() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-207");
    assert!(
        !planted,
        "a dictionary standing outside a comprehension element is not yielded"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_207_verdict_is_caught() {
    let target = target_with(
        FIRST_ELEMENT_DICT_MODULE,
        FIRST_ELEMENT_DICT_GATE,
        "caught207",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-207"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the fiftieth format.

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
fn test_version_forty_nine_pack_still_loads_the_two_hundred_and_four() {
    let dir = fresh_dir("pack49");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(204).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/49", entries));
    let loaded = load_seed_pack(&path).expect("the version forty nine pack loads");
    assert_eq!(loaded.len(), 204);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[207].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-208".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[207] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/50", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fifty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_nine_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(203).collect();
    entries.push(standing_entries()[204].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/49", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fifty_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(204).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/50", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "two hundred and four entries under the version fiftieth format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 208);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[207], "seed-207");
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
        written.contains("\"canaryfield-seeds/50\""),
        "the written pack carries the fiftieth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
