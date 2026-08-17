use instrument::InstrumentCatalog;
use protocol::{MarketDataRequest, MarketDataResponse};
use std::sync::Arc;

pub trait MarketDataHandler {
    fn handle(&self, request: MarketDataRequest) -> MarketDataResponse;
}

#[derive(Debug)]
pub struct InstrumentHandler {
    catalog: Arc<InstrumentCatalog>,
}

impl InstrumentHandler {
    pub fn new(catalog: Arc<InstrumentCatalog>) -> Self {
        Self { catalog }
    }
}

impl MarketDataHandler for InstrumentHandler {
    fn handle(&self, request: MarketDataRequest) -> MarketDataResponse {
        match request {
            MarketDataRequest::Instrument { symbol } => match self.catalog.get(&symbol) {
                Some(instrument) => MarketDataResponse::InstrumentInfo {
                    symbol: instrument.symbol().to_owned(),

                    tick_size: instrument.tick_size(),

                    lot_size: instrument.lot_size(),

                    status: instrument.status().as_str().to_owned(),
                },

                None => MarketDataResponse::Error {
                    message: format!("instrument not found: {symbol}"),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InstrumentHandler, MarketDataHandler};
    use instrument::{Instrument, InstrumentCatalog, InstrumentStatus};
    use protocol::{MarketDataRequest, MarketDataResponse};
    use std::sync::Arc;

    #[test]
    fn query_existing_instrument() {
        let catalog = Arc::new(InstrumentCatalog::new(vec![Instrument::new(
            "BTCUSDT",
            1,
            1,
            InstrumentStatus::Active,
        )]));

        let handler = InstrumentHandler::new(catalog);

        let response = handler.handle(MarketDataRequest::Instrument {
            symbol: "BTCUSDT".to_string(),
        });

        assert_eq!(
            response,
            MarketDataResponse::InstrumentInfo {
                symbol: "BTCUSDT".to_string(),
                tick_size: 1,
                lot_size: 1,
                status: "ACTIVE".to_string(),
            }
        );
    }

    #[test]
    fn query_unknown_instrument() {
        let catalog = Arc::new(InstrumentCatalog::new(Vec::new()));

        let handler = InstrumentHandler::new(catalog);

        let response = handler.handle(MarketDataRequest::Instrument {
            symbol: "UNKNOWN".to_string(),
        });

        assert_eq!(
            response,
            MarketDataResponse::Error {
                message: "instrument not found: UNKNOWN".to_string(),
            }
        );
    }
}
