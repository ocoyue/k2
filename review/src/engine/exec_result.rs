use crate::model::order::Order;
use crate::model::summary::OrderBookSummary;
#[derive(Debug)]
pub enum ExecResult {
    AddSucc { id: u32 },
    FindSucc(Order),
    ReduceSucc { id: u32, remaining: u64 },
    RemoveSucc { id: u32 },
    SummarySucc(OrderBookSummary),
}
