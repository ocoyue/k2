use protocol::{MarketDataRequest, MarketDataResponse};

pub trait MarketDataHandler {
    fn handle(&self, request: MarketDataRequest) -> MarketDataResponse;
}

#[derive(Debug, Default)]
pub struct InstrumentHandler;

impl MarketDataHandler for InstrumentHandler {
    fn handle(&self, request: MarketDataRequest) -> MarketDataResponse {
        match request {
            MarketDataRequest::Instrument { symbol } => MarketDataResponse::InstrumentInfo {
                symbol,
                tick_size: 1,
                lot_size: 1,
                status: "ACTIVE".to_string(),
            },
        }
    }
}
