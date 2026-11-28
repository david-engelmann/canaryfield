//! Feature slice 45 pins, the registry grown a fifty third time.
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
        "canaryfield-feature45-{}-{tag}",
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
    assert_eq!(SEEDS[223].id, "seed-223", "the last seed stands at 223");
}

// Seed-220, dropped later slice bound.

const BOTH_BOUNDS_MODULE: &str = "row = grid[alpha:beta]\nkeep = gamma\n";
const BOTH_BOUNDS_GATE: &str = "#!/bin/sh\ngrep -q \"grid\\[alpha:beta\\]\" mod.py\n";

#[test]
fn test_seed_220_plants_on_a_slice_carrying_both_bounds() {
    let (planted, _) = applied_render(BOTH_BOUNDS_MODULE, "seed-220");
    assert!(planted, "a slice carrying both bounds plants");
}

#[test]
fn test_seed_220_clears_the_later_bound() {
    let (_, render) = applied_render(BOTH_BOUNDS_MODULE, "seed-220");
    assert_eq!(render, "row = grid[alpha:]\nkeep = gamma");
}

#[test]
fn test_seed_220_clears_the_later_bound_of_every_slice_in_a_text() {
    let source = "first = grid[alpha:beta]\nsecond = grid[one:two]\n";
    let (_, render) = applied_render(source, "seed-220");
    assert_eq!(render, "first = grid[alpha:]\nsecond = grid[one:]");
}

#[test]
fn test_seed_220_clears_the_later_bound_of_a_stepped_slice() {
    let source = "row = grid[alpha:beta:stride]\n";
    let (_, render) = applied_render(source, "seed-220");
    assert_eq!(render, "row = grid[alpha::stride]");
}

#[test]
fn test_seed_220_leaves_a_slice_carrying_one_bound_standing() {
    let (planted, render) = applied_render("row = grid[alpha:]\n", "seed-220");
    assert!(!planted, "a slice carrying one bound does not plant");
    assert_eq!(render, "row = grid[alpha:]");
}

#[test]
fn test_seed_220_leaves_an_upper_only_slice_standing() {
    let (planted, render) = applied_render("row = grid[:beta]\n", "seed-220");
    assert!(!planted, "a slice carrying one bound does not plant");
    assert_eq!(render, "row = grid[:beta]");
}

