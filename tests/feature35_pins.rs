//! Feature slice 35 pins, the registry grown a forty third time.
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
        "canaryfield-feature35-{}-{tag}",
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
fn test_registry_has_two_hundred_and_four_unique_seeds() {
    assert_eq!(SEEDS.len(), 204, "the registry stands at 184 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 204, "every seed id stands unique");
    assert_eq!(SEEDS[203].id, "seed-203", "the last seed stands at 187");
}

// Seed-180, dropped last comprehension iterable tuple element.

const LAST_ITER_MODULE: &str = "values = [run(item) for item in (alpha, beta)]\nkeep = gamma\n";
const LAST_ITER_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_180_plants_on_a_comprehension_whose_iterable_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_ITER_MODULE, "seed-180");
    assert!(
        planted,
        "a comprehension iterable that is a two element tuple plants"
    );
}

#[test]
fn test_seed_180_removes_the_last_comprehension_iterable_tuple_element() {
    let (_, render) = applied_render(LAST_ITER_MODULE, "seed-180");
    assert_eq!(
        render,
        "values = [run(item) for item in (alpha,)]\nkeep = gamma"
    );
}

#[test]
fn test_seed_180_keeps_the_earlier_iterable_elements_standing() {
    let source = "values = [run(item) for item in (alpha, beta, gamma)]\n";
    let (_, render) = applied_render(source, "seed-180");
    assert_eq!(render, "values = [run(item) for item in (alpha, beta)]");
}

#[test]
fn test_seed_180_removes_the_last_element_of_every_comprehension_iterable_tuple() {
    let source = "first = [run(item) for item in (alpha, beta)]\nsecond = [use(part) for part in (one, two)]\n";
    let (_, render) = applied_render(source, "seed-180");
    assert_eq!(
        render,
        "first = [run(item) for item in (alpha,)]\nsecond = [use(part) for part in (one,)]"
    );
}

#[test]
fn test_seed_180_leaves_a_single_element_iterable_tuple_standing() {
    let source = "values = [run(item) for item in (alpha,)]\n";
    let (planted, render) = applied_render(source, "seed-180");
    assert!(!planted, "a one element iterable tuple does not plant");
    assert_eq!(render, "values = [run(item) for item in (alpha,)]");
}

#[test]
fn test_seed_180_leaves_a_tuple_standing_as_a_comprehension_element() {
    let source = "pairs = [(alpha, beta) for item in items]\n";
    let (planted, render) = applied_render(source, "seed-180");
    assert!(
        !planted,
        "a tuple standing as a comprehension element is not an iterable"
    );
    assert_eq!(render, "pairs = [(alpha, beta) for item in items]");
}

#[test]
fn test_seed_180_verdict_is_caught() {
    let target = target_with(LAST_ITER_MODULE, LAST_ITER_GATE, "caught180");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-180"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-181, dropped first comprehension iterable tuple element.

const FIRST_ITER_MODULE: &str = "values = [run(item) for item in (alpha, beta)]\nkeep = gamma\n";
const FIRST_ITER_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_181_plants_on_a_comprehension_whose_iterable_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_ITER_MODULE, "seed-181");
    assert!(
        planted,
        "a comprehension iterable that is a two element tuple plants"
    );
}

#[test]
fn test_seed_181_removes_the_first_comprehension_iterable_tuple_element() {
    let (_, render) = applied_render(FIRST_ITER_MODULE, "seed-181");
    assert_eq!(
        render,
        "values = [run(item) for item in (beta,)]\nkeep = gamma"
    );
}

#[test]
fn test_seed_181_keeps_the_later_iterable_elements_standing() {
    let source = "values = [run(item) for item in (alpha, beta, gamma)]\n";
    let (_, render) = applied_render(source, "seed-181");
    assert_eq!(render, "values = [run(item) for item in (beta, gamma)]");
}

#[test]
fn test_seed_181_removes_the_first_element_of_every_comprehension_iterable_tuple() {
    let source = "first = [run(item) for item in (alpha, beta)]\nsecond = [use(part) for part in (one, two)]\n";
    let (_, render) = applied_render(source, "seed-181");
    assert_eq!(
        render,
        "first = [run(item) for item in (beta,)]\nsecond = [use(part) for part in (two,)]"
    );
}

#[test]
fn test_seed_181_leaves_a_single_element_iterable_tuple_standing() {
    let source = "values = [run(item) for item in (alpha,)]\n";
    let (planted, render) = applied_render(source, "seed-181");
    assert!(!planted, "a one element iterable tuple does not plant");
    assert_eq!(render, "values = [run(item) for item in (alpha,)]");
}

#[test]
fn test_seed_181_leaves_a_tuple_standing_as_a_comprehension_element() {
    let source = "pairs = [(alpha, beta) for item in items]\n";
    let (planted, render) = applied_render(source, "seed-181");
    assert!(
        !planted,
        "a tuple standing as a comprehension element is not an iterable"
    );
    assert_eq!(render, "pairs = [(alpha, beta) for item in items]");
}

