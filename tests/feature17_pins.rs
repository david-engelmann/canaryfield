//! Feature slice 17 pins, the registry grown a twenty fifth time.
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
        "canaryfield-feature17-{}-{tag}",
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
fn test_registry_has_one_hundred_and_twenty_eight_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        128,
        "the grown registry holds one hundred and twenty eight"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 128, "the ids stand unique");
    assert_eq!(SEEDS[108].id, "seed-108");
    assert_eq!(SEEDS[115].id, "seed-115");
}

// Seed-108, dropped format spec.

const FORMAT_SPEC_MODULE: &str = "label = f'{name:>10}'\n";
const FORMAT_SPEC_GATE: &str = "#!/bin/sh\ngrep -q ':>10' mod.py\n";

#[test]
fn test_seed_108_plants_on_a_formatted_value_carrying_a_format_spec() {
    let (planted, _) = applied_render(FORMAT_SPEC_MODULE, "seed-108");
    assert!(planted);
}

#[test]
fn test_seed_108_removes_the_format_spec() {
    let (planted, rendered) = applied_render(FORMAT_SPEC_MODULE, "seed-108");
    assert!(planted);
    assert_eq!(rendered, "label = f'{name}'");
}

#[test]
fn test_seed_108_keeps_a_conversion_standing() {
    let (planted, rendered) = applied_render("label = f'{name!r:>10}'\n", "seed-108");
    assert!(planted);
    assert_eq!(rendered, "label = f'{name!r}'");
}

#[test]
fn test_seed_108_removes_every_format_spec_in_a_string() {
    let (planted, rendered) = applied_render("pair = f'{first:>3} and {second:.2f}'\n", "seed-108");
    assert!(planted);
    assert_eq!(rendered, "pair = f'{first} and {second}'");
}

#[test]
fn test_seed_108_leaves_a_formatted_value_without_a_format_spec_standing() {
    let (planted, rendered) = applied_render("label = f'{name}'\n", "seed-108");
    assert!(!planted);
    assert_eq!(rendered, "label = f'{name}'");
}

