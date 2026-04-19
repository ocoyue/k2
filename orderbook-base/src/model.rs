use crate::error::ParseErr;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    BUY,
    SELL,
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
    pub fn price(&self) -> f64 {
        self.price
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
// impl Display for Order {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "id: {}, price: {}, qty: {}", self.id, self.price, self.qty)
//     }
// }
#[derive(Debug, PartialEq)]
pub enum Command {
    ADD(Order),
    CANCEL(u32),
    REDUCE { id: u32, qty: u32 },
    GET(u32),
    SUMMARY,
}
#[derive(Debug, PartialEq)]
pub enum ExecuteResult {
    Count(i32),
    Exists(bool),
    Order(Order),
    Added,
    Canceled,
    Reduced,
    Deleted,
    Summary(Summary),
}
#[derive(PartialEq, Debug)]
pub struct Summary {
    pub count: u32,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_value: f64,
}
impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "我是Summary :\ncount = {}\nbuy count = {}\nsell count = {}\ntotal_value = {}",
            self.count, self.buy_count, self.sell_count, self.total_value
        )
    }
}
