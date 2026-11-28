//! Feature slice 3 pins, the registry grown an eleventh time.
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
        std::env::temp_dir().join(format!("canaryfield-feature3-{}-{tag}", std::process::id()));
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

const SLICE_MODULE: &str = "def check(xs):\n    return xs[1:3]\n";
const SLICE_GATE: &str = "#!/bin/sh\ngrep -q \"xs\\[1:3\\]\" mod.py\n";
const AUG_MODULE: &str = "def check(v):\n    total = 1\n    total += v\n    return total\n";
const AUG_GATE: &str = "#!/bin/sh\ngrep -q \"total += v\" mod.py\n";
const PARAM_MODULE: &str = "def check(a, b):\n    return a - b\n";
const PARAM_GATE: &str = "#!/bin/sh\ngrep -q \"def check(a, b)\" mod.py\n";
const IMPORT_MODULE: &str = "import os\n\ndef check():\n    return os.name\n";
const IMPORT_GATE: &str = "#!/bin/sh\ngrep -q \"^import os\" mod.py\n";

// The registry pins of F3-1.

#[test]
fn test_registry_has_two_hundred_and_twenty_four_unique_seeds() {
    assert_eq!(SEEDS.len(), 224, "the grown registry holds two hundred");
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 224, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-223"));
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
    let grown: Vec<&str> = SEEDS[52..].iter().map(|seed| seed.name).collect();
    assert_eq!(
        grown,
        [
            "dropped slice bound",
            "dropped augmentation",
            "swapped parameter order",
            "dropped import",
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
            "dropped first for loop list element",
            "dropped last assignment value set element",
            "dropped first assignment value set element",
            "dropped last comparison operand set element",
            "dropped first comparison operand set element",
            "dropped last assignment value dict entry",
            "dropped first assignment value dict entry",
            "dropped last call argument dict entry",
            "dropped first call argument dict entry",
            "dropped last return value dict entry",
            "dropped first return value dict entry",
            "dropped last comprehension element dict entry",
            "dropped first comprehension element dict entry",
            "dropped last binop operand list element",
            "dropped first binop operand list element",
            "dropped last augmented assignment value tuple element",
            "dropped first augmented assignment value tuple element",
            "flipped first comparison operator",
            "flipped first boolean operator",
            "flipped first arithmetic operator",
            "flipped first augmented assignment operator",
            "flipped last comparison operator",
            "flipped last boolean operator",
            "flipped last arithmetic operator",
            "flipped last augmented assignment operator", "dropped later slice bound", "dropped earlier slice bound", "dropped slice bound pair", "dropped store subscript"
        ]
    );
}

// The seed-52 pins of F3-2.

#[test]
fn test_seed_fifty_two_plants_and_is_caught() {
    let target = target_with(SLICE_MODULE, SLICE_GATE, "caught52");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-52"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_two_drops_the_lower_bound_of_a_slice() {
    let (planted, rendered) = applied_render("def f(xs):\n    return xs[1:3]\n", "seed-52");
    assert!(planted);
    assert_eq!(rendered, "def f(xs):\n    return xs[:3]");
}

#[test]
fn test_seed_fifty_two_drops_the_upper_bound_where_no_lower_stands() {
    let (planted, rendered) = applied_render("def f(xs):\n    return xs[:3]\n", "seed-52");
    assert!(planted);
    assert_eq!(rendered, "def f(xs):\n    return xs[:]");
}

#[test]
fn test_seed_fifty_two_plants_every_bounded_slice_in_one_file() {
    let (planted, rendered) =
        applied_render("def f(xs, ys):\n    return xs[1:3] + ys[2:]\n", "seed-52");
    assert!(planted);
    assert_eq!(rendered, "def f(xs, ys):\n    return xs[:3] + ys[:]");
}

#[test]
fn test_seed_fifty_two_leaves_the_step_of_a_slice_standing() {
    let (planted, rendered) = applied_render("def f(xs):\n    return xs[1:8:2]\n", "seed-52");
    assert!(planted);
    assert_eq!(rendered, "def f(xs):\n    return xs[:8:2]");
}

#[test]
fn test_seed_fifty_two_leaves_an_unbounded_slice_standing() {
    let (planted, rendered) = applied_render("def f(xs):\n    return xs[:]\n", "seed-52");
    assert!(!planted);
    assert_eq!(rendered, "def f(xs):\n    return xs[:]");
}

#[test]
fn test_seed_fifty_two_plants_a_slice_standing_in_a_slice_bound_at_its_own_level() {
    let (planted, rendered) =
        applied_render("def f(xs, ys):\n    return xs[1:ys[2:3]]\n", "seed-52");
    assert!(planted);
    assert_eq!(rendered, "def f(xs, ys):\n    return xs[:ys[:3]]");
}

