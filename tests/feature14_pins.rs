//! Feature slice 14 pins, the registry grown a twenty second time.
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
        "canaryfield-feature14-{}-{tag}",
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
    assert_eq!(SEEDS[96].id, "seed-96");
    assert_eq!(SEEDS[99].id, "seed-99");
}

// Seed-96, dropped class keyword argument.

const CLASS_MODULE: &str = "class Widget(Base, metaclass=Registry):\n    pass\n";
const CLASS_GATE: &str = "#!/bin/sh\ngrep -q \"metaclass=Registry\" mod.py\n";

#[test]
fn test_seed_96_plants_and_is_caught() {
    let target = target_with(CLASS_MODULE, CLASS_GATE, "caught96");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-96"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_96_removes_the_first_named_keyword_and_keeps_the_rest() {
    let (planted, rendered) = applied_render(
        "class Widget(Base, metaclass=Registry, option=1):\n    pass\n",
        "seed-96",
    );
    assert!(planted);
    assert_eq!(rendered, "class Widget(Base, option=1):\n    pass");
}

#[test]
fn test_seed_96_plants_on_every_class_carrying_a_keyword() {
    let (planted, rendered) = applied_render(
        "class First(Base, metaclass=Registry):\n    pass\n\nclass Second(Base, metaclass=Registry):\n    pass\n",
        "seed-96",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "class First(Base):\n    pass\n\nclass Second(Base):\n    pass"
    );
}

#[test]
fn test_seed_96_leaves_a_nameless_keyword_standing() {
    let (planted, rendered) = applied_render("class Widget(**options):\n    pass\n", "seed-96");
    assert!(!planted);
    assert_eq!(rendered, "class Widget(**options):\n    pass");
}

#[test]
fn test_seed_96_leaves_a_class_without_a_keyword_standing() {
    let (planted, rendered) = applied_render("class Widget(Base):\n    pass\n", "seed-96");
    assert!(!planted);
    assert_eq!(rendered, "class Widget(Base):\n    pass");
}

#[test]
fn test_seed_96_is_not_applicable_without_a_class_keyword() {
    let target = target_with("def check():\n    return 1\n", CLASS_GATE, "na96");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-96"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-97, swapped loop else bodies.

const LOOP_MODULE: &str =
    "def check():\n    for item in items:\n        work(item)\n    else:\n        finish()\n    return 1\n";
const LOOP_GATE: &str =
    "#!/bin/sh\ngrep -A1 \"for item in items:\" mod.py | grep -q \"work(item)\"\n";

#[test]
fn test_seed_97_plants_and_is_caught() {
    let target = target_with(LOOP_MODULE, LOOP_GATE, "caught97");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-97"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_97_exchanges_a_loop_body_with_its_else_body() {
    let (planted, rendered) = applied_render(LOOP_MODULE, "seed-97");
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n        finish()\n    else:\n        work(item)\n    return 1"
    );
}

#[test]
fn test_seed_97_exchanges_a_while_loop_body_with_its_else_body() {
    let (planted, rendered) = applied_render(
        "def check():\n    while ready:\n        work()\n    else:\n        finish()\n    return 1\n",
        "seed-97",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    while ready:\n        finish()\n    else:\n        work()\n    return 1"
    );
}

#[test]
fn test_seed_97_leaves_a_loop_without_an_else_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    for item in items:\n        work(item)\n    return 1\n",
        "seed-97",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n        work(item)\n    return 1"
    );
}

#[test]
fn test_seed_97_leaves_a_try_else_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    try:\n        work()\n    except ValueError:\n        handle()\n    else:\n        finish()\n    return 1\n",
        "seed-97",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    try:\n        work()\n    except ValueError:\n        handle()\n    else:\n        finish()\n    return 1"
    );
}

#[test]
fn test_seed_97_is_not_applicable_without_a_loop_else() {
    let target = target_with("def check():\n    return 1\n", LOOP_GATE, "na97");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-97"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_97_leaves_a_loop_whose_body_carries_a_break_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    for item in items:\n        break\n    else:\n        finish()\n    return 1\n",
        "seed-97",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n        break\n    else:\n        finish()\n    return 1"
    );
}

#[test]
fn test_seed_97_leaves_a_loop_whose_else_carries_a_break_for_the_loop_around_it_standing() {
    let (planted, rendered) = applied_render(
        "def check():\n    while outer:\n        for item in items:\n            work(item)\n        else:\n            break\n    return 1\n",
        "seed-97",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    while outer:\n        for item in items:\n            work(item)\n        else:\n            break\n    return 1"
    );
}

#[test]
fn test_seed_97_exchanges_where_a_break_stands_inside_a_nested_loop() {
    let (planted, rendered) = applied_render(
        "def check():\n    for item in items:\n        for part in item:\n            break\n    else:\n        finish()\n    return 1\n",
        "seed-97",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check():\n    for item in items:\n        finish()\n    else:\n        for part in item:\n            break\n    return 1"
    );
}

