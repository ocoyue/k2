use crate::Instrument;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InstrumentCatalog {
    instruments: HashMap<String, Instrument>,
}

impl InstrumentCatalog {
    pub fn new(instruments: Vec<Instrument>) -> Self {
        let instruments = instruments
            .into_iter()
            .map(|instrument| {
                let symbol = instrument.symbol().to_owned();
                (symbol, instrument)
            })
            .collect();

        Self { instruments }
    }

    pub fn get(&self, symbol: &str) -> Option<&Instrument> {
        self.instruments.get(symbol)
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::InstrumentCatalog;
    use crate::{Instrument, InstrumentStatus};

    #[test]
    fn find_instrument_by_symbol() {
        let catalog = InstrumentCatalog::new(vec![Instrument::new(
            "BTCUSDT",
            1,
            1,
            InstrumentStatus::Active,
        )]);

        let instrument = catalog.get("BTCUSDT").unwrap();

        assert_eq!(instrument.symbol(), "BTCUSDT");
        assert_eq!(instrument.tick_size(), 1);
        assert_eq!(instrument.lot_size(), 1);
        assert_eq!(instrument.status(), InstrumentStatus::Active);
    }

    #[test]
    fn unknown_instrument_returns_none() {
        let catalog = InstrumentCatalog::new(Vec::new());

        assert!(catalog.get("UNKNOWN").is_none());
    }
}
