use crate::model::{Order, Summary};
#[derive(Debug, PartialEq,Clone)]
pub enum ExeOk {
    Order(Order),
    Added,
    Canceled,
    Reduced,
    Clear,
    Summary(Summary),
    // Shutdown,
}
