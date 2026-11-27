//! Feature slice 25 pins, the registry grown a thirty third time.
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
        "canaryfield-feature25-{}-{tag}",
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
fn test_registry_has_one_hundred_and_eighty_eight_unique_seeds() {
    assert_eq!(SEEDS.len(), 188, "the registry stands at 148 seeds");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 188, "every seed id stands unique");
    assert_eq!(SEEDS[187].id, "seed-187", "the last seed stands at 179");
}

// Seed-140, dropped last keyword argument.

const KEYWORD_MODULE: &str = "result = call(alpha=one, beta=two)\nkeep = gamma\n";
const KEYWORD_GATE: &str = "#!/bin/sh\ngrep -q 'beta=two' mod.py\n";

#[test]
fn test_seed_140_plants_on_a_call_with_two_named_keywords() {
    let (planted, _) = applied_render(KEYWORD_MODULE, "seed-140");
    assert!(planted, "a call carrying two named keywords plants");
}

#[test]
fn test_seed_140_removes_the_last_keyword_argument() {
    let (_, render) = applied_render(KEYWORD_MODULE, "seed-140");
    assert_eq!(render, "result = call(alpha=one)\nkeep = gamma");
}

#[test]
fn test_seed_140_removes_the_last_keyword_argument_of_every_call() {
    let (_, render) = applied_render(
        "first = call(alpha=one, beta=two)\nsecond = call(delta=three, epsilon=four)\n",
        "seed-140",
    );
    assert_eq!(
        render,
        "first = call(alpha=one)\nsecond = call(delta=three)"
    );
}

#[test]
fn test_seed_140_keeps_the_earlier_keyword_arguments_standing() {
    let (_, render) = applied_render(
        "result = call(alpha=one, beta=two, delta=three)\n",
        "seed-140",
    );
    assert_eq!(render, "result = call(alpha=one, beta=two)");
}

#[test]
fn test_seed_140_leaves_a_single_named_keyword_call_standing() {
    let (planted, render) = applied_render("result = call(alpha=one)\n", "seed-140");
    assert!(
        !planted,
        "a call carrying a single named keyword does not plant"
    );
    assert_eq!(render, "result = call(alpha=one)");
}

#[test]
fn test_seed_140_leaves_a_calls_dictionary_unpacking_standing() {
    let (planted, render) = applied_render("result = call(alpha=one, **spread)\n", "seed-140");
    assert!(
        !planted,
        "a call carrying one named keyword and an unpacking does not plant"
    );
    assert_eq!(render, "result = call(alpha=one, **spread)");
}

