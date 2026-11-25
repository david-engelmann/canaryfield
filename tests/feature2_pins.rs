//! Feature slice 2 pins, the registry grown a tenth time.
//!
//! The behaviour pins drive the applies directly over parsed
//! sources and compare the rendering or the dump the port
//! spells against the spelling the contract's obligations
//! state, and the verdict pins drive the run over synthetic
//! targets whose gates turn on the planted behaviour, in the
//! shape feature slice 1 set. The pack pins build their
//! documents from the crate's own pack data with one fact
//! changed at a time, so each refusal is read against the
//! single fact that earns it.

use std::fs;
use std::path::PathBuf;

use canaryfield::engine::{run, RunRequest};
use canaryfield::json::{self, JsonValue};
use canaryfield::pyapply::apply_seed;
use canaryfield::pyarena::{parse_tree, render_tree};
use canaryfield::pydump::dump_tree;
use canaryfield::pyplant::plant_seed;
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

fn applied_dump(source: &str, seed: &str) -> (bool, String) {
    let mut tree = parse_tree(source).expect("the source parses");
    let planted = apply_seed(&mut tree, seed).expect("the application stands");
    (
        planted,
        dump_tree(&tree).expect("the tree dumps").to_string(),
    )
}

fn fresh_dir(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("canaryfield-feature2-{}-{tag}", std::process::id()));
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
                .expect("the row carries the gate cell")
                .to_string();
        }
    }
    panic!("the result carries the {seed} row");
}

const TRY_MODULE: &str = "def check(x):\n    try:\n        y = int(x)\n    except ValueError:\n        return False\n    else:\n        return y == 1\n";
const TRY_GATE: &str = "#!/bin/sh\ngrep -q \"return y == 1\" mod.py\n";
const RAISE_MODULE: &str = "def check():\n    try:\n        raise ValueError(\"inner\")\n    except ValueError as e:\n        raise RuntimeError(\"outer\") from e\n";

// The registry pins of F2-1.

#[test]
fn test_registry_has_one_hundred_and_forty_four_unique_seeds() {
    assert_eq!(
        SEEDS.len(),
        144,
        "the grown registry holds one hundred and forty four"
    );
    let mut ids: Vec<&str> = SEEDS.iter().map(|seed| seed.id).collect();
    let standing = ids.clone();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 144, "every seed id stands once");
    assert_eq!(standing.first(), Some(&"seed-00"));
    assert_eq!(standing.last(), Some(&"seed-143"));
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
    let grown: Vec<&str> = SEEDS[48..].iter().map(|seed| seed.name).collect();
    assert_eq!(
        grown,
        [
            "dropped try else clause",
            "shifted slice step",
            "swapped conditional expression arms",
            "dropped exception chain",
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
            "dropped first boolean operand"
        ]
    );
}

// The seed-48 pins of F2-2.

#[test]
fn test_seed_forty_eight_plants_and_is_caught() {
    let target = target_with(TRY_MODULE, TRY_GATE, "caught48");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-48"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_forty_eight_drops_the_else_clause_of_a_try() {
    let (planted, rendered) = applied_render(
        "try:\n    risky()\nexcept ValueError:\n    recover()\nelse:\n    celebrate()\nfinally:\n    tidy()\n",
        "seed-48",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    risky()\nexcept ValueError:\n    recover()\nfinally:\n    tidy()"
    );
}

#[test]
fn test_seed_forty_eight_drops_the_else_clause_of_a_starred_try() {
    let (planted, dump) = applied_dump(
        "try:\n    a()\nexcept* ValueError:\n    pass\nelse:\n    b()\n",
        "seed-48",
    );
    assert!(planted);
    assert_eq!(
        dump,
        "Module(body=[TryStar(body=[Expr(value=Call(func=Name(id='a', ctx=Load()), args=[], keywords=[]))], handlers=[ExceptHandler(type=Name(id='ValueError', ctx=Load()), body=[Pass()])], orelse=[], finalbody=[])], type_ignores=[])"
    );
}

#[test]
fn test_seed_forty_eight_plants_every_try_else_in_one_file() {
    let (planted, rendered) = applied_render(
        "try:\n    a()\nexcept ValueError:\n    pass\nelse:\n    b()\ntry:\n    c()\nexcept TypeError:\n    pass\nelse:\n    d()\n",
        "seed-48",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    a()\nexcept ValueError:\n    pass\ntry:\n    c()\nexcept TypeError:\n    pass"
    );
}

