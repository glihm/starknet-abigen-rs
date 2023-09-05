#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
pub use error::{Error, Result};

mod ty;
mod types;

