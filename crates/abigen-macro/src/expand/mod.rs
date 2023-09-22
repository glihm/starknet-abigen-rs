// TODO: in general, we need to expand a code that is correctly formatted.
// For now, we use #[rustfmt::skip] to avoid the auto-format from cargo-fmt
// on the quote!, but we should care about trailing commas for instance.

pub(crate) mod contract;
pub(crate) mod r#enum;
pub(crate) mod r#struct;
//pub(crate) mod r#function;
pub(crate) mod generic;
pub(crate) mod utils;

