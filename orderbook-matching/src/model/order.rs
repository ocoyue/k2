use std::fmt::{Display, Formatter};
use crate::error::ParseErr;
use crate::model::{Price, Side};

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    id: u32,
    price: Price,
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
        if qty == 0 {
            return Err(ParseErr::InvalidQuantity(qty));
        }
        Ok(Order {
            id,
            price: Price::from_f64(price)?,
            qty,
            side,
        })
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn price(&self) -> Price {
        self.price
    }
    pub fn qty(&self) -> u32 {
        self.qty
    }
    pub fn side(&self) -> Side {
        self.side
    }
    pub fn value(&self) -> i64 {
        self.price.ticks() * (self.qty as i64)
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