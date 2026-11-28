//! Feature slice 30 pins, the registry grown a thirty eighth time.
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
        "canaryfield-feature30-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 220, "the registry stands at 164 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "every seed id stands unique");
    assert_eq!(SEEDS[219].id, "seed-219", "the last seed stands at 179");
}

// Seed-160, dropped last tuple argument element.

const LAST_ARGUMENT_MODULE: &str = "report = combine((alpha, beta))\nkeep = gamma\n";
const LAST_ARGUMENT_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_160_plants_on_a_call_carrying_a_two_element_tuple_argument() {
    let (planted, _) = applied_render(LAST_ARGUMENT_MODULE, "seed-160");
    assert!(
        planted,
        "a call carrying a two element tuple argument plants"
    );
}

#[test]
fn test_seed_160_removes_the_last_tuple_argument_element() {
    let (_, render) = applied_render(LAST_ARGUMENT_MODULE, "seed-160");
    assert_eq!(render, "report = combine((alpha,))\nkeep = gamma");
}

#[test]
fn test_seed_160_keeps_the_earlier_argument_elements_standing() {
    let (_, render) = applied_render("report = combine((alpha, beta, gamma))\n", "seed-160");
    assert_eq!(render, "report = combine((alpha, beta))");
}

#[test]
fn test_seed_160_removes_the_last_element_of_every_tuple_argument() {
    let source = "report = combine((alpha, beta), (one, two))\n";
    let (_, render) = applied_render(source, "seed-160");
    assert_eq!(render, "report = combine((alpha,), (one,))");
}

#[test]
fn test_seed_160_leaves_a_single_element_tuple_argument_standing() {
    let (planted, render) = applied_render("report = combine((alpha,))\n", "seed-160");
    assert!(
        !planted,
        "a call carrying a one element tuple argument does not plant"
    );
    assert_eq!(render, "report = combine((alpha,))");
}

