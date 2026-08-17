#[derive(Debug, PartialEq, Eq)]
pub enum MarketDataRequest {
    Instrument { symbol: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarketDataResponse {
    InstrumentInfo {
        symbol: String,
        tick_size: u64,
        lot_size: u64,
        status: String,
    },

    Error {
        message: String,
    },
}

pub struct MarketDataCodec;

impl MarketDataCodec {
    pub fn decode(input: &str) -> Result<MarketDataRequest, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["INSTRUMENT", symbol] => Ok(MarketDataRequest::Instrument {
                symbol: symbol.to_string(),
            }),

            [] => Err("empty market-data request".to_string()),

            _ => Err("unknown or malformed market-data request".to_string()),
        }
    }

    pub fn encode(response: MarketDataResponse) -> String {
        match response {
            MarketDataResponse::InstrumentInfo {
                symbol,
                tick_size,
                lot_size,
                status,
            } => {
                format!(
                    "INSTRUMENT {symbol} \
                     tick_size={tick_size} \
                     lot_size={lot_size} \
                     status={status}\n"
                )
            }

            MarketDataResponse::Error { message } => {
                format!("ERR {message}\n")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MarketDataCodec, MarketDataRequest, MarketDataResponse};

    #[test]
    fn decode_instrument_request() {
        let result = MarketDataCodec::decode("INSTRUMENT BTCUSDT").unwrap();

        assert_eq!(
            result,
            MarketDataRequest::Instrument {
                symbol: "BTCUSDT".to_string(),
            }
        );
    }

    #[test]
    fn reject_unknown_request() {
        let result = MarketDataCodec::decode("HELLO");

        assert_eq!(
            result,
            Err("unknown or malformed market-data request".to_string())
        );
    }

    #[test]
    fn encode_instrument_info() {
        let response = MarketDataResponse::InstrumentInfo {
            symbol: "BTCUSDT".to_string(),
            tick_size: 1,
            lot_size: 1,
            status: "ACTIVE".to_string(),
        };

        let result = MarketDataCodec::encode(response);

        assert_eq!(
            result,
            "INSTRUMENT BTCUSDT \
             tick_size=1 \
             lot_size=1 \
             status=ACTIVE\n"
        );
    }
}
