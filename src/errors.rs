//! The crate error type.

use std::fmt;

/// The one error the seed pack reading raises.
///
/// Every refusal the Python loader promises is carried by this
/// error with the same text the Python raises, as the contract
/// port section states. The wording of a JSON decode refusal and
/// of a file access refusal belongs to the decoder and the
/// operating system side of this crate rather than to the pack
/// reading itself, and the port section states that boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanaryfieldError {
    message: String,
}

impl CanaryfieldError {
    /// Build an error carrying the given message text.
    pub fn new(message: impl Into<String>) -> Self {
        CanaryfieldError {
            message: message.into(),
        }
    }
}

impl fmt::Display for CanaryfieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CanaryfieldError {}
