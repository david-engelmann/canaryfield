//! Conformance tests for Rust port slice 1.
//!
//! The cases in tests/conformance/port1_cases.json were recorded
//! from the landed Python package by the probe program kept in
//! the Round 14 scratch. Each load case carries the exact pack
//! document text the Python loader was fed and the answer it
//! gave, the entries field by field in document order or the
//! refusal word by word with the token {PATH} standing where
//! the recorded pack path stood. The data case carries the bytes
//! the Python writer writes for the current pack. These tests
//! stand their own fresh pack path in for the token and compare
//! the Rust answers against the recordings at content level, and
//! they run offline.

use std::fs;
use std::path::PathBuf;

use canaryfield::json::{self, JsonValue};
use canaryfield::{load_seed_pack, seed_pack_text, write_seed_pack};

fn recordings() -> Vec<JsonValue> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/conformance/port1_cases.json"
    );
    let text = fs::read_to_string(path).expect("recordings file reads");
    let parsed = json::parse(&text).expect("recordings parse");
    match parsed.get("cases") {
        Some(JsonValue::Array(cases)) => cases.clone(),
        other => panic!("recordings carry no case list: {other:?}"),
    }
}

fn text_of(value: &JsonValue) -> &str {
    match value {
        JsonValue::Str(text) => text,
        other => panic!("expected a text value, got {other:?}"),
    }
}

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-conformance-{}-{tag}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    dir
}

fn entry_fields(value: &JsonValue) -> Vec<(String, JsonValue)> {
    match value {
        JsonValue::Object(pairs) => pairs.clone(),
        other => panic!("expected an entry object, got {other:?}"),
    }
}

#[test]
fn load_cases_match_the_recorded_python_answers() {
    let mut compared = 0;
    for case in recordings() {
        if text_of(case.get("kind").expect("kind")) != "load" {
            continue;
        }
        let name = text_of(case.get("name").expect("name")).to_string();
        let document = text_of(case.get("document").expect("document")).to_string();
        let dir = fresh_dir(&name);
        let pack = dir.join("pack.json");
        fs::write(&pack, &document).expect("pack document writes");
        let answer = load_seed_pack(&pack);
        if let Some(expected_error) = case.get("expected_error") {
            let wanted = text_of(expected_error).replace("{PATH}", &pack.display().to_string());
            match answer {
                Ok(entries) => panic!(
                    "case {name}: the Python refused this pack and the port loaded {} entries",
                    entries.len()
                ),
                Err(exc) => assert_eq!(exc.to_string(), wanted, "case {name}"),
            }
        } else {
            let expected = case.get("expected_entries").expect("expected entries");
            let expected_list = match expected {
                JsonValue::Array(items) => items,
                other => panic!("case {name}: expected entries not a list: {other:?}"),
            };
            match answer {
                Err(exc) => panic!(
                    "case {name}: the Python loaded this pack and the port refused it: {exc}"
                ),
                Ok(entries) => {
                    assert_eq!(entries.len(), expected_list.len(), "case {name}");
                    for (got, wanted) in entries.iter().zip(expected_list.iter()) {
                        assert_eq!(got.fields(), entry_fields(wanted).as_slice(), "case {name}");
                    }
                }
            }
        }
        compared += 1;
    }
    assert_eq!(compared, 56, "the load case census stands at 56");
}

#[test]
fn the_pack_document_and_written_bytes_match_the_recording() {
    let mut compared = 0;
    for case in recordings() {
        if text_of(case.get("kind").expect("kind")) != "data" {
            continue;
        }
        let expected_text = text_of(case.get("expected_text").expect("expected text"));
        assert_eq!(
            seed_pack_text(),
            expected_text,
            "the rendered pack document"
        );
        let dir = fresh_dir("data");
        let nested = dir.join("nested").join("deeper").join("pack.json");
        let returned = write_seed_pack(&nested).expect("the pack writes");
        assert_eq!(returned, nested);
        let written = fs::read_to_string(&nested).expect("the written pack reads");
        assert_eq!(written, expected_text, "the written pack bytes");
        compared += 1;
    }
    assert_eq!(compared, 1, "the data case census stands at 1");
}

#[test]
fn the_rendered_pack_matches_the_committed_example_byte_for_byte() {
    let example = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/seed-pack.json");
    let committed = fs::read_to_string(example).expect("the committed example reads");
    assert_eq!(seed_pack_text(), committed);
}
