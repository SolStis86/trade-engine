# Rust Alpaca Ingest Quickstart

This document describes the first Rust implementation milestone for ingesting Alpaca historical stock bars.

## Current scope

The first ingest implementation supports:

```text
Rust workspace
market-core crate
alpaca-ingest crate
trade-engine CLI
historical Alpaca stock bars
single-symbol requests
pagination via next_page_token
normalization into canonical Candle records
JSONL output
basic candle validation
```

Out of scope for this first cut:

```text
Postgres persistence
Parquet output
live websocket ingestion
multi-symbol batching
rate-limit backoff policy
crypto/forex/options data
raw payload audit storage
```

## Workspace crates

```text
crates/
  market-core/
    Core market data types and candle validation.

  alpaca-ingest/
    Alpaca API config, response models, historical bars client and normalization.

  trade-engine-cli/
    Command-line interface for invoking ingestion jobs.
```

## Environment

Copy `.env.example` and populate the required values:

```text
ALPACA_API_KEY_ID=
ALPACA_API_SECRET_KEY=
ALPACA_DATA_BASE_URL=https://data.alpaca.markets
ALPACA_FEED=iex
```

The Rust config loader also accepts Alpaca's `APCA_*` names:

```text
APCA_API_KEY_ID
APCA_API_SECRET_KEY
```

## Historical stock bars command

Run:

```bash
cargo run -p trade-engine-cli -- \
  ingest historical-alpaca-stock-bars \
  --symbol AAPL \
  --timeframe 1Min \
  --start 2025-01-01T00:00:00Z \
  --end 2025-01-02T00:00:00Z \
  --feed iex \
  --output data/aapl-1min.jsonl
```

To write to stdout, omit `--output`:

```bash
cargo run -p trade-engine-cli -- \
  ingest historical-alpaca-stock-bars \
  --symbol AAPL \
  --timeframe 1Day \
  --start 2025-01-01T00:00:00Z \
  --end 2025-02-01T00:00:00Z
```

## Output format

The first output format is newline-delimited JSON.

Example:

```json
{"id":"...","symbol":"AAPL","timeframe":"1Min","timestamp":"2026-07-02T14:31:00Z","open":100.12,"high":100.3,"low":100.05,"close":100.22,"volume":18200.0,"trade_count":94,"vwap":100.18,"source":"Alpaca","feed":"iex","is_complete":true}
```

This is deliberately simple so the next stage can pipe data into:

```text
Postgres persistence
Parquet writer
Rust feature engine
```

## Canonical candle validation

The `market-core` crate validates:

```text
OHLC values are positive
volume is non-negative
high >= open/high/close/low relationship is valid
low <= open/high/close relationship is valid
symbol is non-empty
timeframe is supported
```

Supported initial timeframes:

```text
1Min
5Min
15Min
30Min
1Hour
4Hour
1Day
```

## Alpaca response mapping

Alpaca bar fields are mapped as:

```text
t  -> timestamp
o  -> open
h  -> high
l  -> low
c  -> close
v  -> volume
n  -> trade_count
vw -> vwap
```

The normalized candle also stores:

```text
source = Alpaca
feed = iex/sip/etc
is_complete = true
```

## Next implementation steps

1. Add JSONL reader/writer utilities.
2. Add Parquet output from ingest.
3. Add Postgres persistence.
4. Add multi-symbol historical ingestion.
5. Add retry/backoff and provider error classification.
6. Add ingestion run metadata.
7. Add data quality event detection.
8. Add live websocket ingestion.
9. Connect completed candles to feature generation.

## Design decision

Ingestion is now Rust-first.

Earlier documentation suggested TypeScript/NestJS ingestion. The revised architecture is:

```text
Alpaca ingest: Rust
Canonical candle types: Rust
Feature calculation: Rust
Backtesting: Rust
Policy simulation: Rust
ML training: Python
API/dashboard: TypeScript later
```

This keeps the entire market-data pipeline deterministic and strongly typed from the point of ingestion.
