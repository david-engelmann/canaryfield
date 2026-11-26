//! The command line shell of PORT6.
//!
//! The binary handed arguments that name no door of the
//! earlier slices performs the work the Python command line
//! performed, composing the run and the renderings in
//! process. The scan reproduces the CPython 3.12 argparse
//! model for this parser under PORT6-1 and PORT6-2, the
//! value readings are the Python's own, and the run
//! composition follows PORT6-3 and PORT6-4. Arguments are
//! read as bytes end to end, and a value the scan must
//! carry as text that is not text is refused in the
//! shell's own wording, as PORT6-1 states.

use std::ffi::OsString;
use std::io::Write;
use std::path::Path;

use crate::engine::{request_from, run};
use crate::json::JsonValue;
use crate::pyunparse::str_repr;
use crate::report::render_report;
use crate::{write_seed_pack, SEED_PACK_FORMAT};

pub const USAGE: &str = "usage: canaryfield [-h] [--target TARGET] [--gate NAME=COMMAND]\n                   [--plant {copy,git-branches}] [--write-seed-pack PATH]\n                   [--out OUT] [--json] [--timeout TIMEOUT]\n                   [--work-root WORK_ROOT] [--keep-scratch]\n";

pub const HELP: &str = "usage: canaryfield [-h] [--target TARGET] [--gate NAME=COMMAND]\n                   [--plant {copy,git-branches}] [--write-seed-pack PATH]\n                   [--out OUT] [--json] [--timeout TIMEOUT]\n                   [--work-root WORK_ROOT] [--keep-scratch]\n\nPlant seeded defects into scratch copies of a target tree and score which\ngates catch each one.\n\noptions:\n  -h, --help            show this help message and exit\n  --target TARGET       target tree to copy and plant into\n  --gate NAME=COMMAND   a named gate command, repeatable\n  --plant {copy,git-branches}\n                        planting mode, copy for scratch copies or git-branches\n                        for a scratch clone\n  --write-seed-pack PATH\n                        write the canaryfield-seeds/42 seed pack to PATH and\n                        exit\n  --out OUT             directory to write catch-table.txt and catch-\n                        table.json into\n  --json                print the JSON report to stdout\n  --timeout TIMEOUT     per gate run timeout in seconds\n  --work-root WORK_ROOT\n                        scratch directory root, defaults to a temp directory\n  --keep-scratch        keep the scratch copies after the run\n";

/// One valued flag of the parser, in declaration order.
struct Valued {
    name: &'static str,
    field: Field,
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Target,
    Gate,
    Plant,
    WriteSeedPack,
    Out,
    Timeout,
    WorkRoot,
}

const VALUED: &[Valued] = &[
    Valued {
        name: "--target",
        field: Field::Target,
    },
    Valued {
        name: "--gate",
        field: Field::Gate,
    },
    Valued {
        name: "--plant",
        field: Field::Plant,
    },
    Valued {
        name: "--write-seed-pack",
        field: Field::WriteSeedPack,
    },
    Valued {
        name: "--out",
        field: Field::Out,
    },
    Valued {
        name: "--timeout",
        field: Field::Timeout,
    },
    Valued {
        name: "--work-root",
        field: Field::WorkRoot,
    },
];

const STORE_TRUE: &[(&str, Field2)] = &[
    ("--json", Field2::Json),
    ("--keep-scratch", Field2::KeepScratch),
];

#[derive(Clone, Copy, PartialEq)]
enum Field2 {
    Json,
    KeepScratch,
}

/// The scan's outcome, or the exit the scan already owes.
enum Scan {
    Help,
    Refusal(String),
    Parsed(Box<Args>),
}

#[derive(Default)]
struct Args {
    target: Option<String>,
    gates: Vec<(String, String)>,
    plant: String,
    write_seed_pack: Option<String>,
    out: Option<String>,
    json: bool,
    timeout: String,
    work_root: Option<String>,
    keep_scratch: bool,
}

fn repr_of(text: &str) -> String {
    let points: Vec<u32> = text.chars().map(|ch| ch as u32).collect();
    str_repr(&points)
}

fn refusal(message: String) -> Scan {
    Scan::Refusal(message)
}

fn error_exit(message: &str) -> i32 {
    let mut err = std::io::stderr().lock();
    let _ = err.write_all(USAGE.as_bytes());
    let _ = err.write_all(format!("canaryfield: error: {message}\n").as_bytes());
    2
}

