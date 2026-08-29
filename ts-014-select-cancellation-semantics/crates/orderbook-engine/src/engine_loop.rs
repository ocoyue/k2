use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use std::path::{Path, PathBuf};

use crate::EngineProxy;
use crate::orderbook::OrderBook;

use event::{EngineEvent, Sequencer};

use crate::checkpoint::create_checkpoint;
use crate::recovery::RecoveredEngineState;
use journal::JournalFile;

use std::{io, thread};
use tokio::sync::mpsc::{self, Receiver};

const ENGINE_COMMAND_QUEUE_CAPACITY: usize = 8192;
pub fn start_engine_loop(
    recovered: RecoveredEngineState,
    journal_path: impl AsRef<Path>,
    snapshot_path: impl AsRef<Path>,
    checkpoint_interval: u64,
) -> io::Result<EngineProxy> {
    if checkpoint_interval == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "checkpoint interval must be greater than zero",
        ));
    }
    let snapshot_path = snapshot_path.as_ref().to_path_buf();
    let RecoveredEngineState {
        book,
        last_applied_seq,
    } = recovered;

    let sequencer = Sequencer::resume_after(last_applied_seq);
    let journal = JournalFile::open_or_create(journal_path)?;

    if last_applied_seq > 0 {
        match journal.current_offset() {
            Ok(journal_offset) => {
                if let Err(error) =
                    create_checkpoint(&snapshot_path, &book, last_applied_seq, journal_offset)
                {
                    eprintln!("bootstrap checkpoint failed: {error}");
                }
            }

            Err(error) => {
                eprintln!("failed to read journal offset for bootstrap checkpoint: {error}");
            }
        }
    }

    let (tx, rx) = mpsc::channel(ENGINE_COMMAND_QUEUE_CAPACITY);

    thread::spawn(move || {
        run_engine_loop(
            rx,
            journal,
            book,
            sequencer,
            last_applied_seq,
            snapshot_path,
            checkpoint_interval,
        );
    });

    Ok(EngineProxy::new(tx))
}

fn run_engine_loop(
    mut receiver: Receiver<EngineCommand>,
    mut journal: JournalFile,
    mut book: OrderBook,
    mut sequencer: Sequencer,
    mut last_applied_seq: u64,
    snapshot_path: PathBuf,
    checkpoint_interval: u64,
) {
    println!("engine thread: {:?}", thread::current().id());

    while let Some(command) = receiver.blocking_recv() {
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
        if last_applied_seq % checkpoint_interval == 0 {
            match journal.current_offset() {
                Ok(journal_offset) => {
                    if let Err(error) =
                        create_checkpoint(&snapshot_path, &book, last_applied_seq, journal_offset)
                    {
                        eprintln!("periodic checkpoint failed: {error}");
                    }
                }

                Err(error) => {
                    eprintln!("failed to read journal offset for checkpoint: {error}");
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::recover_engine_state;
    use snapshot::SnapshotFile;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[tokio::test(flavor = "current_thread")]
    async fn engine_creates_periodic_checkpoint() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let journal_path = std::env::temp_dir().join(format!("periodic-{unique}.journal"));

        let snapshot_path = std::env::temp_dir().join(format!("periodic-{unique}.snapshot"));

        let recovered = recover_engine_state(&journal_path, &snapshot_path).unwrap();

        let proxy = start_engine_loop(recovered, &journal_path, &snapshot_path, 2).unwrap();

        proxy.add_order(1, "BTCUSDT".to_string(), 10).await;

        proxy.add_order(2, "ETHUSDT".to_string(), 20).await;

        for _ in 0..100 {
            if snapshot_path.exists() {
                break;
            }

            std::thread::sleep(Duration::from_millis(1));
        }

        let snapshot = SnapshotFile::load(&snapshot_path).unwrap().unwrap();

        assert_eq!(snapshot.as_of_seq(), 2);
        assert_eq!(snapshot.orders().len(), 2);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn checkpoint_waits_for_next_interval() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let journal_path = std::env::temp_dir().join(format!("interval-{unique}.journal"));

        let snapshot_path = std::env::temp_dir().join(format!("interval-{unique}.snapshot"));

        let recovered = recover_engine_state(&journal_path, &snapshot_path).unwrap();

        let proxy = start_engine_loop(recovered, &journal_path, &snapshot_path, 2).unwrap();

        proxy.add_order(1, "BTCUSDT".to_string(), 10).await;

        assert!(!snapshot_path.exists());

        proxy.add_order(2, "ETHUSDT".to_string(), 20).await;

        for _ in 0..100 {
            if snapshot_path.exists() {
                break;
            }

            std::thread::sleep(Duration::from_millis(1));
        }

        let snapshot = SnapshotFile::load(&snapshot_path).unwrap().unwrap();

        assert_eq!(snapshot.as_of_seq(), 2);
    }
    #[tokio::test(flavor = "current_thread")]
    async fn bounded_mpsc_applies_backpressure_when_full() {
        let (sender, mut receiver) = mpsc::channel(1);

        sender.send(10).await.unwrap();

        let sender2 = sender.clone();

        let send_handle = tokio::spawn(async move {
            sender2.send(20).await.unwrap();
        });

        tokio::task::yield_now().await;

        assert!(!send_handle.is_finished());

        let first_value = receiver.recv().await;
        assert_eq!(first_value, Some(10));

        send_handle.await.unwrap();

        let second_value = receiver.recv().await;
        assert_eq!(second_value, Some(20));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn bounded_mpsc_try_send_reports_full() {
        let (sender, _receiver) = tokio::sync::mpsc::channel(1);

        sender.try_send(10).unwrap();

        let result = sender.try_send(20);

        assert!(matches!(
            result,
            Err(tokio::sync::mpsc::error::TrySendError::Full(20))
        ));
    }
}
