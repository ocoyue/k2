// engine -> control the whole lifecycle of OrderBook

// input: Command output: Result

use crate::engine::exec_result::ExecResult;
use crate::error::ExecErr;
use crate::error::orderbook_error::OrderBookError;
use crate::model::command::Command;
use crate::state_machine::OrderBook;

pub struct Engine {
    orderbook: OrderBook,
}
impl Engine {
    pub fn new(ob: OrderBook) -> Self {
        Self { orderbook: ob }
    }
    pub fn execute(&mut self, cmd: Command) -> Result<ExecResult, ExecErr> {
        // println!("{:?}", cmd);
        match cmd {
            Command::Add(o) => {
                let id = o.id();
                self.orderbook.add(o)?;
                Ok(ExecResult::AddSucc { id })
            }
            Command::Get(id) => {
                let order = self
                    .orderbook
                    .get(id)
                    .cloned()
                    .ok_or(OrderBookError::NotFound(id))?;

                Ok(ExecResult::FindSucc(order))
            }
            Command::Cancel(id) => {
                self.orderbook.cancel(id)?;
                Ok(ExecResult::RemoveSucc { id })
            }
            Command::Reduce { id, amount } => {
                let remaining = self.orderbook.reduce(id, amount)?;
                Ok(ExecResult::ReduceSucc { id, remaining })
            }
            Command::Summary => Ok(ExecResult::SummarySucc(self.orderbook.summary()?)),
        }
    }
}
