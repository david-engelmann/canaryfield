//! Feature slice 4 pins, the registry grown a twelfth time.
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
        std::env::temp_dir().join(format!("canaryfield-feature4-{}-{tag}", std::process::id()));
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

const RETURN_MODULE: &str = "def check(x):\n    y = x + 1\n    return y\n";
const RETURN_GATE: &str = "#!/bin/sh\ngrep -q \"return y\" mod.py\n";
const BOOL_MODULE: &str = "def check(a, b):\n    return a and b\n";
const BOOL_GATE: &str = "#!/bin/sh\ngrep -q \"a and b\" mod.py\n";
const COMP_MODULE: &str = "def check(xs):\n    return [x for x in xs if x > 0]\n";
const COMP_GATE: &str = "#!/bin/sh\ngrep -q \"if x > 0\" mod.py\n";
const CHAIN_MODULE: &str = "def check(a, b, c):\n    return a < b < c\n";
const CHAIN_GATE: &str = "#!/bin/sh\ngrep -q \"a < b < c\" mod.py\n";

// The registry pins of F4-1.

#[test]
fn test_registry_has_one_hundred_and_ninety_six_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        196,
        "the grown registry holds one hundred and ninety six"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 196, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-195"));
    for (index, seed) in SEEDS.iter().enumerate() {
        assert_eq!(
            seed.id,
            format!("seed-{index:02}"),
            "the ids run in order at {index}"
        );
    }
}

#[test]
fn test_seed_zero_is_the_stale_version_string() {
    let first = SEEDS.first().expect("a first seed stands");
    assert_eq!(first.id, "seed-00");
    assert_eq!(first.name, "stale version string");
}

