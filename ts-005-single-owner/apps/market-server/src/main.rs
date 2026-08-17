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
use instrument::{Instrument, InstrumentCatalog, InstrumentStatus};
use market_gateway::{InstrumentHandler, MarketSession};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

const ADDRESS: &str = "127.0.0.1:9001";

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind market server");

    let catalog = Arc::new(create_instrument_catalog());

    println!("market server listening on {ADDRESS}");

    println!("loaded {} instruments", catalog.len());

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let address = stream.peer_addr().ok();

                let catalog = Arc::clone(&catalog);

                thread::spawn(move || {
                    if let Some(address) = address {
                        println!("market client connected: {address}");
                    }

                    let handler = InstrumentHandler::new(catalog);

                    let session = MarketSession::new(stream, handler);

                    if let Err(error) = session.run() {
                        eprintln!("market session error: {error}");
                    }

                    if let Some(address) = address {
                        println!("market client disconnected: {address}");
                    }
                });
            }

            Err(error) => {
                eprintln!("failed to accept market client: {error}");
            }
        }
    }
}

fn create_instrument_catalog() -> InstrumentCatalog {
    InstrumentCatalog::new(vec![
        Instrument::new("BTCUSDT", 1, 1, InstrumentStatus::Active),
        Instrument::new("ETHUSDT", 1, 1, InstrumentStatus::Active),
        Instrument::new("SOLUSDT", 1, 1, InstrumentStatus::Halted),
    ])
}
