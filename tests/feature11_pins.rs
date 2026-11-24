//! Feature slice 11 pins, the registry grown a nineteenth time.
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
        "canaryfield-feature11-{}-{tag}",
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
fn test_registry_has_one_hundred_and_sixteen_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        116,
        "the grown registry holds one hundred and sixteen"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 116, "the ids stand unique");
    assert_eq!(SEEDS[84].id, "seed-84");
    assert_eq!(SEEDS[87].id, "seed-87");
}

// Seed-84, dropped with item.

const WITH_MODULE: &str = "def check():\n    with alpha() as a, beta() as b:\n        return b\n";
const WITH_GATE: &str = "#!/bin/sh\ngrep -q \"with alpha() as a, beta() as b\" mod.py\n";

#[test]
fn test_seed_eighty_four_plants_and_is_caught() {
    let target = target_with(WITH_MODULE, WITH_GATE, "caught84");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-84"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_four_removes_the_first_item() {
    let (planted, rendered) =
        applied_render("with alpha() as a, beta() as b:\n    return b\n", "seed-84");
    assert!(planted);
    assert_eq!(rendered, "with beta() as b:\n    return b");
}

#[test]
fn test_seed_eighty_four_plants_every_with_in_one_file() {
    let (planted, rendered) = applied_render(
        "with alpha() as a, beta() as b:\n    first = a\nwith gamma() as c, delta() as d:\n    second = c\n",
        "seed-84",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "with beta() as b:\n    first = a\nwith delta() as d:\n    second = c"
    );
}

#[test]
fn test_seed_eighty_four_leaves_a_single_item_standing() {
    let (planted, rendered) = applied_render("with alpha() as a:\n    return a\n", "seed-84");
    assert!(!planted);
    assert_eq!(rendered, "with alpha() as a:\n    return a");
}

#[test]
fn test_seed_eighty_four_not_applicable_without_two_items() {
    let target = target_with("def check():\n    return 1\n", WITH_GATE, "na84");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-84"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-85, swapped try handler order.

const TRY_MODULE: &str = "def check():\n    try:\n        return int('x')\n    except ValueError:\n        return 1\n    except TypeError:\n        return 2\n";
const TRY_GATE: &str = "#!/bin/sh\ngrep \"except\" mod.py | head -1 | grep -q \"ValueError\"\n";

#[test]
fn test_seed_eighty_five_plants_and_is_caught() {
    let target = target_with(TRY_MODULE, TRY_GATE, "caught85");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-85"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_five_exchanges_the_first_two_handlers() {
    let (planted, rendered) = applied_render(
        "try:\n    value = int('x')\nexcept ValueError:\n    value = 1\nexcept TypeError:\n    value = 2\n",
        "seed-85",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    value = int('x')\nexcept TypeError:\n    value = 2\nexcept ValueError:\n    value = 1"
    );
}

#[test]
fn test_seed_eighty_five_plants_every_try_in_one_file() {
    let (planted, rendered) = applied_render(
        "try:\n    value = int('x')\nexcept ValueError:\n    value = 1\nexcept TypeError:\n    value = 2\ntry:\n    other = int('y')\nexcept KeyError:\n    other = 3\nexcept IndexError:\n    other = 4\n",
        "seed-85",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    value = int('x')\nexcept TypeError:\n    value = 2\nexcept ValueError:\n    value = 1\ntry:\n    other = int('y')\nexcept IndexError:\n    other = 4\nexcept KeyError:\n    other = 3"
    );
}

#[test]
fn test_seed_eighty_five_leaves_a_bare_second_handler_standing() {
    let (planted, rendered) = applied_render(
        "try:\n    value = int('x')\nexcept ValueError:\n    value = 1\nexcept:\n    value = 2\n",
        "seed-85",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "try:\n    value = int('x')\nexcept ValueError:\n    value = 1\nexcept:\n    value = 2"
    );
}

#[test]
fn test_seed_eighty_five_not_applicable_without_two_typed_handlers() {
    let target = target_with(
        "def check():\n    try:\n        return int('x')\n    except ValueError:\n        return 1\n",
        TRY_GATE,
        "na85",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-85"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-86, dropped argument annotation.

const ANNOTATED_MODULE: &str = "def check(value: int):\n    return value\n";
const ANNOTATED_GATE: &str = "#!/bin/sh\ngrep -q \"value: int\" mod.py\n";

#[test]
fn test_seed_eighty_six_plants_and_is_caught() {
    let target = target_with(ANNOTATED_MODULE, ANNOTATED_GATE, "caught86");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-86"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_six_clears_the_annotation() {
    let (planted, rendered) =
        applied_render("def check(value: int):\n    return value\n", "seed-86");
    assert!(planted);
    assert_eq!(rendered, "def check(value):\n    return value");
}

#[test]
fn test_seed_eighty_six_plants_every_annotated_argument_in_one_file() {
    let (planted, rendered) = applied_render(
        "def check(first: int, second: str):\n    return first\n",
        "seed-86",
    );
    assert!(planted);
    assert_eq!(rendered, "def check(first, second):\n    return first");
}

#[test]
fn test_seed_eighty_six_leaves_an_annotated_assignment_standing() {
    let (planted, rendered) = applied_render("count: int = 3\n", "seed-86");
    assert!(!planted);
    assert_eq!(rendered, "count: int = 3");
}

#[test]
fn test_seed_eighty_six_not_applicable_without_an_annotation() {
    let target = target_with("def check():\n    return 1\n", ANNOTATED_GATE, "na86");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-86"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-87, swapped assignment target order.

const TARGET_MODULE: &str = "def check():\n    first, second = 1, 2\n    return first\n";
const TARGET_GATE: &str = "#!/bin/sh\ngrep -q \"first, second = 1, 2\" mod.py\n";

#[test]
fn test_seed_eighty_seven_plants_and_is_caught() {
    let target = target_with(TARGET_MODULE, TARGET_GATE, "caught87");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-87"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_seven_exchanges_the_first_two_elements() {
    let (planted, rendered) = applied_render("first, second = 1, 2\n", "seed-87");
    assert!(planted);
    assert_eq!(rendered, "second, first = (1, 2)");
}

#[test]
fn test_seed_eighty_seven_plants_every_assignment_in_one_file() {
    let (planted, rendered) =
        applied_render("first, second = 1, 2\nthird, fourth = 3, 4\n", "seed-87");
    assert!(planted);
    assert_eq!(rendered, "second, first = (1, 2)\nfourth, third = (3, 4)");
}

#[test]
fn test_seed_eighty_seven_leaves_chained_name_targets_standing() {
    let (planted, rendered) = applied_render("left = right = 1\n", "seed-87");
    assert!(!planted);
    assert_eq!(rendered, "left = right = 1");
}

#[test]
fn test_seed_eighty_seven_not_applicable_without_a_sequence_target() {
    let target = target_with("def check():\n    return 1\n", TARGET_GATE, "na87");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-87"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F11-6.

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
fn test_version_nineteen_pack_still_loads_the_eighty_four() {
    let dir = fresh_dir("pack19");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(84).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/19", entries));
    let loaded = load_seed_pack(&path).expect("the version nineteen pack loads");
    assert_eq!(loaded.len(), 84);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-83".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[87].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-99".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[87] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/20", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_nineteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(83).collect();
    entries.push(standing_entries()[84].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/19", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version nineteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(84).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/20", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "eighty four entries under the version twenty format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 116);
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
        written.contains("\"canaryfield-seeds/27\""),
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
