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
//! stays with the Python in this slice, including the seed apply
//! functions and the hand written parser for the Python source
//! shapes they read, the engine that plants the seeds and runs
//! the gates, the report module, and the command line beyond the
//! pack writing path the binary carries.

pub mod errors;
pub mod json;
pub mod registry;
pub mod seedpack;

pub use crate::errors::CanaryfieldError;
pub use crate::json::JsonValue;
pub use crate::registry::{SeedMeta, SEEDS};
pub use crate::seedpack::{
    load_seed_pack, pack_entry_count, registry_for_format, seed_pack_data, seed_pack_text,
    write_seed_pack, SeedEntry, PACK_ENTRY_COUNTS, SEED_PACK_FORMAT,
};
