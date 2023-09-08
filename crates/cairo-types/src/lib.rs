#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
pub use error::{Error, Result};

pub mod ty;
pub use ty::CairoType;

pub mod types;
