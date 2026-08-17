use model::Order;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub(crate) enum EngineCommand {
    AddOrder {
        id: u64,
        symbol: String,
        qty: u64,
        reply: Sender<AddOrderResult>,
    },

    GetBook {
        reply: Sender<BookSnapshot>,
    },
}
pub struct AddOrderResult {
    pub id: u64,
}

pub struct BookSnapshot {
    pub orders: Vec<Order>,
}
