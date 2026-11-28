//! Feature slice 40 pins, the registry grown a forty eighth time.
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
        "canaryfield-feature40-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 224, "the registry stands at 224 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 224, "every seed id stands unique");
    assert_eq!(SEEDS[223].id, "seed-223", "the last seed stands at 203");
}

// Seed-200, dropped last assignment value dict entry.

const LAST_ASSIGN_DICT_MODULE: &str = "table = {'a': alpha, 'b': beta}\nkeep = gamma\n";
const LAST_ASSIGN_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'b': beta\" mod.py\n";

#[test]
fn test_seed_200_plants_on_an_assignment_value_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(LAST_ASSIGN_DICT_MODULE, "seed-200");
    assert!(planted, "an assignment value dictionary plants");
}

#[test]
fn test_seed_200_removes_the_last_assignment_value_dict_entry() {
    let (_, render) = applied_render(LAST_ASSIGN_DICT_MODULE, "seed-200");
    assert_eq!(render, "table = {'a': alpha}\nkeep = gamma");
}

#[test]
fn test_seed_200_keeps_the_earlier_value_entries_standing() {
    let source = "table = {'a': alpha, 'b': beta, 'c': gamma}\n";
    let (_, render) = applied_render(source, "seed-200");
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_200_removes_the_last_entry_of_every_assignment_value_dictionary() {
    let source = "table = {'a': alpha, 'b': beta}\nothers = {'x': one, 'y': two}\n";
    let (_, render) = applied_render(source, "seed-200");
    assert_eq!(render, "table = {'a': alpha}\nothers = {'x': one}");
}

#[test]
fn test_seed_200_leaves_a_single_entry_assignment_value_dictionary_standing() {
    let source = "table = {'a': alpha}\n";
    let (planted, render) = applied_render(source, "seed-200");
    assert!(
        !planted,
        "a one entry assignment value dictionary does not plant"
    );
    assert_eq!(render, "table = {'a': alpha}");
}

#[test]
fn test_seed_200_leaves_a_dictionary_standing_outside_an_assignment_value() {
    let (planted, render) = applied_render("pick({'a': alpha, 'b': beta})\n", "seed-200");
    assert!(
        !planted,
        "a dictionary standing outside an assignment value is not handed to a target"
    );
    assert_eq!(render, "pick({'a': alpha, 'b': beta})");
}

#[test]
fn test_seed_200_verdict_is_caught() {
    let target = target_with(LAST_ASSIGN_DICT_MODULE, LAST_ASSIGN_DICT_GATE, "caught200");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-200"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-201, dropped first assignment value dict entry.

const FIRST_ASSIGN_DICT_MODULE: &str = "table = {'a': alpha, 'b': beta}\nkeep = gamma\n";
const FIRST_ASSIGN_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'a': alpha\" mod.py\n";

#[test]
fn test_seed_201_plants_on_an_assignment_value_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(FIRST_ASSIGN_DICT_MODULE, "seed-201");
    assert!(planted, "an assignment value dictionary plants");
}

#[test]
fn test_seed_201_removes_the_first_assignment_value_dict_entry() {
    let (_, render) = applied_render(FIRST_ASSIGN_DICT_MODULE, "seed-201");
    assert_eq!(render, "table = {'b': beta}\nkeep = gamma");
}

#[test]
fn test_seed_201_keeps_the_later_value_entries_standing() {
    let source = "table = {'a': alpha, 'b': beta, 'c': gamma}\n";
    let (_, render) = applied_render(source, "seed-201");
    assert_eq!(render, "table = {'b': beta, 'c': gamma}");
}

#[test]
fn test_seed_201_removes_the_first_entry_of_every_assignment_value_dictionary() {
    let source = "table = {'a': alpha, 'b': beta}\nothers = {'x': one, 'y': two}\n";
    let (_, render) = applied_render(source, "seed-201");
    assert_eq!(render, "table = {'b': beta}\nothers = {'y': two}");
}

#[test]
fn test_seed_201_leaves_a_single_entry_assignment_value_dictionary_standing() {
    let source = "table = {'a': alpha}\n";
    let (planted, render) = applied_render(source, "seed-201");
    assert!(
        !planted,
        "a one entry assignment value dictionary does not plant"
    );
    assert_eq!(render, "table = {'a': alpha}");
}

#[test]
fn test_seed_201_leaves_a_dictionary_standing_outside_an_assignment_value() {
    let (planted, render) = applied_render("pick({'a': alpha, 'b': beta})\n", "seed-201");
    assert!(
        !planted,
        "a dictionary standing outside an assignment value is not handed to a target"
    );
    assert_eq!(render, "pick({'a': alpha, 'b': beta})");
}

#[test]
fn test_seed_201_verdict_is_caught() {
    let target = target_with(
        FIRST_ASSIGN_DICT_MODULE,
        FIRST_ASSIGN_DICT_GATE,
        "caught201",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-201"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-202, dropped last call argument dict entry.

const LAST_ARG_DICT_MODULE: &str = "result = pick({'a': alpha, 'b': beta})\nkeep = gamma\n";
const LAST_ARG_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'b': beta\" mod.py\n";

#[test]
fn test_seed_202_plants_on_a_call_argument_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(LAST_ARG_DICT_MODULE, "seed-202");
    assert!(planted, "a call argument dictionary plants");
}

#[test]
fn test_seed_202_removes_the_last_call_argument_dict_entry() {
    let (_, render) = applied_render(LAST_ARG_DICT_MODULE, "seed-202");
    assert_eq!(render, "result = pick({'a': alpha})\nkeep = gamma");
}

#[test]
fn test_seed_202_keeps_the_earlier_argument_entries_standing() {
    let source = "result = pick({'a': alpha, 'b': beta, 'c': gamma})\n";
    let (_, render) = applied_render(source, "seed-202");
    assert_eq!(render, "result = pick({'a': alpha, 'b': beta})");
}

#[test]
fn test_seed_202_removes_the_last_entry_of_every_call_argument_dictionary() {
    let source = "result = pick({'a': alpha, 'b': beta})\nothers = choose({'x': one, 'y': two})\n";
    let (_, render) = applied_render(source, "seed-202");
    assert_eq!(
        render,
        "result = pick({'a': alpha})\nothers = choose({'x': one})"
    );
}

#[test]
fn test_seed_202_leaves_a_single_entry_call_argument_dictionary_standing() {
    let source = "result = pick({'a': alpha})\n";
    let (planted, render) = applied_render(source, "seed-202");
    assert!(
        !planted,
        "a one entry call argument dictionary does not plant"
    );
    assert_eq!(render, "result = pick({'a': alpha})");
}

#[test]
fn test_seed_202_leaves_a_dictionary_standing_outside_a_call_argument() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-202");
    assert!(
        !planted,
        "a dictionary standing outside a call argument is not passed to a call"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_202_verdict_is_caught() {
    let target = target_with(LAST_ARG_DICT_MODULE, LAST_ARG_DICT_GATE, "caught202");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-202"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-203, dropped first call argument dict entry.

const FIRST_ARG_DICT_MODULE: &str = "result = pick({'a': alpha, 'b': beta})\nkeep = gamma\n";
const FIRST_ARG_DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'a': alpha\" mod.py\n";

#[test]
fn test_seed_203_plants_on_a_call_argument_that_is_a_two_entry_dictionary() {
    let (planted, _) = applied_render(FIRST_ARG_DICT_MODULE, "seed-203");
    assert!(planted, "a call argument dictionary plants");
}

#[test]
fn test_seed_203_removes_the_first_call_argument_dict_entry() {
    let (_, render) = applied_render(FIRST_ARG_DICT_MODULE, "seed-203");
    assert_eq!(render, "result = pick({'b': beta})\nkeep = gamma");
}

#[test]
fn test_seed_203_keeps_the_later_argument_entries_standing() {
    let source = "result = pick({'a': alpha, 'b': beta, 'c': gamma})\n";
    let (_, render) = applied_render(source, "seed-203");
    assert_eq!(render, "result = pick({'b': beta, 'c': gamma})");
}

#[test]
fn test_seed_203_removes_the_first_entry_of_every_call_argument_dictionary() {
    let source = "result = pick({'a': alpha, 'b': beta})\nothers = choose({'x': one, 'y': two})\n";
    let (_, render) = applied_render(source, "seed-203");
    assert_eq!(
        render,
        "result = pick({'b': beta})\nothers = choose({'y': two})"
    );
}

#[test]
fn test_seed_203_leaves_a_single_entry_call_argument_dictionary_standing() {
    let source = "result = pick({'a': alpha})\n";
    let (planted, render) = applied_render(source, "seed-203");
    assert!(
        !planted,
        "a one entry call argument dictionary does not plant"
    );
    assert_eq!(render, "result = pick({'a': alpha})");
}

#[test]
fn test_seed_203_leaves_a_dictionary_standing_outside_a_call_argument() {
    let (planted, render) = applied_render("table = {'a': alpha, 'b': beta}\n", "seed-203");
    assert!(
        !planted,
        "a dictionary standing outside a call argument is not passed to a call"
    );
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_203_verdict_is_caught() {
    let target = target_with(FIRST_ARG_DICT_MODULE, FIRST_ARG_DICT_GATE, "caught203");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-203"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty ninth format.

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
fn test_version_forty_eight_pack_still_loads_the_two_hundred() {
    let dir = fresh_dir("pack48");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(200).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/48", entries));
    let loaded = load_seed_pack(&path).expect("the version forty eight pack loads");
    assert_eq!(loaded.len(), 200);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_nine_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[203].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-204".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[203] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/49", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_eight_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(199).collect();
    entries.push(standing_entries()[200].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/48", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_nine_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(200).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/49", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "two hundred entries under the version forty ninth format are refused"
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
        "the written pack carries the forty ninth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
