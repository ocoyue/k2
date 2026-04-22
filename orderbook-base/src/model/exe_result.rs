use crate::model::{Order, Summary};
#[derive(Debug, PartialEq)]
pub enum ExeResult {
    Order(Order),
    Added,
    Canceled,
    Reduced,
    Clear,
    Summary(Summary),
}
