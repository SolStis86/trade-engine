use chrono::{DateTime, Utc};
use market_core::{Candle, MarketDataSource, Symbol, Timeframe};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlpacaBarsResponse {
    pub bars: Vec<AlpacaBar>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlpacaBar {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "o")]
    pub open: f64,

    #[serde(rename = "h")]
    pub high: f64,

    #[serde(rename = "l")]
    pub low: f64,

    #[serde(rename = "c")]
    pub close: f64,

    #[serde(rename = "v")]
    pub volume: f64,

    #[serde(rename = "n")]
    pub trade_count: Option<u64>,

    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}

impl AlpacaBar {
    pub fn into_candle(
        self,
        symbol: Symbol,
        timeframe: Timeframe,
        feed: Option<String>,
    ) -> Result<Candle, market_core::MarketDataError> {
        Candle::new(
            symbol,
            timeframe,
            self.timestamp,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.trade_count,
            self.vwap,
            MarketDataSource::Alpaca,
            feed,
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_alpaca_bar() {
        let json = r#"
        {
          "t": "2023-09-29T04:00:00Z",
          "o": 172.02,
          "h": 173.07,
          "l": 170.341,
          "c": 171.21,
          "v": 51861083,
          "n": 535134,
          "vw": 171.599691
        }
        "#;

        let bar: AlpacaBar = serde_json::from_str(json).unwrap();

        assert_eq!(bar.open, 172.02);
        assert_eq!(bar.trade_count, Some(535134));
        assert_eq!(bar.vwap, Some(171.599691));
    }

    #[test]
    fn normalizes_to_candle() {
        let bar = AlpacaBar {
            timestamp: "2023-09-29T04:00:00Z".parse().unwrap(),
            open: 172.02,
            high: 173.07,
            low: 170.341,
            close: 171.21,
            volume: 51861083.0,
            trade_count: Some(535134),
            vwap: Some(171.599691),
        };

        let candle = bar
            .into_candle(
                Symbol::new("AAPL").unwrap(),
                Timeframe::new("1Day").unwrap(),
                Some("sip".to_string()),
            )
            .unwrap();

        assert_eq!(candle.symbol.as_str(), "AAPL");
        assert_eq!(candle.timeframe.as_str(), "1Day");
        assert_eq!(candle.source.as_str(), "alpaca");
    }
}
