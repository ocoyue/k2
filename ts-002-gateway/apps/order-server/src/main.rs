use order_gateway::{OrderSession, SimpleOrderHandler};
use std::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:9000";

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("failed to bind order server");
    println!("order server listening on {ADDRESS}");

    let (stream, address) = listener.accept().expect("failed to accept order client");
    println!("order client connected: {address}");

    let session = OrderSession::new(stream, SimpleOrderHandler);

    if let Err(error) = session.run() {
        eprintln!("order session error: {error}");
    }
}
