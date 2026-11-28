//! Feature slice 27 pins, the registry grown a thirty fifth time.
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
        "canaryfield-feature27-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 224, "the registry stands at 152 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 224, "every seed id stands unique");
    assert_eq!(SEEDS[223].id, "seed-223", "the last seed stands at 179");
}

// Seed-148, dropped last f-string element.

const FSTRING_MODULE: &str = "label = f'{alpha} and {beta}'\nplain = f'{gamma}'\n";
const FSTRING_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_148_plants_on_an_fstring_with_two_elements() {
    let (planted, _) = applied_render(FSTRING_MODULE, "seed-148");
    assert!(planted, "an f-string carrying two elements plants");
}

#[test]
fn test_seed_148_removes_the_last_fstring_element() {
    let (_, render) = applied_render(FSTRING_MODULE, "seed-148");
    assert_eq!(render, "label = f'{alpha} and '\nplain = f'{gamma}'");
}

#[test]
fn test_seed_148_removes_a_trailing_literal_element() {
    let (planted, render) = applied_render("label = f'head {alpha} tail'\n", "seed-148");
    assert!(planted);
    assert_eq!(render, "label = f'head {alpha}'");
}

#[test]
fn test_seed_148_keeps_the_earlier_elements_standing() {
    let (_, render) = applied_render("label = f'{alpha} {beta} {gamma}'\n", "seed-148");
    assert_eq!(render, "label = f'{alpha} {beta} '");
}

#[test]
fn test_seed_148_leaves_a_single_element_fstring_standing() {
    let (planted, render) = applied_render("plain = f'{gamma}'\n", "seed-148");
    assert!(
        !planted,
        "an f-string carrying a single element does not plant"
    );
    assert_eq!(render, "plain = f'{gamma}'");
}