/// Read a value's bytes as text, refusing in the shell's
/// own wording where the bytes are not text.
fn as_text(bytes: &[u8]) -> Result<String, Scan> {
    match std::str::from_utf8(bytes) {
        Ok(text) => Ok(text.to_string()),
        Err(_) => Err(refusal(
            "an argument carries bytes that are not text".to_string(),
        )),
    }
}

/// The Python int reading of PORT6-2: a leading sign
/// standing, surrounding whitespace stripped, underscores
/// between digits standing, and Unicode decimal digits
/// standing as Python's own reading takes them. The value
/// is carried as its normalized decimal text, the spelling
/// the bridge's document carries, so a value beyond this
/// machine's integer stands as Python accepts it.
fn py_int_text(text: &str) -> Option<String> {
    let trimmed = text.trim_matches(|ch: char| ch.is_whitespace());
    if trimmed.is_empty() {
        return None;
    }
    let (negative, digits) = match trimmed.as_bytes()[0] {
        b'+' => (false, &trimmed[1..]),
        b'-' => (true, &trimmed[1..]),
        _ => (false, trimmed),
    };
    if digits.is_empty() {
        return None;
    }
    let mut values: Vec<u32> = Vec::new();
    let mut previous_underscore = false;
    let mut previous_digit = false;
    for ch in digits.chars() {
        if ch == '_' {
            if !previous_digit || previous_underscore {
                return None;
            }
            previous_underscore = true;
            previous_digit = false;
            continue;
        }
        let value = decimal_digit(ch)?;
        previous_underscore = false;
        previous_digit = true;
        values.push(value);
    }
    if previous_underscore || values.is_empty() {
        return None;
    }
    let first = values.iter().position(|value| *value != 0);
    let Some(first) = first else {
        return Some("0".to_string());
    };
    let body: String = values[first..]
        .iter()
        .map(|value| char::from_digit(*value, 10).expect("a decimal digit"))
        .collect();
    if negative {
        Some(format!("-{body}"))
    } else {
        Some(body)
    }
}

/// The decimal digit blocks of the Unicode data
/// CPython 3.12 carries, each block ten codepoints wide
/// except the mathematical block at 0x1d7ce, which carries
/// five styles of ten. A codepoint in a block reads as
/// its offset in the block, by tens where the block runs
/// long, which is the value Python's int reading gives it.
const DECIMAL_DIGIT_BLOCKS: &[u32] = &[
    0x30, 0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6, 0xd66,
    0xde6, 0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90,
    0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xa9f0, 0xaa50, 0xabf0, 0xff10,
    0x104a0, 0x10d30, 0x11066, 0x110f0, 0x11136, 0x111d0, 0x112f0, 0x11450, 0x114d0, 0x11650,
    0x116c0, 0x11730, 0x118e0, 0x11950, 0x11c50, 0x11d50, 0x11da0, 0x11f50, 0x16a60, 0x16ac0,
    0x16b50, 0x1d7ce, 0x1e140, 0x1e2f0, 0x1e4f0, 0x1e950, 0x1fbf0,
];

fn decimal_digit(ch: char) -> Option<u32> {
    if ch.is_ascii_digit() {
        return ch.to_digit(10);
    }
    let point = ch as u32;
    let block = DECIMAL_DIGIT_BLOCKS
        .iter()
        .rev()
        .find(|start| point >= **start && point - **start < 50)?;
    let offset = point - block;
    if offset < 10 || *block == 0x1d7ce {
        Some(offset % 10)
    } else {
        None
    }
}

/// The gate type's partition of PORT6-2.
fn parse_gate(text: &str) -> Result<(String, String), Scan> {
    let sentence = || {
        refusal(format!(
            "argument --gate: gate must be written as NAME=COMMAND, got {}",
            repr_of(text)
        ))
    };
    let Some(at) = text.find('=') else {
        return Err(sentence());
    };
    let name = text[..at].trim();
    let command = text[at + 1..].trim();
    if name.is_empty() || command.is_empty() {
        return Err(sentence());
    }
    Ok((name.to_string(), command.to_string()))
}

