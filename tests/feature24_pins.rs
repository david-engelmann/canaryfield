//! Feature slice 24 pins, the registry grown a thirty second time.
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
        "canaryfield-feature24-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 220, "the registry stands at 148 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "every seed id stands unique");
    assert_eq!(SEEDS[219].id, "seed-219", "the last seed stands at 179");
}

// Seed-136, dropped unpacking target element.

const TARGET_MODULE: &str = "first, second = (alpha, beta)\nkeep = gamma\n";
const TARGET_GATE: &str = "#!/bin/sh\ngrep -q 'first, second = ' mod.py\n";

#[test]
fn test_seed_136_plants_on_an_unpacking_target() {
    let (planted, _) = applied_render(TARGET_MODULE, "seed-136");
    assert!(planted, "an unpacking target plants");
}

#[test]
fn test_seed_136_removes_the_first_element() {
    let (_, render) = applied_render(TARGET_MODULE, "seed-136");
    assert_eq!(render, "second, = (alpha, beta)\nkeep = gamma");
}

#[test]
fn test_seed_136_removes_the_first_element_of_every_unpacking_target() {
    let (_, render) = applied_render(
        "first, second = pair\nalpha, beta, gamma = triple\n",
        "seed-136",
    );
    assert_eq!(render, "second, = pair\nbeta, gamma = triple");
}

#[test]
fn test_seed_136_keeps_the_remaining_elements_standing() {
    let (_, render) = applied_render("one, two, three, four = quad\n", "seed-136");
    assert_eq!(render, "two, three, four = quad");
}

#[test]
fn test_seed_136_leaves_a_single_element_target_standing() {
    let (planted, render) = applied_render("first, = pair\n", "seed-136");
    assert!(
        !planted,
        "a target carrying a single element does not plant"
    );
    assert_eq!(render, "first = pair");
}

#[test]
fn test_seed_136_leaves_a_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (first, second)\n", "seed-136");
    assert!(!planted, "a value tuple is not an unpacking target");
    assert_eq!(render, "pair = (first, second)");
}

#[test]
fn test_seed_136_removes_the_first_element_of_a_target_standing_inside_a_target() {
    let (_, render) = applied_render("(first, second), third = pair\n", "seed-136");
    assert_eq!(render, "third, = pair");
}

#[test]
fn test_seed_136_verdict_is_caught() {
    let target = target_with(TARGET_MODULE, TARGET_GATE, "caught136");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-136"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-137, dropped last unpacking target element.

#[test]
fn test_seed_137_plants_on_an_unpacking_target() {
    let (planted, _) = applied_render(TARGET_MODULE, "seed-137");
    assert!(planted, "an unpacking target plants");
}

#[test]
fn test_seed_137_removes_the_last_element() {
    let (_, render) = applied_render(TARGET_MODULE, "seed-137");
    assert_eq!(render, "first, = (alpha, beta)\nkeep = gamma");
}

#[test]
fn test_seed_137_removes_the_last_element_of_every_unpacking_target() {
    let (_, render) = applied_render(
        "first, second = pair\nalpha, beta, gamma = triple\n",
        "seed-137",
    );
    assert_eq!(render, "first, = pair\nalpha, beta = triple");
}

#[test]
fn test_seed_137_keeps_the_earlier_elements_standing() {
    let (_, render) = applied_render("one, two, three, four = quad\n", "seed-137");
    assert_eq!(render, "one, two, three = quad");
}

#[test]
fn test_seed_137_leaves_a_single_element_target_standing() {
    let (planted, render) = applied_render("first, = pair\n", "seed-137");
    assert!(
        !planted,
        "a target carrying a single element does not plant"
    );
    assert_eq!(render, "first = pair");
}

#[test]
fn test_seed_137_leaves_a_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (first, second)\n", "seed-137");
    assert!(!planted, "a value tuple is not an unpacking target");
    assert_eq!(render, "pair = (first, second)");
}

