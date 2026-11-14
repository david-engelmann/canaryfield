//! The canaryfield binary is a thin shell over the crate.
//!
//! Handed the write seed pack flag and a path, it writes the
//! current seed pack to that path and prints the line the
//! Python command line prints. Handed the plant seed flag, a
//! root, and a seed id, it plants that seed over the root
//! through the crate planting and prints the planted and
//! skipped names as one JSON document, the door the Python
//! engine drives under the bridge. Handed the render report
//! flag, the path of a result document written as JSON, and
//! the name of a form, the word text or the word json, it
//! prints that rendering's bytes as they stand, the door the
//! Python command line drives under the report bridge, and it
//! answers a refusal with the refusal's words on the error
//! stream and a nonzero exit. Handed the run field flag and
//! the path of a request document written as JSON, it
//! performs the run of PORT5 and prints the result document,
//! the door the Python engine drives under its bridge,
//! answering a refusal the same way. Any other argument shape
//! is answered with a usage line and a nonzero exit.

use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use canaryfield::json::{dumps_indent2, JsonValue};
use canaryfield::pyplant::{plant_seed, PlantError};
use canaryfield::report::render_report;

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
    } else if args.len() == 4 && args[1] == "--render-report" {
        let staged = std::fs::read_to_string(&args[2])
            .map_err(|error| canaryfield::CanaryfieldError::new(error.to_string()))
            .and_then(|text| canaryfield::json::parse(&text))
            .and_then(|document| render_report(&document, &args[3]));
        match staged {
            Ok(rendering) => {
                let mut out = std::io::stdout().lock();
                match out.write_all(rendering.as_bytes()) {
                    Ok(_) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("{error}");
                        ExitCode::from(2)
                    }
                }
            }
            Err(exc) => {
                eprintln!("{exc}");
                ExitCode::from(2)
            }
        }
    } else if args.len() == 3 && args[1] == "--run-field" {
        let staged = std::fs::read_to_string(&args[2])
            .map_err(|error| error.to_string())
            .and_then(|text| canaryfield::json::parse(&text).map_err(|error| error.to_string()))
            .and_then(|document| canaryfield::engine::request_from(&document))
            .and_then(|request| canaryfield::engine::run(&request));
        match staged {
            Ok(result) => {
                println!("{}", dumps_indent2(&result));
                ExitCode::SUCCESS
            }
            Err(wording) => {
                eprintln!("{wording}");
                ExitCode::from(2)
            }
        }
    } else {
        eprintln!(
            "usage: canaryfield --write-seed-pack PATH | --plant-seed ROOT SEED | --render-report PATH FORM | --run-field PATH"
        );
        ExitCode::from(2)
    }
}
