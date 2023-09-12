// TODO: in general, we need to expand a code that is correctly formatted.
// For now, we use #[rustfmt::skip] to avoid the auto-format from cargo-fmt
// on the quote!, but we should care about trailing commas for instance.

mod r#enum;
mod r#event;
mod r#function;
mod r#struct;
mod utils;