#[test]
fn test_seed_220_verdict_is_caught() {
    let target = target_with(BOTH_BOUNDS_MODULE, BOTH_BOUNDS_GATE, "caught220");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-220"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-221, dropped earlier slice bound.

#[test]
fn test_seed_221_plants_on_a_slice_carrying_both_bounds() {
    let (planted, _) = applied_render(BOTH_BOUNDS_MODULE, "seed-221");
    assert!(planted, "a slice carrying both bounds plants");
}

#[test]
fn test_seed_221_clears_the_earlier_bound() {
    let (_, render) = applied_render(BOTH_BOUNDS_MODULE, "seed-221");
    assert_eq!(render, "row = grid[:beta]\nkeep = gamma");
}

#[test]
fn test_seed_221_clears_the_earlier_bound_of_every_slice_in_a_text() {
    let source = "first = grid[alpha:beta]\nsecond = grid[one:two]\n";
    let (_, render) = applied_render(source, "seed-221");
    assert_eq!(render, "first = grid[:beta]\nsecond = grid[:two]");
}

#[test]
fn test_seed_221_clears_the_earlier_bound_of_a_stepped_slice() {
    let source = "row = grid[alpha:beta:stride]\n";
    let (_, render) = applied_render(source, "seed-221");
    assert_eq!(render, "row = grid[:beta:stride]");
}

#[test]
fn test_seed_221_leaves_a_slice_carrying_one_bound_standing() {
    let (planted, render) = applied_render("row = grid[:beta]\n", "seed-221");
    assert!(!planted, "a slice carrying one bound does not plant");
    assert_eq!(render, "row = grid[:beta]");
}

#[test]
fn test_seed_221_leaves_a_lower_only_slice_standing() {
    let (planted, render) = applied_render("row = grid[alpha:]\n", "seed-221");
    assert!(!planted, "a slice carrying one bound does not plant");
    assert_eq!(render, "row = grid[alpha:]");
}

#[test]
fn test_seed_221_verdict_is_caught() {
    let target = target_with(BOTH_BOUNDS_MODULE, BOTH_BOUNDS_GATE, "caught221");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-221"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-222, dropped slice bound pair.

#[test]
fn test_seed_222_plants_on_a_slice_carrying_both_bounds() {
    let (planted, _) = applied_render(BOTH_BOUNDS_MODULE, "seed-222");
    assert!(planted, "a slice carrying both bounds plants");
}

#[test]
fn test_seed_222_clears_both_bounds() {
    let (_, render) = applied_render(BOTH_BOUNDS_MODULE, "seed-222");
    assert_eq!(render, "row = grid[:]\nkeep = gamma");
}

#[test]
fn test_seed_222_clears_both_bounds_of_every_slice_in_a_text() {
    let source = "first = grid[alpha:beta]\nsecond = grid[one:two]\n";
    let (_, render) = applied_render(source, "seed-222");
    assert_eq!(render, "first = grid[:]\nsecond = grid[:]");
}

#[test]
fn test_seed_222_clears_both_bounds_of_a_stepped_slice() {
    let source = "row = grid[alpha:beta:stride]\n";
    let (_, render) = applied_render(source, "seed-222");
    assert_eq!(render, "row = grid[::stride]");
}

#[test]
fn test_seed_222_leaves_a_slice_carrying_one_bound_standing() {
    let (planted, render) = applied_render("row = grid[alpha:]\n", "seed-222");
    assert!(!planted, "a slice carrying one bound does not plant");
    assert_eq!(render, "row = grid[alpha:]");
}

#[test]
fn test_seed_222_leaves_a_boundless_slice_standing() {
    let (planted, render) = applied_render("row = grid[:]\n", "seed-222");
    assert!(!planted, "a slice carrying no bound does not plant");
    assert_eq!(render, "row = grid[:]");
}

#[test]
fn test_seed_222_verdict_is_caught() {
    let target = target_with(BOTH_BOUNDS_MODULE, BOTH_BOUNDS_GATE, "caught222");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-222"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-223, dropped store subscript.

const STORE_SUBSCRIPT_MODULE: &str = "grid[alpha] = beta\nkeep = gamma\n";
const STORE_SUBSCRIPT_GATE: &str = "#!/bin/sh\ngrep -q \"grid\\[alpha\\] = beta\" mod.py\n";

#[test]
fn test_seed_223_plants_on_a_store_subscript() {
    let (planted, _) = applied_render(STORE_SUBSCRIPT_MODULE, "seed-223");
    assert!(planted, "a store subscript plants");
}

#[test]
fn test_seed_223_replaces_the_target_with_the_collection() {
    let (_, render) = applied_render(STORE_SUBSCRIPT_MODULE, "seed-223");
    assert_eq!(render, "grid = beta\nkeep = gamma");
}

#[test]
fn test_seed_223_replaces_a_target_inside_a_tuple_target() {
    let source = "(grid[alpha], other) = pair\n";
    let (_, render) = applied_render(source, "seed-223");
    assert_eq!(render, "(grid, other) = pair");
}

#[test]
fn test_seed_223_replaces_an_augmented_assignment_target() {
    let source = "grid[alpha] += beta\n";
    let (_, render) = applied_render(source, "seed-223");
    assert_eq!(render, "grid += beta");
}

#[test]
fn test_seed_223_leaves_a_load_subscript_standing() {
    let (planted, render) = applied_render("row = grid[alpha]\n", "seed-223");
    assert!(!planted, "a load subscript does not plant");
    assert_eq!(render, "row = grid[alpha]");
}

#[test]
fn test_seed_223_leaves_a_delete_subscript_standing() {
    let (planted, render) = applied_render("del grid[alpha]\n", "seed-223");
    assert!(!planted, "a delete subscript does not plant");
    assert_eq!(render, "del grid[alpha]");
}

#[test]
fn test_seed_223_verdict_is_caught() {
    let target = target_with(STORE_SUBSCRIPT_MODULE, STORE_SUBSCRIPT_GATE, "caught223");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-223"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the fifty fourth format.

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
fn test_version_fifty_three_pack_still_loads_the_two_hundred_and_twenty() {
    let dir = fresh_dir("pack53");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(220).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/53", entries));
    let loaded = load_seed_pack(&path).expect("the version fifty three pack loads");
    assert_eq!(loaded.len(), 220);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_four_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[223].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-224".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[223] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/54", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version fifty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_fifty_three_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(219).collect();
    entries.push(standing_entries()[220].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/53", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version fifty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_fifty_four_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(220).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/54", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "two hundred and twenty entries under the version fifty fourth format are refused"
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
        "the written pack carries the fifty fourth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
