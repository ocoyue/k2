use order_gateway::{OrderSession, SimpleOrderHandler};
use orderbook_engine::{recover_engine_state, start_engine_loop};
use std::net::TcpListener;
use std::thread;

const ADDRESS: &str = "127.0.0.1:9000";
const JOURNAL_PATH: &str = "data/order.journal";

fn main() {
    // 1. Recovery Phase
    let recovered = recover_engine_state(JOURNAL_PATH).expect("failed to recover engine state");

    // 2. Runtime Phase
    let engine_proxy =
        start_engine_loop(recovered, JOURNAL_PATH).expect("failed to start engine loop");

    // 3. Network Bootstrap
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind order server");

    println!("order server listening on {ADDRESS}");

    // 4. Accept
    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let address = stream.peer_addr().ok();
                let proxy = engine_proxy.clone();

                // 5. Session
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
