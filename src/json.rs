//! The JSON reading and writing the seed pack file needs.
//!
//! The Python pack reading stands on the standard library JSON
//! decoder and encoder, and this module carries the same ground
//! with no dependency outside the standard library. A document is
//! read into a [`JsonValue`], and an object keeps its keys in
//! document order with a key written twice resolving to its last
//! value, the reading a Python dict gives the document. The
//! rendering matches the Python encoder at a two space indent
//! with non ASCII text escaped, which is the shape the Python
//! pack writer writes, and [`py_repr`] renders a value the way
//! the Python repr renders it, which is the shape the loader
//! refusal for an entry missing fields carries.

use crate::errors::CanaryfieldError;
use crate::pyunparse;

/// A parsed JSON document value.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// The null value.
    Null,
    /// A true or false value.
    Bool(bool),
    /// A whole number, carried as its Python int repr text.
    Int(String),
    /// A number with a fraction or an exponent, or one of the
    /// three non finite spellings the Python decoder accepts.
    Float(f64),
    /// A text value.
    Str(String),
    /// A list of values in document order.
    Array(Vec<JsonValue>),
    /// A mapping of keys to values in first written order, with
    /// a key written twice holding its last value.
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// Look a key up in an object value, answering the last value
    /// the document wrote for it and nothing for any other shape.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(pairs) => pairs
                .iter()
                .rev()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value),
            _ => None,
        }
    }

    /// Answer the text of a text value and nothing for any other
    /// shape.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::Str(text) => Some(text),
            _ => None,
        }
    }
}

/// Parse a JSON document the way the Python decoder parses one.
///
/// The whitespace of the format is the only padding, a key
/// written twice inside one object resolves to its last value,
/// and the non finite spellings the Python decoder accepts are
/// accepted. A text that does not parse is refused with an error
/// whose wording belongs to this decoder, as the contract port
/// section states, and a document nested deeper than four
/// thousand levels is refused at that bound rather than followed
/// down, the Python decoder meeting such a document with its
/// recursion error instead.
pub fn parse(text: &str) -> Result<JsonValue, CanaryfieldError> {
    let mut parser = Parser {
        chars: text.chars().collect(),
        pos: 0,
    };
    parser.skip_whitespace();
    let value = parser.parse_value(0)?;
    parser.skip_whitespace();
    if parser.pos != parser.chars.len() {
        return Err(parser.fail("extra data after the document"));
    }
    Ok(value)
}

