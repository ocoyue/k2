use crate::error::ParseErr;
use crate::model::{Command, Order, Side};
use std::str::FromStr;

pub fn parse_str(s: &str) -> Result<Command, ParseErr> {
    let (cmd, rest) = s.split_once(',').ok_or_else(|| ParseErr::InvalidLine {
        line: s.to_string(),
    })?;
    match cmd {
        "ADD" | "add" => Ok(Command::ADD(Order::from_str(rest)?)),

        _ => Err(ParseErr::InvalidCommand {
            cmd: cmd.to_string(),
        }),
    }
}
impl FromStr for Order {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "1,101,100,buy"
        let vec1: Vec<&str> = s.split(',').map(str::trim).collect();
        let id = vec1[0]
            .trim()
            .parse::<u32>()
            .map_err(|err| ParseErr::InvalidOrder {
                reason: err.to_string(),
            })?;

        let price = vec1[1]
            .trim()
            .parse::<f64>()
            .map_err(|err| ParseErr::InvalidOrder {
                reason: err.to_string(),
            })?;

        let qty: u32 = vec1[2]
            .trim()
            .parse::<u32>()
            .map_err(|err| ParseErr::InvalidOrder {
                reason: err.to_string(),
            })?;

        Order::new(id, price, qty, Side::from_str(vec1[3])?)
    }
}
impl FromStr for Side {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "BUY" | "buy" => Ok(Side::BUY),
            "SELL" | "sell" => Ok(Side::SELL),
            _ => Err(ParseErr::InvalidSide {
                side: s.to_string(),
            }),
        }
    }
}
