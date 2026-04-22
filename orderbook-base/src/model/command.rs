use crate::model::Order;
#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Order),
    Cancel(u32),
    Reduce { id: u32, qty: u32 },
    Get(u32),
    Summary,
}