#[test]
fn test_seed_metadata_is_complete() {
    for seed in SEEDS {
        assert!(!seed.name.is_empty(), "{} carries a name", seed.id);
        assert!(
            !seed.defect_class.is_empty(),
            "{} carries a defect class",
            seed.id
        );
        assert!(
            !seed.description.is_empty(),
            "{} carries a description",
            seed.id
        );
    }
    let grown: Vec<&str> = SEEDS[56..].iter().map(|seed| seed.name).collect();
    assert_eq!(
        grown,
        [
            "dropped return statement",
            "swapped boolean operands",
            "dropped comprehension condition",
            "flipped later comparison operator",
            "swapped dictionary entry values",
            "swapped sequence literal elements",
            "dropped assert statement",
            "added numeric negation",
            "dropped with target",
            "dropped assignment expression",
            "dropped delete statement",
            "dropped match case",
            "swapped slice bounds",
            "swapped branch bodies",
            "dropped match guard",
            "dropped yield-from delegation",
            "swapped comprehension generator order",
            "dropped argument unpacking",
            "swapped formatted string values",
            "dropped call statement",
            "swapped decorator order",
            "dropped parameter default",
            "swapped base class order",
            "swapped match case order",
            "swapped with items",
            "dropped asynchronous modifier",
            "swapped keyword argument order",
            "swapped import names",
            "dropped with item",
            "swapped try handler order",
            "dropped argument annotation",
            "swapped assignment target order",
            "dropped return annotation",
            "swapped statement order",
            "swapped comprehension condition order",
            "dropped starred argument",
            "dropped raise statement",
            "dropped starred display element",
            "dropped loop control statement",
            "swapped try else bodies",
            "dropped class keyword argument",
            "swapped loop else bodies",
            "dropped starred parameter",
            "dropped docstring",
            "dropped parameter",
            "dropped elif branch",
            "dropped import alias",
            "dropped yield value",
            "dropped assert message",
            "dropped formatted value conversion",
            "swapped loop target elements",
            "dropped keyword-only parameter",
            "dropped format spec",
            "dropped comparison tail",
            "dropped imported name",
            "dropped keyword-only default",
            "dropped slice step",
            "dropped annotation value",
            "dropped formatted value",
            "dropped raised error message",
            "dropped set element",
            "dropped decorator call",
            "dropped attribute",
            "dropped exception type",
            "dropped subscript",
            "dropped arithmetic operand",
            "dropped first call argument",
            "dropped first condition term",
            "dropped tuple element",
            "dropped conditional test",
            "dropped comparison head",
            "dropped assignment",
            "dropped annotated assignment",
            "dropped function definition",
            "dropped class definition",
            "dropped exception handler name",
            "dropped assignment target",
            "dropped last tuple element",
            "dropped last base class",
            "dropped decorator argument",
            "dropped unpacking target element",
            "dropped last unpacking target element",
            "dropped keyword unpacking",
            "dropped last dictionary entry",
            "dropped last keyword argument",
            "dropped last list element",
            "dropped first imported name",
            "dropped first boolean operand",
            "dropped last call argument",
            "dropped last parameter",
            "dropped subscript index element",
            "dropped last exception handler",
            "dropped last f-string element",
            "dropped last return element",
            "dropped first subscript index element",
            "dropped last set element",
            "dropped first f-string element",
            "dropped first return element",
            "dropped last match case",
            "dropped last decorator",
            "dropped last assignment target",
            "dropped dictionary unpacking entry",
            "dropped first assignment value element",
            "dropped last assignment value element",
            "dropped last tuple argument element",
            "dropped first tuple argument element",
            "dropped last for loop tuple element",
            "dropped first for loop tuple element",
            "dropped last annotated assignment value element",
            "dropped first annotated assignment value element",
            "dropped last comprehension tuple element",
            "dropped first comprehension tuple element",
            "dropped last lambda body tuple element",
            "dropped first lambda body tuple element",
            "dropped last comparison operand tuple element",
            "dropped first comparison operand tuple element",
            "dropped last parameter default tuple element",
            "dropped first parameter default tuple element",
            "dropped last conditional expression tuple element",
            "dropped first conditional expression tuple element",
            "dropped last exception handler type tuple element",
            "dropped first exception handler type tuple element",
            "dropped last keyword value tuple element",
            "dropped first keyword value tuple element",
            "dropped last comprehension iterable tuple element",
            "dropped first comprehension iterable tuple element",
            "dropped last nested tuple element",
            "dropped first nested tuple element",
            "dropped last list argument element",
            "dropped first list argument element",
            "dropped last comparison operand list element",
            "dropped first comparison operand list element",
            "dropped last assignment value list element",
            "dropped first assignment value list element",
            "dropped last return value list element",
            "dropped first return value list element",
            "dropped last nested list element",
            "dropped first nested list element",
            "dropped last for loop list element",
            "dropped first for loop list element"
        ]
    );
}

// The seed-56 pins of F4-2.

#[test]
fn test_seed_fifty_six_plants_and_is_caught() {
    let target = target_with(RETURN_MODULE, RETURN_GATE, "caught56");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-56"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_six_removes_a_return_statement() {
    let (planted, rendered) = applied_render("def f(x):\n    y = x + 1\n    return y\n", "seed-56");
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    y = x + 1");
}

#[test]
fn test_seed_fifty_six_plants_every_return_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    y = x + 1\n    return y\n\ndef g(x):\n    z = x + 2\n    return z\n",
        "seed-56",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(x):\n    y = x + 1\n\ndef g(x):\n    z = x + 2"
    );
}

