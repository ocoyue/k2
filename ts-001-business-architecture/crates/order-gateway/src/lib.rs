//! Order Gateway.
//!
//! Responsibility:
//!
//! Receive client trading requests.
//!
//! Future flow:
//!
//! Client
//!   |
//! TCP
//!   |
//! Order Gateway
//!   |
//! Protocol
//!   |
//! Order Engine
//!
//! This crate does not:
//!
//! - match orders
//! - store order state
//! - persist journal