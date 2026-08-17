use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use std::path::Path;

use crate::EngineProxy;
use crate::orderbook::OrderBook;

use event::{EngineEvent, Sequencer};

use journal::JournalFile;
use replay::replay_journal;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{io, thread};

pub fn start_engine(journal_path: impl AsRef<Path>) -> io::Result<EngineProxy> {
    let journal_path = journal_path.as_ref();

    let mut book = OrderBook::new();

    let last_applied_seq = replay_journal(journal_path, |sequenced_event| {
        book.apply(sequenced_event.event());
    })?;
    println!("replay completed: as_of_seq={}", last_applied_seq);
    let sequencer = Sequencer::resume_after(last_applied_seq);

    let journal = JournalFile::open_or_create(journal_path)?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        run_engine_loop(rx, journal, book, sequencer, last_applied_seq);
    });

    Ok(EngineProxy::new(tx))
}

fn run_engine_loop(
    receiver: Receiver<EngineCommand>,
    mut journal: JournalFile,
    mut book: OrderBook,
    mut sequencer: Sequencer,
    mut last_applied_seq: u64,
) {
    println!("engine thread: {:?}", thread::current().id());

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

                journal.append(&seq_event).expect(
                    "journal append failed; \
             engine cannot continue safely",
                );

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
        let proxy = start_engine("data/order.journal").unwrap();

        proxy.add_order(1, "BTCUSDT".to_string(), 10);

        let first = proxy.get_book();

        assert_eq!(first.as_of_seq, 1);

        proxy.add_order(2, "ETHUSDT".to_string(), 20);

        let second = proxy.get_book();

        assert_eq!(second.as_of_seq, 2);

        assert_eq!(second.orders.len(), 2);
    }
}
