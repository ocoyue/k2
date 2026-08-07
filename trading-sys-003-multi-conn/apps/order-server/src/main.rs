use order_gateway::{OrderSession, SimpleOrderHandler};
use orderbook_engine::{BookService, MiniOrderBook};
use std::net::TcpListener;
use std::thread;

const ADDRESS: &str = "127.0.0.1:9000";

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind order server");

    println!("order server listening on {ADDRESS}");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let address = stream.peer_addr().ok();

                thread::spawn(move || {
                    if let Some(address) = address {
                        println!("order client connected: {address}");
                    }

                    let book = MiniOrderBook::new();

                    let service = BookService::new(book);

                    let handler = SimpleOrderHandler::new(service);

                    let session = OrderSession::new(stream, handler);

                    if let Err(error) = session.run() {
                        eprintln!("order session error: {error}");
                    }
                });
            }

            Err(error) => {
                eprintln!("failed to accept order client: {error}");
            }
        }
    }
}
