use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use std::path::Path;

use crate::EngineProxy;
use crate::orderbook::OrderBook;

use event::{EngineEvent, Sequencer};

use crate::recovery::RecoveredEngineState;
use journal::JournalFile;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{io, thread};

pub fn start_engine_loop(
    recovered: RecoveredEngineState,
    journal_path: impl AsRef<Path>,
) -> io::Result<EngineProxy> {
    let RecoveredEngineState {
        book,
        last_applied_seq,
    } = recovered;

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

// crates/orderbook-engine/src/engine_loop.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recovery::recover_engine_state;
    use event::{EngineEvent, SequencedEvent};
    use journal::JournalFile;
    use std::fs::remove_file;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn engine_continues_sequence_after_recovery() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("engine-recovery-{unique}.journal"));

        {
            let mut journal = JournalFile::create_new(&path).unwrap();

            journal
                .append(&SequencedEvent::new(
                    1,
                    EngineEvent::OrderAdded {
                        id: 1,
                        symbol: "BTCUSDT".to_string(),
                        qty: 10,
                    },
                ))
                .unwrap();

            journal
                .append(&SequencedEvent::new(
                    2,
                    EngineEvent::OrderAdded {
                        id: 2,
                        symbol: "ETHUSDT".to_string(),
                        qty: 20,
                    },
                ))
                .unwrap();
        }

        let recovered = recover_engine_state(&path).unwrap();
        let proxy = start_engine_loop(recovered, &path).unwrap();

        let add_result = proxy.add_order(3, "SOLUSDT".to_string(), 30);

        assert_eq!(add_result.seq_id, 3);

        let snapshot = proxy.get_book();

        assert_eq!(snapshot.as_of_seq, 3);
        assert_eq!(snapshot.orders.len(), 3);

        remove_file(path).unwrap();
    }
}
