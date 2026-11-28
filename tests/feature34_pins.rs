//! Feature slice 34 pins, the registry grown a forty second time.
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
        "canaryfield-feature34-{}-{tag}",
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
    assert_eq!(SEEDS.len(), 220, "the registry stands at 180 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 220, "every seed id stands unique");
    assert_eq!(SEEDS[219].id, "seed-219", "the last seed stands at 179");
}

// Seed-176, dropped last exception handler type tuple element.

const LAST_TYPE_MODULE: &str =
    "try:\n    run()\nexcept (AlphaError, BetaError):\n    recover()\nkeep = gamma\n";
const LAST_TYPE_GATE: &str = "#!/bin/sh\ngrep -q 'BetaError' mod.py\n";

#[test]
fn test_seed_176_plants_on_a_handler_whose_type_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_TYPE_MODULE, "seed-176");
    assert!(planted, "a handler type that is a two element tuple plants");
}

#[test]
fn test_seed_176_removes_the_last_exception_handler_type_tuple_element() {
    let (_, render) = applied_render(LAST_TYPE_MODULE, "seed-176");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (AlphaError,):\n    recover()\nkeep = gamma"
    );
}

#[test]
fn test_seed_176_keeps_the_earlier_type_elements_standing() {
    let source = "try:\n    run()\nexcept (AlphaError, BetaError, GammaError):\n    recover()\n";
    let (_, render) = applied_render(source, "seed-176");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (AlphaError, BetaError):\n    recover()"
    );
}

#[test]
fn test_seed_176_removes_the_last_element_of_every_handler_type_tuple() {
    let source = "try:\n    run()\nexcept (AlphaError, BetaError):\n    recover()\nexcept (OneError, TwoError):\n    mend()\n";
    let (_, render) = applied_render(source, "seed-176");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (AlphaError,):\n    recover()\nexcept (OneError,):\n    mend()"
    );
}

#[test]
fn test_seed_176_leaves_a_single_element_type_tuple_standing() {
    let source = "try:\n    run()\nexcept (AlphaError,):\n    recover()\n";
    let (planted, render) = applied_render(source, "seed-176");
    assert!(!planted, "a one element type tuple does not plant");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (AlphaError,):\n    recover()"
    );
}

#[test]
fn test_seed_176_leaves_a_tuple_standing_outside_a_handler_type() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-176");
    assert!(
        !planted,
        "a tuple standing as a plain value is not a handler type"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_176_verdict_is_caught() {
    let target = target_with(LAST_TYPE_MODULE, LAST_TYPE_GATE, "caught176");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-176"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-177, dropped first exception handler type tuple element.

const FIRST_TYPE_MODULE: &str =
    "try:\n    run()\nexcept (AlphaError, BetaError):\n    recover()\nkeep = gamma\n";
const FIRST_TYPE_GATE: &str = "#!/bin/sh\ngrep -q 'AlphaError' mod.py\n";

#[test]
fn test_seed_177_plants_on_a_handler_whose_type_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_TYPE_MODULE, "seed-177");
    assert!(planted, "a handler type that is a two element tuple plants");
}

#[test]
fn test_seed_177_removes_the_first_exception_handler_type_tuple_element() {
    let (_, render) = applied_render(FIRST_TYPE_MODULE, "seed-177");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (BetaError,):\n    recover()\nkeep = gamma"
    );
}

#[test]
fn test_seed_177_keeps_the_later_type_elements_standing() {
    let source = "try:\n    run()\nexcept (AlphaError, BetaError, GammaError):\n    recover()\n";
    let (_, render) = applied_render(source, "seed-177");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (BetaError, GammaError):\n    recover()"
    );
}

#[test]
fn test_seed_177_removes_the_first_element_of_every_handler_type_tuple() {
    let source = "try:\n    run()\nexcept (AlphaError, BetaError):\n    recover()\nexcept (OneError, TwoError):\n    mend()\n";
    let (_, render) = applied_render(source, "seed-177");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (BetaError,):\n    recover()\nexcept (TwoError,):\n    mend()"
    );
}

#[test]
fn test_seed_177_leaves_a_single_element_type_tuple_standing() {
    let source = "try:\n    run()\nexcept (AlphaError,):\n    recover()\n";
    let (planted, render) = applied_render(source, "seed-177");
    assert!(!planted, "a one element type tuple does not plant");
    assert_eq!(
        render,
        "try:\n    run()\nexcept (AlphaError,):\n    recover()"
    );
}

