use crate::error::ParseErr;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price {
    ticks: i64,
}
impl Price {
    fn new(ticks: i64) -> Self {
        Self { ticks }
    }
    pub fn from_ticks(ticks: i64) -> Result<Self, ParseErr> {
        if ticks <= 0 {
            Err(ParseErr::InvalidTicks(ticks))
        } else {
            Ok(Self::new(ticks))
        }
    }

    pub fn from_f64(price: f64) -> Result<Self, ParseErr> {
        if price <= 0.00 {
            Err(ParseErr::InvalidPrice(price))
        } else {
            let ticks = (price * 100.00).round() as i64;
            Ok(Self::from_ticks(ticks)?)
        }
    }

    pub fn ticks(&self) -> i64 {
        self.ticks
    }
}
impl Display for Price {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let major = self.ticks / 100;
        let minor = self.ticks % 100;
        write!(f, "{}.{:02}", major, minor)
    }
}
