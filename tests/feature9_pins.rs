//! Feature slice 9 pins, the registry grown a seventeenth time.
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
    let dir =
        std::env::temp_dir().join(format!("canaryfield-feature9-{}-{tag}", std::process::id()));
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
fn test_registry_has_two_hundred_and_sixteen_unique_seeds() {
    assert_eq!(SEEDS.len(), 216, "the grown registry holds two hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 216, "the ids stand unique");
    assert_eq!(SEEDS[76].id, "seed-76");
    assert_eq!(SEEDS[79].id, "seed-79");
}

// Seed-76, swapped decorator order.

const DECO_MODULE: &str = "def alpha(fn):\n    return fn\n\ndef beta(fn):\n    return fn\n\n@alpha\n@beta\ndef check(x):\n    return x + 1\n";
const DECO_GATE: &str = "#!/bin/sh\nawk '/^@/{print; exit}' mod.py | grep -q \"@alpha\"\n";

#[test]
fn test_seed_seventy_six_plants_and_is_caught() {
    let target = target_with(DECO_MODULE, DECO_GATE, "caught76");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-76"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_six_exchanges_the_first_two_decorators() {
    let (planted, rendered) = applied_render("@alpha\n@beta\ndef f(x):\n    return x\n", "seed-76");
    assert!(planted);
    assert_eq!(rendered, "@beta\n@alpha\ndef f(x):\n    return x");
}

#[test]
fn test_seed_seventy_six_plants_every_definition_in_one_file() {
    let (planted, rendered) = applied_render(
        "@alpha\n@beta\ndef f(x):\n    return x\n\n@gamma\n@delta\nclass C:\n    pass\n",
        "seed-76",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "@beta\n@alpha\ndef f(x):\n    return x\n\n@delta\n@gamma\nclass C:\n    pass"
    );
}

#[test]
fn test_seed_seventy_six_leaves_a_single_decorator_standing() {
    let (planted, rendered) = applied_render("@alpha\ndef f(x):\n    return x\n", "seed-76");
    assert!(!planted);
    assert_eq!(rendered, "@alpha\ndef f(x):\n    return x");
}

#[test]
fn test_seed_seventy_six_not_applicable_without_two_decorators() {
    let target = target_with("def check(x):\n    return x + 1\n", DECO_GATE, "na76");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-76"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-77, dropped parameter default.

const DEFAULT_MODULE: &str = "def check(a, b=2):\n    return a + b\n";
const DEFAULT_GATE: &str = "#!/bin/sh\ngrep -q \"b=2\" mod.py\n";

#[test]
fn test_seed_seventy_seven_plants_and_is_caught() {
    let target = target_with(DEFAULT_MODULE, DEFAULT_GATE, "caught77");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-77"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_seven_drops_the_first_default() {
    let (planted, rendered) = applied_render("def f(a, b=2, c=3):\n    return a\n", "seed-77");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c=3):\n    return a");
}

#[test]
fn test_seed_seventy_seven_keeps_the_later_defaults_with_their_parameters() {
    let (planted, rendered) = applied_render("def f(a=1, b=2):\n    return a + b\n", "seed-77");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b=2):\n    return a + b");
}

#[test]
fn test_seed_seventy_seven_plants_every_definition_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(a=1):\n    return a\n\ng = lambda x=5: x\n",
        "seed-77",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(a):\n    return a\ng = lambda x: x");
}

#[test]
fn test_seed_seventy_seven_leaves_a_keyword_default_standing() {
    let (planted, rendered) = applied_render("def f(*, b=2):\n    return b\n", "seed-77");
    assert!(!planted);
    assert_eq!(rendered, "def f(*, b=2):\n    return b");
}

