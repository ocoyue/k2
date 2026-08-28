use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

#[derive(Clone, Debug)]
pub struct EngineProxy {
    sender: Sender<EngineCommand>,
}

impl EngineProxy {
    pub(crate) fn new(sender: Sender<EngineCommand>) -> Self {
        Self { sender }
    }
    pub async fn add_order(&self, id: u64, symbol: String, qty: u64) -> AddOrderResult {
        let (reply_tx, reply_rx) = oneshot::channel();

        let command = EngineCommand::AddOrder {
            id,
            symbol,
            qty,
            reply: reply_tx,
        };

        self.sender
            .send(command)
            .await
            .expect("engine disconnected");

        reply_rx.await.expect("engine reply disconnected")
    }

    pub async fn get_book(&self) -> BookSnapshot {
        let (reply_tx, reply_rx) = oneshot::channel();

        let command = EngineCommand::GetBook { reply: reply_tx };

        self.sender
            .send(command)
            .await
            .expect("engine disconnected");

        reply_rx.await.expect("engine reply disconnected")
    }
}