#[test]
fn test_seed_137_verdict_is_caught() {
    let target = target_with(TARGET_MODULE, TARGET_GATE, "caught137");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-137"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-138, dropped keyword unpacking.

const UNPACK_MODULE: &str = "result = call(**values)\nkeep = gamma\n";
const UNPACK_GATE: &str = "#!/bin/sh\ngrep -q 'values' mod.py\n";

#[test]
fn test_seed_138_plants_on_a_call_unpacking() {
    let (planted, _) = applied_render(UNPACK_MODULE, "seed-138");
    assert!(planted, "a call unpacking plants");
}

#[test]
fn test_seed_138_removes_the_unpacking() {
    let (_, render) = applied_render(UNPACK_MODULE, "seed-138");
    assert_eq!(render, "result = call()\nkeep = gamma");
}

#[test]
fn test_seed_138_removes_the_unpacking_of_every_call() {
    let (_, render) = applied_render("one = call(**first)\ntwo = send(**second)\n", "seed-138");
    assert_eq!(render, "one = call()\ntwo = send()");
}

#[test]
fn test_seed_138_keeps_the_named_keyword_arguments_standing() {
    let (_, render) = applied_render("result = call(name=alpha, **values)\n", "seed-138");
    assert_eq!(render, "result = call(name=alpha)");
}

#[test]
fn test_seed_138_leaves_a_named_keyword_call_standing() {
    let (planted, render) = applied_render("result = call(name=alpha)\n", "seed-138");
    assert!(
        !planted,
        "a call carrying only named keywords does not plant"
    );
    assert_eq!(render, "result = call(name=alpha)");
}

#[test]
fn test_seed_138_verdict_is_caught() {
    let target = target_with(UNPACK_MODULE, UNPACK_GATE, "caught138");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-138"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-139, dropped last dictionary entry.

const DICT_MODULE: &str = "table = {'a': alpha, 'b': beta}\nkeep = gamma\n";
const DICT_GATE: &str = "#!/bin/sh\ngrep -q \"'b': beta\" mod.py\n";

#[test]
fn test_seed_139_plants_on_a_dictionary_display() {
    let (planted, _) = applied_render(DICT_MODULE, "seed-139");
    assert!(planted, "a dictionary display plants");
}

#[test]
fn test_seed_139_removes_the_last_entry() {
    let (_, render) = applied_render(DICT_MODULE, "seed-139");
    assert_eq!(render, "table = {'a': alpha}\nkeep = gamma");
}

#[test]
fn test_seed_139_removes_the_last_entry_of_every_dictionary_display() {
    let (_, render) = applied_render(
        "one = {'a': alpha, 'b': beta}\ntwo = {'x': one, 'y': two, 'z': three}\n",
        "seed-139",
    );
    assert_eq!(render, "one = {'a': alpha}\ntwo = {'x': one, 'y': two}");
}

#[test]
fn test_seed_139_keeps_the_earlier_entries_standing() {
    let (_, render) = applied_render("table = {'a': alpha, 'b': beta, 'c': gamma}\n", "seed-139");
    assert_eq!(render, "table = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_139_leaves_a_single_entry_display_standing() {
    let (planted, render) = applied_render("table = {'a': alpha}\n", "seed-139");
    assert!(!planted, "a display carrying a single entry does not plant");
    assert_eq!(render, "table = {'a': alpha}");
}

#[test]
fn test_seed_139_leaves_a_list_display_standing() {
    let (planted, render) = applied_render("items = [alpha, beta]\n", "seed-139");
    assert!(!planted, "a list display is not a dictionary display");
    assert_eq!(render, "items = [alpha, beta]");
}

#[test]
fn test_seed_139_verdict_is_caught() {
    let target = target_with(DICT_MODULE, DICT_GATE, "caught139");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-139"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The seed pack at one hundred and forty.

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
fn test_version_thirty_two_pack_still_loads_the_one_hundred_and_thirty_six() {
    let dir = fresh_dir("pack32");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(136).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/32", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty two pack loads");
    assert_eq!(loaded.len(), 136);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_three_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[139].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-143".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[139] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/33", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_two_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(135).collect();
    entries.push(standing_entries()[136].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/32", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_three_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(136).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/33", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and thirty six entries under the version thirty three format are refused"
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
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[219], "seed-219");
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
        "the written pack carries the thirty third format"
    );
    let _ = fs::remove_dir_all(&dir);
}
