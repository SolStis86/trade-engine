# Technical Architecture: Rust-First Market Data Pipeline

This document defines the technical direction for candle ingestion, feature calculation, simulation and policy scoring.

## Recommendation

Use a Rust-first architecture for the entire deterministic market-data path:

```text
Alpaca ingestion: Rust
Canonical candle types: Rust
Feature calculation: Rust
Backtesting / policy simulation: Rust
ML training and research: Python
API and dashboard: TypeScript / NestJS later
```

The market-data path should be deterministic, strongly typed and testable from the point data enters the system.

## Why Rust for ingestion as well?

Rust should own ingestion because ingestion is not merely API plumbing. It defines the first canonical boundary of the system:

```text
provider payload -> normalized candle -> validated candle -> downstream feature snapshot
```

Keeping this in Rust gives:

```text
one canonical Candle type
one validation layer
one deterministic ingestion pipeline
strong error modelling
better performance for large historical pulls
clean path to Parquet writing
clean path to live rolling-state feature generation
less training/serving skew risk
```

TypeScript can still orchestrate jobs and expose APIs later, but it should not own the core market-data normalization logic.

## Current Rust workspace

```text
crates/
  market-core/
    Core market data types and validation.

  alpaca-ingest/
    Alpaca API client, response models, pagination and normalization.

  trade-engine-cli/
    CLI entrypoint for ingestion and future batch jobs.
```

## Language responsibilities

### Rust

Use Rust for:

```text
Alpaca historical ingest
Alpaca live websocket ingest
canonical candle normalization
candle validation
JSONL / Parquet / Postgres persistence adapters
feature calculation
rolling window state
indicator computation
session-aware VWAP calculation
range and structure detection
candidate policy simulation
backtest loops
cost-adjusted R calculation
policy scoring
walk-forward report generation
```

### Python

Use Python for:

```text
model training
feature importance analysis
probability calibration
notebooks and research
LightGBM / XGBoost / CatBoost experimentation
model serving if needed
```

Python should consume Rust-generated features. It should not redefine canonical features.

### TypeScript / NestJS

Use TypeScript later for:

```text
public API
dashboard API
user/workspace configuration
instrument configuration
job orchestration
model metadata API
websocket fanout to UI
```

TypeScript should call Rust binaries/services rather than duplicate ingestion or feature logic.

## Proposed service boundary

```text
              +---------------------+
              |     Alpaca API      |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Rust Alpaca Ingest  |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Canonical Candles   |
              | JSONL/Postgres/Parq |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Rust Feature Engine |
              +----------+----------+
                         |
          +--------------+--------------+
          |                             |
          v                             v
+---------------------+       +---------------------+
| Feature Snapshots   |       | Backtest Engine     |
+---------------------+       +---------------------+
          |                             |
          +--------------+--------------+
                         v
              +---------------------+
              | Training Dataset    |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Python ML Service   |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Live Policy Output  |
              +---------------------+
```

## Current ingest implementation

The first ingest implementation supports Alpaca historical stock bars.

Command:

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

Current output is normalized JSONL. This will later be extended to Postgres and Parquet sinks.

## Alpaca ingest responsibilities

The Rust ingest layer should:

```text
connect to Alpaca historical market data API
connect to Alpaca real-time websocket streams
fetch historical candles for configured symbols/timeframes
subscribe to live bars/trades/quotes where needed
normalize Alpaca payloads into internal Candle records
validate candle integrity
write JSONL, Parquet and/or Postgres outputs
handle pagination and rate limits
handle reconnects and resubscriptions
record ingestion run metadata
record data quality events
```

## Alpaca data model considerations

Alpaca stock bar payloads include compact fields such as:

```text
t  timestamp
o  open
h  high
l  low
c  close
v  volume
n  trade count
vw volume-weighted price
```

The internal candle model preserves the standard OHLCV fields plus optional provider metadata:

```json
{
  "symbol": "AAPL",
  "timeframe": "1Min",
  "timestamp": "2026-07-02T14:31:00Z",
  "open": 100.12,
  "high": 100.30,
  "low": 100.05,
  "close": 100.22,
  "volume": 18200,
  "trade_count": 94,
  "vwap": 100.18,
  "source": "alpaca",
  "feed": "iex",
  "is_complete": true
}
```

Alpaca's documentation describes the historical and live market-data hosts as `data.alpaca.markets` and `stream.data.alpaca.markets`, and the stock historical bars examples show `t`, `o`, `h`, `l`, `c`, `v`, `n`, `vw` plus `next_page_token`. This is the response shape used by the Rust normalization layer.

## Storage recommendation

Use two storage forms:

### 1. Operational store

Use Postgres for:

```text
recent candles
feature snapshots
live decisions
backtest metadata
model metadata
user/watchlist configuration
```

### 2. Research/backtest store

Use Parquet for:

```text
large historical candle archives
feature matrices
training datasets
backtest result exports
```

The current JSONL output is a bootstrap format that keeps the first ingest implementation simple.

## Rust integration options

### CLI first

Start with Rust binaries for ingestion, feature generation and backtesting.

Pros:

```text
simple process boundary
easy to test
easy to run in CI
good for batch jobs
low integration complexity
```

### Rust library later

Expose Rust functions to other runtimes via N-API, FFI or service calls if required.

### Rust microservice later

Run ingestion/feature/backtest as dedicated services if workloads require independent scaling.

## Determinism requirements

Given the same:

```text
provider payloads
config
normalization version
feature schema version
```

The system must produce the same:

```text
canonical candles
feature snapshots
backtest results
policy scores
```

## First technical milestone

```text
Rust Alpaca historical ingest
  -> normalized Candle records
  -> JSONL output
  -> Parquet writer
  -> Postgres persistence
  -> Rust feature generation
```

## Next technical steps

```text
1. Add Parquet writer for candles.
2. Add Postgres persistence adapter.
3. Add ingestion run metadata.
4. Add data quality event detection.
5. Add multi-symbol historical batching.
6. Add retry/backoff classification.
7. Add live websocket ingestion.
8. Connect completed candles to Rust feature generation.
```

## Summary

The market-data path is now Rust-first.

```text
Alpaca -> Rust ingest -> canonical candles -> Rust features -> Rust backtests -> Python ML
```

This gives the project a clean deterministic core while leaving Python and TypeScript for the places where they are strongest.
