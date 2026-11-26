//! Feature slice 13 pins, the registry grown a twenty first time.
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
        "canaryfield-feature13-{}-{tag}",
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
fn test_registry_has_one_hundred_and_sixty_eight_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        168,
        "the grown registry holds one hundred and sixty eight"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 168, "the ids stand unique");
    assert_eq!(SEEDS[92].id, "seed-92");
    assert_eq!(SEEDS[95].id, "seed-95");
}

// Seed-92, dropped raise statement.

const RAISE_MODULE: &str = "def check():\n    raise ValueError('bad')\n    return 1\n";
const RAISE_GATE: &str = "#!/bin/sh\ngrep -q \"raise ValueError\" mod.py\n";

#[test]
fn test_seed_92_plants_and_is_caught() {
    let target = target_with(RAISE_MODULE, RAISE_GATE, "caught92");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-92"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_92_removes_a_raise_statement() {
    let (planted, rendered) = applied_render(
        "def check():\n    raise ValueError('bad')\n    return 1\n",
        "seed-92",
    );
    assert!(planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_92_plants_every_raise_in_one_file() {
    let (planted, rendered) = applied_render(
        "def first():\n    raise ValueError('a')\n    return 1\ndef second():\n    raise TypeError('b')\n    return 2\n",
        "seed-92",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def first():\n    return 1\n\ndef second():\n    return 2"
    );
}

#[test]
fn test_seed_92_leaves_a_lone_raise_in_a_body_standing() {
    let (planted, rendered) =
        applied_render("def check():\n    raise ValueError('bad')\n", "seed-92");
    assert!(!planted);
    assert_eq!(rendered, "def check():\n    raise ValueError('bad')");
}

#[test]
fn test_seed_92_is_not_applicable_without_a_raise() {
    let target = target_with("def check():\n    return 1\n", RAISE_GATE, "na92");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-92"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-93, dropped starred display element.

const DISPLAY_MODULE: &str = "def check():\n    return [*parts, 0]\n";
const DISPLAY_GATE: &str = "#!/bin/sh\ngrep -q \"\\*parts\" mod.py\n";

#[test]
fn test_seed_93_plants_and_is_caught() {
    let target = target_with(DISPLAY_MODULE, DISPLAY_GATE, "caught93");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-93"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_93_removes_the_first_starred_element_and_keeps_the_rest() {
    let (planted, rendered) = applied_render("total = [first, *parts, last]\n", "seed-93");
    assert!(planted);
    assert_eq!(rendered, "total = [first, last]");
}

#[test]
fn test_seed_93_plants_on_every_display_carrying_a_starred_element() {
    let (planted, rendered) = applied_render("first = [*a, 1]\nsecond = {*b, 2}\n", "seed-93");
    assert!(planted);
    assert_eq!(rendered, "first = [1]\nsecond = {2}");
}

#[test]
fn test_seed_93_leaves_an_unstarred_display_standing() {
    let (planted, rendered) = applied_render("total = [first, last]\n", "seed-93");
    assert!(!planted);
    assert_eq!(rendered, "total = [first, last]");
}

#[test]
fn test_seed_93_is_not_applicable_without_a_starred_element() {
    let target = target_with("def check():\n    return [1, 2]\n", DISPLAY_GATE, "na93");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-93"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-94, dropped loop control statement.

const CONTROL_MODULE: &str =
    "def check():\n    for item in items:\n        break\n        work(item)\n    return 1\n";
const CONTROL_GATE: &str = "#!/bin/sh\ngrep -q \"break\" mod.py\n";

#[test]
fn test_seed_94_plants_and_is_caught() {
    let target = target_with(CONTROL_MODULE, CONTROL_GATE, "caught94");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-94"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_94_removes_a_break_statement() {
    let (planted, rendered) = applied_render(
        "def check():\n    for item in items:\n        break\n        work(item)\n    return 1\n",
        "seed-94",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n        work(item)\n    return 1"
    );
}

#[test]
fn test_seed_94_removes_a_continue_statement() {
    let (planted, rendered) = applied_render(
        "def check():\n    while items:\n        continue\n        work()\n    return 1\n",
        "seed-94",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    while items:\n        work()\n    return 1"
    );
}

#[test]
fn test_seed_94_plants_the_inner_loop_control_on_its_own_loop() {
    let (planted, rendered) = applied_render(
        "def check():\n    for row in rows:\n        for cell in row:\n            break\n            use(cell)\n        done(row)\n    return 1\n",
        "seed-94",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    for row in rows:\n        for cell in row:\n            use(cell)\n        done(row)\n    return 1"
    );
}

#[test]
fn test_seed_94_leaves_a_control_inside_a_nested_definition_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    for item in items:\n        def helper():\n            break\n        work(item)\n    return 1\n",
        "seed-94",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n\n        def helper():\n            break\n        work(item)\n    return 1"
    );
}

#[test]
fn test_seed_94_is_not_applicable_without_a_loop_control() {
    let target = target_with(
        "def check():\n    for item in items:\n        work(item)\n    return 1\n",
        CONTROL_GATE,
        "na94",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-94"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-95, swapped try else bodies.

const TRY_MODULE: &str = "def risky():\n    return 1\n\ndef check():\n    try:\n        value = risky()\n    except ValueError:\n        return -1\n    else:\n        return value\n";
const TRY_GATE: &str = "#!/bin/sh\npython3 -c \"import mod; assert mod.check() == 1\"\n";

#[test]
fn test_seed_95_plants_and_is_caught() {
    let target = target_with(TRY_MODULE, TRY_GATE, "caught95");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-95"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_95_exchanges_the_try_and_else_bodies() {
    let (planted, rendered) = applied_render(
        "def check():\n    try:\n        work()\n    except ValueError:\n        handle()\n    else:\n        finish()\n",
        "seed-95",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    try:\n        finish()\n    except ValueError:\n        handle()\n    else:\n        work()"
    );
}

#[test]
fn test_seed_95_plants_every_try_in_one_file() {
    let (planted, rendered) = applied_render(
        "def first():\n    try:\n        work()\n    except ValueError:\n        handle()\n    else:\n        finish()\ndef second():\n    try:\n        open_it()\n    except OSError:\n        report()\n    else:\n        close_it()\n",
        "seed-95",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def first():\n    try:\n        finish()\n    except ValueError:\n        handle()\n    else:\n        work()\n\ndef second():\n    try:\n        close_it()\n    except OSError:\n        report()\n    else:\n        open_it()"
    );
}

#[test]
fn test_seed_95_leaves_a_try_without_an_else_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    try:\n        work()\n    except ValueError:\n        handle()\n",
        "seed-95",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    try:\n        work()\n    except ValueError:\n        handle()"
    );
}

#[test]
fn test_seed_95_is_not_applicable_without_an_else_body() {
    let target = target_with(
        "def check():\n    try:\n        return 1\n    except ValueError:\n        return -1\n",
        TRY_GATE,
        "na95",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-95"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F13-6.

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
fn test_version_twenty_one_pack_still_loads_the_ninety_two() {
    let dir = fresh_dir("pack21");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(92).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/21", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty one pack loads");
    assert_eq!(loaded.len(), 92);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-91".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_two_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[95].clone();
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
    entries[95] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/22", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_one_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(91).collect();
    entries.push(standing_entries()[92].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/21", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_two_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(92).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/22", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "ninety two entries under the version twenty two format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 168);
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
        written.contains("\"canaryfield-seeds/40\""),
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
