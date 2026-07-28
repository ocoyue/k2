// engine -> control the whole lifecycle of OrderBook

// input: Command output: Result

use crate::model::command::Command;
use crate::state_machine::OrderBook;

pub struct Engine {
    orderbook: OrderBook,
}
impl Engine {
    pub fn new(ob:OrderBook) -> Self {
        Self{
            orderbook: ob,
        }
    }
    pub fn execute(&mut self, cmd: Command) {
        match cmd    {
            Command::Add(o ) => {
                self.orderbook.add(o);
                println!("Added new order");
            }
            Command::Get(id) => {
                let rst = self.orderbook.get(id);
                println!("{:?}",rst)
            }
            Command::Cancel (id)  => {}
            Command::Reduce { id,qty } => {}
            Command::Summary => {}
        }
    }
}




