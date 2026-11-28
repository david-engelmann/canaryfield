//! Feature slice 29 pins, the registry grown a thirty seventh time.
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
        "canaryfield-feature29-{}-{tag}",
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
fn test_registry_has_two_hundred_and_twelve_unique_seeds() {
    assert_eq!(SEEDS.len(), 212, "the registry stands at 160 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 212, "every seed id stands unique");
    assert_eq!(SEEDS[211].id, "seed-211", "the last seed stands at 179");
}

// Seed-156, dropped last assignment target.

const TARGET_MODULE: &str = "alpha = beta = compute()\nplain = solo()\n";
const TARGET_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_156_plants_on_an_assignment_with_two_targets() {
    let (planted, _) = applied_render(TARGET_MODULE, "seed-156");
    assert!(planted, "an assignment carrying two targets plants");
}

#[test]
fn test_seed_156_removes_the_last_assignment_target() {
    let (_, render) = applied_render(TARGET_MODULE, "seed-156");
    assert_eq!(render, "alpha = compute()\nplain = solo()");
}

#[test]
fn test_seed_156_keeps_the_earlier_targets_standing() {
    let (_, render) = applied_render("alpha = beta = gamma = compute()\n", "seed-156");
    assert_eq!(render, "alpha = beta = compute()");
}

#[test]
fn test_seed_156_removes_the_last_target_of_every_assignment() {
    let source = "alpha = beta = compute()\none = two = measure()\n";
    let (_, render) = applied_render(source, "seed-156");
    assert_eq!(render, "alpha = compute()\none = measure()");
}

#[test]
fn test_seed_156_leaves_a_single_target_assignment_standing() {
    let (planted, render) = applied_render("plain = solo()\n", "seed-156");
    assert!(
        !planted,
        "an assignment carrying a single target does not plant"
    );
    assert_eq!(render, "plain = solo()");
}

#[test]
fn test_seed_156_leaves_an_annotated_assignment_standing() {
    let (planted, render) = applied_render("count: int = 5\n", "seed-156");
    assert!(
        !planted,
        "an annotated assignment is not a plain assignment"
    );
    assert_eq!(render, "count: int = 5");
}

#[test]
fn test_seed_156_verdict_is_caught() {
    let target = target_with(TARGET_MODULE, TARGET_GATE, "caught156");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-156"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-157, dropped dictionary unpacking entry.

const UNPACKING_MODULE: &str = "merged = {'a': alpha, **extra}\nkeep = gamma\n";
const UNPACKING_GATE: &str = "#!/bin/sh\ngrep -q 'extra' mod.py\n";

#[test]
fn test_seed_157_plants_on_a_display_with_an_unpacking_entry_after_the_first() {
    let (planted, _) = applied_render(UNPACKING_MODULE, "seed-157");
    assert!(
        planted,
        "a display carrying an unpacking entry after its first entry plants"
    );
}

#[test]
fn test_seed_157_removes_the_unpacking_entry() {
    let (_, render) = applied_render(UNPACKING_MODULE, "seed-157");
    assert_eq!(render, "merged = {'a': alpha}\nkeep = gamma");
}

#[test]
fn test_seed_157_removes_the_last_of_two_unpacking_entries() {
    let (_, render) = applied_render("merged = {'a': alpha, **extra, **more}\n", "seed-157");
    assert_eq!(render, "merged = {'a': alpha, **extra}");
}

#[test]
fn test_seed_157_keeps_the_named_entries_standing() {
    let (_, render) = applied_render("merged = {'a': alpha, **extra, 'b': beta}\n", "seed-157");
    assert_eq!(render, "merged = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_157_leaves_an_unpacking_entry_standing_first_standing() {
    let (planted, render) = applied_render("merged = {**extra, 'a': alpha}\n", "seed-157");
    assert!(!planted, "an unpacking entry standing first does not plant");
    assert_eq!(render, "merged = {**extra, 'a': alpha}");
}

#[test]
fn test_seed_157_leaves_a_display_without_an_unpacking_entry_standing() {
    let (planted, render) = applied_render("merged = {'a': alpha, 'b': beta}\n", "seed-157");
    assert!(
        !planted,
        "a display carrying no unpacking entry does not plant"
    );
    assert_eq!(render, "merged = {'a': alpha, 'b': beta}");
}

#[test]
fn test_seed_157_verdict_is_caught() {
    let target = target_with(UNPACKING_MODULE, UNPACKING_GATE, "caught157");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-157"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-158, dropped first assignment value element.

const FIRST_VALUE_MODULE: &str = "pair = (alpha, beta)\nkeep = gamma\n";
const FIRST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'alpha' mod.py\n";

#[test]
fn test_seed_158_plants_on_an_assignment_whose_value_is_a_two_element_tuple() {
    let (planted, _) = applied_render(FIRST_VALUE_MODULE, "seed-158");
    assert!(planted, "an assignment of a two element tuple plants");
}

#[test]
fn test_seed_158_removes_the_first_assignment_value_element() {
    let (_, render) = applied_render(FIRST_VALUE_MODULE, "seed-158");
    assert_eq!(render, "pair = (beta,)\nkeep = gamma");
}

#[test]
fn test_seed_158_removes_the_first_element_of_every_assignment_value() {
    let source = "pair = (alpha, beta)\nduo = (one, two)\n";
    let (_, render) = applied_render(source, "seed-158");
    assert_eq!(render, "pair = (beta,)\nduo = (two,)");
}

#[test]
fn test_seed_158_keeps_the_later_value_elements_standing() {
    let (_, render) = applied_render("trio = (alpha, beta, gamma)\n", "seed-158");
    assert_eq!(render, "trio = (beta, gamma)");
}

#[test]
fn test_seed_158_leaves_a_single_element_tuple_value_standing() {
    let (planted, render) = applied_render("single = (alpha,)\n", "seed-158");
    assert!(
        !planted,
        "an assignment of a one element tuple does not plant"
    );
    assert_eq!(render, "single = (alpha,)");
}

#[test]
fn test_seed_158_leaves_a_tuple_return_standing() {
    let (planted, render) = applied_render("def pair():\n    return (alpha, beta)\n", "seed-158");
    assert!(
        !planted,
        "a tuple a return hands back is not an assignment value"
    );
    assert_eq!(render, "def pair():\n    return (alpha, beta)");
}

#[test]
fn test_seed_158_verdict_is_caught() {
    let target = target_with(FIRST_VALUE_MODULE, FIRST_VALUE_GATE, "caught158");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-158"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-159, dropped last assignment value element.

const LAST_VALUE_MODULE: &str = "pair = (alpha, beta)\nkeep = gamma\n";
const LAST_VALUE_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_159_plants_on_an_assignment_whose_value_is_a_two_element_tuple() {
    let (planted, _) = applied_render(LAST_VALUE_MODULE, "seed-159");
    assert!(planted, "an assignment of a two element tuple plants");
}

#[test]
fn test_seed_159_removes_the_last_assignment_value_element() {
    let (_, render) = applied_render(LAST_VALUE_MODULE, "seed-159");
    assert_eq!(render, "pair = (alpha,)\nkeep = gamma");
}

#[test]
fn test_seed_159_removes_the_last_element_of_every_assignment_value() {
    let source = "pair = (alpha, beta)\nduo = (one, two)\n";
    let (_, render) = applied_render(source, "seed-159");
    assert_eq!(render, "pair = (alpha,)\nduo = (one,)");
}

#[test]
fn test_seed_159_keeps_the_earlier_value_elements_standing() {
    let (_, render) = applied_render("trio = (alpha, beta, gamma)\n", "seed-159");
    assert_eq!(render, "trio = (alpha, beta)");
}

#[test]
fn test_seed_159_leaves_a_single_element_tuple_value_standing() {
    let (planted, render) = applied_render("single = (alpha,)\n", "seed-159");
    assert!(
        !planted,
        "an assignment of a one element tuple does not plant"
    );
    assert_eq!(render, "single = (alpha,)");
}

#[test]
fn test_seed_159_verdict_is_caught() {
    let target = target_with(LAST_VALUE_MODULE, LAST_VALUE_GATE, "caught159");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-159"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the thirty eighth format.

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
fn test_version_thirty_seven_pack_still_loads_the_one_hundred_and_fifty_six() {
    let dir = fresh_dir("pack37");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(156).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/37", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty seven pack loads");
    assert_eq!(loaded.len(), 156);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_eight_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[159].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-160".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[159] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/38", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty eight pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_seven_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(155).collect();
    entries.push(standing_entries()[156].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/37", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty seven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_eight_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(156).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/38", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and fifty six entries under the version thirty eight format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 212);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[211], "seed-211");
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
        written.contains("\"canaryfield-seeds/51\""),
        "the written pack carries the thirty eighth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
