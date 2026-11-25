//! Feature slice 28 pins, the registry grown a thirty sixth time.
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
        "canaryfield-feature28-{}-{tag}",
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
fn test_registry_has_one_hundred_and_fifty_six_unique_seeds() {
    assert_eq!(SEEDS.len(), 156, "the registry stands at 156 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 156, "every seed id stands unique");
    assert_eq!(SEEDS[155].id, "seed-155", "the last seed stands at 155");
}

// Seed-152, dropped first f-string element.

const FIRST_FSTRING_MODULE: &str = "label = f'{alpha} and {beta}'\nplain = f'{gamma}'\n";
const FIRST_FSTRING_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_152_plants_on_an_fstring_with_two_elements() {
    let (planted, _) = applied_render(FIRST_FSTRING_MODULE, "seed-152");
    assert!(planted, "an f-string carrying two elements plants");
}

#[test]
fn test_seed_152_removes_the_first_fstring_element() {
    let (_, render) = applied_render(FIRST_FSTRING_MODULE, "seed-152");
    assert_eq!(render, "label = f' and {beta}'\nplain = f'{gamma}'");
}

#[test]
fn test_seed_152_removes_a_leading_literal_element() {
    let (planted, render) = applied_render("label = f'head {alpha} tail'\n", "seed-152");
    assert!(planted);
    assert_eq!(render, "label = f'{alpha} tail'");
}

#[test]
fn test_seed_152_keeps_the_later_elements_standing() {
    let (_, render) = applied_render("label = f'{alpha} {beta} {gamma}'\n", "seed-152");
    assert_eq!(render, "label = f' {beta} {gamma}'");
}

#[test]
fn test_seed_152_leaves_a_single_element_fstring_standing() {
    let (planted, render) = applied_render("plain = f'{gamma}'\n", "seed-152");
    assert!(
        !planted,
        "an f-string carrying a single element does not plant"
    );
    assert_eq!(render, "plain = f'{gamma}'");
}

#[test]
fn test_seed_152_verdict_is_caught() {
    let target = target_with(FIRST_FSTRING_MODULE, FIRST_FSTRING_GATE, "caught152");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-152"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-153, dropped first return element.

const FIRST_RETURN_MODULE: &str = "def pair():\n    return (alpha, beta)\n";
const FIRST_RETURN_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_153_plants_on_a_tuple_return() {
    let (planted, _) = applied_render(FIRST_RETURN_MODULE, "seed-153");
    assert!(planted, "a return of a two element tuple plants");
}

#[test]
fn test_seed_153_removes_the_first_return_element() {
    let (_, render) = applied_render(FIRST_RETURN_MODULE, "seed-153");
    assert_eq!(render, "def pair():\n    return (beta,)");
}

#[test]
fn test_seed_153_removes_the_first_element_of_every_tuple_return() {
    let source = "def pair():\n    return (alpha, beta)\n\ndef duo():\n    return (one, two)\n";
    let (_, render) = applied_render(source, "seed-153");
    assert_eq!(
        render,
        "def pair():\n    return (beta,)\n\ndef duo():\n    return (two,)"
    );
}

#[test]
fn test_seed_153_keeps_the_later_return_elements_standing() {
    let (_, render) = applied_render("def trio():\n    return (alpha, beta, gamma)\n", "seed-153");
    assert_eq!(render, "def trio():\n    return (beta, gamma)");
}

#[test]
fn test_seed_153_leaves_a_single_element_tuple_return_standing() {
    let (planted, render) = applied_render("def single():\n    return (alpha,)\n", "seed-153");
    assert!(!planted, "a return of a one element tuple does not plant");
    assert_eq!(render, "def single():\n    return (alpha,)");
}

#[test]
fn test_seed_153_leaves_a_plain_return_standing() {
    let (planted, render) = applied_render("def plain():\n    return alpha\n", "seed-153");
    assert!(!planted, "a return of a plain value does not plant");
    assert_eq!(render, "def plain():\n    return alpha");
}

#[test]
fn test_seed_153_verdict_is_caught() {
    let target = target_with(FIRST_RETURN_MODULE, FIRST_RETURN_GATE, "caught153");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-153"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-154, dropped first set element.

const FIRST_SET_MODULE: &str = "seen = {alpha, beta}\nkeep = gamma\n";
const FIRST_SET_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_154_plants_on_a_set_display_with_two_elements() {
    let (planted, _) = applied_render(FIRST_SET_MODULE, "seed-154");
    assert!(planted, "a set display carrying two elements plants");
}

#[test]
fn test_seed_154_removes_the_first_set_element() {
    let (_, render) = applied_render(FIRST_SET_MODULE, "seed-154");
    assert_eq!(render, "seen = {beta}\nkeep = gamma");
}

#[test]
fn test_seed_154_removes_the_first_element_of_every_set_display() {
    let source = "first = {alpha, beta}\nsecond = {one, two}\n";
    let (_, render) = applied_render(source, "seed-154");
    assert_eq!(render, "first = {beta}\nsecond = {two}");
}

#[test]
fn test_seed_154_keeps_the_later_set_elements_standing() {
    let (_, render) = applied_render("seen = {alpha, beta, gamma}\n", "seed-154");
    assert_eq!(render, "seen = {beta, gamma}");
}

#[test]
fn test_seed_154_leaves_a_single_element_set_standing() {
    let (planted, render) = applied_render("seen = {alpha}\n", "seed-154");
    assert!(
        !planted,
        "a set display carrying a single element does not plant"
    );
    assert_eq!(render, "seen = {alpha}");
}

#[test]
fn test_seed_154_leaves_a_list_display_standing() {
    let (planted, render) = applied_render("items = [alpha, beta]\n", "seed-154");
    assert!(!planted, "a list display does not plant under the set seed");
    assert_eq!(render, "items = [alpha, beta]");
}

#[test]
fn test_seed_154_verdict_is_caught() {
    let target = target_with(FIRST_SET_MODULE, FIRST_SET_GATE, "caught154");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-154"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-155, dropped last decorator.

const DECORATOR_MODULE: &str = "@alpha\n@beta\ndef wrapped():\n    return 1\n";
const DECORATOR_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_155_plants_on_a_definition_with_two_decorators() {
    let (planted, _) = applied_render(DECORATOR_MODULE, "seed-155");
    assert!(planted, "a definition carrying two decorators plants");
}

#[test]
fn test_seed_155_removes_the_last_decorator() {
    let (_, render) = applied_render(DECORATOR_MODULE, "seed-155");
    assert_eq!(render, "@alpha\ndef wrapped():\n    return 1");
}

#[test]
fn test_seed_155_removes_the_last_decorator_of_every_definition() {
    let source =
        "@alpha\n@beta\ndef first():\n    return 1\n\n@one\n@two\ndef second():\n    return 2\n";
    let (_, render) = applied_render(source, "seed-155");
    assert_eq!(
        render,
        "@alpha\ndef first():\n    return 1\n\n@one\ndef second():\n    return 2"
    );
}

#[test]
fn test_seed_155_removes_the_last_decorator_of_a_class() {
    let (_, render) = applied_render("@one\n@two\nclass Box:\n    pass\n", "seed-155");
    assert_eq!(render, "@one\nclass Box:\n    pass");
}

#[test]
fn test_seed_155_keeps_the_earlier_decorators_standing() {
    let source = "@alpha\n@beta\n@gamma\ndef wrapped():\n    return 1\n";
    let (_, render) = applied_render(source, "seed-155");
    assert_eq!(render, "@alpha\n@beta\ndef wrapped():\n    return 1");
}

#[test]
fn test_seed_155_leaves_a_single_decorator_standing() {
    let (planted, render) = applied_render("@alpha\ndef wrapped():\n    return 1\n", "seed-155");
    assert!(
        !planted,
        "a definition carrying one decorator does not plant"
    );
    assert_eq!(render, "@alpha\ndef wrapped():\n    return 1");
}

#[test]
fn test_seed_155_verdict_is_caught() {
    let target = target_with(DECORATOR_MODULE, DECORATOR_GATE, "caught155");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-155"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the thirty seventh format.

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
fn test_version_thirty_six_pack_still_loads_the_one_hundred_and_fifty_two() {
    let dir = fresh_dir("pack36");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(152).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/36", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty six pack loads");
    assert_eq!(loaded.len(), 152);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_seven_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[155].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-156".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[155] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/37", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_six_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(151).collect();
    entries.push(standing_entries()[152].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/36", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty six pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_seven_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(152).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/37", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and fifty two entries under the version thirty seven format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 156);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[155], "seed-155");
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
        written.contains("\"canaryfield-seeds/37\""),
        "the written pack carries the thirty seventh format"
    );
    let _ = fs::remove_dir_all(&dir);
}
