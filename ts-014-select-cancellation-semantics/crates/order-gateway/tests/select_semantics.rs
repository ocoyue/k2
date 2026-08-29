use std::future::pending;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::mpsc;

struct DropProbe {
    dropped: Arc<AtomicBool>,
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
    }
}

#[tokio::test(flavor = "current_thread")]
async fn select_drops_losing_future() {
    let dropped = Arc::new(AtomicBool::new(false));

    let drop_probe = DropProbe {
        dropped: Arc::clone(&dropped),
    };

    let losing_future = async move {
        let _drop_probe = drop_probe;

        pending::<()>().await;
    };

    tokio::select! {
        _ = losing_future => {
            panic!("pending future must not complete");
        }

        _ = async {} => {}
    }

    assert!(dropped.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "current_thread")]
async fn mpsc_recv_remains_usable_after_losing_select_branch() {
    let (sender, mut receiver) = mpsc::channel(1);

    tokio::select! {
        biased;

        message = receiver.recv() => {
            panic!("recv should still be pending, got {message:?}");
        }

        _ = async {} => {}
    }

    sender.send(42).await.unwrap();

    let received = receiver.recv().await;

    assert_eq!(received, Some(42));
}
