use crate::error::ParseErr;

#[derive(Debug, PartialEq)]
pub enum Side {
    BUY,
    SELL,
}
#[derive(Debug, PartialEq)]
pub struct Order {
    id: u32,
    price: f64,
    qty: u32,
    side: Side,
}
impl Order {
    pub fn new(id: u32, price: f64, qty: u32, side: Side) -> Result<Order, ParseErr> {
        if id ==0 {
            return Err(ParseErr::InvalidOrder {
                reason: "id must be positive".to_string(),
            });
        }
        if price <= 0.0 {
            return Err(ParseErr::InvalidOrder {
                reason: "price must be positive".to_string(),
            });
        }
        if qty == 0 {
            return Err(ParseErr::InvalidOrder {
                reason: "qty must be positive".to_string(),
            });
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
}
#[derive(Debug, PartialEq)]
pub enum Command {
    ADD(Order),
}
