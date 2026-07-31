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
        match cmd {
            Command::Add(o) => {
                let id = o.id();
                match self.orderbook.add(o) {
                    
                    Ok(_) =>   Ok(ExecResult::AddSucc { id }),
                    Err(e)=>Err(ExecErr::OrderBookError( e))
                }
            }
            Command::Get(id) => {
                if let Some(o) = self.orderbook.get(id) {
                    // 如何clone？
                    Ok(ExecResult::FindSucc(o.clone()))
                } else {
                    Err(ExecErr::OrderBookError(OrderBookError::NotFound(id)))
                }
            }
            Command::Cancel(id) =>  {
                match self.orderbook.cancel(id) {
                    Ok(_) => Ok(ExecResult::RemoveSucc {id}),
                    Err(e)=>Err(ExecErr::OrderBookError( e))
                }
                
            }
            Command::Reduce { id,qty } => {
                match self.orderbook.reduce(id, qty) {
                    Ok(rst) =>   Ok(rst),
                    Err(e)=>Err(ExecErr::OrderBookError( e))
                }
            }
            Command::Summary=> Ok(ExecResult::SummarySucc {count: self.orderbook.len()})
        }
    }
}
