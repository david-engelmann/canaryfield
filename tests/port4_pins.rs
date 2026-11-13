//! Pins for Rust port slice 4, the report renderings of
//! PORT4.
//!
//! Each pin carries its expected answer stated from the
//! landed Python's wording and spelling, canaryfield/report.py
//! as it stood when the slice opened, not derived from the
//! recordings the conformance suite replays. The format pins
//! cover the heading block, the target lines, the clean exit
//! lines, the summary lines, the width rule over Unicode
//! scalar values, the sorted keys and the escaping and the
//! float spellings and the one trailing newline of the JSON
//! form, and the continuation and unparseable lines in their
//! standing places. The refusal pins carry the port's refusal
//! texts word for word, one for each member of the PORT4-6
//! family, and two pins carry the report bridge's refusal
//! sentences against the command line's own source.

use std::fs;
use std::process::Command;

use canaryfield::json::{self, JsonValue};
use canaryfield::report::{render_json, render_report, render_text};

const BASE: &str = r#"{
  "gates": ["alpha"],
  "target": "demo-target",
  "target_unchanged": true,
  "clean_exits": {"alpha": 0},
  "rows": [
    {
      "seed": "seed-01",
      "name": "first seed",
      "planted_file": "mod.py",
      "gates": {"alpha": "caught"},
      "planted_files": ["mod.py"]
    }
  ],
  "unparseable_files": [],
  "summary": {
    "alpha": {"caught": 1, "waved-through": 0, "not-applicable": 0, "gate-broken": 0}
  }
}"#;

const BASE_TEXT: &str = "canaryfield catch table\n=======================\n\ntarget           demo-target\ntarget unchanged true\nclean exit for gate alpha is 0\n\nseed     name        planted file  alpha\n-------  ----------  ------------  ------\nseed-01  first seed  mod.py        caught\n\nsummary per gate\n  alpha  caught 1  waved-through 0  not-applicable 0  gate-broken 0\n";

fn base() -> JsonValue {
    json::parse(BASE).expect("the base document parses")
}

fn set(value: &mut JsonValue, key: &str, item: JsonValue) {
    match value {
        JsonValue::Object(pairs) => {
            for pair in pairs.iter_mut() {
                if pair.0 == key {
                    pair.1 = item;
                    return;
                }
            }
            pairs.push((key.to_string(), item));
        }
        other => panic!("expected an object, got {other:?}"),
    }
}

fn remove(value: &mut JsonValue, key: &str) {
    match value {
        JsonValue::Object(pairs) => pairs.retain(|pair| pair.0 != key),
        other => panic!("expected an object, got {other:?}"),
    }
}

fn row(doc: &mut JsonValue, index: usize) -> &mut JsonValue {
    match doc.get_mut_rows().get_mut(index) {
        Some(item) => item,
        None => panic!("row {index} stands"),
    }
}

trait RowsMut {
    fn get_mut_rows(&mut self) -> &mut Vec<JsonValue>;
}

impl RowsMut for JsonValue {
    fn get_mut_rows(&mut self) -> &mut Vec<JsonValue> {
        match self {
            JsonValue::Object(pairs) => {
                for pair in pairs.iter_mut() {
                    if pair.0 == "rows" {
                        match &mut pair.1 {
                            JsonValue::Array(items) => return items,
                            other => panic!("rows stand as a list, got {other:?}"),
                        }
                    }
                }
                panic!("rows stand in the document");
            }
            other => panic!("expected an object, got {other:?}"),
        }
    }
}

fn text(value: &str) -> JsonValue {
    JsonValue::Str(value.to_string())
}

fn refusal(doc: &JsonValue) -> String {
    match render_text(doc) {
        Ok(rendered) => panic!("the shape stands refused, the port rendered {rendered:?}"),
        Err(exc) => exc.to_string(),
    }
}

#[test]
fn the_minimal_table_stands_word_for_word() {
    let rendered = render_text(&base()).expect("the base document renders");
    assert_eq!(rendered, BASE_TEXT);
}

#[test]
fn the_heading_block_and_the_target_lines_stand_in_their_wording() {
    let mut doc = base();
    set(
        &mut doc,
        "target",
        text("a much longer target name past the padding"),
    );
    set(&mut doc, "target_unchanged", JsonValue::Bool(false));
    let rendered = render_text(&doc).expect("renders");
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(lines[0], "canaryfield catch table");
    assert_eq!(lines[1], "=======================");
    assert_eq!(lines[1].chars().count(), 23);
    assert_eq!(lines[2], "");
    assert_eq!(
        lines[3],
        "target           a much longer target name past the padding"
    );
    assert_eq!(lines[4], "target unchanged false");
    assert_eq!(lines[5], "clean exit for gate alpha is 0");
}

