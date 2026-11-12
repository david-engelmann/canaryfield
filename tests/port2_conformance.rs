//! Conformance tests for Rust port slice 2, the seed applies
//! and plant_seed of PORT2.
//!
//! The cases in tests/conformance/port2_cases.json were
//! recorded from the landed Python package by the generator
//! at tools/gen_port2_recordings.py, run against the base
//! tree before the Python seed module retired. Each apply
//! case carries a source, the dump and the rendering the
//! Python spelled for it, and for every seed the answer of
//! a first and a second application, the bool and the dump
//! and the rendering after each, a surface standing in the
//! record only where it moved. An application the Python
//! raised out of is recorded with the exception it raised,
//! and the port's refusal is held against it. Each plant
//! case carries a staged directory and, for every seed,
//! the planted list, the skipped list, and the bytes of
//! every file afterwards. These tests replay the recordings
//! against the crate, offline, and compare exactly.

use std::fs;
use std::path::PathBuf;

use canaryfield::json::{self, JsonValue};
use canaryfield::pyapply::apply_seed;
use canaryfield::pyarena::{parse_tree, render_tree};
use canaryfield::pydump::dump_tree;
use canaryfield::pyplant::{plant_seed, PlantError};

fn recordings() -> Vec<JsonValue> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/conformance/port2_cases.json"
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

fn bool_of(value: &JsonValue) -> bool {
    match value {
        JsonValue::Bool(flag) => *flag,
        other => panic!("expected a bool value, got {other:?}"),
    }
}

fn list_of(value: &JsonValue) -> &[JsonValue] {
    match value {
        JsonValue::Array(items) => items,
        other => panic!("expected a list value, got {other:?}"),
    }
}

fn int_of(value: &JsonValue) -> usize {
    match value {
        JsonValue::Int(text) => text.parse().expect("a whole number"),
        other => panic!("expected a number value, got {other:?}"),
    }
}

/// Whether a recorded spelling matches a spelled text. A
/// recording spells a short text whole and a long one by
/// its sha256 and its length in characters.
fn spelling_matches(recorded: &JsonValue, text: &str) -> bool {
    if let Some(whole) = recorded.get("t") {
        return text_of(whole) == text;
    }
    let digest = text_of(recorded.get("sha256").expect("a digest stands"));
    let len = int_of(recorded.get("len").expect("a length stands"));
    text.chars().count() == len && sha256_hex(text.as_bytes()) == digest
}

/// Whether a recorded byte spelling matches file bytes.
fn bytes_match(recorded: &JsonValue, bytes: &[u8]) -> bool {
    if let Some(whole) = recorded.get("t") {
        return text_of(whole).as_bytes() == bytes;
    }
    let digest = text_of(recorded.get("b64sha").expect("a digest stands"));
    let len = int_of(recorded.get("len").expect("a length stands"));
    bytes.len() == len && sha256_hex(bytes) == digest
}

/// One surface answer, the spelling a surface produced or
/// the marker that it refused the way the Python raised.
enum Surface {
    Spelled(String),
    Raised,
}

fn check_surface(recorded: &JsonValue, surface: Surface, what: &str, case: &str, seed: &str) {
    match (recorded, surface) {
        (JsonValue::Str(marker), Surface::Raised) => {
            assert_eq!(marker, "recursion", "{case} {seed} {what} raised marker");
        }
        (JsonValue::Str(marker), Surface::Spelled(_)) => {
            panic!("{case} {seed} {what}: recorded {marker} but the port spelled it");
        }
        (_, Surface::Raised) => {
            panic!("{case} {seed} {what}: recorded a spelling but the port refused");
        }
        (recorded, Surface::Spelled(text)) => {
            assert!(
                spelling_matches(recorded, &text),
                "{case} {seed} {what} spelling differs"
            );
        }
    }
}

