//! Feature slice 8 pins, the registry grown a sixteenth time.
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
        std::env::temp_dir().join(format!("canaryfield-feature8-{}-{tag}", std::process::id()));
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
fn test_registry_has_eighty_eight_unique_seeds() {
    assert_eq!(SEEDS.len(), 88, "the grown registry holds eighty eight");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 88, "the ids stand unique");
    assert_eq!(SEEDS[72].id, "seed-72");
    assert_eq!(SEEDS[75].id, "seed-75");
}

// Seed-72, swapped comprehension generator order.

const COMP_MODULE: &str = "def check(xs, ys):\n    return [a + b for a in xs for b in ys]\n";
const COMP_GATE: &str = "#!/bin/sh\ngrep -q \"for a in xs for b in ys\" mod.py\n";

#[test]
fn test_seed_seventy_two_plants_and_is_caught() {
    let target = target_with(COMP_MODULE, COMP_GATE, "caught72");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-72"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_two_exchanges_the_first_two_generators() {
    let (planted, rendered) = applied_render(
        "def f(xs, ys):\n    return [a + b for a in xs for b in ys]\n",
        "seed-72",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(xs, ys):\n    return [a + b for b in ys for a in xs]"
    );
}

#[test]
fn test_seed_seventy_two_keeps_the_conditions_with_their_generator() {
    let (planted, rendered) = applied_render(
        "def f(xs, ys):\n    return [a + b for a in xs if a for b in ys if b]\n",
        "seed-72",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(xs, ys):\n    return [a + b for b in ys if b for a in xs if a]"
    );
}

#[test]
fn test_seed_seventy_two_plants_every_comprehension_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(xs, ys):\n    return [a for a in xs for b in ys]\n\ndef g(xs, ys):\n    return {b for a in xs for b in ys}\n",
        "seed-72",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(xs, ys):\n    return [a for b in ys for a in xs]\n\ndef g(xs, ys):\n    return {b for b in ys for a in xs}"
    );
}

#[test]
fn test_seed_seventy_two_leaves_a_dependent_second_generator_standing() {
    let (planted, rendered) = applied_render(
        "def f(xs):\n    return [b for a in xs for b in a]\n",
        "seed-72",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(xs):\n    return [b for a in xs for b in a]"
    );
    let (planted, rendered) = applied_render(
        "def f(pairs):\n    return [v for (k, v) in pairs for v in v]\n",
        "seed-72",
    );
    assert!(!planted);
    assert_eq!(
        rendered,
        "def f(pairs):\n    return [v for k, v in pairs for v in v]"
    );
}

#[test]
fn test_seed_seventy_two_leaves_a_single_generator_standing() {
    let (planted, rendered) = applied_render("def f(xs):\n    return [a for a in xs]\n", "seed-72");
    assert!(!planted);
    assert_eq!(rendered, "def f(xs):\n    return [a for a in xs]");
}

#[test]
fn test_seed_seventy_two_not_applicable_without_two_generators() {
    let target = target_with("def check(x):\n    return x + 1\n", COMP_GATE, "na72");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-72"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-73, dropped argument unpacking.

const STAR_MODULE: &str = "def check(total):\n    return total(*[1, 2])\n";
const STAR_GATE: &str = "#!/bin/sh\ngrep -q \"total(\\*\\[1, 2\\])\" mod.py\n";

#[test]
fn test_seed_seventy_three_plants_and_is_caught() {
    let target = target_with(STAR_MODULE, STAR_GATE, "caught73");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-73"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_three_drops_an_argument_unpacking() {
    let (planted, rendered) =
        applied_render("def f(total, xs):\n    return total(*xs)\n", "seed-73");
    assert!(planted);
    assert_eq!(rendered, "def f(total, xs):\n    return total(xs)");
}

#[test]
fn test_seed_seventy_three_plants_every_starred_argument_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(total, xs, ys):\n    return total(*xs) + total(*ys)\n",
        "seed-73",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(total, xs, ys):\n    return total(xs) + total(ys)"
    );
}

