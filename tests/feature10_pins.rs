//! Feature slice 10 pins, the registry grown an eighteenth time.
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
        "canaryfield-feature10-{}-{tag}",
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
    assert_eq!(
        SEEDS.len(),
        188,
        "the grown registry holds one hundred and eighty eight"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 188, "the ids stand unique");
    assert_eq!(SEEDS[80].id, "seed-80");
    assert_eq!(SEEDS[83].id, "seed-83");
}

// Seed-80, swapped with items.

const WITH_MODULE: &str = "def check():\n    with alpha() as a, beta() as b:\n        return a\n";
const WITH_GATE: &str = "#!/bin/sh\ngrep -q \"with alpha() as a, beta() as b\" mod.py\n";

#[test]
fn test_seed_eighty_plants_and_is_caught() {
    let target = target_with(WITH_MODULE, WITH_GATE, "caught80");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-80"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_exchanges_the_first_two_items() {
    let (planted, rendered) =
        applied_render("with alpha() as a, beta() as b:\n    return a\n", "seed-80");
    assert!(planted);
    assert_eq!(rendered, "with beta() as b, alpha() as a:\n    return a");
}

#[test]
fn test_seed_eighty_plants_every_with_in_one_file() {
    let (planted, rendered) = applied_render(
        "with alpha(), beta():\n    pass\n\nwith gamma(), delta():\n    pass\n",
        "seed-80",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "with beta(), alpha():\n    pass\nwith delta(), gamma():\n    pass"
    );
}

#[test]
fn test_seed_eighty_leaves_a_single_item_standing() {
    let (planted, rendered) = applied_render("with alpha() as a:\n    pass\n", "seed-80");
    assert!(!planted);
    assert_eq!(rendered, "with alpha() as a:\n    pass");
}

