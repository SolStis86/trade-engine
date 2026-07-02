use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(value: impl Into<String>) -> Result<Self, MarketDataError> {
        let value = value.into().trim().to_uppercase();

        if value.is_empty() {
            return Err(MarketDataError::InvalidSymbol("symbol cannot be empty".to_string()));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timeframe(String);

impl Timeframe {
    pub fn new(value: impl Into<String>) -> Result<Self, MarketDataError> {
        let value = value.into();

        match value.as_str() {
            "1Min" | "5Min" | "15Min" | "30Min" | "1Hour" | "4Hour" | "1Day" => Ok(Self(value)),
            _ => Err(MarketDataError::InvalidTimeframe(value)),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Timeframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub id: Uuid,
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub trade_count: Option<u64>,
    pub vwap: Option<f64>,
    pub source: MarketDataSource,
    pub feed: Option<String>,
    pub is_complete: bool,
}

impl Candle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        symbol: Symbol,
        timeframe: Timeframe,
        timestamp: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        trade_count: Option<u64>,
        vwap: Option<f64>,
        source: MarketDataSource,
        feed: Option<String>,
        is_complete: bool,
    ) -> Result<Self, MarketDataError> {
        let candle = Self {
            id: Uuid::new_v4(),
            symbol,
            timeframe,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            trade_count,
            vwap,
            source,
            feed,
            is_complete,
        };

        candle.validate()?;

        Ok(candle)
    }

    pub fn validate(&self) -> Result<(), MarketDataError> {
        if self.open <= 0.0 || self.high <= 0.0 || self.low <= 0.0 || self.close <= 0.0 {
            return Err(MarketDataError::InvalidOhlc("OHLC values must be positive".to_string()));
        }

        if self.volume < 0.0 {
            return Err(MarketDataError::InvalidOhlc("volume cannot be negative".to_string()));
        }

        if self.high < self.open || self.high < self.close || self.high < self.low {
            return Err(MarketDataError::InvalidOhlc("high must be greater than or equal to open, low and close".to_string()));
        }

        if self.low > self.open || self.low > self.close || self.low > self.high {
            return Err(MarketDataError::InvalidOhlc("low must be less than or equal to open, high and close".to_string()));
        }

        Ok(())
    }

    pub fn deterministic_key(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.source.as_str(),
            self.feed.as_deref().unwrap_or("default"),
            self.symbol,
            self.timeframe,
            self.timestamp.to_rfc3339()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketDataSource {
    Alpaca,
}

impl MarketDataSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alpaca => "alpaca",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleBatch {
    pub symbol: Symbol,
    pub timeframe: Timeframe,
    pub candles: Vec<Candle>,
    pub source: MarketDataSource,
    pub feed: Option<String>,
}

#[derive(Debug, Error)]
pub enum MarketDataError {
    #[error("invalid symbol: {0}")]
    InvalidSymbol(String),

    #[error("invalid timeframe: {0}")]
    InvalidTimeframe(String),

    #[error("invalid OHLC candle: {0}")]
    InvalidOhlc(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_valid_candle() {
        let candle = Candle::new(
            Symbol::new("aapl").unwrap(),
            Timeframe::new("1Min").unwrap(),
            Utc::now(),
            100.0,
            101.0,
            99.0,
            100.5,
            10_000.0,
            Some(10),
            Some(100.4),
            MarketDataSource::Alpaca,
            Some("iex".to_string()),
            true,
        );

        assert!(candle.is_ok());
    }

    #[test]
    fn rejects_invalid_high() {
        let candle = Candle::new(
            Symbol::new("AAPL").unwrap(),
            Timeframe::new("1Min").unwrap(),
            Utc::now(),
            100.0,
            99.0,
            98.0,
            100.5,
            10_000.0,
            None,
            None,
            MarketDataSource::Alpaca,
            None,
            true,
        );

        assert!(candle.is_err());
    }
}