// Seed-98, dropped starred parameter.

const STAR_MODULE: &str = "def check(first, *args, **kwargs):\n    return first\n";
const STAR_GATE: &str = "#!/bin/sh\ngrep -q \"\\*args\" mod.py\n";

#[test]
fn test_seed_98_plants_and_is_caught() {
    let target = target_with(STAR_MODULE, STAR_GATE, "caught98");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-98"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_98_removes_both_starred_parameters() {
    let (planted, rendered) = applied_render(STAR_MODULE, "seed-98");
    assert!(planted);
    assert_eq!(rendered, "def check(first):\n    return first");
}

#[test]
fn test_seed_98_removes_a_lone_variable_positional_parameter() {
    let (planted, rendered) = applied_render("def check(*args):\n    return 1\n", "seed-98");
    assert!(planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_98_removes_a_lone_variable_keyword_parameter() {
    let (planted, rendered) = applied_render("def check(**kwargs):\n    return 1\n", "seed-98");
    assert!(planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_98_removes_a_lambda_starred_parameter() {
    let (planted, rendered) = applied_render("check = lambda *args: 1\n", "seed-98");
    assert!(planted);
    assert_eq!(rendered, "check = lambda: 1");
}

#[test]
fn test_seed_98_leaves_keyword_only_parameters_standing() {
    let (planted, rendered) =
        applied_render("def check(first, *, flag):\n    return flag\n", "seed-98");
    assert!(!planted);
    assert_eq!(rendered, "def check(first, *, flag):\n    return flag");
}

#[test]
fn test_seed_98_is_not_applicable_without_a_starred_parameter() {
    let target = target_with("def check():\n    return 1\n", STAR_GATE, "na98");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-98"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-99, dropped docstring.

const DOCSTRING_MODULE: &str = "def check():\n    'Check notes.'\n    return 1\n";
const DOCSTRING_GATE: &str = "#!/bin/sh\ngrep -q \"Check notes\" mod.py\n";

#[test]
fn test_seed_99_plants_and_is_caught() {
    let target = target_with(DOCSTRING_MODULE, DOCSTRING_GATE, "caught99");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-99"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_99_removes_a_function_docstring() {
    let (planted, rendered) = applied_render(DOCSTRING_MODULE, "seed-99");
    assert!(planted);
    assert_eq!(rendered, "def check():\n    return 1");
}

#[test]
fn test_seed_99_removes_a_module_docstring() {
    let (planted, rendered) = applied_render("'Module notes.'\nvalue = 1\n", "seed-99");
    assert!(planted);
    assert_eq!(rendered, "value = 1");
}

#[test]
fn test_seed_99_removes_a_class_docstring() {
    let (planted, rendered) =
        applied_render("class Widget:\n    'Widget notes.'\n    pass\n", "seed-99");
    assert!(planted);
    assert_eq!(rendered, "class Widget:\n    pass");
}

#[test]
fn test_seed_99_removes_a_module_docstring_standing_alone() {
    let (planted, rendered) = applied_render("'Only notes.'\n", "seed-99");
    assert!(planted);
    assert_eq!(rendered, "");
}

#[test]
fn test_seed_99_leaves_a_string_standing_later_in_a_body() {
    let (planted, rendered) = applied_render(
        "def check():\n    return 1\n    'Late string.'\n",
        "seed-99",
    );
    assert!(!planted);
    assert_eq!(rendered, "def check():\n    return 1\n    'Late string.'");
}

#[test]
fn test_seed_99_leaves_a_string_standing_first_in_an_if_body() {
    let (planted, rendered) = applied_render(
        "def check():\n    if ready:\n        'Not a docstring.'\n    return 1\n",
        "seed-99",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def check():\n    if ready:\n        'Not a docstring.'\n    return 1"
    );
}

#[test]
fn test_seed_99_leaves_a_lone_docstring_in_a_body_standing() {
    let (planted, rendered) = applied_render("def check():\n    'Only notes.'\n", "seed-99");
    assert!(!planted);
    assert_eq!(rendered, "def check():\n    \"\"\"Only notes.\"\"\"");
}

#[test]
fn test_seed_99_is_not_applicable_without_a_docstring() {
    let target = target_with("def check():\n    return 1\n", DOCSTRING_GATE, "na99");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-99"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F14-6.

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
fn test_version_twenty_two_pack_still_loads_the_ninety_six() {
    let dir = fresh_dir("pack22");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(96).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/22", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty two pack loads");
    assert_eq!(loaded.len(), 96);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-95".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_three_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[99].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-104".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[99] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/23", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_two_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(95).collect();
    entries.push(standing_entries()[96].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/22", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_three_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(96).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/23", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "ninety six entries under the version twenty three format are refused"
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
        "the written pack carries the twenty third format"
    );
    let _ = fs::remove_dir_all(&dir);
}