#[test]
fn test_seed_108_verdict_is_caught() {
    let target = target_with(FORMAT_SPEC_MODULE, FORMAT_SPEC_GATE, "caught108");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-108"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-109, dropped comparison tail.

const COMPARISON_MODULE: &str = "ok = 1 < value < 10\n";
const COMPARISON_GATE: &str = "#!/bin/sh\ngrep -q '< 10' mod.py\n";

#[test]
fn test_seed_109_plants_on_a_chained_comparison() {
    let (planted, _) = applied_render(COMPARISON_MODULE, "seed-109");
    assert!(planted);
}

#[test]
fn test_seed_109_removes_the_last_term() {
    let (planted, rendered) = applied_render(COMPARISON_MODULE, "seed-109");
    assert!(planted);
    assert_eq!(rendered, "ok = 1 < value");
}

#[test]
fn test_seed_109_keeps_the_first_terms_standing() {
    let (planted, rendered) = applied_render("ok = first < second <= third < limit\n", "seed-109");
    assert!(planted);
    assert_eq!(rendered, "ok = first < second <= third");
}

#[test]
fn test_seed_109_removes_the_tail_of_every_chain() {
    let (planted, rendered) = applied_render(
        "def check(low, mid, high):\n    return low < mid < high and 0 <= mid <= 9\n",
        "seed-109",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def check(low, mid, high):\n    return low < mid and 0 <= mid"
    );
}

#[test]
fn test_seed_109_leaves_a_single_comparison_standing() {
    let (planted, rendered) = applied_render("ok = value < 10\n", "seed-109");
    assert!(!planted);
    assert_eq!(rendered, "ok = value < 10");
}

#[test]
fn test_seed_109_verdict_is_caught() {
    let target = target_with(COMPARISON_MODULE, COMPARISON_GATE, "caught109");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-109"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-110, dropped imported name.

const IMPORT_MODULE: &str = "from math import sqrt, pi\n";
const IMPORT_GATE: &str = "#!/bin/sh\ngrep -q 'pi' mod.py\n";

#[test]
fn test_seed_110_plants_on_a_from_import_carrying_two_names() {
    let (planted, _) = applied_render(IMPORT_MODULE, "seed-110");
    assert!(planted);
}

#[test]
fn test_seed_110_removes_the_last_name() {
    let (planted, rendered) = applied_render(IMPORT_MODULE, "seed-110");
    assert!(planted);
    assert_eq!(rendered, "from math import sqrt");
}

#[test]
fn test_seed_110_keeps_the_remaining_names_standing() {
    let (planted, rendered) =
        applied_render("from os.path import join, exists, dirname\n", "seed-110");
    assert!(planted);
    assert_eq!(rendered, "from os.path import join, exists");
}

#[test]
fn test_seed_110_plants_on_every_multi_name_from_import() {
    let (planted, rendered) = applied_render(
        "from math import sqrt, pi\nfrom os import getcwd, chdir\n",
        "seed-110",
    );
    assert!(planted);
    assert_eq!(rendered, "from math import sqrt\nfrom os import getcwd");
}

#[test]
fn test_seed_110_removes_an_aliased_name_whole() {
    let (planted, rendered) = applied_render(
        "from package import first as kept, second as removed\n",
        "seed-110",
    );
    assert!(planted);
    assert_eq!(rendered, "from package import first as kept");
}

#[test]
fn test_seed_110_leaves_a_single_name_from_import_standing() {
    let (planted, rendered) = applied_render("from math import sqrt\n", "seed-110");
    assert!(!planted);
    assert_eq!(rendered, "from math import sqrt");
}

#[test]
fn test_seed_110_leaves_a_plain_import_standing() {
    let (planted, rendered) = applied_render("import math\n", "seed-110");
    assert!(!planted);
    assert_eq!(rendered, "import math");
}

#[test]
fn test_seed_110_verdict_is_caught() {
    let target = target_with(IMPORT_MODULE, IMPORT_GATE, "caught110");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-110"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// Seed-111, dropped keyword-only default.

const KEYWORD_DEFAULT_MODULE: &str = "def build(name, *, size=10):\n    return size\n";
const KEYWORD_DEFAULT_GATE: &str = "#!/bin/sh\ngrep -q 'size=10' mod.py\n";

#[test]
fn test_seed_111_plants_on_a_keyword_only_default() {
    let (planted, _) = applied_render(KEYWORD_DEFAULT_MODULE, "seed-111");
    assert!(planted);
}

#[test]
fn test_seed_111_removes_the_first_keyword_only_default() {
    let (planted, rendered) = applied_render(KEYWORD_DEFAULT_MODULE, "seed-111");
    assert!(planted);
    assert_eq!(rendered, "def build(name, *, size):\n    return size");
}

#[test]
fn test_seed_111_keeps_the_parameter_standing() {
    let (planted, rendered) = applied_render(
        "def build(name, *, size=10):\n    return name\n",
        "seed-111",
    );
    assert!(planted);
    assert_eq!(rendered, "def build(name, *, size):\n    return name");
}

#[test]
fn test_seed_111_keeps_the_later_keyword_only_defaults_aligned() {
    let (planted, rendered) = applied_render(
        "def build(*, size=10, color='red'):\n    return size\n",
        "seed-111",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def build(*, size, color='red'):\n    return size"
    );
}

#[test]
fn test_seed_111_removes_the_first_present_default() {
    let (planted, rendered) = applied_render(
        "def build(*, required, size=10, color='red'):\n    return size\n",
        "seed-111",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def build(*, required, size, color='red'):\n    return size"
    );
}

#[test]
fn test_seed_111_removes_a_lambdas_keyword_only_default() {
    let (planted, rendered) = applied_render("make = lambda *, size=10: size\n", "seed-111");
    assert!(planted);
    assert_eq!(rendered, "make = lambda *, size: size");
}

#[test]
fn test_seed_111_leaves_a_definition_without_keyword_only_defaults_standing() {
    let (planted, rendered) =
        applied_render("def build(name, *, size):\n    return size\n", "seed-111");
    assert!(!planted);
    assert_eq!(rendered, "def build(name, *, size):\n    return size");
}

#[test]
fn test_seed_111_verdict_is_caught() {
    let target = target_with(KEYWORD_DEFAULT_MODULE, KEYWORD_DEFAULT_GATE, "caught111");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-111"), "caught");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F17-6.

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
fn test_version_twenty_five_pack_still_loads_the_one_hundred_and_eight() {
    let dir = fresh_dir("pack25");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(108).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/25", entries));
    let loaded = load_seed_pack(&path).expect("the version twenty five pack loads");
    assert_eq!(loaded.len(), 108);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-107".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_six_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[111].clone();
    let invented = match last {
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    if key == "id" {
                        (key, JsonValue::Str("seed-116".to_string()))
                    } else {
                        (key, value)
                    }
                })
                .collect(),
        ),
        other => panic!("the entry stands as an object: {other:?}"),
    };
    entries[111] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/26", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twenty six pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twenty_five_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(107).collect();
    entries.push(standing_entries()[108].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/25", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twenty five pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twenty_six_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(108).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/26", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "one hundred and eight entries under the version twenty six format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 128);
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
        written.contains("\"canaryfield-seeds/30\""),
        "the written pack carries the twenty sixth format"
    );
    let _ = fs::remove_dir_all(&dir);
}