#[test]
fn test_seed_160_leaves_an_assignment_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-160");
    assert!(
        !planted,
        "a tuple an assignment hands over is not a call argument"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_160_verdict_is_caught() {
    let target = target_with(LAST_ARGUMENT_MODULE, LAST_ARGUMENT_GATE, "caught160");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-160"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-161, dropped first tuple argument element.

const FIRST_ARGUMENT_MODULE: &str = "report = combine((alpha, beta))\nkeep = gamma\n";
const FIRST_ARGUMENT_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_161_plants_on_a_call_carrying_a_two_element_tuple_argument() {
    let (planted, _) = applied_render(FIRST_ARGUMENT_MODULE, "seed-161");
    assert!(
        planted,
        "a call carrying a two element tuple argument plants"
    );
}

#[test]
fn test_seed_161_removes_the_first_tuple_argument_element() {
    let (_, render) = applied_render(FIRST_ARGUMENT_MODULE, "seed-161");
    assert_eq!(render, "report = combine((beta,))\nkeep = gamma");
}

#[test]
fn test_seed_161_keeps_the_later_argument_elements_standing() {
    let (_, render) = applied_render("report = combine((alpha, beta, gamma))\n", "seed-161");
    assert_eq!(render, "report = combine((beta, gamma))");
}

#[test]
fn test_seed_161_removes_the_first_element_of_every_tuple_argument() {
    let source = "report = combine((alpha, beta), (one, two))\n";
    let (_, render) = applied_render(source, "seed-161");
    assert_eq!(render, "report = combine((beta,), (two,))");
}

#[test]
fn test_seed_161_leaves_a_single_element_tuple_argument_standing() {
    let (planted, render) = applied_render("report = combine((alpha,))\n", "seed-161");
    assert!(
        !planted,
        "a call carrying a one element tuple argument does not plant"
    );
    assert_eq!(render, "report = combine((alpha,))");
}

#[test]
fn test_seed_161_leaves_an_assignment_value_tuple_standing() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-161");
    assert!(
        !planted,
        "a tuple an assignment hands over is not a call argument"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_161_verdict_is_caught() {
    let target = target_with(FIRST_ARGUMENT_MODULE, FIRST_ARGUMENT_GATE, "caught161");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-161"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-162, dropped last for loop tuple element.

const LAST_ITERABLE_MODULE: &str = "for item in (alpha, beta):\n    keep = item\n";
const LAST_ITERABLE_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_162_plants_on_a_for_over_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_ITERABLE_MODULE, "seed-162");
    assert!(planted, "a for over a two element tuple plants");
}

#[test]
fn test_seed_162_removes_the_last_for_loop_tuple_element() {
    let (_, render) = applied_render(LAST_ITERABLE_MODULE, "seed-162");
    assert_eq!(render, "for item in (alpha,):\n    keep = item");
}

#[test]
fn test_seed_162_keeps_the_earlier_iterable_elements_standing() {
    let source = "for item in (alpha, beta, gamma):\n    keep = item\n";
    let (_, render) = applied_render(source, "seed-162");
    assert_eq!(render, "for item in (alpha, beta):\n    keep = item");
}

#[test]
fn test_seed_162_removes_the_last_element_of_every_for_iterable() {
    let source =
        "for item in (alpha, beta):\n    keep = item\nfor part in (one, two):\n    keep = part\n";
    let (_, render) = applied_render(source, "seed-162");
    assert_eq!(
        render,
        "for item in (alpha,):\n    keep = item\nfor part in (one,):\n    keep = part"
    );
}

#[test]
fn test_seed_162_leaves_a_single_element_tuple_iterable_standing() {
    let source = "for item in (alpha,):\n    keep = item\n";
    let (planted, render) = applied_render(source, "seed-162");
    assert!(!planted, "a for over a one element tuple does not plant");
    assert_eq!(render, "for item in (alpha,):\n    keep = item");
}

#[test]
fn test_seed_162_leaves_a_call_tuple_argument_standing() {
    let (planted, render) = applied_render("report = combine((alpha, beta))\n", "seed-162");
    assert!(
        !planted,
        "a tuple standing as a call argument is not a for iterable"
    );
    assert_eq!(render, "report = combine((alpha, beta))");
}

#[test]
fn test_seed_162_verdict_is_caught() {
    let target = target_with(LAST_ITERABLE_MODULE, LAST_ITERABLE_GATE, "caught162");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-162"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-163, dropped first for loop tuple element.

const FIRST_ITERABLE_MODULE: &str = "for item in (alpha, beta):\n    keep = item\n";
const FIRST_ITERABLE_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_163_plants_on_a_for_over_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_ITERABLE_MODULE, "seed-163");
    assert!(planted, "a for over a two element tuple plants");
}

#[test]
fn test_seed_163_removes_the_first_for_loop_tuple_element() {
    let (_, render) = applied_render(FIRST_ITERABLE_MODULE, "seed-163");
    assert_eq!(render, "for item in (beta,):\n    keep = item");
}

#[test]
fn test_seed_163_keeps_the_later_iterable_elements_standing() {
    let source = "for item in (alpha, beta, gamma):\n    keep = item\n";
    let (_, render) = applied_render(source, "seed-163");
    assert_eq!(render, "for item in (beta, gamma):\n    keep = item");
}

#[test]
fn test_seed_163_leaves_a_single_element_tuple_iterable_standing() {
    let source = "for item in (alpha,):\n    keep = item\n";
    let (planted, render) = applied_render(source, "seed-163");
    assert!(!planted, "a for over a one element tuple does not plant");
    assert_eq!(render, "for item in (alpha,):\n    keep = item");
}

#[test]
fn test_seed_163_leaves_a_for_over_a_name_standing() {
    let source = "for item in values:\n    keep = item\n";
    let (planted, render) = applied_render(source, "seed-163");
    assert!(!planted, "a for over a name does not plant");
    assert_eq!(render, "for item in values:\n    keep = item");
}

#[test]
fn test_seed_163_verdict_is_caught() {
    let target = target_with(FIRST_ITERABLE_MODULE, FIRST_ITERABLE_GATE, "caught163");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-163"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the thirty ninth format.

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
fn test_version_thirty_eight_pack_still_loads_the_one_hundred_and_sixty() {
    let dir = fresh_dir("pack38");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(160).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/38", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty eight pack loads");
    assert_eq!(loaded.len(), 160);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_nine_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[163].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-164".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[163] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/39", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty nine pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_eight_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(159).collect();
    entries.push(standing_entries()[160].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/38", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_nine_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(160).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/39", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and sixty entries under the version thirty nine format are refused"
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
        "the written pack carries the thirty ninth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
