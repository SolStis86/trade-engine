use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlpacaIngestError {
    #[error("missing required config value: {0}")]
    MissingConfig(&'static str),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("url error: {0}")]
    Url(#[from] url::ParseError),

    #[error("market data normalization error: {0}")]
    MarketData(#[from] market_core::MarketDataError),
}
