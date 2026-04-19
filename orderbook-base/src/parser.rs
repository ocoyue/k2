use crate::error::ParseErr;
use crate::model::{Command, ExecuteResult, Order, Side};
use std::str::FromStr;

impl FromStr for Command {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Command, ParseErr> {
        let (cmd, rest) = s
            .split_once(',')
            .map(|(cmd, rest)| (cmd.trim(), rest.trim()))
            .unwrap_or((s.trim(), ""));

        match cmd {
            "ADD" | "add" => parse_add(rest),
            "CANCEL" | "cancel" => parse_cancel(rest),
            "REDUCE" | "reduce" => parse_reduce(rest),
            "GET" | "get" => parse_get(rest),
            "SUMMARY" | "summary" => Ok(Command::SUMMARY),
            _ => Err(ParseErr::InvalidCommand {
                cmd: cmd.to_string(),
            }),
        }
    }
}
impl FromStr for Order {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "1,101,100,buy"
        let vec1: Vec<&str> = s.split(',').map(str::trim).collect();

        if vec1.len() != 4 {
            return Err(ParseErr::InvalidParaCount {
                line: s.to_string(),
            });
        }

        let id = vec1[0]
            .trim()
            .parse::<u32>()
            .map_err(|err| ParseErr::InvalidDigit(err.to_string()))?;

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

pub(crate) fn parse_add(s: &str) -> Result<Command, ParseErr> {
    Ok(Command::ADD(Order::from_str(s)?))
}
pub(crate) fn parse_cancel(s: &str) -> Result<Command, ParseErr> {
    Ok(Command::CANCEL(
        s.parse::<u32>()
            .map_err(|err| ParseErr::Internal(err.to_string()))?,
    ))
}

pub(crate) fn parse_reduce(s: &str) -> Result<Command, ParseErr> {
    let vec1: Vec<&str> = s.split(',').map(|s| s.trim()).collect();
    if vec1.len() != 2 {
        return Err(ParseErr::InvalidParaCount {
            line: s.to_string(),
        });
    };
    let id = vec1[0]
        .parse::<u32>()
        .map_err(|err| ParseErr::InvalidDigit(err.to_string()))?;
    let qty = vec1[1]
        .parse::<u32>()
        .map_err(|err| ParseErr::InvalidDigit(err.to_string()))?;

    if qty == 0 {
        return Err(ParseErr::InvalidQuantity(0));
    };
    Ok(Command::REDUCE { id, qty })
}

pub(crate) fn parse_get(s: &str) -> Result<Command, ParseErr> {
    let order_id = s
        .parse::<u32>()
        .map_err(|err| ParseErr::InvalidDigit(err.to_string()))?;

    Ok(Command::GET(order_id))
}
