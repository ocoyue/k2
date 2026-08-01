use crate::error::orderbook_error::OrderBookError;
use crate::model::order::Order;
use crate::model::summary::OrderBookSummary;

#[derive(Debug)]
pub struct OrderBook {
    orders: Vec<Order>,
}
impl OrderBook {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn add(&mut self, order: Order) -> Result<(), OrderBookError> {
        if self.orders.iter().any(|o| o.id() == order.id()) {
            return Err(OrderBookError::DuplicateId(order.id()));
        }
        self.orders.push(order);
        Ok(())
    }
    pub fn get(&self, id: u32) -> Option<&Order> {
        self.orders.iter().find(|o| o.id() == id)
    }
    pub fn cancel(&mut self, id: u32) -> Result<(), OrderBookError> {
        let index = self.orders.iter().position(|o| o.id() == id);
        match index {
            Some(i) => {
                self.orders.remove(i);
                Ok(())
            }
            None => Err(OrderBookError::NotFound(id)),
        }
    }
    pub fn reduce(&mut self, id: u32, amount: u64) -> Result<u64, OrderBookError> {
        let index = self
            .orders
            .iter()
            .position(|order| order.id() == id)
            .ok_or(OrderBookError::NotFound(id))?;

        let remaining = self.orders[index].reduce(amount)?;
        if remaining == 0 {
            self.orders.remove(index);
        }
        Ok(remaining)
    }
    pub fn summary(&self) -> Result<OrderBookSummary, OrderBookError> {
        let mut buy_count = 0;
        let mut sell_count = 0;
        let mut total_quantity = 0;

        for order in &self.orders {
            if order.is_buy() {
                buy_count += 1;
            } else {
                sell_count += 1;
            }

            total_quantity += order.quantity();
        }

        Ok(OrderBookSummary {
            order_count: self.orders.len(),
            buy_count,
            sell_count,
            total_quantity,
        })
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }
}
