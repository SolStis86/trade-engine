use crate::AlpacaIngestError;

#[derive(Debug, Clone)]
pub struct AlpacaConfig {
    pub api_key: String,
    pub api_secret: String,
    pub data_base_url: String,
    pub default_feed: Option<String>,
}

impl AlpacaConfig {
    pub fn from_env() -> Result<Self, AlpacaIngestError> {
        let api_key = std::env::var("ALPACA_API_KEY_ID")
            .or_else(|_| std::env::var("APCA_API_KEY_ID"))
            .map_err(|_| AlpacaIngestError::MissingConfig("ALPACA_API_KEY_ID or APCA_API_KEY_ID"))?;

        let api_secret = std::env::var("ALPACA_API_SECRET_KEY")
            .or_else(|_| std::env::var("APCA_API_SECRET_KEY"))
            .map_err(|_| AlpacaIngestError::MissingConfig("ALPACA_API_SECRET_KEY or APCA_API_SECRET_KEY"))?;

        let data_base_url = std::env::var("ALPACA_DATA_BASE_URL")
            .unwrap_or_else(|_| "https://data.alpaca.markets".to_string());

        let default_feed = std::env::var("ALPACA_FEED").ok();

        Ok(Self {
            api_key,
            api_secret,
            data_base_url,
            default_feed,
        })
    }
}