/// The deepest nesting of lists and objects inside one document
/// this decoder follows. The Python decoder meets a deeper
/// document with its recursion error, a crash this decoder does
/// not reproduce, so a deeper document is refused at the bound
/// with the wording of this decoder instead, as the contract
/// port section states.
const MAX_NESTING_DEPTH: usize = 4000;

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn fail(&self, detail: &str) -> CanaryfieldError {
        CanaryfieldError::new(format!(
            "invalid JSON document at character {} ({detail})",
            self.pos
        ))
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.pos += 1;
        }
    }

    fn check_depth(&self, depth: usize) -> Result<(), CanaryfieldError> {
        if depth >= MAX_NESTING_DEPTH {
            Err(self.fail("document nested deeper than four thousand levels"))
        } else {
            Ok(())
        }
    }

    fn parse_value(&mut self, depth: usize) -> Result<JsonValue, CanaryfieldError> {
        match self.peek() {
            Some('{') => {
                self.check_depth(depth)?;
                self.parse_object(depth)
            }
            Some('[') => {
                self.check_depth(depth)?;
                self.parse_array(depth)
            }
            Some('"') => Ok(JsonValue::Str(self.parse_string()?)),
            Some('t') => self.parse_literal("true", JsonValue::Bool(true)),
            Some('f') => self.parse_literal("false", JsonValue::Bool(false)),
            Some('n') => self.parse_literal("null", JsonValue::Null),
            Some('N') => self.parse_literal("NaN", JsonValue::Float(f64::NAN)),
            Some('I') => self.parse_literal("Infinity", JsonValue::Float(f64::INFINITY)),
            Some('-') => {
                if self.chars[self.pos + 1..].starts_with(&['I', 'n', 'f']) {
                    self.parse_literal("-Infinity", JsonValue::Float(f64::NEG_INFINITY))
                } else {
                    self.parse_number()
                }
            }
            Some(ch) if ch.is_ascii_digit() => self.parse_number(),
            _ => Err(self.fail("expected a value")),
        }
    }

    fn parse_literal(
        &mut self,
        literal: &str,
        value: JsonValue,
    ) -> Result<JsonValue, CanaryfieldError> {
        if self.chars[self.pos..].starts_with(&literal.chars().collect::<Vec<_>>()[..]) {
            self.pos += literal.len();
            Ok(value)
        } else {
            Err(self.fail("expected a value"))
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, CanaryfieldError> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        match self.peek() {
            Some('0') => {
                self.pos += 1;
            }
            Some(ch) if ch.is_ascii_digit() => {
                while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                    self.pos += 1;
                }
            }
            _ => return Err(self.fail("expected a number")),
        }
        let mut is_float = false;
        if self.peek() == Some('.') {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                return Err(self.fail("expected a digit after the point"));
            }
            while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some('+' | '-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                return Err(self.fail("expected a digit in the exponent"));
            }
            while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        let raw: String = self.chars[start..self.pos].iter().collect();
        if is_float {
            raw.parse::<f64>()
                .map(JsonValue::Float)
                .map_err(|_| self.fail("number out of range"))
        } else if raw == "-0" {
            Ok(JsonValue::Int("0".to_string()))
        } else {
            Ok(JsonValue::Int(raw))
        }
    }

    fn parse_string(&mut self) -> Result<String, CanaryfieldError> {
        self.pos += 1;
        let mut out = String::new();
        loop {
            let ch = match self.peek() {
                Some(ch) => ch,
                None => return Err(self.fail("unterminated string")),
            };
            self.pos += 1;
            match ch {
                '"' => return Ok(out),
                '\\' => {
                    let esc = self
                        .peek()
                        .ok_or_else(|| self.fail("unterminated escape"))?;
                    self.pos += 1;
                    match esc {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{8}'),
                        'f' => out.push('\u{c}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let first = self.parse_hex4()?;
                            if (0xD800..0xDC00).contains(&first) {
                                if self.peek() == Some('\\')
                                    && self.chars.get(self.pos + 1) == Some(&'u')
                                {
                                    self.pos += 2;
                                    let second = self.parse_hex4()?;
                                    if (0xDC00..0xE000).contains(&second) {
                                        let combined =
                                            0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00);
                                        out.push(
                                            char::from_u32(combined)
                                                .ok_or_else(|| self.fail("bad surrogate pair"))?,
                                        );
                                    } else {
                                        return Err(self.fail("unpaired surrogate in string"));
                                    }
                                } else {
                                    return Err(self.fail("unpaired surrogate in string"));
                                }
                            } else if (0xDC00..0xE000).contains(&first) {
                                return Err(self.fail("unpaired surrogate in string"));
                            } else {
                                out.push(
                                    char::from_u32(first)
                                        .ok_or_else(|| self.fail("bad unicode escape"))?,
                                );
                            }
                        }
                        _ => return Err(self.fail("unknown escape")),
                    }
                }
                ch if (ch as u32) < 0x20 => {
                    return Err(self.fail("control character in string"));
                }
                ch => out.push(ch),
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u32, CanaryfieldError> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let ch = self
                .peek()
                .ok_or_else(|| self.fail("short unicode escape"))?;
            let digit = ch
                .to_digit(16)
                .ok_or_else(|| self.fail("bad unicode escape"))?;
            value = value * 16 + digit;
            self.pos += 1;
        }
        Ok(value)
    }

    fn parse_array(&mut self, depth: usize) -> Result<JsonValue, CanaryfieldError> {
        self.pos += 1;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(JsonValue::Array(items));
        }
        loop {
            self.skip_whitespace();
            items.push(self.parse_value(depth + 1)?);
            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.pos += 1;
                }
                Some(']') => {
                    self.pos += 1;
                    return Ok(JsonValue::Array(items));
                }
                _ => return Err(self.fail("expected a comma or the end of the list")),
            }
        }
    }

    fn parse_object(&mut self, depth: usize) -> Result<JsonValue, CanaryfieldError> {
        self.pos += 1;
        let mut pairs: Vec<(String, JsonValue)> = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(JsonValue::Object(pairs));
        }
        loop {
            self.skip_whitespace();
            if self.peek() != Some('"') {
                return Err(self.fail("expected an object key"));
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.peek() != Some(':') {
                return Err(self.fail("expected a colon after the object key"));
            }
            self.pos += 1;
            self.skip_whitespace();
            let value = self.parse_value(depth + 1)?;
            match pairs.iter_mut().find(|(name, _)| *name == key) {
                Some(slot) => slot.1 = value,
                None => pairs.push((key, value)),
            }
            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.pos += 1;
                }
                Some('}') => {
                    self.pos += 1;
                    return Ok(JsonValue::Object(pairs));
                }
                _ => return Err(self.fail("expected a comma or the end of the object")),
            }
        }
    }
}

