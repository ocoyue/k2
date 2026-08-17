//! Trading instrument reference data.
//!
//! This crate defines instruments and the immutable
//! instrument catalog shared by gateway sessions.
//!
//! The catalog is created during application startup
//! and is read-only after publication.

mod catalog;
mod instrument;

pub use catalog::InstrumentCatalog;
pub use instrument::{Instrument, InstrumentStatus};
