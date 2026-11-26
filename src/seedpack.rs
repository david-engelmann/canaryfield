//! The versioned seed pack file.
//!
//! The pack is the seed registry written out as one JSON
//! document so other tools in the line can consume the same
//! seed definitions without importing this crate. This module
//! is the Rust port of canaryfield/seedpack.py, and it carries
//! the pack document and its writer and its loader with every
//! refusal the Python loader promises, in the same text and in
//! the same order the Python checks them, as the contract port
//! section states.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::errors::CanaryfieldError;
use crate::json::{self, JsonValue};
use crate::registry::{SeedMeta, SEEDS};

/// The format a pack written now carries.
pub const SEED_PACK_FORMAT: &str = "canaryfield-seeds/40";

/// The thirty one pack formats and the entry count each one carries.
pub const PACK_ENTRY_COUNTS: [(&str, usize); 40] = [
    ("canaryfield-seeds/1", 12),
    ("canaryfield-seeds/2", 16),
    ("canaryfield-seeds/3", 20),
    ("canaryfield-seeds/4", 24),
    ("canaryfield-seeds/5", 28),
    ("canaryfield-seeds/6", 32),
    ("canaryfield-seeds/7", 36),
    ("canaryfield-seeds/8", 40),
    ("canaryfield-seeds/9", 44),
    ("canaryfield-seeds/10", 48),
    ("canaryfield-seeds/11", 52),
    ("canaryfield-seeds/12", 56),
    ("canaryfield-seeds/13", 60),
    ("canaryfield-seeds/14", 64),
    ("canaryfield-seeds/15", 68),
    ("canaryfield-seeds/16", 72),
    ("canaryfield-seeds/17", 76),
    ("canaryfield-seeds/18", 80),
    ("canaryfield-seeds/19", 84),
    ("canaryfield-seeds/20", 88),
    ("canaryfield-seeds/21", 92),
    ("canaryfield-seeds/22", 96),
    ("canaryfield-seeds/23", 100),
    ("canaryfield-seeds/24", 104),
    ("canaryfield-seeds/25", 108),
    ("canaryfield-seeds/26", 112),
    ("canaryfield-seeds/27", 116),
    ("canaryfield-seeds/28", 120),
    ("canaryfield-seeds/29", 124),
    ("canaryfield-seeds/30", 128),
    ("canaryfield-seeds/31", 132),
    ("canaryfield-seeds/32", 136),
    ("canaryfield-seeds/33", 140),
    ("canaryfield-seeds/34", 144),
    ("canaryfield-seeds/35", 148),
    ("canaryfield-seeds/36", 152),
    ("canaryfield-seeds/37", 156),
    ("canaryfield-seeds/38", 160),
    ("canaryfield-seeds/39", 164),
    ("canaryfield-seeds/40", 168),
];

/// The entry count a format carries, and nothing for a format
/// the loader does not accept.
pub fn pack_entry_count(format: &str) -> Option<usize> {
    PACK_ENTRY_COUNTS
        .iter()
        .find(|(name, _)| *name == format)
        .map(|(_, count)| *count)
}

/// The registry slice a format names, the first entries of the
/// one registry up to the count its format carries.
pub fn registry_for_format(format: &str) -> Option<&'static [SeedMeta]> {
    pack_entry_count(format).map(|count| &SEEDS[..count])
}

const ENTRY_FIELDS: [&str; 4] = ["id", "name", "defect_class", "description"];

/// One entry of a loaded pack, its fields in document order.
///
/// An entry carries the four fields the format names and any
/// field beyond them the document wrote, exactly as the Python
/// loader returns the entry it read.
#[derive(Debug, Clone, PartialEq)]
pub struct SeedEntry {
    fields: Vec<(String, JsonValue)>,
}

impl SeedEntry {
    /// The value the document wrote for a field, if it wrote
    /// one.
    pub fn get(&self, field: &str) -> Option<&JsonValue> {
        self.fields
            .iter()
            .rev()
            .find(|(name, _)| name == field)
            .map(|(_, value)| value)
    }

    /// The text the document wrote for a field, if it wrote
    /// text.
    pub fn text(&self, field: &str) -> Option<&str> {
        self.get(field).and_then(JsonValue::as_str)
    }

    /// The id of the entry.
    pub fn id(&self) -> Option<&str> {
        self.text("id")
    }

    /// The name of the entry.
    pub fn name(&self) -> Option<&str> {
        self.text("name")
    }

    /// The defect class of the entry.
    pub fn defect_class(&self) -> Option<&str> {
        self.text("defect_class")
    }

    /// The description of the entry.
    pub fn description(&self) -> Option<&str> {
        self.text("description")
    }

    /// The fields of the entry in document order.
    pub fn fields(&self) -> &[(String, JsonValue)] {
        &self.fields
    }
}

