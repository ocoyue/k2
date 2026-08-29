use model::Order;
use tokio::sync::oneshot;

pub(crate) enum EngineCommand {
    AddOrder {
        id: u64,
        symbol: String,
        qty: u64,
        reply: oneshot::Sender<AddOrderResult>,
    },

    GetBook {
        reply: oneshot::Sender<BookSnapshot>,
    },
}
pub struct AddOrderResult {
    pub id: u64,
    pub seq_id: u64,
}

pub struct BookSnapshot {
    pub as_of_seq: u64,
    pub orders: Vec<Order>,
}
