//! The canaryfield binary is a thin shell over the pack
//! writing entry.
//!
//! Handed the write seed pack flag and a path, it writes the
//! current seed pack to that path and prints the line the
//! Python command line prints. Every other part of the command
//! line, the field run and its gates and its reports, stays
//! with the Python in this slice, as the contract port section
//! states, and any other argument shape is answered with a
//! usage line and a nonzero exit.

use std::path::Path;
use std::process::ExitCode;

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
    } else {
        eprintln!("usage: canaryfield --write-seed-pack PATH");
        ExitCode::from(2)
    }
}
