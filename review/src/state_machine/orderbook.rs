use crate::engine::ExecResult;
use crate::engine::ExecResult::ReduceSucc;
use crate::error::order_error::OrderError;
use crate::error::orderbook_error::OrderBookError;
use crate::model::order::Order;

#[derive(Debug)]
pub struct OrderBook {
    orders: Vec<Order>,
}
impl OrderBook {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn print_all(&self) {
        println!();
        for o in &self.orders {
            println!("{:?}", o);
        }
    }
    pub fn add(
        &mut self,
        order:Order
    )->Result<(),OrderBookError>{
        if self.orders.iter().any(|o| o.id() == order.id()) {
            return Err(OrderBookError::DuplicateId(order.id()));
        }
        self.orders.push(order);
        Ok(())
    }
    pub fn get(&self, id: u32) -> Option<&Order> {
        self.orders.iter().find(|o| o.id() == id)
    }
    pub fn cancel(&mut self, id: u32) -> Result<(),OrderBookError> {
        let o = self.orders.iter().find(|o | {o.id()==id});
        if let Some(o) = o {
            self.orders.remove(o.id() as usize);
            Ok(())
        }else { 
            Err(OrderBookError::NotFound(id))
        }
    }
    pub fn reduce(&mut self, id: u32, amount: u64) -> Result<ExecResult, OrderBookError> {
        let order = self.orders.iter_mut().find(|o| o.id() == id);
        match order {
            Some(o) => match o.reduce(amount) {
                Ok(n) => Ok(ReduceSucc { id, remaining: n }),
                Err(OrderError::ReduceAmountExceedsRemaining) => {
                    Err(OrderBookError::OrderError(OrderError::ReduceAmountExceedsRemaining))
                }
                Err(_) => Err(OrderBookError::OrderError(OrderError::ReduceFailed)),
            },
            None => Err(OrderBookError::NotFound(id)),
        }
    }

    pub fn len(&self)->usize{

        self.orders.len()

    }
}