/// Return the pack as a document value shaped like the JSON
/// document the writer writes.
pub fn seed_pack_data() -> JsonValue {
    let entries: Vec<JsonValue> = SEEDS
        .iter()
        .map(|seed| {
            JsonValue::Object(vec![
                ("id".to_string(), JsonValue::Str(seed.id.to_string())),
                ("name".to_string(), JsonValue::Str(seed.name.to_string())),
                (
                    "defect_class".to_string(),
                    JsonValue::Str(seed.defect_class.to_string()),
                ),
                (
                    "description".to_string(),
                    JsonValue::Str(seed.description.to_string()),
                ),
            ])
        })
        .collect();
    JsonValue::Object(vec![
        (
            "format".to_string(),
            JsonValue::Str(SEED_PACK_FORMAT.to_string()),
        ),
        ("seeds".to_string(), JsonValue::Array(entries)),
    ])
}

/// Render the pack document exactly as the writer writes it,
/// the indented JSON and one trailing newline.
pub fn seed_pack_text() -> String {
    let mut text = json::dumps_indent2(&seed_pack_data());
    text.push('\n');
    text
}

/// Write the seed pack to path as JSON and return the path.
///
/// The parent directories of the path are created where they
/// do not stand, the way the Python writer creates them.
pub fn write_seed_pack(path: &Path) -> Result<PathBuf, CanaryfieldError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|exc| {
                CanaryfieldError::new(format!(
                    "cannot create the seed pack directory {} ({exc})",
                    parent.display()
                ))
            })?;
        }
    }
    std::fs::write(path, seed_pack_text()).map_err(|exc| {
        CanaryfieldError::new(format!(
            "cannot write the seed pack {} ({exc})",
            path.display()
        ))
    })?;
    Ok(path.to_path_buf())
}

/// Load a seed pack and return its list of seed entries.
///
/// Raises the crate error when the document is not a pack of an
/// accepted format, or when its entry count disagrees with the
/// count its format version carries, or when its ids are not
/// exactly the ids its format version carries, or when an entry
/// disagrees with the registry on a name or a defect class, in
/// the words and in the order the Python loader raises them.
pub fn load_seed_pack(path: &Path) -> Result<Vec<SeedEntry>, CanaryfieldError> {
    let shown = path.display().to_string();
    let raw = std::fs::read(path).map_err(|exc| {
        CanaryfieldError::new(format!("cannot read the seed pack {shown} ({exc})"))
    })?;
    let text = String::from_utf8(raw)
        .map_err(|_| CanaryfieldError::new(format!("the seed pack is not UTF-8 text: {shown}")))?;
    let data = json::parse(&text)?;
    let not_a_pack = || CanaryfieldError::new(format!("not a canaryfield seed pack: {shown}"));
    if !matches!(data, JsonValue::Object(_)) {
        return Err(not_a_pack());
    }
    let pack_format = match data.get("format") {
        Some(JsonValue::Str(format)) if pack_entry_count(format).is_some() => format.clone(),
        _ => return Err(not_a_pack()),
    };
    let count = pack_entry_count(&pack_format).expect("accepted format");
    let entries_value = match data.get("seeds") {
        Some(JsonValue::Array(items)) => items.clone(),
        _ => {
            return Err(CanaryfieldError::new(format!(
                "seed pack carries no seed list: {shown}"
            )));
        }
    };
    if entries_value.len() != count {
        return Err(CanaryfieldError::new(format!(
            "seed pack carries {} seeds and format {pack_format} carries {count}: {shown}",
            entries_value.len()
        )));
    }
    let mut seen: HashSet<String> = HashSet::new();
    let mut entries: Vec<SeedEntry> = Vec::new();
    for item in &entries_value {
        let fields = match item {
            JsonValue::Object(pairs) => pairs.clone(),
            _ => {
                return Err(CanaryfieldError::new(format!(
                    "seed pack entry is missing fields: {}",
                    json::py_repr(item)
                )));
            }
        };
        let entry = SeedEntry { fields };
        if ENTRY_FIELDS.iter().any(|field| entry.get(field).is_none()) {
            return Err(CanaryfieldError::new(format!(
                "seed pack entry is missing fields: {}",
                json::py_repr(item)
            )));
        }
        for field in ENTRY_FIELDS {
            let holds_text = match entry.text(field) {
                Some(text) => text.chars().any(|ch| !is_python_whitespace(ch)),
                None => false,
            };
            if !holds_text {
                return Err(CanaryfieldError::new(format!(
                    "seed pack entry field is not a non empty piece of text, {field} in {}",
                    json::py_repr(item)
                )));
            }
        }
        let id = entry.id().expect("field text checked").to_string();
        if seen.contains(&id) {
            return Err(CanaryfieldError::new(format!(
                "seed pack carries a duplicate id: {id}"
            )));
        }
        seen.insert(id);
        entries.push(entry);
    }
    let registry = registry_for_format(&pack_format).expect("accepted format");
    let registry_ids: HashSet<&str> = registry.iter().map(|seed| seed.id).collect();
    let seen_refs: HashSet<&str> = seen.iter().map(String::as_str).collect();
    if seen_refs != registry_ids {
        return Err(CanaryfieldError::new(format!(
            "seed pack ids do not match format {pack_format}: {shown}"
        )));
    }
    for entry in &entries {
        let seed = registry
            .iter()
            .find(|seed| seed.id == entry.id().expect("field text checked"))
            .expect("ids matched the registry");
        for (field, wanted) in [("name", seed.name), ("defect_class", seed.defect_class)] {
            if entry.text(field) != Some(wanted) {
                return Err(CanaryfieldError::new(format!(
                    "seed pack entry {} disagrees with the registry on {field}: {shown}",
                    entry.id().expect("field text checked")
                )));
            }
        }
    }
    Ok(entries)
}