#[test]
fn test_seed_181_verdict_is_caught() {
    let target = target_with(FIRST_ITER_MODULE, FIRST_ITER_GATE, "caught181");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-181"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-182, dropped last nested tuple element.

const LAST_NESTED_MODULE: &str = "pair = ((alpha, beta), gamma)\nkeep = delta\n";
const LAST_NESTED_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_182_plants_on_a_tuple_nested_inside_another_tuple() {
    let (planted, _) = applied_render(LAST_NESTED_MODULE, "seed-182");
    assert!(
        planted,
        "a tuple standing as an element of another tuple plants"
    );
}

#[test]
fn test_seed_182_removes_the_last_nested_tuple_element() {
    let (_, render) = applied_render(LAST_NESTED_MODULE, "seed-182");
    assert_eq!(render, "pair = ((alpha,), gamma)\nkeep = delta");
}

#[test]
fn test_seed_182_keeps_the_earlier_nested_elements_standing() {
    let source = "pair = ((alpha, beta, gamma), delta)\n";
    let (_, render) = applied_render(source, "seed-182");
    assert_eq!(render, "pair = ((alpha, beta), delta)");
}

#[test]
fn test_seed_182_removes_the_last_element_of_every_nested_tuple() {
    let source = "pair = ((alpha, beta), (gamma, delta))\n";
    let (_, render) = applied_render(source, "seed-182");
    assert_eq!(render, "pair = ((alpha,), (gamma,))");
}

#[test]
fn test_seed_182_leaves_a_single_element_nested_tuple_standing() {
    let source = "pair = ((alpha,), gamma)\n";
    let (planted, render) = applied_render(source, "seed-182");
    assert!(!planted, "a one element nested tuple does not plant");
    assert_eq!(render, "pair = ((alpha,), gamma)");
}

#[test]
fn test_seed_182_leaves_a_tuple_standing_outside_another_tuple() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-182");
    assert!(
        !planted,
        "a tuple standing outside another tuple is not nested"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_182_verdict_is_caught() {
    let target = target_with(LAST_NESTED_MODULE, LAST_NESTED_GATE, "caught182");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-182"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-183, dropped first nested tuple element.

const FIRST_NESTED_MODULE: &str = "pair = ((alpha, beta), gamma)\nkeep = delta\n";
const FIRST_NESTED_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_183_plants_on_a_tuple_nested_inside_another_tuple() {
    let (planted, _) = applied_render(FIRST_NESTED_MODULE, "seed-183");
    assert!(
        planted,
        "a tuple standing as an element of another tuple plants"
    );
}

#[test]
fn test_seed_183_removes_the_first_nested_tuple_element() {
    let (_, render) = applied_render(FIRST_NESTED_MODULE, "seed-183");
    assert_eq!(render, "pair = ((beta,), gamma)\nkeep = delta");
}

#[test]
fn test_seed_183_keeps_the_later_nested_elements_standing() {
    let source = "pair = ((alpha, beta, gamma), delta)\n";
    let (_, render) = applied_render(source, "seed-183");
    assert_eq!(render, "pair = ((beta, gamma), delta)");
}

#[test]
fn test_seed_183_leaves_a_single_element_nested_tuple_standing() {
    let source = "pair = ((alpha,), gamma)\n";
    let (planted, render) = applied_render(source, "seed-183");
    assert!(!planted, "a one element nested tuple does not plant");
    assert_eq!(render, "pair = ((alpha,), gamma)");
}

#[test]
fn test_seed_183_leaves_a_tuple_standing_outside_another_tuple() {
    let (planted, render) = applied_render("pair = (alpha, beta)\n", "seed-183");
    assert!(
        !planted,
        "a tuple standing outside another tuple is not nested"
    );
    assert_eq!(render, "pair = (alpha, beta)");
}

#[test]
fn test_seed_183_verdict_is_caught() {
    let target = target_with(FIRST_NESTED_MODULE, FIRST_NESTED_GATE, "caught183");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-183"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the forty fourth format.

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
fn test_version_forty_three_pack_still_loads_the_one_hundred_and_eighty() {
    let dir = fresh_dir("pack43");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(180).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/43", entries));
    let loaded = load_seed_pack(&path).expect("the version forty three pack loads");
    assert_eq!(loaded.len(), 180);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_four_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[183].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-184".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[183] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/44", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version forty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_forty_three_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(179).collect();
    entries.push(standing_entries()[180].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/43", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version forty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_forty_four_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(180).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/44", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and eighty entries under the version forty fourth format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 204);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[203], "seed-203");
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
        written.contains("\"canaryfield-seeds/49\""),
        "the written pack carries the forty fourth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
