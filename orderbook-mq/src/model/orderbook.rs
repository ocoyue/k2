use crate::error::ParseErr;
use crate::model::Side;
use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct OrderBook {
    orders: Vec<Order>,
}

impl OrderBook {
    // pub fn new() -> Self {
    //     Self { orders: Vec::new() }
    // }

    pub fn from_orders(orders: Vec<Order>) -> Self {
        Self { orders }
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn orders_mut(&mut self) -> &mut Vec<Order> {
        &mut self.orders
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.orders.len()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    id: u32,
    price: f64,
    qty: u32,
    side: Side,
}
impl Order {
    pub fn new(id: u32, price: f64, qty: u32, side: Side) -> Result<Order, ParseErr> {
        if id == 0 {
            return Err(ParseErr::InvalidOrder {
                reason: "id must be positive".to_string(),
            });
        }
        if price <= 0.0 {
            return Err(ParseErr::InvalidPrice(price));
        }
        if qty == 0 {
            return Err(ParseErr::InvalidQuantity(qty));
        }
        Ok(Order {
            id,
            price,
            qty,
            side,
        })
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn qty(&self) -> u32 {
        self.qty
    }
    pub fn side(&self) -> Side {
        self.side
    }
    pub fn value(&self) -> f64 {
        self.price * self.qty as f64
    }

    pub fn set_qty(&mut self, qty: u32) {
        self.qty = qty
    }
}
impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ORDER id={} qty={} price={} side={}",
            self.id, self.qty, self.price, self.side
        )
    }
}