#[test]
fn test_seed_fifty_six_removes_an_early_return_and_the_flow_falls_through() {
    let (planted, rendered) = applied_render(
        "def f(x):\n    if x:\n        y = 1\n        return y\n    return 2\n",
        "seed-56",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    if x:\n        y = 1");
}

#[test]
fn test_seed_fifty_six_removes_a_bare_return_too() {
    let (planted, rendered) = applied_render("def f(x):\n    print(x)\n    return\n", "seed-56");
    assert!(planted);
    assert_eq!(rendered, "def f(x):\n    print(x)");
}

#[test]
fn test_seed_fifty_six_never_empties_a_body() {
    let (planted, rendered) = applied_render("def f(x):\n    return x\n", "seed-56");
    assert!(!planted);
    assert_eq!(rendered, "def f(x):\n    return x");
}

#[test]
fn test_seed_fifty_six_not_applicable_without_a_return() {
    let target = target_with(RETURN_MODULE, RETURN_GATE, "na56");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-56"), "caught");
    let bare = target_with("def check(x):\n    print(x)\n", RETURN_GATE, "na56b");
    let result = run_target(&bare);
    assert_eq!(cell_of(&result, "seed-56"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
    let _ = fs::remove_dir_all(&bare);
}

// The seed-57 pins of F4-3.

#[test]
fn test_seed_fifty_seven_plants_and_is_caught() {
    let target = target_with(BOOL_MODULE, BOOL_GATE, "caught57");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-57"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_seven_exchanges_the_first_two_values_of_a_boolean_operation() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return a and b\n", "seed-57");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b):\n    return b and a");
}

#[test]
fn test_seed_fifty_seven_plants_every_boolean_operation_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(a, b, c, d):\n    return (a and b) or (c and d)\n",
        "seed-57",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(a, b, c, d):\n    return d and c or (b and a)"
    );
}

#[test]
fn test_seed_fifty_seven_keeps_the_operator_and_the_later_values_standing() {
    let (planted, rendered) =
        applied_render("def f(a, b, c):\n    return a or b or c\n", "seed-57");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c):\n    return b or a or c");
}

#[test]
fn test_seed_fifty_seven_removes_no_term() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return a and b\n", "seed-57");
    assert!(planted);
    assert!(
        rendered.contains("a"),
        "the first term stands in the render"
    );
    assert!(
        rendered.contains("b"),
        "the second term stands in the render"
    );
}

#[test]
fn test_seed_fifty_seven_not_applicable_without_a_boolean_operation() {
    let target = target_with(BOOL_MODULE, BOOL_GATE, "na57");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-57"), "caught");
    let bare = target_with("def check(a):\n    return a\n", BOOL_GATE, "na57b");
    let result = run_target(&bare);
    assert_eq!(cell_of(&result, "seed-57"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
    let _ = fs::remove_dir_all(&bare);
}

// The seed-58 pins of F4-4.

#[test]
fn test_seed_fifty_eight_plants_and_is_caught() {
    let target = target_with(COMP_MODULE, COMP_GATE, "caught58");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-58"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_eight_removes_the_first_condition_of_a_comprehension_clause() {
    let (planted, rendered) = applied_render(
        "def f(xs):\n    return [x for x in xs if x > 0]\n",
        "seed-58",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(xs):\n    return [x for x in xs]");
}

#[test]
fn test_seed_fifty_eight_plants_every_clause_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(xs, ys):\n    return [x for x in xs if x] + [y for y in ys if y]\n",
        "seed-58",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(xs, ys):\n    return [x for x in xs] + [y for y in ys]"
    );
}

#[test]
fn test_seed_fifty_eight_plants_in_a_generator_expression_and_a_dictionary_comprehension() {
    let (planted, rendered) = applied_render(
        "def f(xs):\n    return (x for x in xs if x), {x: x for x in xs if x}\n",
        "seed-58",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(xs):\n    return ((x for x in xs), {x: x for x in xs})"
    );
}

#[test]
fn test_seed_fifty_eight_keeps_the_later_conditions_standing() {
    let (planted, rendered) = applied_render(
        "def f(xs):\n    return [x for x in xs if x > 0 if x < 9]\n",
        "seed-58",
    );
    assert!(planted);
    assert_eq!(rendered, "def f(xs):\n    return [x for x in xs if x < 9]");
}

#[test]
fn test_seed_fifty_eight_leaves_a_clause_without_a_condition_standing() {
    let (planted, rendered) = applied_render("def f(xs):\n    return [x for x in xs]\n", "seed-58");
    assert!(!planted);
    assert_eq!(rendered, "def f(xs):\n    return [x for x in xs]");
}

#[test]
fn test_seed_fifty_eight_not_applicable_without_a_comprehension_condition() {
    let target = target_with(COMP_MODULE, COMP_GATE, "na58");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-58"), "caught");
    let bare = target_with(
        "def check(xs):\n    return [x for x in xs]\n",
        COMP_GATE,
        "na58b",
    );
    let result = run_target(&bare);
    assert_eq!(cell_of(&result, "seed-58"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
    let _ = fs::remove_dir_all(&bare);
}

// The seed-59 pins of F4-5.

#[test]
fn test_seed_fifty_nine_plants_and_is_caught() {
    let target = target_with(CHAIN_MODULE, CHAIN_GATE, "caught59");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-59"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_nine_flips_the_second_operator_of_a_comparison() {
    let (planted, rendered) = applied_render("def f(a, b, c):\n    return a < b < c\n", "seed-59");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c):\n    return a < b > c");
}

#[test]
fn test_seed_fifty_nine_flips_every_later_operator_in_a_longer_chain() {
    let (planted, rendered) =
        applied_render("def f(a, b, c, d):\n    return a < b < c < d\n", "seed-59");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c, d):\n    return a < b > c > d");
}

#[test]
fn test_seed_fifty_nine_flips_a_later_membership_operator() {
    let (planted, rendered) = applied_render("def f(a, b, c):\n    return a < b in c\n", "seed-59");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c):\n    return a < b not in c");
}

