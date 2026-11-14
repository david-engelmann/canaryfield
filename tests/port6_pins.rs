//! Pins seated by the PORT6-6 audit of the retiring Python
//! suites.
//!
//! The audit of tests/test_registry.py and
//! tests/test_seedpack.py against the crate's standing
//! tests found their substance pinned by the PORT1 load
//! battery and the pack data case, with one gap: the
//! Python pins refuse an invented or grown registry id
//! under the version one and version two pack formats by
//! name, where the recorded battery carries the grown
//! shape under version nine and the invented shape under
//! the current format. These pins carry the version one
//! and version two spellings of both refusals against the
//! crate loader, so a loader that read a format's slice
//! loosely fails here. The remaining Python pins retire
//! with the suite as surfaces of the Python package
//! itself, as the PORT6 build report names them.

use std::fs;
use std::path::PathBuf;

use canaryfield::json::{dumps_indent2, JsonValue};
use canaryfield::{load_seed_pack, registry_for_format, seed_pack_data};

fn fresh_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-port6-pins-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("the pin directory stands up");
    dir
}

/// Rewrite the current pack document to an earlier format
/// over that format's registry slice, with one entry's id
/// replaced as the caller directs.
fn doctored_pack(name: &str, format: &str, replace_at: usize, new_id: &str) -> PathBuf {
    let slice = registry_for_format(format).expect("the format's slice stands");
    let mut data = seed_pack_data();
    let pairs = match &mut data {
        JsonValue::Object(pairs) => pairs,
        other => panic!("the pack data stands as an object: {other:?}"),
    };
    for (key, value) in pairs.iter_mut() {
        if key == "format" {
            *value = JsonValue::Str(format.to_string());
        }
        if key == "seeds" {
            let by_id: Vec<JsonValue> = match seed_pack_data() {
                JsonValue::Object(top) => match top
                    .iter()
                    .find(|(key, _)| key == "seeds")
                    .map(|(_, value)| value)
                {
                    Some(JsonValue::Array(items)) => items.clone(),
                    other => panic!("the seed list stands as a list: {other:?}"),
                },
                other => panic!("the pack data stands as an object: {other:?}"),
            };
            let mut chosen: Vec<JsonValue> = slice
                .iter()
                .map(|entry| {
                    by_id
                        .iter()
                        .find(|item| item.get("id") == Some(&JsonValue::Str(entry.id.to_string())))
                        .expect("the slice entry stands in the pack data")
                        .clone()
                })
                .collect();
            if let JsonValue::Object(entry_pairs) = &mut chosen[replace_at] {
                for (key, value) in entry_pairs.iter_mut() {
                    if key == "id" {
                        *value = JsonValue::Str(new_id.to_string());
                    }
                }
            }
            *value = JsonValue::Array(chosen);
        }
    }
    let dir = fresh_dir(name);
    let path = dir.join("pack.json");
    fs::write(&path, dumps_indent2(&data)).expect("the doctored pack writes");
    path
}

#[test]
fn a_version_one_pack_with_an_invented_id_is_refused_on_the_id_set() {
    let path = doctored_pack("v1-invented", "canaryfield-seeds/1", 0, "seed-99");
    let err = load_seed_pack(&path).expect_err("the pack is refused");
    assert!(
        err.to_string()
            .contains("seed pack ids do not match format canaryfield-seeds/1"),
        "the refusal names the format's id set, got: {err}"
    );
}

#[test]
fn a_version_one_pack_carrying_a_grown_registry_id_is_refused_on_the_id_set() {
    // Entry eleven of the version one slice is seed-11; the
    // Python pin replaces it with the seed-12 entry, the
    // first seed outside the slice.
    let path = doctored_pack("v1-grown", "canaryfield-seeds/1", 11, "seed-12");
    let err = load_seed_pack(&path).expect_err("the pack is refused");
    assert!(
        err.to_string()
            .contains("seed pack ids do not match format canaryfield-seeds/1"),
        "the refusal names the format's id set, got: {err}"
    );
}

#[test]
fn a_version_two_pack_with_an_invented_id_is_refused_on_the_id_set() {
    let path = doctored_pack("v2-invented", "canaryfield-seeds/2", 15, "seed-99");
    let err = load_seed_pack(&path).expect_err("the pack is refused");
    assert!(
        err.to_string()
            .contains("seed pack ids do not match format canaryfield-seeds/2"),
        "the refusal names the format's id set, got: {err}"
    );
}
