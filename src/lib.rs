//! canaryfield asks whether the harness catches planted defects.
//!
//! This crate is the Rust port of the canaryfield Python
//! package, and the landed Python is the behavioral spec it is
//! judged against. Port slice 1 covers the seed registry as data
//! and the versioned seed pack file, the pack document and its
//! writer and its loader with every refusal the Python loader
//! promises. The registry module holds the forty eight seeds as
//! a data table, the JSON module carries the document reading
//! and writing the pack file needs, and the seed pack module
//! holds the pack surface over both.
//!
//! Everything the port section of the contract does not name
//! stays with the Python in this slice, including the engine
//! machinery beyond the planting, the report module, and the
//! command line beyond the pack writing path the binary
//! carries.
//!
//! Port slice 2 adds the tree layer and the seed surface of
//! PORT2. The pyast, pytoken, pyparse, pyunparse, pywalk, and
//! pydata modules carry the CPython 3.12 tree the readings run
//! over, the shapes first built and proven for the ostensible
//! port and adapted here under this repo's own contract. The
//! pyarena module lowers that tree into the mutable arena the
//! applies plant into, pydump spells the tree the way
//! ast.dump spells it for the change test, pyapply carries
//! the forty eight seed applies, and pyplant carries
//! plant_seed over a directory.
//!
//! Port slice 4 adds the report module of PORT4. The report
//! module renders the engine's result as the text catch table
//! and as the JSON document, the two renderings
//! canaryfield/report.py carried before it retired, and the
//! binary's render report door prints the one rendering the
//! caller names for the command line's bridge.

//! Port slice 5 adds the engine module of PORT5. The engine
//! module carries the field run whole, the copy mode and the
//! git branch mode with their gates and the hash walk over
//! the SHA-256 module, and the binary's run field door takes
//! a request document and prints the run's result document
//! for the engine bridge in canaryfield/engine.py, which
//! stands as the run's Python surface from this slice on.

pub mod cli;
pub mod engine;
pub mod errors;
pub mod json;
pub mod pyapply;
pub mod pyarena;
pub mod pyast;
pub mod pydata;
pub mod pydump;
pub mod pyparse;
pub mod pyplant;
pub mod pytoken;
pub mod pyunparse;
pub mod pywalk;
pub mod registry;
pub mod report;
pub mod seedpack;
pub mod sha256;
pub mod xid_tables;

pub use crate::errors::CanaryfieldError;
pub use crate::json::JsonValue;
pub use crate::registry::{SeedMeta, SEEDS};
pub use crate::report::{render_json, render_report, render_text};
pub use crate::seedpack::{
    load_seed_pack, pack_entry_count, registry_for_format, seed_pack_data, seed_pack_text,
    write_seed_pack, SeedEntry, PACK_ENTRY_COUNTS, SEED_PACK_FORMAT,
};
