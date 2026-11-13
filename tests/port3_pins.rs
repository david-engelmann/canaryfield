//! Pins for Rust port slice 3, the repr unification of
//! PORT3. The seam set spells the separator and format
//! codepoints through both spelling surfaces, the pack
//! surface of json::py_repr and the tree surface of
//! pyunparse::str_repr, and the lone surrogate through
//! the tree surface, each pin carrying the spelling the
//! interpreter itself gives as its expected answer. The
//! source census pin reads src/json.rs as text and holds
//! that the deleted static table and the private
//! printable reading stand nowhere in it.

use canaryfield::json::{self, JsonValue};
use canaryfield::pyunparse::str_repr;

/// The separator codepoints of the seam, each with the
/// spelling the interpreter's repr gives it standing
/// between two letters.
const SEPARATORS: [(u32, &str); 4] = [
    (0x2028, "\\u2028"),
    (0x2029, "\\u2029"),
    (0xa0, "\\xa0"),
    (0x3000, "\\u3000"),
];

/// The format codepoints of the seam, spelled the same
/// way.
const FORMATS: [(u32, &str); 4] = [
    (0x200b, "\\u200b"),
    (0xfeff, "\\ufeff"),
    (0x2060, "\\u2060"),
    (0xad, "\\xad"),
];

fn both_surfaces(codepoint: u32, escaped: &str) {
    let ch = char::from_u32(codepoint).expect("the seam codepoint is a char");
    let text = format!("a{ch}b");
    let expected = format!("'a{escaped}b'");
    assert_eq!(
        json::py_repr(&JsonValue::Str(text)),
        expected,
        "pack surface, U+{codepoint:04X}"
    );
    assert_eq!(
        str_repr(&[0x61, codepoint, 0x62]),
        expected,
        "tree surface, U+{codepoint:04X}"
    );
}

#[test]
fn separator_codepoints_spell_identically_through_both_surfaces() {
    for (codepoint, escaped) in SEPARATORS {
        both_surfaces(codepoint, escaped);
    }
}

#[test]
fn format_codepoints_spell_identically_through_both_surfaces() {
    for (codepoint, escaped) in FORMATS {
        both_surfaces(codepoint, escaped);
    }
}

#[test]
fn the_lone_surrogate_spells_through_the_tree_surface() {
    assert_eq!(str_repr(&[0xd800]), "'\\ud800'");
    assert_eq!(str_repr(&[0x61, 0xd800, 0x62]), "'a\\ud800b'");
}

#[test]
fn the_deleted_table_and_private_reading_stand_nowhere_in_json_rs() {
    let source = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/json.rs"))
        .expect("src/json.rs reads as text");
    assert!(
        !source.contains("NON_PRINTABLE_RANGES"),
        "the static printable table stands in src/json.rs"
    );
    assert!(
        !source.contains("is_printable"),
        "the private printable reading stands in src/json.rs"
    );
}