#[test]
fn test_seed_148_verdict_is_caught() {
    let target = target_with(FSTRING_MODULE, FSTRING_GATE, "caught148");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-148"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-149, dropped last return element.

const RETURN_MODULE: &str = "def pair():\n    return (alpha, beta)\n";
const RETURN_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_149_plants_on_a_tuple_return() {
    let (planted, _) = applied_render(RETURN_MODULE, "seed-149");
    assert!(planted, "a return of a two element tuple plants");
}

#[test]
fn test_seed_149_removes_the_last_return_element() {
    let (_, render) = applied_render(RETURN_MODULE, "seed-149");
    assert_eq!(render, "def pair():\n    return (alpha,)");
}

#[test]
fn test_seed_149_removes_the_last_element_of_every_tuple_return() {
    let source = "def pair():\n    return (alpha, beta)\n\ndef duo():\n    return (one, two)\n";
    let (_, render) = applied_render(source, "seed-149");
    assert_eq!(
        render,
        "def pair():\n    return (alpha,)\n\ndef duo():\n    return (one,)"
    );
}

#[test]
fn test_seed_149_keeps_the_earlier_return_elements_standing() {
    let (_, render) = applied_render("def trio():\n    return (alpha, beta, gamma)\n", "seed-149");
    assert_eq!(render, "def trio():\n    return (alpha, beta)");
}

#[test]
fn test_seed_149_leaves_a_single_element_tuple_return_standing() {
    let (planted, render) = applied_render("def single():\n    return (alpha,)\n", "seed-149");
    assert!(!planted, "a return of a one element tuple does not plant");
    assert_eq!(render, "def single():\n    return (alpha,)");
}

#[test]
fn test_seed_149_leaves_a_plain_return_standing() {
    let (planted, render) = applied_render("def plain():\n    return alpha\n", "seed-149");
    assert!(!planted, "a return of a plain value does not plant");
    assert_eq!(render, "def plain():\n    return alpha");
}

#[test]
fn test_seed_149_verdict_is_caught() {
    let target = target_with(RETURN_MODULE, RETURN_GATE, "caught149");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-149"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-150, dropped first subscript index element.

const FIRST_INDEX_MODULE: &str = "item = grid[alpha, beta]\nkeep = gamma\n";
const FIRST_INDEX_GATE: &str = "#!/bin/sh\ngrep -q 'alpha, beta' mod.py\n";

#[test]
fn test_seed_150_plants_on_a_tuple_index_subscript() {
    let (planted, _) = applied_render(FIRST_INDEX_MODULE, "seed-150");
    assert!(planted, "a subscript with a two element tuple index plants");
}

#[test]
fn test_seed_150_removes_the_first_index_element() {
    let (_, render) = applied_render(FIRST_INDEX_MODULE, "seed-150");
    assert_eq!(render, "item = grid[beta,]\nkeep = gamma");
}

#[test]
fn test_seed_150_removes_the_first_element_of_every_tuple_index() {
    let source = "first = grid[alpha, beta]\nsecond = table[one, two, three]\n";
    let (_, render) = applied_render(source, "seed-150");
    assert_eq!(render, "first = grid[beta,]\nsecond = table[two, three]");
}

#[test]
fn test_seed_150_keeps_the_later_index_elements_standing() {
    let (_, render) = applied_render("item = table[one, two, three]\n", "seed-150");
    assert!(
        render.contains("table[two, three]"),
        "the later elements stand: {render}"
    );
}

#[test]
fn test_seed_150_leaves_a_single_index_subscript_standing() {
    let (planted, render) = applied_render("item = grid[alpha]\n", "seed-150");
    assert!(!planted, "a subscript with a single index does not plant");
    assert_eq!(render, "item = grid[alpha]");
}

#[test]
fn test_seed_150_leaves_a_slice_subscript_standing() {
    let (planted, render) = applied_render("item = grid[alpha:beta]\n", "seed-150");
    assert!(!planted, "a subscript with a slice index does not plant");
    assert_eq!(render, "item = grid[alpha:beta]");
}

#[test]
fn test_seed_150_verdict_is_caught() {
    let target = target_with(FIRST_INDEX_MODULE, FIRST_INDEX_GATE, "caught150");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-150"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-151, dropped last set element.

const SET_MODULE: &str = "seen = {alpha, beta}\nkeep = gamma\n";
const SET_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_151_plants_on_a_set_display_with_two_elements() {
    let (planted, _) = applied_render(SET_MODULE, "seed-151");
    assert!(planted, "a set display carrying two elements plants");
}

#[test]
fn test_seed_151_removes_the_last_set_element() {
    let (_, render) = applied_render(SET_MODULE, "seed-151");
    assert_eq!(render, "seen = {alpha}\nkeep = gamma");
}

#[test]
fn test_seed_151_removes_the_last_element_of_every_set_display() {
    let source = "first = {alpha, beta}\nsecond = {one, two}\n";
    let (_, render) = applied_render(source, "seed-151");
    assert_eq!(render, "first = {alpha}\nsecond = {one}");
}

#[test]
fn test_seed_151_keeps_the_earlier_set_elements_standing() {
    let (_, render) = applied_render("seen = {alpha, beta, gamma}\n", "seed-151");
    assert_eq!(render, "seen = {alpha, beta}");
}

#[test]
fn test_seed_151_leaves_a_single_element_set_standing() {
    let (planted, render) = applied_render("seen = {alpha}\n", "seed-151");
    assert!(
        !planted,
        "a set display carrying a single element does not plant"
    );
    assert_eq!(render, "seen = {alpha}");
}

#[test]
fn test_seed_151_leaves_a_list_display_standing() {
    let (planted, render) = applied_render("items = [alpha, beta]\n", "seed-151");
    assert!(!planted, "a list display does not plant under the set seed");
    assert_eq!(render, "items = [alpha, beta]");
}

#[test]
fn test_seed_151_verdict_is_caught() {
    let target = target_with(SET_MODULE, SET_GATE, "caught151");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-151"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the thirty sixth format.
// The pack at the thirty fifth format.

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
fn test_version_thirty_five_pack_still_loads_the_one_hundred_and_forty_eight() {
    let dir = fresh_dir("pack35");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(148).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/35", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty five pack loads");
    assert_eq!(loaded.len(), 148);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_six_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[151].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-152".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[151] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/36", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty six pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_five_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(147).collect();
    entries.push(standing_entries()[148].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/35", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty five pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_six_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(148).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/36", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and forty eight entries under the version thirty six format are refused"
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
        "the written pack carries the thirty sixth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