#[test]
fn the_clean_exit_line_spells_a_timeout_as_minus_one() {
    let mut doc = base();
    let mut exits = doc.get("clean_exits").expect("exits").clone();
    set(&mut exits, "alpha", JsonValue::Int("-1".to_string()));
    set(&mut doc, "clean_exits", exits);
    let rendered = render_text(&doc).expect("renders");
    assert!(rendered.contains("\nclean exit for gate alpha is -1\n"));
}

#[test]
fn the_width_counts_unicode_scalar_values() {
    let mut doc = base();
    let first = row(&mut doc, 0);
    set(first, "seed", text("s"));
    set(first, "name", text("éx"));
    let rendered = render_text(&doc).expect("renders");
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(lines[7], "seed  name  planted file  alpha");
    assert_eq!(lines[8], "----  ----  ------------  ------");
    assert_eq!(lines[9], "s     éx   mod.py        caught");
}

#[test]
fn the_continuation_line_follows_its_row_and_joins_every_file() {
    let mut doc = base();
    let first = row(&mut doc, 0);
    set(
        first,
        "planted_files",
        JsonValue::Array(vec![text("one.py"), text("two.py")]),
    );
    let rendered = render_text(&doc).expect("renders");
    let lines: Vec<&str> = rendered.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.starts_with("seed-01"))
        .expect("the row line stands");
    assert_eq!(lines[at + 1], "  planted in one.py, two.py");
}

#[test]
fn a_single_planted_file_stands_with_no_continuation() {
    let rendered = render_text(&base()).expect("renders");
    assert!(!rendered.contains("planted in"));
}

#[test]
fn the_unparseable_line_stands_after_a_blank_line() {
    let mut doc = base();
    set(
        &mut doc,
        "unparseable_files",
        JsonValue::Array(vec![text("bad.py"), text("worse.py")]),
    );
    let rendered = render_text(&doc).expect("renders");
    assert!(
        rendered.contains("\n\nfiles skipped as unparseable bad.py, worse.py\n\nsummary per gate")
    );
}

#[test]
fn an_empty_unparseable_list_stands_with_no_line() {
    let rendered = render_text(&base()).expect("renders");
    assert!(!rendered.contains("unparseable"));
}

#[test]
fn a_none_and_an_empty_planted_file_both_read_as_none() {
    for value in [JsonValue::Null, text("")] {
        let mut doc = base();
        set(row(&mut doc, 0), "planted_file", value);
        let rendered = render_text(&doc).expect("renders");
        assert!(
            rendered.contains("none"),
            "the column reads none: {rendered}"
        );
    }
}

#[test]
fn the_summary_line_stands_in_its_wording_and_spacing() {
    let rendered = render_text(&base()).expect("renders");
    let lines: Vec<&str> = rendered.lines().collect();
    let last = lines[lines.len() - 1];
    assert_eq!(
        last,
        "  alpha  caught 1  waved-through 0  not-applicable 0  gate-broken 0"
    );
}