fn check_application(
    case_name: &str,
    seed_id: &str,
    record: &JsonValue,
    tree: &mut canaryfield::pyarena::Tree,
    standing_dump: &JsonValue,
    standing_render: &JsonValue,
) -> (JsonValue, JsonValue) {
    if let Some(raised) = record.get("apply") {
        assert_eq!(text_of(raised), "raised", "{case_name} {seed_id}");
        match apply_seed(tree, seed_id) {
            Err(_) => return (standing_dump.clone(), standing_render.clone()),
            Ok(answer) => {
                panic!("{case_name} {seed_id}: recorded a raise, the port answered {answer}")
            }
        }
    }
    let want = bool_of(record.get("bool").expect("a bool stands"));
    match apply_seed(tree, seed_id) {
        Ok(answer) => assert_eq!(answer, want, "{case_name} {seed_id} apply bool"),
        Err(error) => panic!("{case_name} {seed_id}: the port refused, {}", error.message),
    }
    let dump_record = record.get("dump").unwrap_or(standing_dump);
    let dump_surface = match dump_tree(tree) {
        Ok(text) => Surface::Spelled(text),
        Err(_) => Surface::Raised,
    };
    check_surface(dump_record, dump_surface, "dump", case_name, seed_id);
    let render_record = record.get("render").unwrap_or(standing_render);
    let render_surface = match render_tree(tree) {
        Ok(text) => Surface::Spelled(text),
        Err(_) => Surface::Raised,
    };
    check_surface(render_record, render_surface, "render", case_name, seed_id);
    (dump_record.clone(), render_record.clone())
}

#[test]
fn apply_cases_match_the_recorded_python_answers() {
    let mut compared = 0usize;
    let mut refused = 0usize;
    for case in recordings() {
        if text_of(case.get("kind").expect("a kind stands")) != "apply" {
            continue;
        }
        let name = text_of(case.get("name").expect("a name stands")).to_string();
        let source = text_of(case.get("source").expect("a source stands")).to_string();
        if text_of(case.get("parse").expect("a parse answer stands")) == "refused" {
            refused += 1;
            assert!(
                parse_tree(&source).is_err(),
                "{name}: recorded a parse refusal, the port parsed it"
            );
            continue;
        }
        let dump0 = case.get("dump0").expect("a dump stands").clone();
        let render0 = case.get("render0").expect("a render stands").clone();
        let seeds = case.get("seeds").expect("seed answers stand");
        for seed in canaryfield::registry::SEEDS {
            let entry = seeds.get(seed.id).expect("a seed entry stands");
            let mut tree = parse_tree(&source)
                .unwrap_or_else(|error| panic!("{name}: the port refused the parse: {}", error.message));
            let first = entry.get("first").expect("a first application stands");
            let (dump1, render1) =
                check_application(&name, seed.id, first, &mut tree, &dump0, &render0);
            if let Some(second) = entry.get("second") {
                check_application(&name, seed.id, second, &mut tree, &dump1, &render1);
            }
            compared += 1;
        }
    }
    assert!(compared > 4000, "the battery compared {compared} seed rows");
    assert!(refused > 0, "the battery held parse refusals");
}

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-port2-{}-{tag}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    dir
}

fn stage(case: &JsonValue, tag: &str) -> (PathBuf, PathBuf) {
    let base = fresh_dir(tag);
    let root = base.join("proj");
    fs::create_dir_all(&root).expect("project dir creates");
    let files = case.get("files").expect("files stand");
    if let JsonValue::Object(entries) = files {
        for (rel, spec) in entries {
            if rel == "__outside__" {
                let outside = base.join("outside.py");
                fs::write(&outside, text_of(spec.get("t").expect("outside text"))).expect("outside writes");
                continue;
            }
            let path = root.join(rel);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent dir creates");
            }
            if let Some(link) = spec.get("link") {
                #[cfg(unix)]
                std::os::unix::fs::symlink(text_of(link), &path).expect("symlink creates");
            } else if let Some(encoded) = spec.get("bytes_b64") {
                fs::write(&path, base64_decode(text_of(encoded))).expect("bytes write");
            } else {
                fs::write(&path, text_of(spec.get("t").expect("file text"))).expect("file writes");
            }
        }
    }
    (base, root)
}

