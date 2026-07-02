use crate::{AlpacaBarsResponse, AlpacaConfig, AlpacaIngestError};
use chrono::{DateTime, Utc};
use market_core::{Candle, Symbol, Timeframe};
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

#[derive(Debug, Clone)]
pub struct AlpacaClient {
    http: reqwest::Client,
    config: AlpacaConfig,
}

impl AlpacaClient {
    pub fn new(config: AlpacaConfig) -> Result<Self, AlpacaIngestError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(&config.api_key)
                .map_err(|err| AlpacaIngestError::InvalidRequest(err.to_string()))?,
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(&config.api_secret)
                .map_err(|err| AlpacaIngestError::InvalidRequest(err.to_string()))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent("trade-engine/alpaca-ingest/0.1.0")
            .build()?;

        Ok(Self { http, config })
    }

    pub async fn fetch_stock_bars(
        &self,
        request: FetchStockBarsRequest,
    ) -> Result<Vec<Candle>, AlpacaIngestError> {
        let symbol = Symbol::new(request.symbol.clone())?;
        let timeframe = Timeframe::new(request.timeframe.clone())?;
        let feed = request.feed.or_else(|| self.config.default_feed.clone());
        let mut page_token: Option<String> = None;
        let mut candles = Vec::new();

        loop {
            let response = self
                .fetch_stock_bars_page(&request, feed.as_deref(), page_token.as_deref())
                .await?;

            for bar in response.bars {
                candles.push(bar.into_candle(symbol.clone(), timeframe.clone(), feed.clone())?);
            }

            page_token = response.next_page_token;

            if page_token.is_none() {
                break;
            }
        }

        candles.sort_by_key(|candle| candle.timestamp);

        Ok(candles)
    }

    async fn fetch_stock_bars_page(
        &self,
        request: &FetchStockBarsRequest,
        feed: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<AlpacaBarsResponse, AlpacaIngestError> {
        let mut url = Url::parse(&self.config.data_base_url)?;
        url.set_path(&format!("/v2/stocks/{}/bars", request.symbol));

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("timeframe", &request.timeframe);
            pairs.append_pair("start", &request.start.to_rfc3339());
            pairs.append_pair("end", &request.end.to_rfc3339());
            pairs.append_pair("limit", &request.limit.to_string());

            if let Some(feed) = feed {
                pairs.append_pair("feed", feed);
            }

            if let Some(adjustment) = &request.adjustment {
                pairs.append_pair("adjustment", adjustment);
            }

            if let Some(page_token) = page_token {
                pairs.append_pair("page_token", page_token);
            }
        }

        tracing::debug!(%url, "fetching Alpaca stock bars page");

        let response = self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<AlpacaBarsResponse>()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Clone)]
pub struct FetchStockBarsRequest {
    pub symbol: String,
    pub timeframe: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub feed: Option<String>,
    pub adjustment: Option<String>,
    pub limit: u32,
}

impl FetchStockBarsRequest {
    pub fn validate(&self) -> Result<(), AlpacaIngestError> {
        if self.symbol.trim().is_empty() {
            return Err(AlpacaIngestError::InvalidRequest("symbol cannot be empty".to_string()));
        }

        if self.end <= self.start {
            return Err(AlpacaIngestError::InvalidRequest("end must be after start".to_string()));
        }

        if self.limit == 0 || self.limit > 10_000 {
            return Err(AlpacaIngestError::InvalidRequest("limit must be between 1 and 10000".to_string()));
        }

        Ok(())
    }
}