#[test]
fn the_json_form_sorts_keys_at_every_level_and_stands_empty_containers_inline() {
    let doc = json::parse(r#"{"b": 1, "a": {"y": [1, 2], "x": {}}, "c": []}"#).expect("parses");
    assert_eq!(
        render_json(&doc),
        "{\n  \"a\": {\n    \"x\": {},\n    \"y\": [\n      1,\n      2\n    ]\n  },\n  \"b\": 1,\n  \"c\": []\n}\n"
    );
}

#[test]
fn the_json_form_escapes_the_python_way() {
    let doc = JsonValue::Object(vec![("k".to_string(), text("quo\"te\\back\nline\u{1}é😀"))]);
    assert_eq!(
        render_json(&doc),
        "{\n  \"k\": \"quo\\\"te\\\\back\\nline\\u0001\\u00e9\\ud83d\\ude00\"\n}\n"
    );
}

#[test]
fn the_json_form_spells_floats_the_python_way() {
    for (value, spelling) in [
        (1e20, "1e+20"),
        (1e-07, "1e-07"),
        (-0.0, "-0.0"),
        (f64::INFINITY, "Infinity"),
        (f64::NEG_INFINITY, "-Infinity"),
        (f64::NAN, "NaN"),
    ] {
        let doc = JsonValue::Object(vec![("v".to_string(), JsonValue::Float(value))]);
        assert_eq!(
            render_json(&doc),
            format!("{{\n  \"v\": {spelling}\n}}\n"),
            "float {value} spells {spelling}"
        );
    }
}

#[test]
fn the_json_form_carries_a_big_whole_number_digit_for_digit() {
    let doc = json::parse(r#"{"v": 123456789012345678901234567890}"#).expect("parses");
    assert_eq!(
        render_json(&doc),
        "{\n  \"v\": 123456789012345678901234567890\n}\n"
    );
}

#[test]
fn the_json_form_ends_with_exactly_one_newline() {
    let rendered = render_json(&base());
    assert!(rendered.ends_with("}\n") && !rendered.ends_with("}\n\n"));
}

#[test]
fn the_text_form_ignores_keys_outside_its_subset() {
    let mut doc = base();
    set(&mut doc, "branch", text("canaryfield/seed-01"));
    set(&mut doc, "plant_mode", text("git-branches"));
    let first = row(&mut doc, 0);
    set(first, "defect_class", text("boolean-flip"));
    let rendered = render_text(&doc).expect("renders");
    assert_eq!(rendered, BASE_TEXT);
}

#[test]
fn missing_keys_are_refused_naming_the_key() {
    let mut doc = base();
    remove(&mut doc, "gates");
    assert_eq!(refusal(&doc), "the result holds no key named \"gates\"");

    let mut doc = base();
    remove(row(&mut doc, 0), "seed");
    assert_eq!(refusal(&doc), "row 1 holds no key named \"seed\"");

    let mut doc = base();
    let mut exits = doc.get("clean_exits").expect("exits").clone();
    remove(&mut exits, "alpha");
    set(&mut doc, "clean_exits", exits);
    assert_eq!(refusal(&doc), "the clean exits hold no key named \"alpha\"");

    let mut doc = base();
    let first = row(&mut doc, 0);
    let mut gates = first.get("gates").expect("gates").clone();
    remove(&mut gates, "alpha");
    set(first, "gates", gates);
    assert_eq!(refusal(&doc), "row 1's gates hold no key named \"alpha\"");

    let mut doc = base();
    let mut summary = doc.get("summary").expect("summary").clone();
    let mut entry = summary.get("alpha").expect("entry").clone();
    remove(&mut entry, "caught");
    set(&mut summary, "alpha", entry);
    set(&mut doc, "summary", summary);
    assert_eq!(
        refusal(&doc),
        "the summary for gate \"alpha\" holds no key named \"caught\""
    );
}

#[test]
fn an_empty_rows_list_is_refused() {
    let mut doc = base();
    set(&mut doc, "rows", JsonValue::Array(vec![]));
    assert_eq!(
        refusal(&doc),
        "the rows list stands empty, and a table width holds no row to read from"
    );
}

#[test]
fn wrongly_kinded_values_are_refused_by_position() {
    let cases: Vec<(JsonValue, &str)> = {
        let mut built: Vec<(JsonValue, &str)> = Vec::new();
        let mut doc = base();
        set(row(&mut doc, 0), "seed", JsonValue::Int("7".to_string()));
        built.push((
            doc,
            "row 1's seed stands as a whole number where the shape asks text",
        ));
        let mut doc = base();
        set(
            row(&mut doc, 0),
            "seed",
            JsonValue::Array(vec![text("seed-01")]),
        );
        built.push((
            doc,
            "row 1's seed stands as a list where the shape asks text",
        ));
        let mut doc = base();
        set(&mut doc, "target", JsonValue::Int("5".to_string()));
        built.push((
            doc,
            "the target stands as a whole number where the shape asks text",
        ));
        let mut doc = base();
        set(&mut doc, "target_unchanged", text("yes"));
        built.push((
            doc,
            "the target_unchanged value stands as text where the shape asks a truth value",
        ));
        let mut doc = base();
        set(
            row(&mut doc, 0),
            "planted_file",
            JsonValue::Int("0".to_string()),
        );
        built.push((
            doc,
            "row 1's planted file stands as a whole number where the shape asks text or none",
        ));
        let mut doc = base();
        set(
            row(&mut doc, 0),
            "planted_files",
            JsonValue::Object(vec![("a.py".to_string(), JsonValue::Int("1".to_string()))]),
        );
        built.push((
            doc,
            "row 1's planted files value stands as an object where the shape asks a list",
        ));
        let mut doc = base();
        set(row(&mut doc, 0), "planted_files", text("ab"));
        built.push((
            doc,
            "row 1's planted files value stands as text where the shape asks a list",
        ));
        let mut doc = base();
        set(
            row(&mut doc, 0),
            "planted_files",
            JsonValue::Array(vec![JsonValue::Int("7".to_string())]),
        );
        built.push((
            doc,
            "an entry of row 1's planted files stands as a whole number where the shape asks text",
        ));
        let mut doc = base();
        set(
            &mut doc,
            "unparseable_files",
            JsonValue::Array(vec![JsonValue::Int("7".to_string())]),
        );
        built.push((
            doc,
            "an entry of the unparseable files stands as a whole number where the shape asks text",
        ));
        let mut doc = base();
        set(&mut doc, "unparseable_files", text("ab"));
        built.push((
            doc,
            "the unparseable files value stands as text where the shape asks a list",
        ));
        let mut doc = base();
        set(&mut doc, "gates", JsonValue::Object(vec![]));
        built.push((
            doc,
            "the gates value stands as an object where the shape asks a list",
        ));
        let mut doc = base();
        set(&mut doc, "rows", text("row"));
        built.push((
            doc,
            "the rows value stands as text where the shape asks a list",
        ));
        let mut doc = base();
        set(row(&mut doc, 0), "gates", JsonValue::Null);
        built.push((
            doc,
            "row 1's gates value stands as null where the shape asks an object",
        ));
        let mut doc = base();
        let mut summary = doc.get("summary").expect("summary").clone();
        set(&mut summary, "alpha", JsonValue::Int("4".to_string()));
        set(&mut doc, "summary", summary);
        built.push((
            doc,
            "the summary for gate \"alpha\" stands as a whole number where the shape asks an object",
        ));
        built
    };
    for (doc, expected) in cases {
        assert_eq!(refusal(&doc), expected);
    }
    assert_eq!(
        refusal(&JsonValue::Array(vec![JsonValue::Int("1".to_string())])),
        "the result stands as a list where the shape asks an object"
    );
}

#[test]
fn the_bridge_refusal_sentences_stand_in_the_command_line_source() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/canaryfield/cli.py");
    let source = fs::read_to_string(path).expect("the command line source reads");
    assert!(
        source.contains("the report renderings stand in the Rust crate under PORT4"),
        "the no binary sentence stands in the bridge"
    );
    assert!(
        source.contains("the named canaryfield binary does not stand:"),
        "the named binary sentence stands in the bridge"
    );
}

#[test]
fn the_door_renders_and_refuses_through_the_binary() {
    let dir = std::env::temp_dir().join(format!("canaryfield-port4-pins-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    let document = dir.join("result.json");
    fs::write(&document, BASE).expect("document writes");
    let binary = env!("CARGO_BIN_EXE_canaryfield");
    let rendered = Command::new(binary)
        .arg("--render-report")
        .arg(&document)
        .arg("text")
        .output()
        .expect("the door runs");
    assert!(rendered.status.success());
    assert_eq!(String::from_utf8(rendered.stdout).expect("utf8"), BASE_TEXT);
    let refused = Command::new(binary)
        .arg("--render-report")
        .arg(&document)
        .arg("xml")
        .output()
        .expect("the door runs");
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(
        String::from_utf8(refused.stderr).expect("utf8"),
        "the report form stands as \"xml\" where the door asks the word text or the word json\n"
    );
    let missing = Command::new(binary)
        .arg("--render-report")
        .arg(dir.join("no-such-document.json"))
        .arg("text")
        .output()
        .expect("the door runs");
    assert!(!missing.status.success());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_door_answers_an_unknown_argument_shape_with_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_canaryfield"))
        .arg("--render-report")
        .output()
        .expect("the door runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("utf8");
    assert!(
        stderr.starts_with("usage: canaryfield"),
        "usage line: {stderr}"
    );
}

#[test]
fn render_report_names_the_two_forms_and_no_other() {
    let doc = base();
    assert_eq!(
        render_report(&doc, "text").expect("text renders"),
        BASE_TEXT
    );
    assert!(render_report(&doc, "json")
        .expect("json renders")
        .ends_with("}\n"));
    match render_report(&doc, "yaml") {
        Ok(_) => panic!("a third form stands refused"),
        Err(exc) => assert_eq!(
            exc.to_string(),
            "the report form stands as \"yaml\" where the door asks the word text or the word json"
        ),
    }
}
