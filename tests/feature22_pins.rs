//! Feature slice 22 pins, the registry grown a thirtieth time.
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
        "canaryfield-feature22-{}-{tag}",
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
fn test_registry_has_one_hundred_and_fifty_two_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        152,
        "the grown registry holds one hundred and fifty two"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 152, "the ids stand unique");
    assert_eq!(SEEDS[128].id, "seed-128");
    assert_eq!(SEEDS[131].id, "seed-131");
}

// Seed-128, dropped annotated assignment.

const ANNOTATED_MODULE: &str = "value: int = alpha\nkeep = beta\n";
const ANNOTATED_GATE: &str = "#!/bin/sh\ngrep -q 'value: int = alpha' mod.py\n";

#[test]
fn test_seed_128_plants_on_an_annotated_assignment() {
    let (planted, _) = applied_render(ANNOTATED_MODULE, "seed-128");
    assert!(planted, "an annotated assignment plants");
}

#[test]
fn test_seed_128_removes_the_annotated_assignment() {
    let (_, render) = applied_render(ANNOTATED_MODULE, "seed-128");
    assert_eq!(render, "keep = beta");
}

#[test]
fn test_seed_128_removes_a_bare_annotation() {
    let (_, render) = applied_render("value: int\nkeep = beta\n", "seed-128");
    assert_eq!(render, "keep = beta");
}

#[test]
fn test_seed_128_removes_every_annotated_assignment_in_a_body() {
    let (_, render) = applied_render(
        "def f():\n    value: int = alpha\n    other: int = beta\n    return value\n",
        "seed-128",
    );
    assert_eq!(render, "def f():\n    return value");
}

#[test]
fn test_seed_128_never_empties_a_body() {
    let (planted, render) = applied_render("def f():\n    value: int = alpha\n", "seed-128");
    assert!(!planted, "the only statement of a body is never planted");
    assert_eq!(render, "def f():\n    value: int = alpha");
}

#[test]
fn test_seed_128_leaves_a_plain_assignment_standing() {
    let (planted, render) = applied_render("value = alpha\n", "seed-128");
    assert!(!planted, "a plain assignment is seed-127's ground");
    assert_eq!(render, "value = alpha");
}