#[test]
fn test_seed_seventy_seven_not_applicable_without_a_default() {
    let target = target_with("def check(a, b):\n    return a + b\n", DEFAULT_GATE, "na77");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-77"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-78, swapped base class order.

const BASE_MODULE: &str =
    "class Alpha:\n    pass\n\nclass Beta:\n    pass\n\nclass Check(Alpha, Beta):\n    pass\n";
const BASE_GATE: &str = "#!/bin/sh\ngrep -q \"class Check(Alpha, Beta)\" mod.py\n";

#[test]
fn test_seed_seventy_eight_plants_and_is_caught() {
    let target = target_with(BASE_MODULE, BASE_GATE, "caught78");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-78"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_eight_exchanges_the_first_two_bases() {
    let (planted, rendered) = applied_render("class C(Alpha, Beta):\n    pass\n", "seed-78");
    assert!(planted);
    assert_eq!(rendered, "class C(Beta, Alpha):\n    pass");
}

#[test]
fn test_seed_seventy_eight_keeps_the_class_keywords_standing() {
    let (planted, rendered) = applied_render(
        "class C(Alpha, Beta, metaclass=Meta):\n    pass\n",
        "seed-78",
    );
    assert!(planted);
    assert_eq!(rendered, "class C(Beta, Alpha, metaclass=Meta):\n    pass");
}

#[test]
fn test_seed_seventy_eight_plants_every_class_in_one_file() {
    let (planted, rendered) = applied_render(
        "class C(Alpha, Beta):\n    pass\n\nclass D(Gamma, Delta):\n    pass\n",
        "seed-78",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "class C(Beta, Alpha):\n    pass\n\nclass D(Delta, Gamma):\n    pass"
    );
}

#[test]
fn test_seed_seventy_eight_leaves_a_single_base_standing() {
    let (planted, rendered) = applied_render("class C(Alpha):\n    pass\n", "seed-78");
    assert!(!planted);
    assert_eq!(rendered, "class C(Alpha):\n    pass");
}

#[test]
fn test_seed_seventy_eight_not_applicable_without_two_bases() {
    let target = target_with("class Check:\n    pass\n", BASE_GATE, "na78");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-78"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-79, swapped match case order.

const MATCH_MODULE: &str =
    "def check(x):\n    match x:\n        case 1:\n            return \"one\"\n        case 2:\n            return \"two\"\n";
const MATCH_GATE: &str = "#!/bin/sh\nawk '/case/{print; exit}' mod.py | grep -q \"case 1:\"\n";

#[test]
fn test_seed_seventy_nine_plants_and_is_caught() {
    let target = target_with(MATCH_MODULE, MATCH_GATE, "caught79");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-79"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_nine_exchanges_the_first_two_cases() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    match x:\n        case 1:\n            return \"one\"\n        case 2:\n            return \"two\"\n",
        "seed-79",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(x):\n    match x:\n        case 2:\n            return 'two'\n        case 1:\n            return 'one'"
    );
}

#[test]
fn test_seed_seventy_nine_keeps_the_guards_with_their_case() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    match x:\n        case y if y > 0:\n            return \"pos\"\n        case 0:\n            return \"zero\"\n",
        "seed-79",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(x):\n    match x:\n        case 0:\n            return 'zero'\n        case y if y > 0:\n            return 'pos'"
    );
}

#[test]
fn test_seed_seventy_nine_plants_every_match_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    match x:\n        case 1:\n            return \"one\"\n        case 2:\n            return \"two\"\n\ndef g(y):\n    match y:\n        case \"a\":\n            return 1\n        case \"b\":\n            return 2\n",
        "seed-79",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(x):\n    match x:\n        case 2:\n            return 'two'\n        case 1:\n            return 'one'\n\ndef g(y):\n    match y:\n        case 'b':\n            return 2\n        case 'a':\n            return 1"
    );
}

#[test]
fn test_seed_seventy_nine_leaves_a_single_case_standing() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    match x:\n        case 1:\n            return \"one\"\n",
        "seed-79",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(x):\n    match x:\n        case 1:\n            return 'one'"
    );
}

#[test]
fn test_seed_seventy_nine_not_applicable_without_two_cases() {
    let target = target_with("def check(x):\n    return x + 1\n", MATCH_GATE, "na79");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-79"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of obligation F9-6.

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
fn test_version_seventeen_pack_still_loads_the_seventy_six() {
    let dir = fresh_dir("pack17");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(76).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/17", entries));
    let loaded = load_seed_pack(&path).expect("the version seventeen pack loads");
    assert_eq!(loaded.len(), 76);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-75".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_eighteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[79].clone();
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
    entries[79] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/18", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version eighteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_seventeen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(75).collect();
    entries.push(standing_entries()[76].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/17", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version seventeen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_eighteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(76).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/18", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "seventy six entries under the version eighteen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 216);
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
        written.contains("\"canaryfield-seeds/52\""),
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
