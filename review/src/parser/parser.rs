use crate::error::parse_error::ParseError;
use crate::error::parse_error::ParseError::{
    BuildOrderFailed, IdText, NoneHead, PriceText, QtyText, UnknownCommand,
};
use crate::model::command::Command;
use crate::model::order::Order;
use crate::model::side::Side;
use std::str::FromStr;

const ADD_FIELDS: usize = 5;
const GET_FIELDS: usize = 2;
const CANCEL_FIELDS: usize = 2;
const REDUCE_FIELDS: usize = 3;
const SUMMARY_FIELDS: usize = 1;

// "add ,1,buy,88.8,100"
// "get, 1"

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cmd(s)
    }
}
pub(crate) fn parse_cmd(input: &str) -> Result<Command, ParseError> {
    if input.is_empty() {
        return Err(NoneHead);
    }
    let parts: Vec<_> = input.split(',').map(str::trim).collect();
    let head = parts[0].to_ascii_uppercase();

    match head.as_ref() {
        "ADD" => parse_add(&parts),
        "GET" => parse_get(&parts),
        "CANCEL" => parse_cancel(&parts),
        "REDUCE" => parse_reduce(&parts),
        "SUMMARY" => parse_summary(&parts),
        _ => Err(UnknownCommand),
    }
}
pub(crate) fn parse_add(body: &[&str]) -> Result<Command, ParseError> {
    if body.len() != ADD_FIELDS {
        return Err(ParseError::InvalidLength);
    }
    let id = body[1].trim().parse::<u32>().map_err(|_| IdText)?;

    let side = Side::from_str(body[2].trim())?;

    let price = body[3].trim().parse::<f64>().map_err(|_| PriceText)?;

    let quantity = body[4].trim().parse::<u64>().map_err(|_| QtyText)?;

    let rs_order = Order::new(id, side, price, quantity);

    match rs_order {
        Ok(o) => Ok(Command::Add(o)),
        Err(e) => Err(BuildOrderFailed(e)),
    }
}
pub(crate) fn parse_get(body: &[&str]) -> Result<Command, ParseError> {
    if body.len() != GET_FIELDS {
        return Err(ParseError::InvalidLength);
    }
    match body[1].trim().parse::<u32>() {
        Ok(id) => Ok(Command::Get(id)),
        Err(_) => Err(IdText),
    }
}

pub(crate) fn parse_cancel(body: &[&str]) -> Result<Command, ParseError> {
    if body.len() != CANCEL_FIELDS {
        return Err(ParseError::InvalidLength);
    }
    let id = body[1].trim().parse::<u32>().map_err(|_| IdText)?;

    Ok(Command::Cancel(id))
}

pub(crate) fn parse_reduce(body: &[&str]) -> Result<Command, ParseError> {
    // reduce, 1, 33
    if body.len() != REDUCE_FIELDS {
        return Err(ParseError::InvalidLength);
    }
    let id = body[1].trim().parse::<u32>().map_err(|_| IdText)?;
    let qty = body[2].trim().parse::<u64>().map_err(|_| QtyText)?;
    Ok(Command::Reduce { id, amount: qty })
}
pub(crate) fn parse_summary(body: &[&str]) -> Result<Command, ParseError> {
    // summary
    if body.len() != SUMMARY_FIELDS {
        return Err(ParseError::InvalidLength);
    }
    Ok(Command::Summary)
}
