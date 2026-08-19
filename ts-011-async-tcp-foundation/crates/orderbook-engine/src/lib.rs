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

mod checkpoint;
mod engine_loop;
mod engine_message;
mod engine_proxy;
mod orderbook;
mod recovery;

pub use engine_loop::start_engine_loop;
pub use engine_message::{AddOrderResult, BookSnapshot};
pub use engine_proxy::EngineProxy;
pub use recovery::{RecoveredEngineState, recover_engine_state};
