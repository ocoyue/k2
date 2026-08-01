use crate::model::order::Order;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Order),
    Get(u32),
    Cancel(u32),
    Reduce { id: u32, amount: u64 },
    Summary,
}
