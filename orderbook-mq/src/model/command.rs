use std::sync::mpsc::Sender;
use crate::error::ExeErr;
use crate::model::{ExeOk, Order};
pub struct OrderbookReq {
    pub cmd: OrderbookCmd,
    pub reply: Sender<Result<ExeOk, ExeErr>>,
}
impl OrderbookReq {
    pub fn new(cmd: OrderbookCmd, reply: Sender<Result<ExeOk, ExeErr>>) -> Self {
        OrderbookReq { cmd, reply }
    }
}

#[derive(Debug,PartialEq)]
pub enum OrderbookCmd {
    Add(Order),
    Cancel(u32),
    Reduce { id: u32, qty: u32 },
    Get(u32),
    Summary,
    Shutdown,
}