/// Render a value as JSON with a two space indent, the shape
/// the Python encoder writes for the pack document, with non
/// ASCII text escaped the way that encoder escapes it.
pub fn dumps_indent2(value: &JsonValue) -> String {
    let mut out = String::new();
    write_value(value, 0, &mut out);
    out
}

fn write_indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

fn write_value(value: &JsonValue, level: usize, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(true) => out.push_str("true"),
        JsonValue::Bool(false) => out.push_str("false"),
        JsonValue::Int(raw) => out.push_str(raw),
        JsonValue::Float(number) => out.push_str(&float_text(*number, true)),
        JsonValue::Str(text) => write_string(text, out),
        JsonValue::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                write_indent(out, level + 1);
                write_value(item, level + 1, out);
                if index + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            write_indent(out, level);
            out.push(']');
        }
        JsonValue::Object(pairs) => {
            if pairs.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            for (index, (key, item)) in pairs.iter().enumerate() {
                write_indent(out, level + 1);
                write_string(key, out);
                out.push_str(": ");
                write_value(item, level + 1, out);
                if index + 1 < pairs.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            write_indent(out, level);
            out.push('}');
        }
    }
}

fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ch if (ch as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch if (ch as u32) < 0x7F => out.push(ch),
            ch => {
                let cp = ch as u32;
                if cp < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", cp));
                } else {
                    let rest = cp - 0x10000;
                    out.push_str(&format!(
                        "\\u{:04x}\\u{:04x}",
                        0xD800 + (rest >> 10),
                        0xDC00 + (rest & 0x3FF)
                    ));
                }
            }
        }
    }
    out.push('"');
}

