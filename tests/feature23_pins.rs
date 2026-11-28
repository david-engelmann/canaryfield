//! Feature slice 23 pins, the registry grown a thirty first time.
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
        "canaryfield-feature23-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 220, "the grown registry holds two hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "the ids stand unique");
    assert_eq!(SEEDS[132].id, "seed-132");
    assert_eq!(SEEDS[135].id, "seed-135");
}

// Seed-132, dropped assignment target.

const CHAINED_MODULE: &str = "first = second = alpha\nkeep = beta\n";
const CHAINED_GATE: &str = "#!/bin/sh\ngrep -q 'first = second = alpha' mod.py\n";

#[test]
fn test_seed_132_plants_on_a_chained_assignment() {
    let (planted, _) = applied_render(CHAINED_MODULE, "seed-132");
    assert!(planted, "a chained assignment plants");
}

#[test]
fn test_seed_132_removes_the_first_target() {
    let (_, render) = applied_render(CHAINED_MODULE, "seed-132");
    assert_eq!(render, "second = alpha\nkeep = beta");
}

#[test]
fn test_seed_132_removes_the_first_target_of_every_chained_assignment() {
    let (_, render) = applied_render("first = second = alpha\nother = last = beta\n", "seed-132");
    assert_eq!(render, "second = alpha\nlast = beta");
}

#[test]
fn test_seed_132_keeps_the_remaining_targets_of_a_longer_chain() {
    let (_, render) = applied_render("first = second = third = alpha\n", "seed-132");
    assert_eq!(render, "second = third = alpha");
}

#[test]
fn test_seed_132_leaves_a_single_target_assignment_standing() {
    let (planted, render) = applied_render("value = alpha\n", "seed-132");
    assert!(!planted, "an assignment carrying one target does not plant");
    assert_eq!(render, "value = alpha");
}

#[test]
fn test_seed_132_leaves_an_annotated_assignment_standing() {
    let (planted, render) = applied_render("value: int = alpha\n", "seed-132");
    assert!(
        !planted,
        "an annotated assignment is not a chained assignment"
    );
    assert_eq!(render, "value: int = alpha");
}

