use crate::engine_message::{AddOrderResult, BookSnapshot, EngineCommand};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct EngineProxy {
    sender: Sender<EngineCommand>,
}

impl EngineProxy {
    pub(crate) fn new(sender: Sender<EngineCommand>) -> Self {
        Self { sender }
    }
    pub fn add_order(&self, id: u64, symbol: String, qty: u64) -> AddOrderResult {
        let (reply_tx, reply_rx) = mpsc::channel();

        let command = EngineCommand::AddOrder {
            id,
            symbol,
            qty,
            reply: reply_tx,
        };

        self.sender.send(command).expect("engine disconnected");

        reply_rx.recv().expect("engine reply disconnected")
    }

    pub fn get_book(&self) -> BookSnapshot {
        let (reply_tx, reply_rx) = mpsc::channel();

        let command = EngineCommand::GetBook { reply: reply_tx };

        self.sender.send(command).expect("engine disconnected");

        reply_rx.recv().expect("engine reply disconnected")
    }
}