fn collect_after(root: &PathBuf) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("dir reads") {
            let path = entry.expect("entry reads").path();
            let rel = path
                .strip_prefix(root)
                .expect("under root")
                .to_string_lossy()
                .replace('\\', "/");
            if path.is_symlink() {
                out.push((rel, path));
            } else if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                out.push((rel, path));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn plant_cases_match_the_recorded_python_answers() {
    let mut compared = 0usize;
    for case in recordings() {
        if text_of(case.get("kind").expect("a kind stands")) != "plant" {
            continue;
        }
        let name = text_of(case.get("name").expect("a name stands")).to_string();
        let seeds = case.get("seeds").expect("seed answers stand");
        for seed in canaryfield::registry::SEEDS {
            let tag = format!(
                "{}-{}",
                name.replace([':', '/'], "-"),
                seed.id
            );
            let (base, root) = stage(&case, &tag);
            let entry = seeds.get(seed.id).expect("a seed entry stands");
            let mut skipped: Vec<String> = Vec::new();
            let outcome = plant_seed(&root, seed.id, Some(&mut skipped));
            if let Some(raised) = entry.get("raised") {
                let _ = text_of(raised);
                match outcome {
                    Err(PlantError::Refusal(_)) => {}
                    other => panic!("{name} {}: recorded a raise, got {other:?}", seed.id),
                }
            } else {
                let planted = outcome
                    .unwrap_or_else(|error| panic!("{name} {}: the port failed: {error:?}", seed.id));
                let want: Vec<String> = list_of(entry.get("planted").expect("a planted list"))
                    .iter()
                    .map(|item| text_of(item).to_string())
                    .collect();
                assert_eq!(planted, want, "{name} {} planted list", seed.id);
                let want_skipped: Vec<String> =
                    list_of(entry.get("skipped").expect("a skipped list"))
                        .iter()
                        .map(|item| text_of(item).to_string())
                        .collect();
                assert_eq!(skipped, want_skipped, "{name} {} skipped list", seed.id);
            }
            let after = entry.get("after").expect("after bytes stand");
            let mut seen: Vec<String> = Vec::new();
            if let JsonValue::Object(entries) = after {
                for (rel, spec) in entries {
                    seen.push(rel.clone());
                    let path = root.join(rel);
                    if let Some(link) = spec.get("link") {
                        assert!(path.is_symlink(), "{name} {} {rel} stays a link", seed.id);
                        #[cfg(unix)]
                        {
                            let target = fs::read_link(&path).expect("link reads");
                            assert_eq!(
                                target.to_string_lossy(),
                                text_of(link),
                                "{name} {} {rel} link target",
                                seed.id
                            );
                        }
                    } else {
                        let bytes = fs::read(&path)
                            .unwrap_or_else(|_| panic!("{name} {} {rel} reads", seed.id));
                        assert!(
                            bytes_match(spec, &bytes),
                            "{name} {} {rel} bytes differ",
                            seed.id
                        );
                    }
                }
            }
            let present: Vec<String> = collect_after(&root)
                .into_iter()
                .map(|(rel, _)| rel)
                .collect();
            seen.sort();
            assert_eq!(present, seen, "{name} {} file set", seed.id);
            if let Some(outside) = entry.get("outside_after") {
                let bytes = fs::read(base.join("outside.py")).expect("outside reads");
                assert!(bytes_match(outside, &bytes), "{name} {} outside bytes", seed.id);
            }
            compared += 1;
            let _ = fs::remove_dir_all(&base);
        }
    }
    assert!(compared > 400, "the battery compared {compared} plant rows");
}

#[test]
fn unknown_seed_ids_are_refused() {
    let mut tree = parse_tree("x = 1\n").expect("source parses");
    assert!(apply_seed(&mut tree, "seed-99").is_err());
    let dir = fresh_dir("unknown-seed");
    assert!(plant_seed(&dir, "seed-99", None).is_err());
}

#[test]
fn plant_seed_on_a_missing_directory_plants_nothing() {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-port2-missing-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    let planted = plant_seed(&dir, "seed-05", None).expect("a missing root plants nothing");
    assert!(planted.is_empty());
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
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
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

fn base64_decode(text: &str) -> Vec<u8> {
    fn value(ch: u8) -> u32 {
        match ch {
            b'A'..=b'Z' => (ch - b'A') as u32,
            b'a'..=b'z' => (ch - b'a') as u32 + 26,
            b'0'..=b'9' => (ch - b'0') as u32 + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    }
    let bytes: Vec<u8> = text.bytes().filter(|ch| *ch != b'=').collect();
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let mut acc: u32 = 0;
        for (at, ch) in chunk.iter().enumerate() {
            acc |= value(*ch) << (18 - 6 * at);
        }
        let take = match chunk.len() {
            4 => 3,
            3 => 2,
            2 => 1,
            _ => 0,
        };
        for shift in [16, 8, 0].iter().take(take) {
            out.push((acc >> shift) as u8);
        }
    }
    out
}
