//! Pins for the Rust port slice 1 review fixes.
//!
//! The red team and the two axis review of port slice 1 found
//! the repr printable reading narrower than the Python one and
//! the JSON decoder unbounded in its nesting. These tests pin
//! the repairs against answers recorded from the landed Python
//! by the probe program kept in the Round 14 scratch. The repr
//! pins and the past the bound pins fail on the pre fix head for
//! the reasons the review reports name. The at the bound pin
//! passes on both heads by construction, because a document at
//! four thousand levels parsed before the bound stood and must
//! still parse with it, so it stands here as a guard on the
//! bound and not as discrimination. The deep pins run their
//! bodies on threads carrying stacks sized for the descent,
//! because the test harness thread stack is smaller than the
//! descent to the bound and the answer under test is the
//! decoder's and not the harness's.

use std::fs;
use std::path::PathBuf;

use canaryfield::json::{self, JsonValue};
use canaryfield::{load_seed_pack, seed_pack_data};

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-port1-fixes-{}-{tag}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    dir
}

fn entry_pairs_mut(data: &mut JsonValue, index: usize) -> &mut Vec<(String, JsonValue)> {
    let seeds = match data {
        JsonValue::Object(pairs) => pairs
            .iter_mut()
            .find(|(name, _)| name == "seeds")
            .map(|(_, value)| value),
        _ => None,
    }
    .expect("the pack document carries its seed list");
    match seeds {
        JsonValue::Array(items) => match &mut items[index] {
            JsonValue::Object(pairs) => pairs,
            other => panic!("entry is not an object: {other:?}"),
        },
        other => panic!("the seed list is not a list: {other:?}"),
    }
}

fn on_deep_stack<F>(body: F)
where
    F: FnOnce() + Send + 'static,
{
    let handle = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(body)
        .expect("the deep stack thread spawns");
    match handle.join() {
        Ok(()) => {}
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

fn write_document(tag: &str, data: &JsonValue) -> PathBuf {
    let dir = fresh_dir(tag);
    let pack = dir.join("pack.json");
    let mut text = json::dumps_indent2(data);
    text.push('\n');
    fs::write(&pack, text).expect("pack document writes");
    pack
}

#[test]
fn repr_escapes_every_witness_the_review_named() {
    // The witnesses the red team probed, by category. Three
    // format characters first, then the private use character,
    // then two unassigned characters, each with the spelling
    // the Python repr gives it standing between two letters.
    let witnesses: [(char, &str); 11] = [
        ('\u{ad}', "\\xad"),
        ('\u{200b}', "\\u200b"),
        ('\u{200c}', "\\u200c"),
        ('\u{200d}', "\\u200d"),
        ('\u{200e}', "\\u200e"),
        ('\u{2060}', "\\u2060"),
        ('\u{600}', "\\u0600"),
        ('\u{e000}', "\\ue000"),
        ('\u{378}', "\\u0378"),
        ('\u{fdd0}', "\\ufdd0"),
        ('\u{ffff}', "\\uffff"),
    ];
    for (ch, escaped) in witnesses {
        let text = format!("a{ch}b");
        assert_eq!(
            json::py_repr(&JsonValue::Str(text)),
            format!("'a{escaped}b'"),
            "witness U+{:04X}",
            ch as u32
        );
    }
}

#[test]
fn repr_still_stands_printable_text_and_the_plain_space() {
    assert_eq!(
        json::py_repr(&JsonValue::Str("plain text here".into())),
        "'plain text here'"
    );
}

#[test]
fn the_missing_fields_refusal_carries_the_witness_reprs() {
    // Recorded from the landed Python loader. The first entry
    // of the current pack loses its description and gains a
    // note field carrying the format, private use, and
    // unassigned witnesses, and the refusal names the entry
    // with every witness escaped the Python way.
    let note: String = ['a', '\u{ad}', '\u{200b}', '\u{600}', '\u{e000}', '\u{378}', 'b']
        .into_iter()
        .collect();
    let mut data = seed_pack_data();
    let pairs = entry_pairs_mut(&mut data, 0);
    pairs.retain(|(name, _)| name != "description");
    pairs.push(("note".to_string(), JsonValue::Str(note)));
    let pack = write_document("missing-fields-witnesses", &data);
    let err = load_seed_pack(&pack).expect_err("the pack is refused");
    assert_eq!(
        err.to_string(),
        "seed pack entry is missing fields: {'id': 'seed-00', \
         'name': 'stale version string', 'defect_class': 'stale metadata', \
         'note': 'a\\xad\\u200b\\u0600\\ue000\\u0378b'}"
    );
}

#[test]
fn the_second_entry_refusal_carries_the_witness_reprs() {
    // Recorded from the landed Python loader the same way, on
    // the second entry losing its id, so the repr order of the
    // remaining fields is pinned with the witnesses inside it.
    let note: String = ['x', '\u{ad}', '\u{200b}', '\u{600}', '\u{e000}', '\u{378}', 'y']
        .into_iter()
        .collect();
    let mut data = seed_pack_data();
    let pairs = entry_pairs_mut(&mut data, 1);
    pairs.retain(|(name, _)| name != "id");
    pairs.push(("note".to_string(), JsonValue::Str(note)));
    let pack = write_document("missing-fields-witnesses-two", &data);
    let err = load_seed_pack(&pack).expect_err("the pack is refused");
    assert_eq!(
        err.to_string(),
        "seed pack entry is missing fields: {'name': 'dropped return value', \
         'defect_class': 'missing result', \
         'description': 'Drops the value from a return so the caller gets nothing', \
         'note': 'x\\xad\\u200b\\u0600\\ue000\\u0378y'}"
    );
}

#[test]
fn a_document_at_the_nesting_bound_still_parses() {
    // A guard by construction, named as one in the file head.
    // Four thousand nested lists parsed before the bound stood
    // and must still parse with the bound standing.
    on_deep_stack(|| {
        let text = format!("{}{}", "[".repeat(4000), "]".repeat(4000));
        assert!(json::parse(&text).is_ok());
    });
}

#[test]
fn a_document_past_the_nesting_bound_is_refused() {
    on_deep_stack(|| {
        for depth in [4001, 8000] {
            let text = format!("{}{}", "[".repeat(depth), "]".repeat(depth));
            let err = json::parse(&text).expect_err("a deep document is refused");
            assert!(
                err.to_string()
                    .contains("document nested deeper than four thousand levels"),
                "depth {depth} refused with the decoder wording, got: {err}"
            );
        }
        let mut text = String::new();
        for _ in 0..4001 {
            text.push_str("{\"a\":");
        }
        text.push('1');
        for _ in 0..4001 {
            text.push('}');
        }
        let err = json::parse(&text).expect_err("a deep object document is refused");
        assert!(
            err.to_string()
                .contains("document nested deeper than four thousand levels"),
            "deep object refused with the decoder wording, got: {err}"
        );
    });
}

#[test]
fn the_loader_refuses_a_deep_pack_at_the_bound() {
    let dir = fresh_dir("deep-pack");
    let pack = dir.join("pack.json");
    let text = format!("{}{}", "[".repeat(4001), "]".repeat(4001));
    fs::write(&pack, text).expect("deep document writes");
    on_deep_stack(move || {
        let err = load_seed_pack(&pack).expect_err("a deep pack is refused");
        assert!(
            err.to_string()
                .contains("document nested deeper than four thousand levels"),
            "the loader refusal carries the decoder wording, got: {err}"
        );
    });
}
