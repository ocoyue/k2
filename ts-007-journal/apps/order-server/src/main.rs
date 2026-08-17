use order_gateway::{OrderSession, SimpleOrderHandler};
use orderbook_engine::start_engine;
use std::net::TcpListener;
use std::thread;
const ADDRESS: &str = "127.0.0.1:9000";
const JOURNAL_PATH: &str = "data/order.journal";
fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind order server");
    let engine_proxy = start_engine(JOURNAL_PATH).expect("failed to start engine with journal");
    println!("order server listening on {ADDRESS}");
    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let address = stream.peer_addr().ok();
                let proxy = engine_proxy.clone();
                thread::spawn(move || {
                    if let Some(address) = address {
                        println!("order client connected: {address}");
                    }
                    let handler = SimpleOrderHandler::new(proxy);

                    let session = OrderSession::new(stream, handler);

                    if let Err(error) = session.run() {
                        eprintln!("order session error: {error}");
                    }
                });
            }
            Err(error) => {
                eprintln!("accept error: {error}");
            }
        }
    }
}
