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
