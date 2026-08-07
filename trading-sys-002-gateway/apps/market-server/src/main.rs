//! Market Server Application.
//!
//! This binary represents the external entry point
//! for market data distribution.
//!
//! Future architecture:
//!
//! Market Data Engine
//!        |
//!        v
//! Market Gateway
//!        |
//!        v
//! Client
//!
//! Current stage:
//!
//! Only provides application bootstrap.
use market_gateway::{InstrumentHandler, MarketSession};
use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:9001";

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind market server");

    println!("market server listening on {ADDRESS}");

    let (stream, address) = listener.accept().expect("failed to accept market client");

    println!("market client connected: {address}");

    let handler = InstrumentHandler;

    let session = MarketSession::new(stream, handler);

    if let Err(error) = session.run() {
        eprintln!("market session error: {error}");
    }
}
