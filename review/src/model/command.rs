use std::str::FromStr;
use crate::error::parse_error::ParseError;
use crate::model::order::Order;
use crate::parser::parser::parse_cmd;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Order),
    Get(u32),
    Cancel(u32),
    Reduce { id: u32, qty: u64 },
    Summary,
}
impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cmd(s)
    }
}