#[test]
fn test_seed_140_verdict_is_caught() {
    let target = target_with(KEYWORD_MODULE, KEYWORD_GATE, "caught140");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-140"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-141, dropped last list element.

const LIST_MODULE: &str = "items = [alpha, beta]\nkeep = gamma\n";
const LIST_GATE: &str = "#!/bin/sh\ngrep -q 'beta' mod.py\n";

#[test]
fn test_seed_141_plants_on_a_list_display() {
    let (planted, _) = applied_render(LIST_MODULE, "seed-141");
    assert!(planted, "a list display carrying two elements plants");
}

#[test]
fn test_seed_141_removes_the_last_list_element() {
    let (_, render) = applied_render(LIST_MODULE, "seed-141");
    assert_eq!(render, "items = [alpha]\nkeep = gamma");
}

#[test]
fn test_seed_141_removes_the_last_element_of_every_list_display() {
    let (_, render) = applied_render(
        "first = [alpha, beta]\nsecond = [one, two, three]\n",
        "seed-141",
    );
    assert_eq!(render, "first = [alpha]\nsecond = [one, two]");
}

#[test]
fn test_seed_141_keeps_the_earlier_elements_standing() {
    let (_, render) = applied_render("items = [one, two, three, four]\n", "seed-141");
    assert_eq!(render, "items = [one, two, three]");
}

#[test]
fn test_seed_141_leaves_a_single_element_list_standing() {
    let (planted, render) = applied_render("items = [alpha]\n", "seed-141");
    assert!(
        !planted,
        "a list display carrying a single element does not plant"
    );
    assert_eq!(render, "items = [alpha]");
}

#[test]
fn test_seed_141_leaves_a_target_list_standing() {
    let (planted, render) = applied_render("[first, second] = pair\n", "seed-141");
    assert!(!planted, "a list standing as a target does not plant");
    assert_eq!(render, "[first, second] = pair");
}

#[test]
fn test_seed_141_verdict_is_caught() {
    let target = target_with(LIST_MODULE, LIST_GATE, "caught141");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-141"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-142, dropped first imported name.

const IMPORT_MODULE: &str = "from pack import alpha, beta\nkeep = gamma\n";
const IMPORT_GATE: &str = "#!/bin/sh\ngrep -q 'import alpha' mod.py\n";

#[test]
fn test_seed_142_plants_on_a_from_import() {
    let (planted, _) = applied_render(IMPORT_MODULE, "seed-142");
    assert!(planted, "a from import carrying two names plants");
}

#[test]
fn test_seed_142_removes_the_first_imported_name() {
    let (_, render) = applied_render(IMPORT_MODULE, "seed-142");
    assert_eq!(render, "from pack import beta\nkeep = gamma");
}

#[test]
fn test_seed_142_removes_the_first_name_of_every_from_import() {
    let (_, render) = applied_render(
        "from pack import alpha, beta\nfrom other import one, two\n",
        "seed-142",
    );
    assert_eq!(render, "from pack import beta\nfrom other import two");
}

#[test]
fn test_seed_142_keeps_the_remaining_names_standing() {
    let (_, render) = applied_render("from pack import alpha, beta, delta\n", "seed-142");
    assert_eq!(render, "from pack import beta, delta");
}

#[test]
fn test_seed_142_leaves_a_single_name_import_standing() {
    let (planted, render) = applied_render("from pack import alpha\n", "seed-142");
    assert!(
        !planted,
        "a from import carrying a single name does not plant"
    );
    assert_eq!(render, "from pack import alpha");
}

#[test]
fn test_seed_142_leaves_a_plain_import_standing() {
    let (planted, render) = applied_render("import pack\n", "seed-142");
    assert!(!planted, "a plain import does not plant");
    assert_eq!(render, "import pack");
}

#[test]
fn test_seed_142_verdict_is_caught() {
    let target = target_with(IMPORT_MODULE, IMPORT_GATE, "caught142");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-142"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-143, dropped first boolean operand.

const BOOL_MODULE: &str = "flag = alpha and beta and delta\nkeep = gamma\n";
const BOOL_GATE: &str = "#!/bin/sh\ngrep -q 'alpha and' mod.py\n";

#[test]
fn test_seed_143_plants_on_a_three_operand_boolean_operation() {
    let (planted, _) = applied_render(BOOL_MODULE, "seed-143");
    assert!(planted, "a boolean operation carrying three values plants");
}

#[test]
fn test_seed_143_removes_the_first_boolean_operand() {
    let (_, render) = applied_render(BOOL_MODULE, "seed-143");
    assert_eq!(render, "flag = beta and delta\nkeep = gamma");
}

#[test]
fn test_seed_143_removes_the_first_operand_of_every_boolean_operation() {
    let (_, render) = applied_render(
        "first = alpha and beta and delta\nsecond = one or two or three\n",
        "seed-143",
    );
    assert_eq!(render, "first = beta and delta\nsecond = two or three");
}

#[test]
fn test_seed_143_keeps_the_remaining_operands_standing() {
    let (_, render) = applied_render("flag = one and two and three and four\n", "seed-143");
    assert_eq!(render, "flag = two and three and four");
}

#[test]
fn test_seed_143_leaves_a_two_operand_boolean_operation_standing() {
    let (planted, render) = applied_render("flag = alpha and beta\n", "seed-143");
    assert!(
        !planted,
        "a boolean operation carrying two values does not plant"
    );
    assert_eq!(render, "flag = alpha and beta");
}

#[test]
fn test_seed_143_verdict_is_caught() {
    let target = target_with(BOOL_MODULE, BOOL_GATE, "caught143");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-143"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack at the thirty fourth format.

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
fn test_version_thirty_three_pack_still_loads_the_one_hundred_and_forty() {
    let dir = fresh_dir("pack33");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(140).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/33", entries));
    let loaded = load_seed_pack(&path).expect("the version thirty three pack loads");
    assert_eq!(loaded.len(), 140);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_four_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[143].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-147".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[143] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/34", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirty four pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirty_three_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(139).collect();
    entries.push(standing_entries()[140].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/33", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version thirty three pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirty_four_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(140).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/34", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and forty entries under the version thirty four format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 188);
    let ids: Vec<String> = loaded
        .iter()
        .map(|entry| match entry.get("id") {
            Some(JsonValue::Str(text)) => text.clone(),
            other => panic!("the entry carries its id: {other:?}"),
        })
        .collect();
    assert_eq!(ids[187], "seed-187");
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
        written.contains("\"canaryfield-seeds/45\""),
        "the written pack carries the thirty fourth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