#[test]
fn test_seed_forty_eight_shows_the_outer_planting_where_an_else_clause_holds_a_try() {
    let source = "for i in items:\n    pass\nelse:\n    try:\n        a()\n    except ValueError:\n        pass\n    else:\n        b()\n";
    let (planted, rendered) = applied_render(source, "seed-48");
    assert!(planted);
    assert_eq!(
        rendered,
        "for i in items:\n    pass\nelse:\n    try:\n        a()\n    except ValueError:\n        pass",
        "seed-48 drops the try else and leaves the loop else standing"
    );
    let (outer_planted, outer_rendered) = applied_render(source, "seed-44");
    assert!(outer_planted);
    assert_eq!(
        outer_rendered, "for i in items:\n    pass",
        "seed-44 drops the loop else whole and the try inside it goes with it"
    );
}

#[test]
fn test_seed_forty_eight_plants_a_try_standing_in_a_try_body_at_its_own_level() {
    let (planted, rendered) = applied_render(
        "try:\n    try:\n        a()\n    except ValueError:\n        pass\n    else:\n        b()\nexcept TypeError:\n    pass\nelse:\n    c()\n",
        "seed-48",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    try:\n        a()\n    except ValueError:\n        pass\nexcept TypeError:\n    pass"
    );
}

#[test]
fn test_seed_forty_eight_planting_that_would_break_a_nonlocal_binding_is_never_written() {
    let dir = fresh_dir("nonlocal48");
    let module = "def outer():\n    def inner():\n        nonlocal x\n        return x\n    try:\n        pass\n    except ValueError:\n        pass\n    else:\n        x = 1\n    return inner\n";
    fs::write(dir.join("mod.py"), module).expect("the module writes");
    let mut skipped: Vec<String> = Vec::new();
    let planted = plant_seed(&dir, "seed-48", Some(&mut skipped)).expect("the planting stands");
    assert!(
        planted.is_empty(),
        "the planting is refused at the write path and no file is planted"
    );
    let after = fs::read_to_string(dir.join("mod.py")).expect("the module reads");
    assert_eq!(after, module, "the file stands exactly as written");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_forty_eight_leaves_a_try_without_an_else_clause_standing() {
    let source = "try:\n    a()\nfinally:\n    b()\n";
    let (planted, rendered) = applied_render(source, "seed-48");
    assert!(!planted);
    assert_eq!(rendered, "try:\n    a()\nfinally:\n    b()");
}

#[test]
fn test_seed_forty_eight_not_applicable_without_a_try_else() {
    let target = target_with(
        RAISE_MODULE,
        "#!/bin/sh\npython3 -c \"from mod import check\ntry:\n    check()\nexcept RuntimeError:\n    raise SystemExit(0)\nraise SystemExit(1)\"\n",
        "na48",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-48"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-49 pins of F2-3.

#[test]
fn test_seed_forty_nine_plants_and_is_caught() {
    let target = target_with(
        "def pick(xs):\n    return xs[0:4:2]\n",
        "#!/bin/sh\npython3 -c \"from mod import pick; raise SystemExit(0 if pick([1, 2, 3, 4]) == [1, 3] else 1)\"\n",
        "caught49",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-49"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_forty_nine_shifts_a_slice_step_by_one() {
    let (planted, rendered) = applied_render("xs[1:9:2]\n", "seed-49");
    assert!(planted);
    assert_eq!(rendered, "xs[1:9:3]");
}

#[test]
fn test_seed_forty_nine_plants_every_slice_step_in_one_file() {
    let (planted, rendered) = applied_render("a = xs[0:9:2]\nb = ys[1:8:3]\n", "seed-49");
    assert!(planted);
    assert_eq!(rendered, "a = xs[0:9:3]\nb = ys[1:8:4]");
}

#[test]
fn test_seed_forty_nine_leaves_the_bounds_of_a_stepped_slice_standing() {
    let (planted, rendered) = applied_render("xs[lo:hi:2]\n", "seed-49");
    assert!(planted);
    assert_eq!(rendered, "xs[lo:hi:3]");
}

#[test]
fn test_seed_forty_nine_leaves_a_slice_without_a_step_standing() {
    let (planted, rendered) = applied_render("xs[1:9]\n", "seed-49");
    assert!(!planted);
    assert_eq!(rendered, "xs[1:9]");
}

#[test]
fn test_seed_forty_nine_and_the_bound_seeds_leave_each_others_fields_standing() {
    let (stop_planted, stop_rendered) = applied_render("xs[1:9:2]\n", "seed-29");
    assert!(stop_planted);
    assert_eq!(
        stop_rendered, "xs[1:10:2]",
        "seed-29 leaves the step standing"
    );
    let (start_planted, start_rendered) = applied_render("xs[1:9:2]\n", "seed-40");
    assert!(start_planted);
    assert_eq!(
        start_rendered, "xs[2:9:2]",
        "seed-40 leaves the step standing"
    );
    let (step_planted, step_rendered) = applied_render("xs[1:9:2]\n", "seed-49");
    assert!(step_planted);
    assert_eq!(
        step_rendered, "xs[1:9:3]",
        "seed-49 leaves the bounds standing"
    );
}

#[test]
fn test_seed_forty_nine_plants_a_slice_standing_in_a_slice_bound_at_its_own_level() {
    let (planted, rendered) = applied_render("a[b[::2]::3]\n", "seed-49");
    assert!(planted);
    assert_eq!(rendered, "a[b[::3]::4]");
}

#[test]
fn test_seed_forty_nine_not_applicable_without_a_slice_step() {
    let target = target_with(TRY_MODULE, TRY_GATE, "na49");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-49"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-50 pins of F2-4.

#[test]
fn test_seed_fifty_plants_and_is_caught() {
    let target = target_with(
        "def check(x):\n    return \"yes\" if x else \"no\"\n",
        "#!/bin/sh\npython3 -c \"from mod import check; raise SystemExit(0 if check(1) == 'yes' else 1)\"\n",
        "caught50",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-50"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_exchanges_the_arms_of_a_conditional_expression() {
    let (planted, rendered) = applied_render("a if flag else b\n", "seed-50");
    assert!(planted);
    assert_eq!(rendered, "b if flag else a");
}

#[test]
fn test_seed_fifty_keeps_the_test_standing() {
    let (planted, rendered) = applied_render("a if check(x) > 1 else b\n", "seed-50");
    assert!(planted);
    assert_eq!(rendered, "b if check(x) > 1 else a");
}

#[test]
fn test_seed_fifty_exchanges_a_nested_conditional_expression_at_both_levels() {
    let (planted, dump) = applied_dump("x = (a if p else b) if q else c\n", "seed-50");
    assert!(planted);
    assert_eq!(
        dump,
        "Module(body=[Assign(targets=[Name(id='x', ctx=Store())], value=IfExp(test=Name(id='q', ctx=Load()), body=Name(id='c', ctx=Load()), orelse=IfExp(test=Name(id='p', ctx=Load()), body=Name(id='b', ctx=Load()), orelse=Name(id='a', ctx=Load()))))], type_ignores=[])"
    );
}

#[test]
fn test_seed_fifty_exchanges_a_conditional_expression_standing_in_a_test() {
    let (planted, dump) = applied_dump("x = a if (b if flag else c) else d\n", "seed-50");
    assert!(planted);
    assert_eq!(
        dump,
        "Module(body=[Assign(targets=[Name(id='x', ctx=Store())], value=IfExp(test=IfExp(test=Name(id='flag', ctx=Load()), body=Name(id='c', ctx=Load()), orelse=Name(id='b', ctx=Load())), body=Name(id='d', ctx=Load()), orelse=Name(id='a', ctx=Load())))], type_ignores=[])",
        "the expression in the test is exchanged at its own level and the outer expression at its level"
    );
}

#[test]
fn test_seed_fifty_not_applicable_without_a_conditional_expression() {
    let target = target_with(TRY_MODULE, TRY_GATE, "na50");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-50"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The seed-51 pins of F2-5.

#[test]
fn test_seed_fifty_one_plants_and_is_caught() {
    let target = target_with(RAISE_MODULE, "#!/bin/sh\npython3 gate.py\n", "caught51");
    fs::write(
        target.join("gate.py"),
        "from mod import check\ntry:\n    check()\nexcept RuntimeError as r:\n    raise SystemExit(0 if isinstance(r.__cause__, ValueError) else 1)\nraise SystemExit(1)\n",
    )
    .expect("the gate module writes");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-51"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_seed_fifty_one_drops_the_chain_from_a_raise() {
    let (planted, rendered) = applied_render("raise RuntimeError('x') from e\n", "seed-51");
    assert!(planted);
    assert_eq!(rendered, "raise RuntimeError('x')");
}

#[test]
fn test_seed_fifty_one_keeps_the_raised_error_standing() {
    let (planted, rendered) =
        applied_render("raise errors.Make(1, key='k') from cause\n", "seed-51");
    assert!(planted);
    assert_eq!(rendered, "raise errors.Make(1, key='k')");
}

#[test]
fn test_seed_fifty_one_plants_every_chained_raise_in_one_file() {
    let (planted, rendered) = applied_render(
        "try:\n    pass\nexcept ValueError as e:\n    raise RuntimeError('a') from e\nexcept TypeError as e:\n    raise RuntimeError('b') from e\n",
        "seed-51",
    );
    assert!(planted);
    assert_eq!(
        rendered,
        "try:\n    pass\nexcept ValueError as e:\n    raise RuntimeError('a')\nexcept TypeError as e:\n    raise RuntimeError('b')"
    );
}

#[test]
fn test_seed_fifty_one_drops_a_suppressing_chain() {
    let (planted, rendered) = applied_render("raise RuntimeError('x') from None\n", "seed-51");
    assert!(planted);
    assert_eq!(rendered, "raise RuntimeError('x')");
}

#[test]
fn test_seed_fifty_one_leaves_an_unchained_raise_standing() {
    let (planted, rendered) = applied_render("raise ValueError('x')\n", "seed-51");
    assert!(!planted);
    assert_eq!(rendered, "raise ValueError('x')");
}

#[test]
fn test_seed_fifty_one_not_applicable_without_a_chained_raise() {
    let target = target_with(TRY_MODULE, TRY_GATE, "na51");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-51"), "not-applicable");
    let _ = fs::remove_dir_all(&target);
}

// The pack pins of F2-6.

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
fn test_version_ten_pack_still_loads_the_forty_eight() {
    let dir = fresh_dir("pack10");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(48).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/10", entries));
    let loaded = load_seed_pack(&path).expect("the version ten pack loads");
    assert_eq!(loaded.len(), 48);
    assert_eq!(
        loaded.last().and_then(|entry| entry.get("id")),
        Some(&JsonValue::Str("seed-47".to_string()))
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_eleven_pack_with_an_invented_id_is_refused() {
    let dir = fresh_dir("packinvented");
    let mut entries = standing_entries();
    entries[51] = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Str("seed-99".to_string())),
        (
            "name".to_string(),
            JsonValue::Str("dropped exception chain".to_string()),
        ),
        (
            "defect_class".to_string(),
            JsonValue::Str("wrong error".to_string()),
        ),
        (
            "description".to_string(),
            JsonValue::Str(
                "Removes the from clause of a raise so the error arrives without the failure it was chained to as its cause"
                    .to_string(),
            ),
        ),
    ]);
    let path = write_document(&dir, &pack_document("canaryfield-seeds/11", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "an invented id in a version eleven pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_version_ten_pack_carrying_a_grown_registry_id_is_refused() {
    let dir = fresh_dir("packgrown");
    let mut entries: Vec<JsonValue> = standing_entries().into_iter().take(47).collect();
    entries.push(standing_entries()[48].clone());
    let path = write_document(&dir, &pack_document("canaryfield-seeds/10", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "a grown registry id standing in a version ten pack is refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_a_pack_whose_count_disagrees_with_version_eleven_is_refused() {
    let dir = fresh_dir("packcount");
    let entries: Vec<JsonValue> = standing_entries().into_iter().take(48).collect();
    let path = write_document(&dir, &pack_document("canaryfield-seeds/11", entries));
    assert!(
        load_seed_pack(&path).is_err(),
        "forty eight entries under the version eleven format are refused"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_seed_pack_round_trip() {
    let dir = fresh_dir("packroundtrip");
    let path = dir.join("pack.json");
    fs::write(&path, seed_pack_text()).expect("the pack writes");
    let loaded = load_seed_pack(&path).expect("the written pack loads");
    assert_eq!(loaded.len(), 144);
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
        written.contains("\"canaryfield-seeds/34\""),
        "the written pack carries the twenty second format"
    );
    let _ = fs::remove_dir_all(&dir);
}
