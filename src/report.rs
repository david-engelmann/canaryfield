//! The report renderings, the text catch table and the JSON
//! document, ported from canaryfield/report.py under PORT4.
//!
//! The text form reads only the standing subset of the result
//! the contract names and ignores every other key at every
//! level, and the JSON form carries every key the result
//! holds, its object pairs sorted by key at every level before
//! the crate's indented writer renders them. The input is the
//! engine's result shape and nothing wider. Every crash the
//! landed Python meets over that shape's family, and every
//! general reading through which the Python would spell a
//! wrongly kinded value instead of crashing, is refused here
//! in this module's own wording, carried by CanaryfieldError,
//! a missing key named by its key and a wrongly kinded value
//! named by its position and the kind the shape asks there.

use crate::errors::CanaryfieldError;
use crate::json::{dumps_indent2, JsonValue};

fn kind_name(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Null => "null",
        JsonValue::Bool(_) => "a truth value",
        JsonValue::Int(_) => "a whole number",
        JsonValue::Float(_) => "a number",
        JsonValue::Str(_) => "text",
        JsonValue::Array(_) => "a list",
        JsonValue::Object(_) => "an object",
    }
}

fn wrong_kind(position: &str, value: &JsonValue, asked: &str) -> CanaryfieldError {
    CanaryfieldError::new(format!(
        "{position} stands as {} where the shape asks {asked}",
        kind_name(value)
    ))
}

fn missing(owner: &str, key: &str) -> CanaryfieldError {
    CanaryfieldError::new(format!("{owner} holds no key named \"{key}\""))
}

fn key_of<'a>(
    value: &'a JsonValue,
    key: &str,
    owner: &str,
) -> Result<&'a JsonValue, CanaryfieldError> {
    value.get(key).ok_or_else(|| missing(owner, key))
}

fn as_object<'a>(value: &'a JsonValue, position: &str) -> Result<&'a JsonValue, CanaryfieldError> {
    match value {
        JsonValue::Object(_) => Ok(value),
        other => Err(wrong_kind(position, other, "an object")),
    }
}

fn as_text<'a>(value: &'a JsonValue, position: &str) -> Result<&'a str, CanaryfieldError> {
    match value {
        JsonValue::Str(text) => Ok(text),
        other => Err(wrong_kind(position, other, "text")),
    }
}

fn as_int<'a>(value: &'a JsonValue, position: &str) -> Result<&'a str, CanaryfieldError> {
    match value {
        JsonValue::Int(raw) => Ok(raw),
        other => Err(wrong_kind(position, other, "a whole number")),
    }
}

/// Read one of the two optional lists through the Python's
/// get and or reading. A missing key and any falsy value
/// standing in its place read as the empty list, and a truthy
/// value that is no list, and any entry that is not text,
/// stand refused on the shape ground PORT4-6 states.
fn optional_list(
    value: Option<&JsonValue>,
    position: &str,
    entry_position: &str,
) -> Result<Vec<String>, CanaryfieldError> {
    let items = match value {
        None | Some(JsonValue::Null) => return Ok(Vec::new()),
        Some(JsonValue::Str(text)) if text.is_empty() => return Ok(Vec::new()),
        Some(JsonValue::Int(raw)) if raw == "0" => return Ok(Vec::new()),
        Some(JsonValue::Float(number)) if *number == 0.0 => return Ok(Vec::new()),
        Some(JsonValue::Bool(false)) => return Ok(Vec::new()),
        Some(JsonValue::Array(items)) => items,
        Some(JsonValue::Object(pairs)) if pairs.is_empty() => return Ok(Vec::new()),
        Some(other) => return Err(wrong_kind(position, other, "a list")),
    };
    let mut entries = Vec::new();
    for item in items {
        match item {
            JsonValue::Str(text) => entries.push(text.clone()),
            other => return Err(wrong_kind(entry_position, other, "text")),
        }
    }
    Ok(entries)
}

struct Row {
    cells: Vec<String>,
    planted_files: Vec<String>,
}

struct Shape {
    gates: Vec<String>,
    target: String,
    target_unchanged: bool,
    clean_exits: Vec<String>,
    rows: Vec<Row>,
    unparseable_files: Vec<String>,
    summaries: Vec<Vec<String>>,
}