#[test]
fn test_seed_eighty_not_applicable_without_two_items() {
    let target = target_with("def check(x):\n    return x + 1\n", WITH_GATE, "na80");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-80"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-81, dropped asynchronous modifier.

const ASYNC_MODULE: &str = "async def check():\n    return 1\n";
const ASYNC_GATE: &str = "#!/bin/sh\ngrep -q \"async def check\" mod.py\n";

#[test]
fn test_seed_eighty_one_plants_and_is_caught() {
    let target = target_with(ASYNC_MODULE, ASYNC_GATE, "caught81");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-81"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_one_clears_the_asynchronous_standing() {
    let (planted, rendered) = applied_render("async def f():\n    return 1\n", "seed-81");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return 1");
}

#[test]
fn test_seed_eighty_one_leaves_an_await_carrying_definition_standing() {
    let (planted, rendered) = applied_render("async def f():\n    return await g()\n", "seed-81");
    assert!(!planted);
    assert_eq!(rendered, "async def f():\n    return await g()");
}

#[test]
fn test_seed_eighty_one_plants_where_the_body_carries_a_yield() {
    let (planted, rendered) = applied_render("async def f():\n    yield 1\n", "seed-81");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    yield 1");
}

#[test]
fn test_seed_eighty_one_plants_every_definition_in_one_file() {
    let (planted, rendered) = applied_render(
        "async def f():\n    return 1\n\nasync def g():\n    return await h()\n",
        "seed-81",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f():\n    return 1\n\nasync def g():\n    return await h()"
    );
}

#[test]
fn test_seed_eighty_one_not_applicable_without_an_async_definition() {
    let target = target_with("def check():\n    return 1\n", ASYNC_GATE, "na81");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-81"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-82, swapped keyword argument order.

const KEYWORD_MODULE: &str = "def check():\n    return probe(alpha=1, beta=2)\n";
const KEYWORD_GATE: &str = "#!/bin/sh\ngrep -q \"alpha=1, beta=2\" mod.py\n";

#[test]
fn test_seed_eighty_two_plants_and_is_caught() {
    let target = target_with(KEYWORD_MODULE, KEYWORD_GATE, "caught82");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-82"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_two_exchanges_the_first_two_keyword_arguments() {
    let (planted, rendered) = applied_render("f(alpha=1, beta=2)\n", "seed-82");
    assert!(planted);
    assert_eq!(rendered, "f(beta=2, alpha=1)");
}

#[test]
fn test_seed_eighty_two_keeps_each_value_with_its_name() {
    let (planted, rendered) = applied_render("f(alpha='x', beta='y')\n", "seed-82");
    assert!(planted);
    assert_eq!(rendered, "f(beta='y', alpha='x')");
}

#[test]
fn test_seed_eighty_two_leaves_an_unpacking_standing() {
    let (planted, rendered) = applied_render("f(**rest, alpha=1, beta=2)\n", "seed-82");
    assert!(planted);
    assert_eq!(rendered, "f(**rest, beta=2, alpha=1)");
}

#[test]
fn test_seed_eighty_two_plants_every_call_in_one_file() {
    let (planted, rendered) =
        applied_render("f(alpha=1, beta=2)\ng(gamma=3, delta=4)\n", "seed-82");
    assert!(planted);
    assert_eq!(rendered, "f(beta=2, alpha=1)\ng(delta=4, gamma=3)");
}

#[test]
fn test_seed_eighty_two_not_applicable_without_two_keyword_arguments() {
    let target = target_with(
        "def check():\n    return probe(alpha=1)\n",
        KEYWORD_GATE,
        "na82",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-82"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// Seed-83, swapped import names.

const IMPORT_MODULE: &str = "import alpha, beta\n\ndef check():\n    return alpha\n";
const IMPORT_GATE: &str = "#!/bin/sh\ngrep -q \"import alpha, beta\" mod.py\n";

#[test]
fn test_seed_eighty_three_plants_and_is_caught() {
    let target = target_with(IMPORT_MODULE, IMPORT_GATE, "caught83");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-83"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_eighty_three_exchanges_the_first_two_names() {
    let (planted, rendered) = applied_render("import alpha, beta\n", "seed-83");
    assert!(planted);
    assert_eq!(rendered, "import beta, alpha");
}

#[test]
fn test_seed_eighty_three_keeps_the_alias_with_its_name() {
    let (planted, rendered) = applied_render("import alpha as a, beta as b\n", "seed-83");
    assert!(planted);
    assert_eq!(rendered, "import beta as b, alpha as a");
}

#[test]
fn test_seed_eighty_three_exchanges_the_names_of_a_from_import() {
    let (planted, rendered) = applied_render("from pkg import alpha, beta\n", "seed-83");
    assert!(planted);
    assert_eq!(rendered, "from pkg import beta, alpha");
}

#[test]
fn test_seed_eighty_three_plants_every_import_in_one_file() {
    let (planted, rendered) =
        applied_render("import alpha, beta\nimport gamma, delta\n", "seed-83");
    assert!(planted);
    assert_eq!(rendered, "import beta, alpha\nimport delta, gamma");
}

#[test]
fn test_seed_eighty_three_not_applicable_without_two_names() {
    let target = target_with(
        "import alpha\n\ndef check():\n    return 1\n",
        IMPORT_GATE,
        "na83",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-83"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F10-6.

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
fn test_version_eighteen_pack_still_loads_the_eighty() {
    let dir = fresh_dir("pack18");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(80).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/18", entries));
    let loaded = load_seed_pack(&path).expect("the version eighteen pack loads");
    assert_eq!(loaded.len(), 80);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-79".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_nineteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    let last = entries[83].clone();
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
    entries[83] = invented;
    let path = write_document(&dir, &pack_document("canaryfield-seeds/19", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version nineteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_eighteen_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(79).collect();
    entries.push(standing_entries()[80].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/18", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version eighteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_nineteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(80).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/19", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "eighty entries under the version nineteen format are refused"
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
        written.contains("\"canaryfield-seeds/45\""),
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
