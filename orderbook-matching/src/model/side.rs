use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy,Eq)]
pub enum Side {
    Buy,
    Sell,
}
impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