#[test]
fn test_seed_132_verdict_is_caught() {
    let target = target_with(CHAINED_MODULE, CHAINED_GATE, "caught132");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-132"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-133, dropped last tuple element.

const TUPLE_MODULE: &str = "pair = (alpha, beta)\nkeep = gamma\n";
const TUPLE_GATE: &str = "#!/bin/sh\ngrep -q '(alpha, beta)' mod.py\n";

#[test]
fn test_seed_133_plants_on_a_tuple_display() {
    let (planted, _) = applied_render(TUPLE_MODULE, "seed-133");
    assert!(planted, "a tuple display plants");
}

#[test]
fn test_seed_133_removes_the_last_element() {
    let (_, render) = applied_render(TUPLE_MODULE, "seed-133");
    assert_eq!(render, "pair = (alpha,)\nkeep = gamma");
}

#[test]
fn test_seed_133_removes_the_last_element_of_every_tuple_display() {
    let (_, render) = applied_render(
        "pair = (alpha, beta)\ntriple = (one, two, three)\n",
        "seed-133",
    );
    assert_eq!(render, "pair = (alpha,)\ntriple = (one, two)");
}

#[test]
fn test_seed_133_keeps_the_earlier_elements_standing() {
    let (_, render) = applied_render("quad = (one, two, three, four)\n", "seed-133");
    assert_eq!(render, "quad = (one, two, three)");
}

#[test]
fn test_seed_133_leaves_a_single_element_tuple_standing() {
    let (planted, render) = applied_render("pair = (alpha,)\n", "seed-133");
    assert!(!planted, "a tuple carrying a single element does not plant");
    assert_eq!(render, "pair = (alpha,)");
}

#[test]
fn test_seed_133_leaves_an_unpacking_target_standing() {
    let (planted, render) = applied_render("first, second = pair\n", "seed-133");
    assert!(!planted, "an unpacking target is not a display");
    assert_eq!(render, "first, second = pair");
}

#[test]
fn test_seed_133_leaves_a_list_display_standing() {
    let (planted, render) = applied_render("items = [alpha, beta]\n", "seed-133");
    assert!(!planted, "a list display is not a tuple display");
    assert_eq!(render, "items = [alpha, beta]");
}

#[test]
fn test_seed_133_verdict_is_caught() {
    let target = target_with(TUPLE_MODULE, TUPLE_GATE, "caught133");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-133"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-134, dropped last base class.

const BASES_MODULE: &str = "class Thing(Base, Other):\n    pass\nvalue = beta\n";
const BASES_GATE: &str = "#!/bin/sh\ngrep -q 'Other' mod.py\n";

#[test]
fn test_seed_134_plants_on_a_class_with_two_bases() {
    let (planted, _) = applied_render(BASES_MODULE, "seed-134");
    assert!(planted, "a class carrying two bases plants");
}

#[test]
fn test_seed_134_removes_the_last_base() {
    let (_, render) = applied_render(BASES_MODULE, "seed-134");
    assert_eq!(render, "class Thing(Base):\n    pass\nvalue = beta");
}

#[test]
fn test_seed_134_removes_the_last_base_of_every_class() {
    let (_, render) = applied_render(
        "class A(One, Two):\n    pass\nclass B(Three, Four):\n    pass\n",
        "seed-134",
    );
    assert_eq!(
        render,
        "class A(One):\n    pass\n\nclass B(Three):\n    pass"
    );
}

#[test]
fn test_seed_134_keeps_the_earlier_bases_standing() {
    let (_, render) = applied_render("class Thing(One, Two, Three):\n    pass\n", "seed-134");
    assert_eq!(render, "class Thing(One, Two):\n    pass");
}

#[test]
fn test_seed_134_leaves_a_single_base_standing() {
    let (planted, render) = applied_render("class Thing(Base):\n    pass\n", "seed-134");
    assert!(!planted, "a single base is seed-39's ground");
    assert_eq!(render, "class Thing(Base):\n    pass");
}

#[test]
fn test_seed_134_leaves_a_baseless_class_standing() {
    let (planted, render) = applied_render("class Thing:\n    pass\n", "seed-134");
    assert!(!planted, "a class carrying no base does not plant");
    assert_eq!(render, "class Thing:\n    pass");
}

#[test]
fn test_seed_134_verdict_is_caught() {
    let target = target_with(BASES_MODULE, BASES_GATE, "caught134");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-134"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-135, dropped decorator argument.

const DECORATED_MODULE: &str = "@wrap(alpha)\ndef f():\n    return value\n";
const DECORATED_GATE: &str = "#!/bin/sh\ngrep -q 'wrap(alpha)' mod.py\n";

#[test]
fn test_seed_135_plants_on_a_decorator_call() {
    let (planted, _) = applied_render(DECORATED_MODULE, "seed-135");
    assert!(planted, "a decorator standing as a call plants");
}

#[test]
fn test_seed_135_removes_the_first_argument() {
    let (_, render) = applied_render(DECORATED_MODULE, "seed-135");
    assert_eq!(render, "@wrap()\ndef f():\n    return value");
}

#[test]
fn test_seed_135_keeps_the_remaining_arguments_standing() {
    let (_, render) = applied_render(
        "@wrap(alpha, beta)\ndef f():\n    return value\n",
        "seed-135",
    );
    assert_eq!(render, "@wrap(beta)\ndef f():\n    return value");
}

#[test]
fn test_seed_135_removes_the_argument_of_a_class_decorator() {
    let (_, render) = applied_render("@wrap(alpha)\nclass Thing:\n    pass\n", "seed-135");
    assert_eq!(render, "@wrap()\nclass Thing:\n    pass");
}

#[test]
fn test_seed_135_leaves_a_bare_decorator_standing() {
    let (planted, render) = applied_render("@wrap\ndef f():\n    return value\n", "seed-135");
    assert!(
        !planted,
        "a decorator standing as a bare name does not plant"
    );
    assert_eq!(render, "@wrap\ndef f():\n    return value");
}

#[test]
fn test_seed_135_leaves_an_argument_free_call_standing() {
    let (planted, render) = applied_render("@wrap()\ndef f():\n    return value\n", "seed-135");
    assert!(
        !planted,
        "a decorator call carrying no argument does not plant"
    );
    assert_eq!(render, "@wrap()\ndef f():\n    return value");
}

#[test]
fn test_seed_135_leaves_an_ordinary_call_standing() {
    let (planted, render) = applied_render("value = wrap(alpha)\n", "seed-135");
    assert!(!planted, "an ordinary call is seed-122's ground");
    assert_eq!(render, "value = wrap(alpha)");
}

#[test]
fn test_seed_135_verdict_is_caught() {
    let target = target_with(DECORATED_MODULE, DECORATED_GATE, "caught135");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-135"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F23-4.

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
fn test_version_thirty_one_pack_still_loads_the_one_hundred_and_thirty_two() {
    let dir = fresh_dir("pack31");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(132).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/31", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty one pack loads");
    assert_eq!(loaded.len(), 132);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_two_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[135].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-139".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[135] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/32", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_one_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(131).collect();
    entries.push(standing_entries()[132].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/31", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_two_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(132).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/32", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and thirty two entries under the version thirty two format are refused"
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
        written.contains("\"canaryfield-seeds/53\""),
        "the written pack carries the thirty third format"
    );
    let _ = fs::remove_dir_all(&dir);
}
