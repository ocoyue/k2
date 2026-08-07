//! Common domain models.
//!
//! This crate contains data structures shared by different components.
//!
//! Future examples:
//!
//! - Order
//! - Instrument
//! - Trade
//! - Event
//! - Snapshot
//!
//! This crate should not know about:
//!
//! - TCP
//! - Protocol
//! - Database
//! - Engine runtime

#![allow(dead_code)]
pub mod order;

pub use order::*;
