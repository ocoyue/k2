use crate::EngineProxy;
use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use crate::orderbook::OrderBook;
use model::Order;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

// 经过了一层封装，外部只知道需要一个proxy，放在handler里面就行。但是不知道是什么通信方式。
pub fn start_engine() -> EngineProxy {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        run_engine_loop(rx);
    });
    EngineProxy::new(tx)
}
fn run_engine_loop(receiver: Receiver<EngineCommand>) {
    println!("engine thread: {:?}", thread::current().id());
    let mut book = OrderBook::new();

    while let Ok(command) = receiver.recv() {
        match command {
            EngineCommand::AddOrder {
                id,
                symbol,
                qty,
                reply,
            } => {
                println!("engine {:?}: ADD id={id}", thread::current().id());
                let order = Order::new(id, symbol, qty);

                book.add(order);

                let result = AddOrderResult { id };

                let _ = reply.send(result);
            }

            EngineCommand::GetBook { reply } => {
                println!("engine {:?}: BOOK", thread::current().id());
                let result = BookSnapshot {
                    orders: book.snapshot(),
                };

                let _ = reply.send(result);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::start_engine;

    #[test]
    fn cloned_proxies_access_same_engine_state() {
        let proxy_a = start_engine();
        let proxy_b = proxy_a.clone();

        proxy_a.add_order(1, "BTCUSDT".to_string(), 10);

        let snapshot = proxy_b.get_book();

        assert_eq!(snapshot.orders.len(), 1);

        assert_eq!(snapshot.orders[0].id(), 1);
    }
}
