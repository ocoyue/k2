use crate::model::order::Order;

#[derive(Debug,PartialEq)]
pub enum Command {
    Add(Order),
    Get(u32),
}