#[test]
fn test_seed_fifty_two_not_applicable_without_a_slice_bound() {
    let target = target_with(SLICE_MODULE, SLICE_GATE, "na52");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-52"), "caught");
    let bare = target_with("def check(xs):\n    return xs[:]\n", SLICE_GATE, "na52b");
    let result = run_target(&bare);
    assert_eq!(cell_of(&result, "seed-52"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
    let _ = fs::remove_dir_all(&bare);
}

// The seed-53 pins of F3-3.

#[test]
fn test_seed_fifty_three_plants_and_is_caught() {
    let target = target_with(AUG_MODULE, AUG_GATE, "caught53");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-53"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_three_reads_an_augmented_assignment_as_a_plain_assignment() {
    let (planted, rendered) = applied_render(
        "def f(v):\n    total = 1\n    total += v\n    return total\n",
        "seed-53",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(v):\n    total = 1\n    total = v\n    return total"
    );
}

#[test]
fn test_seed_fifty_three_plants_every_augmented_assignment_in_one_file() {
    let (planted, rendered) = applied_render(
        "def f(v):\n    a = 0\n    a += v\n    a *= 2\n    return a\n",
        "seed-53",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "def f(v):\n    a = 0\n    a = v\n    a = 2\n    return a"
    );
}

#[test]
fn test_seed_fifty_three_leaves_a_plain_assignment_standing() {
    let (planted, rendered) = applied_render("def f(v):\n    a = v\n    return a\n", "seed-53");
    assert!(!planted);
    assert_eq!(rendered, "def f(v):\n    a = v\n    return a");
}

#[test]
fn test_seed_fifty_three_not_applicable_without_an_augmented_assignment() {
    let target = target_with(
        "def check(v):\n    total = v\n    return total\n",
        AUG_GATE,
        "na53",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-53"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-54 pins of F3-4.

#[test]
fn test_seed_fifty_four_plants_and_is_caught() {
    let target = target_with(PARAM_MODULE, PARAM_GATE, "caught54");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-54"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_four_exchanges_the_first_two_parameters_of_a_definition() {
    let (planted, rendered) = applied_render("def f(a, b):\n    return a - b\n", "seed-54");
    assert!(planted);
    assert_eq!(rendered, "def f(b, a):\n    return a - b");
}

#[test]
fn test_seed_fifty_four_keeps_the_defaults_at_their_positions() {
    let (planted, rendered) = applied_render("def f(a=1, b=2):\n    return a\n", "seed-54");
    assert!(planted);
    assert_eq!(rendered, "def f(b=1, a=2):\n    return a");
}

#[test]
fn test_seed_fifty_four_exchanges_the_parameters_of_a_lambda() {
    let (planted, rendered) = applied_render("f = lambda a, b: a - b\n", "seed-54");
    assert!(planted);
    assert_eq!(rendered, "f = lambda b, a: a - b");
}

#[test]
fn test_seed_fifty_four_exchanges_across_the_positional_only_boundary() {
    let (planted, rendered) = applied_render("def f(a, /, b):\n    return a\n", "seed-54");
    assert!(planted);
    assert_eq!(rendered, "def f(b, /, a):\n    return a");
}

#[test]
fn test_seed_fifty_four_leaves_a_one_parameter_definition_standing() {
    let (planted, rendered) = applied_render("def f(a):\n    return a\n", "seed-54");
    assert!(!planted);
    assert_eq!(rendered, "def f(a):\n    return a");
}

#[test]
fn test_seed_fifty_four_not_applicable_without_two_positional_parameters() {
    let target = target_with("def check(a):\n    return a\n", PARAM_GATE, "na54");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-54"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-55 pins of F3-5.

#[test]
fn test_seed_fifty_five_plants_and_is_caught() {
    let target = target_with(IMPORT_MODULE, IMPORT_GATE, "caught55");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-55"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_five_removes_an_import_statement() {
    let (planted, rendered) = applied_render("import os\n\nx = 1\n", "seed-55");
    assert!(planted);
    assert_eq!(rendered, "x = 1");
}

#[test]
fn test_seed_fifty_five_removes_a_from_import_statement() {
    let (planted, rendered) = applied_render("from os import path\n\nx = 1\n", "seed-55");
    assert!(planted);
    assert_eq!(rendered, "x = 1");
}

#[test]
fn test_seed_fifty_five_plants_every_import_in_one_file() {
    let (planted, rendered) = applied_render("import os\nimport sys\n\nx = 1\n", "seed-55");
    assert!(planted);
    assert_eq!(rendered, "x = 1");
}

#[test]
fn test_seed_fifty_five_plants_an_import_standing_in_a_function_with_company() {
    let (planted, rendered) =
        applied_render("def f():\n    import os\n    return os.name\n", "seed-55");
    assert!(planted);
    assert_eq!(rendered, "def f():\n    return os.name");
}

#[test]
fn test_seed_fifty_five_never_empties_a_body() {
    let (planted, rendered) = applied_render("def f():\n    import os\n", "seed-55");
    assert!(!planted);
    assert_eq!(rendered, "def f():\n    import os");
}

#[test]
fn test_seed_fifty_five_not_applicable_without_an_import() {
    let target = target_with("def check():\n    return 1\n", IMPORT_GATE, "na55");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-55"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F3-6.

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
fn test_version_eleven_pack_still_loads_the_fifty_two() {
    let dir = fresh_dir("pack11");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(52).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/11", entries));
    let loaded = load_seed_pack(&path).expect("the version eleven pack loads");
    assert_eq!(loaded.len(), 52);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-51".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_twelve_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    entries[55] = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Str("seed-99".to_string())),
        (
            "name".to_string(),
            JsonValue::Str("dropped import".to_string()),
        ),
        (
            "defect_class".to_string(),
            JsonValue::Str("missing input".to_string()),
        ),
        (
            "description".to_string(),
            JsonValue::Str(
                "Removes an import statement so the names it binds are never bound and every read of them meets their absence"
                    .to_string(),
            ),
        ),
    ]);
    let path = write_document(&dir, &pack_document("canaryfield-seeds/12", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version twelve pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_eleven_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(51).collect();
    entries.push(standing_entries()[52].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/11", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version eleven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_twelve_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(52).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/12", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "fifty two entries under the version twelve format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 224);
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
    assert_eq!(loaded.len(), 224);
    let _ = fs::remove_dir_all(&dir);
}