const COUNT_KEYS: [&str; 4] = ["caught", "waved-through", "not-applicable", "gate-broken"];

fn read_shape(result: &JsonValue) -> Result<Shape, CanaryfieldError> {
    let root = as_object(result, "the result")?;
    let gates_value = key_of(root, "gates", "the result")?;
    let gate_items = match gates_value {
        JsonValue::Array(items) => items,
        other => return Err(wrong_kind("the gates value", other, "a list")),
    };
    let mut gates: Vec<String> = Vec::new();
    for item in gate_items {
        match item {
            JsonValue::Str(text) => gates.push(text.clone()),
            other => return Err(wrong_kind("a gate name", other, "text")),
        }
    }
    let target = as_text(key_of(root, "target", "the result")?, "the target")?.to_string();
    let unchanged_value = key_of(root, "target_unchanged", "the result")?;
    let target_unchanged = match unchanged_value {
        JsonValue::Bool(flag) => *flag,
        other => {
            return Err(wrong_kind(
                "the target_unchanged value",
                other,
                "a truth value",
            ))
        }
    };
    let exits_value = as_object(
        key_of(root, "clean_exits", "the result")?,
        "the clean exits value",
    )?;
    let mut clean_exits: Vec<String> = Vec::new();
    for gate in &gates {
        let value = exits_value.get(gate).ok_or_else(|| {
            CanaryfieldError::new(format!("the clean exits hold no key named \"{gate}\""))
        })?;
        clean_exits
            .push(as_int(value, &format!("the clean exit for gate \"{gate}\""))?.to_string());
    }
    let rows_value = key_of(root, "rows", "the result")?;
    let row_items = match rows_value {
        JsonValue::Array(items) => items,
        other => return Err(wrong_kind("the rows value", other, "a list")),
    };
    if row_items.is_empty() {
        return Err(CanaryfieldError::new(
            "the rows list stands empty, and a table width holds no row to read from",
        ));
    }
    let mut rows: Vec<Row> = Vec::new();
    for (index, item) in row_items.iter().enumerate() {
        let number = index + 1;
        let row = as_object(item, &format!("row {number}"))?;
        let seed = as_text(
            key_of(row, "seed", &format!("row {number}"))?,
            &format!("row {number}'s seed"),
        )?
        .to_string();
        let name = as_text(
            key_of(row, "name", &format!("row {number}"))?,
            &format!("row {number}'s name"),
        )?
        .to_string();
        let planted_value = key_of(row, "planted_file", &format!("row {number}"))?;
        let planted = match planted_value {
            JsonValue::Null => "none".to_string(),
            JsonValue::Str(text) if text.is_empty() => "none".to_string(),
            JsonValue::Str(text) => text.clone(),
            other => {
                return Err(wrong_kind(
                    &format!("row {number}'s planted file"),
                    other,
                    "text or none",
                ))
            }
        };
        let cells_value = as_object(
            key_of(row, "gates", &format!("row {number}"))?,
            &format!("row {number}'s gates value"),
        )?;
        let mut cells = vec![seed, name, planted];
        for gate in &gates {
            let value = cells_value.get(gate).ok_or_else(|| {
                CanaryfieldError::new(format!("row {number}'s gates hold no key named \"{gate}\""))
            })?;
            cells.push(
                as_text(value, &format!("row {number}'s cell for gate \"{gate}\""))?.to_string(),
            );
        }
        let planted_files = optional_list(
            row.get("planted_files"),
            &format!("row {number}'s planted files value"),
            &format!("an entry of row {number}'s planted files"),
        )?;
        rows.push(Row {
            cells,
            planted_files,
        });
    }
    let unparseable_files = optional_list(
        root.get("unparseable_files"),
        "the unparseable files value",
        "an entry of the unparseable files",
    )?;
    let summary_value = as_object(key_of(root, "summary", "the result")?, "the summary")?;
    let mut summaries: Vec<Vec<String>> = Vec::new();
    for gate in &gates {
        let entry = summary_value
            .get(gate)
            .ok_or_else(|| missing("the summary", gate))?;
        let entry = as_object(entry, &format!("the summary for gate \"{gate}\""))?;
        let mut counts: Vec<String> = Vec::new();
        for key in COUNT_KEYS {
            let value = entry
                .get(key)
                .ok_or_else(|| missing(&format!("the summary for gate \"{gate}\""), key))?;
            counts
                .push(as_int(value, &format!("the {key} count for gate \"{gate}\""))?.to_string());
        }
        summaries.push(counts);
    }
    Ok(Shape {
        gates,
        target,
        target_unchanged,
        clean_exits,
        rows,
        unparseable_files,
        summaries,
    })
}