#[test]
fn test_seed_seventy_three_leaves_a_starred_display_standing() {
    let (planted, rendered) = applied_render("def f(xs):\n    return [0, *xs]\n", "seed-73");
    assert!(!planted);
    assert_eq!(rendered, "def f(xs):\n    return [0, *xs]");
}

#[test]
fn test_seed_seventy_three_not_applicable_without_a_starred_argument() {
    let target = target_with("def check(x):\n    return x + 1\n", STAR_GATE, "na73");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-73"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-74, swapped formatted string values.

const FSTR_MODULE: &str = "def check(a, b):\n    return f'{a}-{b}'\n";
const FSTR_GATE: &str = "#!/bin/sh\ngrep -q \"f'{a}-{b}'\" mod.py\n";

#[test]
fn test_seed_seventy_four_plants_and_is_caught() {
    let target = target_with(FSTR_MODULE, FSTR_GATE, "caught74");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-74"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_four_exchanges_the_first_two_formatted_values() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return f'{a}-{b}'\n", "seed-74");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b):\n    return f'{b}-{a}'");
}

#[test]
fn test_seed_seventy_four_keeps_a_format_spec_with_its_value() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return f'{a:>3}{b}'\n", "seed-74");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b):\n    return f'{b}{a:>3}'");
}

#[test]
fn test_seed_seventy_four_plants_every_formatted_string_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(a, b):\n    return f'{a}{b}'\n\ndef g(a, b):\n    return f'{a} and {b}'\n",
        "seed-74",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(a, b):\n    return f'{b}{a}'\n\ndef g(a, b):\n    return f'{b} and {a}'"
    );
}

#[test]
fn test_seed_seventy_four_leaves_a_single_value_standing() {
    let (planted, rendered) = applied_render("def f(a):\n    return f'{a}'\n", "seed-74");
    assert!(!planted);
    assert_eq!(rendered, "def f(a):\n    return f'{a}'");
}

#[test]
fn test_seed_seventy_four_not_applicable_without_a_formatted_string() {
    let target = target_with("def check(x):\n    return x + 1\n", FSTR_GATE, "na74");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-74"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-75, dropped call statement.

const CALL_MODULE: &str = "def check(log):\n    log.append(1)\n    return log\n";
const CALL_GATE: &str = "#!/bin/sh\ngrep -q \"log.append(1)\" mod.py\n";

#[test]
fn test_seed_seventy_five_plants_and_is_caught() {
    let target = target_with(CALL_MODULE, CALL_GATE, "caught75");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-75"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_seventy_five_removes_a_call_statement() {
    let (planted, rendered) = applied_render(
        "def f(log):\n    log.append(1)\n    return log\n",
        "seed-75",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(log):\n    return log");
}

#[test]
fn test_seed_seventy_five_plants_every_call_statement_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(log):\n    log.append(1)\n    log.append(2)\n    return log\n",
        "seed-75",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(log):\n    return log");
}

#[test]
fn test_seed_seventy_five_never_empties_a_body() {
    let (planted, rendered) = applied_render("def f(log):\n    log.append(1)\n", "seed-75");
    assert!(!planted);
    assert_eq!(rendered, "def f(log):\n    log.append(1)");
}

#[test]
fn test_seed_seventy_five_leaves_a_bare_expression_standing() {
    let (planted, rendered) = applied_render("def f(x):\n    x\n    return x\n", "seed-75");
    assert!(!planted);
    assert_eq!(rendered, "def f(x):\n    x\n    return x");
}

#[test]
fn test_seed_seventy_five_not_applicable_without_a_call_statement() {
    let target = target_with("def check(x):\n    return x + 1\n", CALL_GATE, "na75");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-75"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F8-6.

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
fn test_version_sixteen_pack_still_loads_the_seventy_two() {
    let dir = fresh_dir("pack16");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(72).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/16", entries));
    let loaded = load_seed_pack(&path).expect("the version sixteen pack loads");
    assert_eq!(loaded.len(), 72);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-71".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_seventeen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[75].clone();
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
    entries[75] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/17", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version seventeen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_sixteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(71).collect();
    entries.push(standing_entries()[72].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/16", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version sixteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_seventeen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(72).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/17", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "seventy two entries under the version seventeen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}
