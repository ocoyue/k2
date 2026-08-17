//! OrderBook Engine.
//!
//! Core trading state machine.
//!
//! Future responsibility:
//!
//! - maintain order state
//! - process commands
//! - generate events
//!
//! Current project only implements minimal features:
//!
//! - ADD
//! - BOOK snapshot
//!
//! This is not a complete exchange matching engine.

mod book;
mod service;
pub use book::MiniOrderBook;
pub use service::{AddOrderResult, BookService, BookSnapshot};
