//! Feature slice 26 pins, the registry grown a thirty fourth time.
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
        "canaryfield-feature26-{}-{tag}",
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
fn test_registry_has_one_hundred_and_forty_eight_unique_seeds() {
    assert_eq!(SEEDS.len(), 148, "the registry stands at 148 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 148, "every seed id stands unique");
    assert_eq!(SEEDS[147].id, "seed-147", "the last seed stands at 147");
}

// Seed-144, dropped last call argument.

const CALL_MODULE: &str = "result = call(alpha, beta)\nkeep = gamma\n";
const CALL_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_144_plants_on_a_call_with_two_positional_arguments() {
    let (planted, _) = applied_render(CALL_MODULE, "seed-144");
    assert!(planted, "a call carrying two positional arguments plants");
}

#[test]
fn test_seed_144_removes_the_last_call_argument() {
    let (_, render) = applied_render(CALL_MODULE, "seed-144");
    assert_eq!(render, "result = call(alpha)\nkeep = gamma");
}

#[test]
fn test_seed_144_removes_the_last_argument_of_every_call() {
    let (_, render) = applied_render(
        "first = call(alpha, beta)\nsecond = call(one, two, three)\n",
        "seed-144",
    );
    assert_eq!(render, "first = call(alpha)\nsecond = call(one, two)");
}

#[test]
fn test_seed_144_keeps_the_earlier_arguments_standing() {
    let (_, render) = applied_render("result = call(one, two, three, four)\n", "seed-144");
    assert_eq!(render, "result = call(one, two, three)");
}

#[test]
fn test_seed_144_leaves_a_single_argument_call_standing() {
    let (planted, render) = applied_render("result = call(alpha)\n", "seed-144");
    assert!(
        !planted,
        "a call carrying a single positional argument does not plant"
    );
    assert_eq!(render, "result = call(alpha)");
}

#[test]
fn test_seed_144_keeps_the_keyword_arguments_standing() {
    let (_, render) = applied_render("result = call(alpha, beta, key=value)\n", "seed-144");
    assert_eq!(render, "result = call(alpha, key=value)");
}

#[test]
fn test_seed_144_verdict_is_caught() {
    let target = target_with(CALL_MODULE, CALL_GATE, "caught144");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-144"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-145, dropped last parameter.

const PARAM_MODULE: &str = "def measure(alpha, beta):\n    return alpha\n";
const PARAM_GATE: &str = "#!/bin/sh\ngrep -q 'alpha, beta' mod.py\n";

#[test]
fn test_seed_145_plants_on_a_definition_with_two_parameters() {
    let (planted, _) = applied_render(PARAM_MODULE, "seed-145");
    assert!(planted, "a definition carrying two parameters plants");
}

#[test]
fn test_seed_145_removes_the_last_parameter() {
    let (_, render) = applied_render(PARAM_MODULE, "seed-145");
    assert_eq!(render, "def measure(alpha):\n    return alpha");
}

#[test]
fn test_seed_145_removes_the_last_parameter_with_its_default() {
    let (_, render) = applied_render(
        "def measure(alpha, beta=two):\n    return alpha\n",
        "seed-145",
    );
    assert_eq!(render, "def measure(alpha):\n    return alpha");
}

#[test]
fn test_seed_145_removes_a_lambdas_last_parameter() {
    let (_, render) = applied_render("pick = lambda alpha, beta: alpha\n", "seed-145");
    assert_eq!(render, "pick = lambda alpha: alpha");
}

#[test]
fn test_seed_145_keeps_the_earlier_parameters_standing() {
    let (_, render) = applied_render(
        "def measure(alpha, beta, delta):\n    return alpha\n",
        "seed-145",
    );
    assert_eq!(render, "def measure(alpha, beta):\n    return alpha");
}

#[test]
fn test_seed_145_leaves_a_single_parameter_definition_standing() {
    let (planted, render) = applied_render("def measure(alpha):\n    return alpha\n", "seed-145");
    assert!(
        !planted,
        "a definition carrying a single parameter does not plant"
    );
    assert_eq!(render, "def measure(alpha):\n    return alpha");
}

#[test]
fn test_seed_145_verdict_is_caught() {
    let target = target_with(
        "def measure(alpha, beta):\n    return alpha + beta\n",
        PARAM_GATE,
        "caught145",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-145"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-146, dropped subscript index element.

const INDEX_MODULE: &str = "item = grid[alpha, beta]\nkeep = gamma\n";
const INDEX_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_146_plants_on_a_tuple_index_subscript() {
    let (planted, _) = applied_render(INDEX_MODULE, "seed-146");
    assert!(planted, "a subscript with a two element tuple index plants");
}

#[test]
fn test_seed_146_removes_the_last_index_element() {
    let (_, render) = applied_render(INDEX_MODULE, "seed-146");
    assert_eq!(render, "item = grid[alpha,]\nkeep = gamma");
}

#[test]
fn test_seed_146_removes_the_last_element_of_every_tuple_index() {
    let (_, render) = applied_render(
        "first = grid[alpha, beta]\nsecond = table[one, two, three]\n",
        "seed-146",
    );
    assert_eq!(render, "first = grid[alpha,]\nsecond = table[one, two]");
}

#[test]
fn test_seed_146_keeps_the_earlier_index_elements_standing() {
    let (_, render) = applied_render("item = grid[one, two, three, four]\n", "seed-146");
    assert_eq!(render, "item = grid[one, two, three]");
}

#[test]
fn test_seed_146_leaves_a_single_index_subscript_standing() {
    let (planted, render) = applied_render("item = grid[alpha]\n", "seed-146");
    assert!(
        !planted,
        "a subscript carrying a single index does not plant"
    );
    assert_eq!(render, "item = grid[alpha]");
}

#[test]
fn test_seed_146_leaves_a_slice_subscript_standing() {
    let (planted, render) = applied_render("item = grid[alpha:beta]\n", "seed-146");
    assert!(!planted, "a subscript carrying a slice does not plant");
    assert_eq!(render, "item = grid[alpha:beta]");
}

#[test]
fn test_seed_146_verdict_is_caught() {
    let target = target_with(INDEX_MODULE, INDEX_GATE, "caught146");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-146"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-147, dropped last exception handler.

const HANDLER_MODULE: &str =
    "try:\n    work()\nexcept Alpha:\n    pass\nexcept Beta:\n    pass\n";
const HANDLER_GATE: &str = "#!/bin/sh\ngrep -q 'Beta' mod.py\n";

#[test]
fn test_seed_147_plants_on_a_try_with_two_handlers() {
    let (planted, _) = applied_render(HANDLER_MODULE, "seed-147");
    assert!(planted, "a try carrying two handlers plants");
}

#[test]
fn test_seed_147_removes_the_last_exception_handler() {
    let (_, render) = applied_render(HANDLER_MODULE, "seed-147");
    assert_eq!(render, "try:\n    work()\nexcept Alpha:\n    pass");
}

#[test]
fn test_seed_147_keeps_the_earlier_handlers_standing() {
    let (_, render) = applied_render(
        "try:\n    work()\nexcept Alpha:\n    pass\nexcept Beta:\n    pass\nexcept Delta:\n    pass\n",
        "seed-147",
    );
    assert_eq!(
        render,
        "try:\n    work()\nexcept Alpha:\n    pass\nexcept Beta:\n    pass"
    );
}

#[test]
fn test_seed_147_leaves_a_single_handler_try_standing() {
    let (planted, render) = applied_render(
        "try:\n    work()\nexcept Alpha:\n    pass\n",
        "seed-147",
    );
    assert!(!planted, "a try carrying a single handler does not plant");
    assert_eq!(render, "try:\n    work()\nexcept Alpha:\n    pass");
}

#[test]
fn test_seed_147_verdict_is_caught() {
    let target = target_with(HANDLER_MODULE, HANDLER_GATE, "caught147");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-147"), "caught");
    let _ = fs::remove_dir_all(&target);
}

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
fn test_version_thirty_four_pack_still_loads_the_one_hundred_and_forty_four() {
    let dir = fresh_dir("pack34");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(144).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/34", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty four pack loads");
    assert_eq!(loaded.len(), 144);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_five_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[147].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-148".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[147] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/35", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty five pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_four_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(143).collect();
    entries.push(standing_entries()[144].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/34", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_five_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(144).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/35", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and forty four entries under the version thirty five format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 148);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[147], "seed-147");
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
        written.contains("\"canaryfield-seeds/35\""),
        "the written pack carries the thirty fifth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
