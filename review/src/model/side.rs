use crate::error::parse_error::ParseError;
use crate::error::parse_error::ParseError::SideText;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}
impl Side {
    pub fn is_buy(&self) -> bool {
        match self {
            Side::Buy => true,
            Side::Sell => false,
        }
    }

    pub fn is_sell(&self) -> bool {
        match self {
            Side::Buy => false,
            Side::Sell => true,
        }
    }
    pub fn parse(text:&str) -> Result<Self, ParseError> {
        match text.trim().to_uppercase().as_str() {
            "BUY" => Ok(Side::Buy),
            "SELL" => Ok(Side::Sell),
            _  => Err(SideText),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buy_side_should_be_buy() {
        assert!(Side::Buy.is_buy());
        assert!(!Side::Buy.is_sell());
    }

    #[test]
    fn sell_side_should_be_sell() {
        assert!(Side::Sell.is_sell());
        assert!(!Side::Sell.is_buy());
    }
}