fn scan(tokens: &[Vec<u8>]) -> Scan {
    let mut args = Args {
        plant: "copy".to_string(),
        timeout: "120".to_string(),
        ..Args::default()
    };
    let mut extras: Vec<Vec<u8>> = Vec::new();
    let mut separator_seen = false;
    let mut index = 0usize;
    while index < tokens.len() {
        let token = &tokens[index];
        index += 1;
        if separator_seen {
            extras.push(token.clone());
            continue;
        }
        if token == b"--" {
            separator_seen = true;
            extras.push(token.clone());
            continue;
        }
        if token.len() > 2 && token.starts_with(b"--") && !token.contains(&b' ') {
            let text = match as_text(token) {
                Ok(text) => text,
                Err(scan) => return scan,
            };
            let (name_part, equals) = match text.find('=') {
                Some(at) => (text[..at].to_string(), Some(text[at + 1..].to_string())),
                None => (text.clone(), None),
            };
            // The candidate long names, in declaration order:
            // help first, then the valued flags, then the
            // store true flags, as the parser declares them.
            let mut candidates: Vec<&str> = Vec::new();
            let mut exact: Option<&str> = None;
            for name in std::iter::once("--help")
                .chain(VALUED.iter().map(|flag| flag.name))
                .chain(STORE_TRUE.iter().map(|(name, _)| *name))
            {
                if name == name_part {
                    exact = Some(name);
                    break;
                }
                if name.starts_with(&name_part) {
                    candidates.push(name);
                }
            }
            let chosen = if let Some(name) = exact {
                name
            } else if candidates.len() == 1 {
                candidates[0]
            } else if candidates.len() > 1 {
                return refusal(format!(
                    "ambiguous option: {} could match {}",
                    name_part,
                    candidates.join(", ")
                ));
            } else {
                extras.push(token.clone());
                continue;
            };
            if chosen == "--help" {
                if let Some(value) = &equals {
                    return refusal(format!(
                        "argument --help: ignored explicit argument {}",
                        repr_of(value)
                    ));
                }
                return Scan::Help;
            }
            if let Some((_, field2)) = STORE_TRUE.iter().find(|(name, _)| *name == chosen) {
                if let Some(value) = &equals {
                    return refusal(format!(
                        "argument {}: ignored explicit argument {}",
                        chosen,
                        repr_of(value)
                    ));
                }
                match field2 {
                    Field2::Json => args.json = true,
                    Field2::KeepScratch => args.keep_scratch = true,
                }
                continue;
            }
            let flag = VALUED
                .iter()
                .find(|flag| flag.name == chosen)
                .expect("a valued flag stands");
            let value = if let Some(value) = equals {
                value
            } else {
                // The value is the next token, and a token
                // that opens like a flag or the separator
                // leaves the flag missing its value.
                let Some(next) = tokens.get(index) else {
                    return refusal(format!("argument {}: expected one argument", chosen));
                };
                if next == b"--"
                    || (next.len() > 1
                        && next.starts_with(b"-")
                        && !next.contains(&b' ')
                        && !looks_like_negative(next))
                {
                    return refusal(format!("argument {}: expected one argument", chosen));
                }
                index += 1;
                match as_text(next) {
                    Ok(text) => text,
                    Err(scan) => return scan,
                }
            };
            match flag.field {
                Field::Target => args.target = Some(value),
                Field::Gate => match parse_gate(&value) {
                    Ok(pair) => args.gates.push(pair),
                    Err(scan) => return scan,
                },
                Field::Plant => {
                    if value != "copy" && value != "git-branches" {
                        return refusal(format!(
                            "argument --plant: invalid choice: {} (choose from 'copy', 'git-branches')",
                            repr_of(&value)
                        ));
                    }
                    args.plant = value;
                }
                Field::WriteSeedPack => args.write_seed_pack = Some(value),
                Field::Out => args.out = Some(value),
                Field::Timeout => match py_int_text(&value) {
                    Some(number) => args.timeout = number,
                    None => {
                        return refusal(format!(
                            "argument --timeout: invalid int value: {}",
                            repr_of(&value)
                        ));
                    }
                },
                Field::WorkRoot => args.work_root = Some(value),
            }
            continue;
        }
        if token.len() > 1 && token.starts_with(b"-") && token != b"-" && !token.contains(&b' ') {
            // A short cluster: help fires where its first
            // letter is h, and any other cluster is an
            // unknown token whole.
            if token[1] == b'h' {
                return Scan::Help;
            }
            extras.push(token.clone());
            continue;
        }
        extras.push(token.clone());
    }
    if !extras.is_empty() {
        let mut named: Vec<String> = Vec::new();
        for extra in &extras {
            match as_text(extra) {
                Ok(text) => named.push(text),
                Err(scan) => return scan,
            }
        }
        return refusal(format!("unrecognized arguments: {}", named.join(" ")));
    }
    Scan::Parsed(Box::new(args))
}