/// The text of a number with a fraction, spelled the way the
/// Python float repr spells it. The encoder spelling of the
/// non finite values differs from the repr spelling, and the
/// caller picks which one it is rendering.
fn float_text(number: f64, for_encoder: bool) -> String {
    if number.is_nan() {
        return if for_encoder { "NaN" } else { "nan" }.to_string();
    }
    if number.is_infinite() {
        let text = if for_encoder { "Infinity" } else { "inf" };
        return if number < 0.0 {
            format!("-{text}")
        } else {
            text.to_string()
        };
    }
    if number == 0.0 {
        return if number.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    let sign = if number < 0.0 { "-" } else { "" };
    let sci = format!("{:e}", number.abs());
    let (mantissa, exponent_text) = sci.split_once('e').expect("scientific form");
    let exponent: i32 = exponent_text.parse().expect("exponent digits");
    let digits: String = mantissa.chars().filter(|ch| *ch != '.').collect();
    let count = digits.len() as i32;
    let mut body = String::new();
    if (-4..16).contains(&exponent) {
        if exponent >= count - 1 {
            body.push_str(&digits);
            for _ in 0..(exponent - (count - 1)) {
                body.push('0');
            }
            body.push_str(".0");
        } else if exponent >= 0 {
            let split = (exponent + 1) as usize;
            body.push_str(&digits[..split]);
            body.push('.');
            body.push_str(&digits[split..]);
        } else {
            body.push_str("0.");
            for _ in 0..(-exponent - 1) {
                body.push('0');
            }
            body.push_str(&digits);
        }
    } else {
        body.push_str(&digits[..1]);
        if count > 1 {
            body.push('.');
            body.push_str(&digits[1..]);
        }
        body.push('e');
        body.push(if exponent < 0 { '-' } else { '+' });
        let magnitude = exponent.abs();
        if magnitude < 10 {
            body.push('0');
        }
        body.push_str(&magnitude.to_string());
    }
    format!("{sign}{body}")
}

/// Render a value the way the Python repr renders it.
///
/// This is the shape the loader refusal for an entry missing
/// fields carries, and the refusal for a field holding no text
/// carries it for the whole entry. Text keeps every printable
/// character standing and escapes the rest, and the quote mark
/// is the double quote exactly when the text holds a single
/// quote and no double quote.
pub fn py_repr(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "None".to_string(),
        JsonValue::Bool(true) => "True".to_string(),
        JsonValue::Bool(false) => "False".to_string(),
        JsonValue::Int(raw) => raw.clone(),
        JsonValue::Float(number) => float_text(*number, false),
        JsonValue::Str(text) => pyunparse::str_repr(&codepoints(text)),
        JsonValue::Array(items) => {
            let parts: Vec<String> = items.iter().map(py_repr).collect();
            format!("[{}]", parts.join(", "))
        }
        JsonValue::Object(pairs) => {
            let parts: Vec<String> = pairs
                .iter()
                .map(|(key, item)| {
                    format!(
                        "{}: {}",
                        pyunparse::str_repr(&codepoints(key)),
                        py_repr(item)
                    )
                })
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
    }
}

/// The codepoints of a text, the input form the tree
/// surface's string spelling reads.
fn codepoints(text: &str) -> Vec<u32> {
    text.chars().map(|ch| ch as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumps_renders_nested_shapes_like_the_python_encoder() {
        let value = parse(r#"{"a": [1, 2.5, "x"], "b": {}, "c": []}"#).expect("parses");
        let expected =
            "{\n  \"a\": [\n    1,\n    2.5,\n    \"x\"\n  ],\n  \"b\": {},\n  \"c\": []\n}";
        assert_eq!(dumps_indent2(&value), expected);
    }

    #[test]
    fn a_duplicate_key_resolves_to_its_last_value() {
        let value = parse(r#"{"id": "seed-99", "id": "seed-00"}"#).expect("parses");
        assert_eq!(
            value.get("id"),
            Some(&JsonValue::Str("seed-00".to_string()))
        );
        assert_eq!(py_repr(&value), "{'id': 'seed-00'}");
    }

    #[test]
    fn repr_picks_the_quote_the_python_picks() {
        assert_eq!(py_repr(&JsonValue::Str("plain".into())), "'plain'");
        assert_eq!(py_repr(&JsonValue::Str("it's".into())), "\"it's\"");
        assert_eq!(
            py_repr(&JsonValue::Str("both ' and \"".into())),
            "'both \\' and \"'"
        );
    }

    #[test]
    fn repr_spells_scalars_the_python_way() {
        assert_eq!(py_repr(&JsonValue::Null), "None");
        assert_eq!(py_repr(&JsonValue::Bool(true)), "True");
        assert_eq!(py_repr(&JsonValue::Float(7.0)), "7.0");
        assert_eq!(py_repr(&JsonValue::Float(7.5)), "7.5");
        assert_eq!(py_repr(&JsonValue::Float(0.0001)), "0.0001");
        assert_eq!(py_repr(&JsonValue::Float(0.00001)), "1e-05");
        assert_eq!(py_repr(&JsonValue::Float(1e16)), "1e+16");
        assert_eq!(py_repr(&JsonValue::Float(1e15)), "1000000000000000.0");
        assert_eq!(py_repr(&JsonValue::Int("7".into())), "7");
    }

    #[test]
    fn parse_refuses_text_that_is_not_a_document() {
        assert!(parse("{not json").is_err());
        assert!(parse("[1, 2] tail").is_err());
        assert!(parse("").is_err());
    }
}
