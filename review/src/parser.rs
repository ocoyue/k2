use crate::error::parse_error::ParseError;
use crate::error::parse_error::ParseError::{BuildOrderFailed, HeadText, IdText, PriceText, QtyText};
use crate::model::command::Command;
use crate::model::order::Order;
use crate::model::side::Side;

// "add ,1,buy,88.8,100"
// "get, 1"
pub(crate) fn parse_cmd(text: String) -> Result<Command, ParseError> {
    let s = text.trim();
    let vec1: Vec<_> = s.split(',').collect();
    let head = vec1.first().unwrap().to_uppercase();
    match head.trim() {
        "ADD" => add_cmd(vec1),
        "GET" => get_cmd(vec1),
        _ => Err(HeadText),
    }
}
pub(crate) fn add_cmd(body: Vec<&str>) -> Result<Command, ParseError> {
    let id = match body[1].trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => return Err(IdText),
    };
    let side = Side::parse(body[2])?;
    let price = match body[3].parse::<f64>() {
        Ok(price) => price,
        Err(_) => return Err(PriceText),
    };

    let quantity = match body[4].parse::<u64>() {
        Ok(quantity) => quantity,
        Err(_) => return Err(QtyText),
    };
    let rs_order = Order::new(id, side, price, quantity);
    match rs_order {
        Ok(o) => Ok(Command::Add(o)),
        Err(_) => Err(BuildOrderFailed),
    }
}
pub(crate) fn get_cmd(body: Vec<&str>) -> Result<Command, ParseError> {
    match body[1].trim().parse::<u32>() {
        Ok(id) => {
            Ok(Command::Get(id))
        }
        Err(_) => Err(IdText),
    }
}
