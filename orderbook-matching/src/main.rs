mod engine;
mod error;
mod model;
mod parser;
mod protocol;
mod session;
mod tcp;

#[cfg(test)]
mod tests;

use crate::engine::run_orderbook_engine;
use crate::model::command::EngineRequest;
use crate::model::orderbook::OrderBook;
use crate::model::{Order, Side};
use crate::session::run_tcp_session;
use crate::tcp::{init_tcp, run_tcp_loop};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::{io, spawn};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. Order book
    let orderbook = init_orderbook();
    // 总接线
    let (tx, rx) = init_mpsc_channel();
    // engine-loop
    let _engine_handle = spawn(run_orderbook_engine(orderbook, rx));
    // tcp-loop
    tcp_loop(tx).await
}

async fn tcp_loop(tx: Sender<EngineRequest>) -> io::Result<()> {
    // TCP Listener
    let listener = init_tcp().await?;
    // tcp-loop
    let closure = move |stream: TcpStream, addr: SocketAddr| {
        let tx = tx.clone();
        run_tcp_session(stream, addr, tx)
    };
    run_tcp_loop(listener, closure).await?;
    Ok(())
}
fn init_orderbook() -> OrderBook {
    OrderBook::from_orders(mock_orders()).expect("init orderbook should be valid")
}
fn init_mpsc_channel() -> (Sender<EngineRequest>, Receiver<EngineRequest>) {
    channel::<EngineRequest>(2048)
}

fn mock_orders()-> Vec<Order> {
    let o1 = Order::new(1, 88.0, 100, Side::Buy).unwrap();
    let o2 = Order::new(2, 88.0, 100, Side::Sell).unwrap();
    let o3 = Order::new(3, 88.0, 100, Side::Sell).unwrap();
    let o4 = Order::new(4, 88.0, 100, Side::Buy).unwrap();
    let o5 = Order::new(5, 88.0, 100, Side::Buy).unwrap();
    vec![o1, o2, o3, o4, o5]
}


