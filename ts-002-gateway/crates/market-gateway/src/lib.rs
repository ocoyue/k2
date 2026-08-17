//! Market Data Gateway.
//!
//! Responsibility:
//!
//! Push market information to clients.
//!
//! Future flow:
//!
//! Market Data Engine
//!        |
//!        v
//! Market Gateway
//!        |
//!        v
//! Client
//!
//! This crate focuses on:
//!
//! - connection management
//! - subscriptions
//! - data delivery

mod handler;
mod session;

pub use handler::{InstrumentHandler, MarketDataHandler};

pub use session::MarketSession;