#[test]
fn test_seed_fifty_nine_keeps_the_first_operator_standing() {
    let (planted, rendered) = applied_render("def f(a, b, c):\n    return a > b < c\n", "seed-59");
    assert!(planted);
    assert_eq!(rendered, "def f(a, b, c):\n    return a > b > c");
}

#[test]
fn test_seed_fifty_nine_leaves_a_single_operator_comparison_standing() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return a < b\n", "seed-59");
    assert!(!planted);
    assert_eq!(rendered, "def f(a, b):\n    return a < b");
}

#[test]
fn test_seed_fifty_nine_not_applicable_without_a_chained_comparison() {
    let target = target_with(CHAIN_MODULE, CHAIN_GATE, "na59");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-59"), "caught");
    let bare = target_with("def check(a, b):\n    return a < b\n", CHAIN_GATE, "na59b");
    let result = run_target(&bare);
    assert_eq!(cell_of(&result, "seed-59"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
    let _ = fs::remove_dir_all(&bare);
}

// The pack pins of F4-6.

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
fn test_version_twelve_pack_still_loads_the_fifty_six() {
    let dir = fresh_dir("pack12");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(56).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/12", entries));
    let loaded = load_seed_pack(&path).expect("the version twelve pack loads");
    assert_eq!(loaded.len(), 56);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-55".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_thirteen_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    entries[59] = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Str("seed-99".to_string())),
        (
            "name".to_string(),
            JsonValue::Str("flipped later comparison operator".to_string()),
        ),
        (
            "defect_class".to_string(),
            JsonValue::Str("wrong predicate".to_string()),
        ),
        (
            "description".to_string(),
            JsonValue::Str(
                "Exchanges every operator after the first in a chained comparison for its paired opposite so the chain closes where the code wrote it to run on"
                    .to_string(),
            ),
        ),
    ]);
    let path = write_document(&dir, &pack_document("canaryfield-seeds/13", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version thirteen pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twelve_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(55).collect();
    entries.push(standing_entries()[56].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/12", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version twelve pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_thirteen_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(56).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/13", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "fifty six entries under the version thirteen format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 196);
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
        .expect("the command runs");
    assert!(
        output.status.success(),
        "the write seed pack command stands"
    );
    let loaded = load_seed_pack(&path).expect("the cli pack loads");
    assert_eq!(loaded.len(), 196);
    let _ = fs::remove_dir_all(&dir);
}