fn render_line(cells: &[String], widths: &[usize]) -> String {
    let padded: Vec<String> = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            let width = widths[index];
            let count = cell.chars().count();
            if count >= width {
                cell.clone()
            } else {
                let mut text = cell.clone();
                for _ in 0..(width - count) {
                    text.push(' ');
                }
                text
            }
        })
        .collect();
    padded.join("  ").trim_end().to_string()
}

/// Render the result as the catch table in text, byte for
/// byte as canaryfield/report.py renders it, refusing every
/// shape outside the engine's result shape as PORT4-6 states.
pub fn render_text(result: &JsonValue) -> Result<String, CanaryfieldError> {
    let shape = read_shape(result)?;
    let mut lines: Vec<String> = Vec::new();
    lines.push("canaryfield catch table".to_string());
    lines.push("=".repeat(23));
    lines.push(String::new());
    lines.push(format!("target           {}", shape.target));
    lines.push(format!(
        "target unchanged {}",
        if shape.target_unchanged {
            "true"
        } else {
            "false"
        }
    ));
    for (gate, exit) in shape.gates.iter().zip(&shape.clean_exits) {
        lines.push(format!("clean exit for gate {gate} is {exit}"));
    }
    lines.push(String::new());
    let mut header = vec![
        "seed".to_string(),
        "name".to_string(),
        "planted file".to_string(),
    ];
    header.extend(shape.gates.iter().cloned());
    let mut widths: Vec<usize> = header.iter().map(|cell| cell.chars().count()).collect();
    for row in &shape.rows {
        for (index, cell) in row.cells.iter().enumerate() {
            widths[index] = widths[index].max(cell.chars().count());
        }
    }
    lines.push(render_line(&header, &widths));
    let dashes: Vec<String> = widths.iter().map(|width| "-".repeat(*width)).collect();
    lines.push(render_line(&dashes, &widths));
    for row in &shape.rows {
        lines.push(render_line(&row.cells, &widths));
        if row.planted_files.len() > 1 {
            lines.push(format!("  planted in {}", row.planted_files.join(", ")));
        }
    }
    if !shape.unparseable_files.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "files skipped as unparseable {}",
            shape.unparseable_files.join(", ")
        ));
    }
    lines.push(String::new());
    lines.push("summary per gate".to_string());
    for (gate, counts) in shape.gates.iter().zip(&shape.summaries) {
        lines.push(format!(
            "  {gate}  caught {}  waved-through {}  not-applicable {}  gate-broken {}",
            counts[0], counts[1], counts[2], counts[3]
        ));
    }
    let mut text = lines.join("\n");
    text.push('\n');
    Ok(text)
}

fn sorted_value(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Array(items) => JsonValue::Array(items.iter().map(sorted_value).collect()),
        JsonValue::Object(pairs) => {
            let mut sorted: Vec<(String, JsonValue)> = pairs
                .iter()
                .map(|(key, item)| (key.clone(), sorted_value(item)))
                .collect();
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            JsonValue::Object(sorted)
        }
        other => other.clone(),
    }
}

/// Render the result as the JSON document, byte for byte as
/// canaryfield/report.py renders it, every key the result
/// holds carried and sorted at every level, with one
/// trailing newline.
pub fn render_json(result: &JsonValue) -> String {
    let mut text = dumps_indent2(&sorted_value(result));
    text.push('\n');
    text
}

/// Render the result document in the form the caller names,
/// the word text or the word json, as the report door drives
/// these two renderings.
pub fn render_report(result: &JsonValue, form: &str) -> Result<String, CanaryfieldError> {
    match form {
        "text" => render_text(result),
        "json" => Ok(render_json(result)),
        other => Err(CanaryfieldError::new(format!(
            "the report form stands as \"{other}\" where the door asks the word text or the word json"
        ))),
    }
}
