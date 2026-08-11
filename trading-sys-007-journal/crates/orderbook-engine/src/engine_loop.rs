use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};

use crate::EngineProxy;
use crate::orderbook::OrderBook;

use event::{EngineEvent, Sequencer};

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

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

    let mut sequencer = Sequencer::new(); // init sequencer seq=1

    let mut last_applied_seq = 0_u64;

    while let Ok(command) = receiver.recv() {
        match command {
            EngineCommand::AddOrder {
                id,
                symbol,
                qty,
                reply,
            } => {
                // 外部进来 Command -> 内部生产事件 -> apply 事件

                let event = EngineEvent::OrderAdded { id, symbol, qty };
                let seq_event = sequencer.assign_sequence(event);

                println!(
                    "engine {:?}: EVENT seq={} {:?}",
                    thread::current().id(),
                    seq_event.seq_id(),
                    seq_event.event(),
                );

                book.apply(seq_event.event());

                last_applied_seq = seq_event.seq_id();

                let result = AddOrderResult {
                    id,
                    seq_id: seq_event.seq_id(),
                };

                let _ = reply.send(result);
            }

            EngineCommand::GetBook { reply } => {
                println!(
                    "engine {:?}: BOOK as_of_seq={}",
                    thread::current().id(),
                    last_applied_seq,
                );

                let result = BookSnapshot {
                    as_of_seq: last_applied_seq,

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
    fn book_snapshot_reports_last_applied_event_seq() {
        let proxy = start_engine();

        proxy.add_order(1, "BTCUSDT".to_string(), 10);

        let first = proxy.get_book();

        assert_eq!(first.as_of_seq, 1);

        proxy.add_order(2, "ETHUSDT".to_string(), 20);

        let second = proxy.get_book();

        assert_eq!(second.as_of_seq, 2);

        assert_eq!(second.orders.len(), 2);
    }
}
