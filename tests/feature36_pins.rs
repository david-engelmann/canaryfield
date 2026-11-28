//! Feature slice 36 pins, the registry grown a forty fourth time.
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
        "canaryfield-feature36-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 224, "the registry stands at 188 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 224, "every seed id stands unique");
    assert_eq!(SEEDS[223].id, "seed-223", "the last seed stands at 191");
}

// Seed-184, dropped last list argument element.

const LAST_ARG_MODULE: &str = "values = use([alpha, beta])\nkeep = gamma\n";
const LAST_ARG_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_184_plants_on_a_call_argument_that_is_a_two_element_list() {
    let (planted, _) = applied_render(LAST_ARG_MODULE, "seed-184");
    assert!(planted, "a call argument that is a two element list plants");
}

#[test]
fn test_seed_184_removes_the_last_list_argument_element() {
    let (_, render) = applied_render(LAST_ARG_MODULE, "seed-184");
    assert_eq!(render, "values = use([alpha])\nkeep = gamma");
}

#[test]
fn test_seed_184_keeps_the_earlier_argument_elements_standing() {
    let source = "values = use([alpha, beta, gamma])\n";
    let (_, render) = applied_render(source, "seed-184");
    assert_eq!(render, "values = use([alpha, beta])");
}

#[test]
fn test_seed_184_removes_the_last_element_of_every_list_argument() {
    let source = "first = use([alpha, beta])\nsecond = send([one, two])\n";
    let (_, render) = applied_render(source, "seed-184");
    assert_eq!(render, "first = use([alpha])\nsecond = send([one])");
}

#[test]
fn test_seed_184_leaves_a_single_element_list_argument_standing() {
    let source = "values = use([alpha])\n";
    let (planted, render) = applied_render(source, "seed-184");
    assert!(!planted, "a one element list argument does not plant");
    assert_eq!(render, "values = use([alpha])");
}

