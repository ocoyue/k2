use crate::model::order::Order;
#[derive(Debug)]
pub enum ExecResult {
    AddSucc { id:u32 },
    FindSucc(Order),
    ReduceSucc { id:u32,remaining:u64 },
    RemoveSucc {
        id:u32,
    },
    SummarySucc {
        count:usize,
    },
}