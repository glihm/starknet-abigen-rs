use alloc::string::{String};


/// Cairo types result.
pub type Result<T> = core::result::Result<T, Error>;

/// A cairo type error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid type string.
    InvalidTypeString(String),
    /// Error during serialization.
    Serialize(String),
    /// Error during deserialization.
    Deserialize(String),
    
}