/// Whether a character is whitespace under the reading the
/// Python strip applies, which is wider than the whitespace of
/// the JSON format itself.
fn is_python_whitespace(ch: char) -> bool {
    matches!(
        ch,
        '\u{9}'..='\u{d}'
            | '\u{1c}'..='\u{1f}'
            | ' '
            | '\u{85}'
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_twenty_three_formats_carry_their_own_counts() {
        assert_eq!(pack_entry_count("canaryfield-seeds/1"), Some(12));
        assert_eq!(pack_entry_count("canaryfield-seeds/10"), Some(48));
        assert_eq!(pack_entry_count("canaryfield-seeds/11"), Some(52));
        assert_eq!(pack_entry_count("canaryfield-seeds/12"), Some(56));
        assert_eq!(pack_entry_count("canaryfield-seeds/13"), Some(60));
        assert_eq!(pack_entry_count("canaryfield-seeds/14"), Some(64));
        assert_eq!(pack_entry_count("canaryfield-seeds/15"), Some(68));
        assert_eq!(pack_entry_count("canaryfield-seeds/16"), Some(72));
        assert_eq!(pack_entry_count("canaryfield-seeds/17"), Some(76));
        assert_eq!(pack_entry_count("canaryfield-seeds/18"), Some(80));
        assert_eq!(pack_entry_count("canaryfield-seeds/19"), Some(84));
        assert_eq!(pack_entry_count("canaryfield-seeds/20"), Some(88));
        assert_eq!(pack_entry_count("canaryfield-seeds/21"), Some(92));
        assert_eq!(pack_entry_count("canaryfield-seeds/22"), Some(96));
        assert_eq!(pack_entry_count("canaryfield-seeds/23"), Some(100));
        assert_eq!(pack_entry_count("canaryfield-seeds/24"), Some(104));
        assert_eq!(pack_entry_count("canaryfield-seeds/25"), Some(108));
        assert_eq!(pack_entry_count("canaryfield-seeds/26"), Some(112));
        assert_eq!(pack_entry_count("other/9"), None);
    }

    #[test]
    fn the_current_document_carries_one_hundred_and_forty_four_entries_in_order() {
        let data = seed_pack_data();
        assert_eq!(
            data.get("format"),
            Some(&JsonValue::Str(SEED_PACK_FORMAT.to_string()))
        );
        match data.get("seeds") {
            Some(JsonValue::Array(items)) => {
                assert_eq!(items.len(), 168);
                assert_eq!(
                    items[0].get("id"),
                    Some(&JsonValue::Str("seed-00".to_string()))
                );
                assert_eq!(
                    items[79].get("id"),
                    Some(&JsonValue::Str("seed-79".to_string()))
                );
                assert_eq!(
                    items[87].get("id"),
                    Some(&JsonValue::Str("seed-87".to_string()))
                );
                assert_eq!(
                    items[91].get("id"),
                    Some(&JsonValue::Str("seed-91".to_string()))
                );
                assert_eq!(
                    items[95].get("id"),
                    Some(&JsonValue::Str("seed-95".to_string()))
                );
                assert_eq!(
                    items[99].get("id"),
                    Some(&JsonValue::Str("seed-99".to_string()))
                );
                assert_eq!(
                    items[103].get("id"),
                    Some(&JsonValue::Str("seed-103".to_string()))
                );
                assert_eq!(
                    items[107].get("id"),
                    Some(&JsonValue::Str("seed-107".to_string()))
                );
                assert_eq!(
                    items[111].get("id"),
                    Some(&JsonValue::Str("seed-111".to_string()))
                );
                assert_eq!(
                    items[131].get("id"),
                    Some(&JsonValue::Str("seed-131".to_string()))
                );
                assert_eq!(
                    items[135].get("id"),
                    Some(&JsonValue::Str("seed-135".to_string()))
                );
                assert_eq!(
                    items[139].get("id"),
                    Some(&JsonValue::Str("seed-139".to_string()))
                );
                assert_eq!(
                    items[143].get("id"),
                    Some(&JsonValue::Str("seed-143".to_string()))
                );
            }
            other => panic!("seeds is not a list: {other:?}"),
        }
    }

    #[test]
    fn the_written_text_ends_with_one_newline() {
        let text = seed_pack_text();
        assert!(text.ends_with("}\n"));
        assert!(!text.ends_with("\n\n"));
    }
}