#[test]
fn test_seed_184_leaves_a_list_standing_outside_a_call_argument() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-184");
    assert!(
        !planted,
        "a list standing outside a call's arguments is not an argument"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_184_verdict_is_caught() {
    let target = target_with(LAST_ARG_MODULE, LAST_ARG_GATE, "caught184");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-184"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-185, dropped first list argument element.

const FIRST_ARG_MODULE: &str = "values = use([alpha, beta])\nkeep = gamma\n";
const FIRST_ARG_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_185_plants_on_a_call_argument_that_is_a_two_element_list() {
    let (planted, _) = applied_render(FIRST_ARG_MODULE, "seed-185");
    assert!(planted, "a call argument that is a two element list plants");
}

#[test]
fn test_seed_185_removes_the_first_list_argument_element() {
    let (_, render) = applied_render(FIRST_ARG_MODULE, "seed-185");
    assert_eq!(render, "values = use([beta])\nkeep = gamma");
}

#[test]
fn test_seed_185_keeps_the_later_argument_elements_standing() {
    let source = "values = use([alpha, beta, gamma])\n";
    let (_, render) = applied_render(source, "seed-185");
    assert_eq!(render, "values = use([beta, gamma])");
}

#[test]
fn test_seed_185_removes_the_first_element_of_every_list_argument() {
    let source = "first = use([alpha, beta])\nsecond = send([one, two])\n";
    let (_, render) = applied_render(source, "seed-185");
    assert_eq!(render, "first = use([beta])\nsecond = send([two])");
}

#[test]
fn test_seed_185_leaves_a_single_element_list_argument_standing() {
    let source = "values = use([alpha])\n";
    let (planted, render) = applied_render(source, "seed-185");
    assert!(!planted, "a one element list argument does not plant");
    assert_eq!(render, "values = use([alpha])");
}

#[test]
fn test_seed_185_leaves_a_list_standing_outside_a_call_argument() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-185");
    assert!(
        !planted,
        "a list standing outside a call's arguments is not an argument"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_185_verdict_is_caught() {
    let target = target_with(FIRST_ARG_MODULE, FIRST_ARG_GATE, "caught185");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-185"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-186, dropped last comparison operand list element.

const LAST_OPERAND_MODULE: &str = "check = [alpha, beta] == target\nkeep = gamma\n";
const LAST_OPERAND_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_186_plants_on_a_comparison_operand_that_is_a_two_element_list() {
    let (planted, _) = applied_render(LAST_OPERAND_MODULE, "seed-186");
    assert!(
        planted,
        "a comparison operand that is a two element list plants"
    );
}

#[test]
fn test_seed_186_removes_the_last_comparison_operand_list_element() {
    let (_, render) = applied_render(LAST_OPERAND_MODULE, "seed-186");
    assert_eq!(render, "check = [alpha] == target\nkeep = gamma");
}

#[test]
fn test_seed_186_keeps_the_earlier_operand_elements_standing() {
    let source = "check = [alpha, beta, gamma] == target\n";
    let (_, render) = applied_render(source, "seed-186");
    assert_eq!(render, "check = [alpha, beta] == target");
}

#[test]
fn test_seed_186_removes_the_last_element_of_every_operand_list() {
    let source = "check = [alpha, beta] == [one, two]\n";
    let (_, render) = applied_render(source, "seed-186");
    assert_eq!(render, "check = [alpha] == [one]");
}

#[test]
fn test_seed_186_leaves_a_single_element_operand_list_standing() {
    let source = "check = [alpha] == target\n";
    let (planted, render) = applied_render(source, "seed-186");
    assert!(!planted, "a one element operand list does not plant");
    assert_eq!(render, "check = [alpha] == target");
}

#[test]
fn test_seed_186_leaves_a_list_standing_outside_a_comparison() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-186");
    assert!(
        !planted,
        "a list standing outside a comparison is not an operand"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_186_verdict_is_caught() {
    let target = target_with(LAST_OPERAND_MODULE, LAST_OPERAND_GATE, "caught186");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-186"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-187, dropped first comparison operand list element.

const FIRST_OPERAND_MODULE: &str = "check = [alpha, beta] == target\nkeep = gamma\n";
const FIRST_OPERAND_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_187_plants_on_a_comparison_operand_that_is_a_two_element_list() {
    let (planted, _) = applied_render(FIRST_OPERAND_MODULE, "seed-187");
    assert!(
        planted,
        "a comparison operand that is a two element list plants"
    );
}

#[test]
fn test_seed_187_removes_the_first_comparison_operand_list_element() {
    let (_, render) = applied_render(FIRST_OPERAND_MODULE, "seed-187");
    assert_eq!(render, "check = [beta] == target\nkeep = gamma");
}

#[test]
fn test_seed_187_keeps_the_later_operand_elements_standing() {
    let source = "check = [alpha, beta, gamma] == target\n";
    let (_, render) = applied_render(source, "seed-187");
    assert_eq!(render, "check = [beta, gamma] == target");
}

#[test]
fn test_seed_187_leaves_a_single_element_operand_list_standing() {
    let source = "check = [alpha] == target\n";
    let (planted, render) = applied_render(source, "seed-187");
    assert!(!planted, "a one element operand list does not plant");
    assert_eq!(render, "check = [alpha] == target");
}

#[test]
fn test_seed_187_leaves_a_list_standing_outside_a_comparison() {
    let (planted, render) = applied_render("pair = [alpha, beta]\n", "seed-187");
    assert!(
        !planted,
        "a list standing outside a comparison is not an operand"
    );
    assert_eq!(render, "pair = [alpha, beta]");
}

#[test]
fn test_seed_187_verdict_is_caught() {
    let target = target_with(FIRST_OPERAND_MODULE, FIRST_OPERAND_GATE, "caught187");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-187"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty fifth format.

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
fn test_version_forty_four_pack_still_loads_the_one_hundred_and_eighty_four() {
    let dir = fresh_dir("pack44");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(184).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/44", entries));
    let loaded = load_seed_pack(&path).expect("the version forty four pack loads");
    assert_eq!(loaded.len(), 184);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_five_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[187].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-188".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[187] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/45", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty five pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_four_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(183).collect();
    entries.push(standing_entries()[184].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/44", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_five_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(184).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/45", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and eighty four entries under the version forty fifth format are refused"
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
        "the written pack carries the forty fifth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
