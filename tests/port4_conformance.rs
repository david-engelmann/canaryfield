//! Conformance tests for Rust port slice 4, the report
//! renderings of PORT4.
//!
//! The cases in tests/conformance/port4_cases.json were
//! recorded from the landed Python report module by
//! tools/gen_port4_recordings.py while that module still
//! stood, over synthetic result documents the generator built
//! itself and over the two committed example tables. A render
//! case carries both renderings the Python gave, a refusal
//! case carries the crash the Python met where it met one and
//! the JSON form's own answer beside it, and the one
//! divergence case carries the text rendering and the encoder
//! crash the Python met at a thousand levels of nesting.
//! These tests run offline and compare the port's answers
//! against the recordings.

use std::fs;

use canaryfield::json::{self, JsonValue};
use canaryfield::report::{render_json, render_text};

fn recordings() -> Vec<JsonValue> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/conformance/port4_cases.json"
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

fn field<'a>(case: &'a JsonValue, key: &str) -> &'a JsonValue {
    case.get(key)
        .unwrap_or_else(|| panic!("case carries no {key}"))
}

fn name_of(case: &JsonValue) -> &str {
    text_of(field(case, "name"))
}

fn kind_of(case: &JsonValue) -> &str {
    text_of(field(case, "kind"))
}

/// A recorded spelling stands whole under t, and a long one
/// stands as its sha256 over its UTF-8 bytes with its length
/// in characters, the convention of the PORT2 recordings.
fn spelling_matches(recorded: &JsonValue, text: &str) -> bool {
    if let Some(whole) = recorded.get("t") {
        return text_of(whole) == text;
    }
    let digest = text_of(recorded.get("sha256").expect("a digest stands"));
    let len = match recorded.get("len").expect("a length stands") {
        JsonValue::Int(raw) => raw.parse::<usize>().expect("length parses"),
        other => panic!("expected a length, got {other:?}"),
    };
    text.chars().count() == len && sha256_hex(text.as_bytes()) == digest
}

#[test]
fn the_case_census_stands_at_102() {
    let cases = recordings();
    assert_eq!(cases.len(), 102, "the case census stands at 102");
    let renders = cases
        .iter()
        .filter(|case| kind_of(case) == "render")
        .count();
    let refusals = cases
        .iter()
        .filter(|case| kind_of(case) == "refusal")
        .count();
    let divergences = cases
        .iter()
        .filter(|case| kind_of(case) == "divergence")
        .count();
    assert_eq!(renders, 19, "render cases stand at 19");
    assert_eq!(refusals, 82, "refusal cases stand at 82");
    assert_eq!(divergences, 1, "divergence cases stand at 1");
}

#[test]
fn render_cases_match_the_recorded_python_renderings() {
    let mut compared = 0;
    for case in recordings() {
        if kind_of(&case) != "render" {
            continue;
        }
        let name = name_of(&case).to_string();
        let input = field(&case, "input").clone();
        let text = render_text(&input)
            .unwrap_or_else(|exc| panic!("case {name}: the port refused a rendering case: {exc}"));
        assert!(
            spelling_matches(field(&case, "text"), &text),
            "case {name}: the text rendering differs from the recording"
        );
        let json = render_json(&input);
        assert!(
            spelling_matches(field(&case, "json"), &json),
            "case {name}: the JSON rendering differs from the recording"
        );
        compared += 1;
    }
    assert_eq!(compared, 19, "every render case compared");
}

#[test]
fn refusal_cases_are_refused_and_their_json_answers_match() {
    let mut compared = 0;
    for case in recordings() {
        if kind_of(&case) != "refusal" {
            continue;
        }
        let name = name_of(&case).to_string();
        let input = field(&case, "input").clone();
        assert!(
            case.get("crash").is_some() != case.get("text_rendered").is_some(),
            "case {name}: exactly one of a crash and a Python rendering stands recorded"
        );
        if let Some(crash) = case.get("crash") {
            let class = text_of(field(crash, "class"));
            assert!(
                matches!(class, "KeyError" | "TypeError" | "AttributeError"),
                "case {name}: the recorded crash class stands outside the family: {class}"
            );
        }
        if let Ok(rendered) = render_text(&input) {
            panic!(
                "case {name}: the Python's family stands refused and the port rendered {rendered:?}"
            );
        }
        if let Some(recorded) = case.get("json") {
            let json = render_json(&input);
            assert!(
                spelling_matches(recorded, &json),
                "case {name}: the JSON rendering differs from the recording"
            );
        }
        compared += 1;
    }
    assert_eq!(compared, 82, "every refusal case compared");
}

#[test]
fn the_divergence_case_renders_in_the_port_where_the_python_encoder_dies() {
    let mut compared = 0;
    for case in recordings() {
        if kind_of(&case) != "divergence" {
            continue;
        }
        let name = name_of(&case).to_string();
        let input = field(&case, "input").clone();
        let crash = field(&case, "json_crash").clone();
        assert_eq!(
            text_of(field(&crash, "class")),
            "RecursionError",
            "case {name}: the recorded encoder crash is the recursion error"
        );
        let text = render_text(&input)
            .unwrap_or_else(|exc| panic!("case {name}: the port refused the text form: {exc}"));
        assert!(
            spelling_matches(field(&case, "text"), &text),
            "case {name}: the text rendering differs from the recording"
        );
        let json = render_json(&input);
        assert!(
            json.ends_with('\n') && json.len() > 6000,
            "case {name}: the port renders the deep document within its bound"
        );
        compared += 1;
    }
    assert_eq!(compared, 1, "the divergence case compared");
}

#[test]
fn the_example_tables_render_to_the_committed_bytes() {
    for directory in ["examples", "examples/two-gates"] {
        let root = format!("{}/{directory}", env!("CARGO_MANIFEST_DIR"));
        let document_text =
            fs::read_to_string(format!("{root}/catch-table.json")).expect("example reads");
        let document = json::parse(&document_text).expect("example parses");
        let text = render_text(&document).expect("example renders as text");
        let committed_text =
            fs::read_to_string(format!("{root}/catch-table.txt")).expect("committed text reads");
        assert_eq!(text, committed_text, "{directory}: text bytes differ");
        let json = render_json(&document);
        assert_eq!(json, document_text, "{directory}: JSON bytes differ");
    }
}

// A compact sha256 over bytes, the digest the recordings
// carry for long spellings, written from the algorithm's
// own constants so the battery needs no dependency.

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256_hex(data: &[u8]) -> String {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (data.len() as u64) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    for block in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
            state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7],
        );
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}
