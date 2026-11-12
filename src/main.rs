//! The canaryfield binary is a thin shell over the crate.
//!
//! Handed the write seed pack flag and a path, it writes the
//! current seed pack to that path and prints the line the
//! Python command line prints. Handed the plant seed flag, a
//! root, and a seed id, it plants that seed over the root
//! through the crate planting and prints the planted and
//! skipped names as one JSON document, the door the Python
//! engine drives under the bridge. Every other part of the
//! command line, the field run and its gates and its
//! reports, stays with the Python in this slice, as the
//! contract port section states, and any other argument
//! shape is answered with a usage line and a nonzero exit.

use std::path::Path;
use std::process::ExitCode;

use canaryfield::json::{dumps_indent2, JsonValue};
use canaryfield::pyplant::{plant_seed, PlantError};

fn names(items: &[String]) -> JsonValue {
    JsonValue::Array(
        items
            .iter()
            .map(|item| JsonValue::Str(item.clone()))
            .collect(),
    )
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--write-seed-pack" {
        match canaryfield::write_seed_pack(Path::new(&args[2])) {
            Ok(_) => {
                println!(
                    "wrote {} seed pack to {}",
                    canaryfield::SEED_PACK_FORMAT,
                    args[2]
                );
                ExitCode::SUCCESS
            }
            Err(exc) => {
                eprintln!("{exc}");
                ExitCode::from(2)
            }
        }
    } else if args.len() == 4 && args[1] == "--plant-seed" {
        let mut skipped: Vec<String> = Vec::new();
        match plant_seed(Path::new(&args[2]), &args[3], Some(&mut skipped)) {
            Ok(planted) => {
                let answer = JsonValue::Object(vec![
                    ("planted".to_string(), names(&planted)),
                    ("skipped".to_string(), names(&skipped)),
                ]);
                println!("{}", dumps_indent2(&answer));
                ExitCode::SUCCESS
            }
            Err(PlantError::Refusal(refusal)) => {
                eprintln!("{}", refusal.message);
                ExitCode::from(2)
            }
            Err(PlantError::Io(error)) => {
                eprintln!("{error}");
                ExitCode::from(2)
            }
        }
    } else {
        eprintln!("usage: canaryfield --write-seed-pack PATH | --plant-seed ROOT SEED");
        ExitCode::from(2)
    }
}