#[test]
fn test_seed_128_verdict_is_caught() {
    let target = target_with(ANNOTATED_MODULE, ANNOTATED_GATE, "caught128");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-128"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-129, dropped function definition.

const FUNCTION_MODULE: &str = "def helper():\n    return alpha\nvalue = helper\n";
const FUNCTION_GATE: &str = "#!/bin/sh\ngrep -q 'def helper' mod.py\n";

#[test]
fn test_seed_129_plants_on_a_function_definition() {
    let (planted, _) = applied_render(FUNCTION_MODULE, "seed-129");
    assert!(planted, "a function definition plants");
}

#[test]
fn test_seed_129_removes_the_function_definition() {
    let (_, render) = applied_render(
        "def helper():\n    return alpha\nvalue = beta\n",
        "seed-129",
    );
    assert_eq!(render, "value = beta");
}

#[test]
fn test_seed_129_removes_an_asynchronous_definition() {
    let (_, render) = applied_render(
        "async def f():\n    return alpha\nvalue = beta\n",
        "seed-129",
    );
    assert_eq!(render, "value = beta");
}

#[test]
fn test_seed_129_removes_a_nested_definition_and_its_holder() {
    let (_, render) = applied_render(
        "def f():\n    def inner():\n        return alpha\n    return inner\n",
        "seed-129",
    );
    assert_eq!(render, "");
}

#[test]
fn test_seed_129_never_empties_a_body() {
    let (planted, render) = applied_render(
        "class C:\n    def m(self):\n        return alpha\n",
        "seed-129",
    );
    assert!(!planted, "the only statement of a body is never planted");
    assert_eq!(render, "class C:\n\n    def m(self):\n        return alpha");
}

#[test]
fn test_seed_129_leaves_a_class_definition_standing() {
    let (planted, render) = applied_render("class Thing:\n    pass\nvalue = beta\n", "seed-129");
    assert!(!planted, "a class definition is seed-130's ground");
    assert_eq!(render, "class Thing:\n    pass\nvalue = beta");
}

#[test]
fn test_seed_129_verdict_is_caught() {
    let target = target_with(FUNCTION_MODULE, FUNCTION_GATE, "caught129");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-129"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-130, dropped class definition.

const CLASS_MODULE: &str = "class Thing:\n    pass\nvalue = beta\n";
const CLASS_GATE: &str = "#!/bin/sh\ngrep -q 'class Thing' mod.py\n";

#[test]
fn test_seed_130_plants_on_a_class_definition() {
    let (planted, _) = applied_render(CLASS_MODULE, "seed-130");
    assert!(planted, "a class definition plants");
}

#[test]
fn test_seed_130_removes_the_class_definition() {
    let (_, render) = applied_render(CLASS_MODULE, "seed-130");
    assert_eq!(render, "value = beta");
}

#[test]
fn test_seed_130_removes_every_class_definition_in_a_body() {
    let (_, render) = applied_render(
        "class A:\n    pass\nclass B:\n    pass\nvalue = beta\n",
        "seed-130",
    );
    assert_eq!(render, "value = beta");
}

#[test]
fn test_seed_130_never_empties_a_body() {
    let (planted, render) = applied_render("def f():\n    class C:\n        pass\n", "seed-130");
    assert!(!planted, "the only statement of a body is never planted");
    assert_eq!(render, "def f():\n\n    class C:\n        pass");
}

#[test]
fn test_seed_130_leaves_a_function_definition_standing() {
    let (planted, render) = applied_render("def helper():\n    return alpha\n", "seed-130");
    assert!(!planted, "a function definition is seed-129's ground");
    assert_eq!(render, "def helper():\n    return alpha");
}

#[test]
fn test_seed_130_verdict_is_caught() {
    let target = target_with(CLASS_MODULE, CLASS_GATE, "caught130");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-130"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-131, dropped exception handler name.

const HANDLER_MODULE: &str = "try:\n    value = alpha\nexcept Error as err:\n    print(err)\n";
const HANDLER_GATE: &str = "#!/bin/sh\ngrep -q 'except Error as err' mod.py\n";

#[test]
fn test_seed_131_plants_on_a_named_handler() {
    let (planted, _) = applied_render(HANDLER_MODULE, "seed-131");
    assert!(planted, "a handler carrying a name plants");
}

#[test]
fn test_seed_131_removes_the_handler_name() {
    let (_, render) = applied_render(HANDLER_MODULE, "seed-131");
    assert_eq!(
        render,
        "try:\n    value = alpha\nexcept Error:\n    print(err)"
    );
}

#[test]
fn test_seed_131_removes_the_name_of_every_handler() {
    let (_, render) = applied_render(
        "try:\n    value = alpha\nexcept Error as err:\n    print(err)\nexcept Other as o:\n    print(o)\n",
        "seed-131",
    );
    assert_eq!(
        render,
        "try:\n    value = alpha\nexcept Error:\n    print(err)\nexcept Other:\n    print(o)"
    );
}

#[test]
fn test_seed_131_leaves_an_unnamed_handler_standing() {
    let (planted, render) = applied_render(
        "try:\n    value = alpha\nexcept Error:\n    print('x')\n",
        "seed-131",
    );
    assert!(!planted, "a handler carrying no name does not plant");
    assert_eq!(
        render,
        "try:\n    value = alpha\nexcept Error:\n    print('x')"
    );
}

#[test]
fn test_seed_131_leaves_a_bare_handler_standing() {
    let (planted, render) = applied_render(
        "try:\n    value = alpha\nexcept:\n    print('x')\n",
        "seed-131",
    );
    assert!(!planted, "a bare handler carries no name to drop");
    assert_eq!(render, "try:\n    value = alpha\nexcept:\n    print('x')");
}

#[test]
fn test_seed_131_verdict_is_caught() {
    let target = target_with(HANDLER_MODULE, HANDLER_GATE, "caught131");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-131"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F22-4.

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
fn test_version_thirty_pack_still_loads_the_one_hundred_and_twenty_eight() {
    let dir = fresh_dir("pack30");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(128).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/30", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty pack loads");
    assert_eq!(loaded.len(), 128);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_one_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[131].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-135".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[131] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/31", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty one pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(127).collect();
    entries.push(standing_entries()[128].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/30", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_one_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(128).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/31", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and twenty eight entries under the version thirty one format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 152);
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
        written.contains("\"canaryfield-seeds/36\""),
        "the written pack carries the thirty first format"
    );
    let _ = fs::remove_dir_all(&dir);
}