#[test]
fn test_seed_177_leaves_a_tuple_standing_outside_a_handler_type() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-177");
    assert!(
        !planted,
        "a tuple standing as a plain value is not a handler type"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_177_verdict_is_caught() {
    let target = target_with(FIRST_TYPE_MODULE, FIRST_TYPE_GATE, "caught177");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-177"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-178, dropped last keyword value tuple element.

const LAST_VALUE_MODULE: &str = "use(scale=(alpha, beta))\nkeep = gamma\n";
const LAST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_178_plants_on_a_keyword_value_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_VALUE_MODULE, "seed-178");
    assert!(
        planted,
        "a keyword value that is a two element tuple plants"
    );
}

#[test]
fn test_seed_178_removes_the_last_keyword_value_tuple_element() {
    let (_, render) = applied_render(LAST_VALUE_MODULE, "seed-178");
    assert_eq!(render, "use(scale=(alpha,))\nkeep = gamma");
}

#[test]
fn test_seed_178_keeps_the_earlier_value_elements_standing() {
    let source = "use(scale=(alpha, beta, gamma))\n";
    let (_, render) = applied_render(source, "seed-178");
    assert_eq!(render, "use(scale=(alpha, beta))");
}

#[test]
fn test_seed_178_removes_the_last_element_of_every_keyword_value_tuple() {
    let source = "use(scale=(alpha, beta), shift=(one, two))\n";
    let (_, render) = applied_render(source, "seed-178");
    assert_eq!(render, "use(scale=(alpha,), shift=(one,))");
}

#[test]
fn test_seed_178_leaves_a_single_element_keyword_value_tuple_standing() {
    let source = "use(scale=(alpha,))\n";
    let (planted, render) = applied_render(source, "seed-178");
    assert!(!planted, "a one element keyword value tuple does not plant");
    assert_eq!(render, "use(scale=(alpha,))");
}

#[test]
fn test_seed_178_leaves_a_tuple_standing_as_a_positional_argument() {
    let (planted, render) = applied_render("use((alpha, beta))\n", "seed-178");
    assert!(
        !planted,
        "a tuple standing as a positional argument is not a keyword value"
    );
    assert_eq!(render, "use((alpha, beta))");
}

#[test]
fn test_seed_178_verdict_is_caught() {
    let target = target_with(LAST_VALUE_MODULE, LAST_VALUE_GATE, "caught178");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-178"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-179, dropped first keyword value tuple element.

const FIRST_VALUE_MODULE: &str = "use(scale=(alpha, beta))\nkeep = gamma\n";
const FIRST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_179_plants_on_a_keyword_value_that_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_VALUE_MODULE, "seed-179");
    assert!(
        planted,
        "a keyword value that is a two element tuple plants"
    );
}

#[test]
fn test_seed_179_removes_the_first_keyword_value_tuple_element() {
    let (_, render) = applied_render(FIRST_VALUE_MODULE, "seed-179");
    assert_eq!(render, "use(scale=(beta,))\nkeep = gamma");
}

#[test]
fn test_seed_179_keeps_the_later_value_elements_standing() {
    let source = "use(scale=(alpha, beta, gamma))\n";
    let (_, render) = applied_render(source, "seed-179");
    assert_eq!(render, "use(scale=(beta, gamma))");
}

#[test]
fn test_seed_179_leaves_a_single_element_keyword_value_tuple_standing() {
    let source = "use(scale=(alpha,))\n";
    let (planted, render) = applied_render(source, "seed-179");
    assert!(!planted, "a one element keyword value tuple does not plant");
    assert_eq!(render, "use(scale=(alpha,))");
}

#[test]
fn test_seed_179_leaves_a_tuple_standing_as_a_positional_argument() {
    let (planted, render) = applied_render("use((alpha, beta))\n", "seed-179");
    assert!(
        !planted,
        "a tuple standing as a positional argument is not a keyword value"
    );
    assert_eq!(render, "use((alpha, beta))");
}

#[test]
fn test_seed_179_verdict_is_caught() {
    let target = target_with(FIRST_VALUE_MODULE, FIRST_VALUE_GATE, "caught179");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-179"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty third format.

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
fn test_version_forty_two_pack_still_loads_the_one_hundred_and_seventy_six() {
    let dir = fresh_dir("pack42");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(176).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/42", entries));
    let loaded = load_seed_pack(&path).expect("the version forty two pack loads");
    assert_eq!(loaded.len(), 176);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_three_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[179].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-180".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[179] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/43", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_two_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(175).collect();
    entries.push(standing_entries()[176].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/42", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty two pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_three_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(176).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/43", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and seventy six entries under the version forty three format are refused"
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
        "the written pack carries the forty third format"
    );
    let _ = fs::remove_dir_all(&dir);
}