/// The parser's negative number matcher whole, a minus
/// and digits, or a minus and a decimal fraction, the two
/// shapes CPython's matcher carries.
fn looks_like_negative(token: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(token) else {
        return false;
    };
    let Some(body) = text.strip_prefix('-') else {
        return false;
    };
    if !body.is_empty() && body.bytes().all(|byte| byte.is_ascii_digit()) {
        return true;
    }
    let Some((head, tail)) = body.split_once('.') else {
        return false;
    };
    head.bytes().all(|byte| byte.is_ascii_digit())
        && !tail.is_empty()
        && tail.bytes().all(|byte| byte.is_ascii_digit())
}

/// Perform the command line's work and return its exit
/// code, under PORT6-1 to PORT6-4.
pub fn shell(arguments: &[OsString]) -> i32 {
    let tokens: Vec<Vec<u8>> = arguments
        .iter()
        .map(|arg| {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStrExt;
                arg.as_os_str().as_bytes().to_vec()
            }
            #[cfg(not(unix))]
            {
                arg.to_string_lossy().into_owned().into_bytes()
            }
        })
        .collect();
    let args = match scan(&tokens) {
        Scan::Help => {
            let mut out = std::io::stdout().lock();
            if out.write_all(HELP.as_bytes()).is_err() {
                return 1;
            }
            return 0;
        }
        Scan::Refusal(message) => return error_exit(&message),
        Scan::Parsed(args) => *args,
    };
    if let Some(path) = &args.write_seed_pack {
        return match write_seed_pack(Path::new(path)) {
            Ok(_) => {
                println!("wrote {} seed pack to {}", SEED_PACK_FORMAT, path);
                0
            }
            Err(exc) => {
                eprintln!("{exc}");
                2
            }
        };
    }
    if args.target.is_none() || args.gates.is_empty() {
        return error_exit(
            "--target and at least one --gate are required unless --write-seed-pack is used",
        );
    }
    let target = args.target.expect("the target stands");
    let gates: Vec<JsonValue> = args
        .gates
        .iter()
        .map(|(name, command)| {
            JsonValue::Array(vec![
                JsonValue::Str(name.clone()),
                JsonValue::Str(command.clone()),
            ])
        })
        .collect();
    let document = JsonValue::Object(vec![
        ("target".to_string(), JsonValue::Str(target)),
        ("gates".to_string(), JsonValue::Array(gates)),
        ("timeout".to_string(), JsonValue::Int(args.timeout.clone())),
        (
            "work_root".to_string(),
            match &args.work_root {
                Some(root) => JsonValue::Str(root.clone()),
                None => JsonValue::Null,
            },
        ),
        (
            "keep_scratch".to_string(),
            JsonValue::Bool(args.keep_scratch),
        ),
        ("plant_mode".to_string(), JsonValue::Str(args.plant.clone())),
    ]);
    let result = match request_from(&document).and_then(|request| run(&request)) {
        Ok(result) => result,
        Err(wording) => {
            eprintln!("error, {wording}");
            return 2;
        }
    };
    let unchanged = matches!(result.get("target_unchanged"), Some(JsonValue::Bool(true)));
    if !unchanged {
        eprintln!("error, the target tree changed during the run");
        return 3;
    }
    if let Some(out) = &args.out {
        let dir = Path::new(out);
        if let Err(error) = std::fs::create_dir_all(dir) {
            eprintln!("error, {error}");
            return 2;
        }
        for (form, file) in [("text", "catch-table.txt"), ("json", "catch-table.json")] {
            match render_report(&result, form) {
                Ok(rendering) => {
                    if let Err(error) = std::fs::write(dir.join(file), rendering) {
                        eprintln!("error, {error}");
                        return 2;
                    }
                }
                Err(exc) => {
                    eprintln!("error, {exc}");
                    return 2;
                }
            }
        }
    }
    let form = if args.json { "json" } else { "text" };
    match render_report(&result, form) {
        Ok(rendering) => {
            let mut out = std::io::stdout().lock();
            if out.write_all(rendering.as_bytes()).is_err() {
                return 1;
            }
            0
        }
        Err(exc) => {
            eprintln!("error, {exc}");
            2
        }
    }
}
