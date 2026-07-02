# Technical Architecture: Alpaca Ingest and Rust Compute Core

This document defines the proposed technical direction for candle ingestion, feature calculation, simulation and policy scoring.

## Recommendation

Use a split architecture:

```text
Alpaca API integration / orchestration: TypeScript or Python
Feature calculation / backtesting / policy simulation: Rust
ML training and experiment work: Python
API and product surface: TypeScript / NestJS
```

The most important technical decision is to keep the high-throughput, deterministic market-data processing separate from API orchestration and model training.

## Why Rust for the compute core?

Rust is the recommended low-level language for the feature engine, backtest engine and policy simulation engine.

Compared with C, Rust gives:

```text
memory safety without garbage collection
excellent performance
strong type system
good package management through Cargo
safe concurrency primitives
excellent testing support
straightforward FFI options
WebAssembly option for browser/edge execution later
better maintainability for a growing codebase
```

C is appropriate for extremely small, performance-critical kernels, but this project is likely to benefit more from Rust's safety and maintainability than from the marginal extra control offered by C.

C++ is also viable, especially for quantitative libraries and high-performance simulation, but it has greater complexity and more footguns than Rust.

## Language responsibilities

### TypeScript / NestJS

Use TypeScript for:

```text
public API
user/workspace configuration
instrument configuration
Alpaca credential management
job orchestration
backtest request coordination
model metadata API
dashboard API
websocket fanout to UI
```

TypeScript is also suitable for simple ingestion workers if the ingestion workload is mostly I/O-bound.

### Rust

Use Rust for:

```text
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

Rust should own deterministic market-state transformations.

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

Python should not own the canonical feature definitions if the production system relies on Rust. If Python needs features for training, it should consume Rust-generated feature snapshots or call the same Rust core.

## Proposed service boundary

```text
              +---------------------+
              |     Alpaca API      |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Ingestion Worker    |
              | TypeScript/Python   |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Candle Store        |
              | Postgres/Parquet    |
              +----------+----------+
                         |
                         v
              +---------------------+
              | Rust Compute Core   |
              | features/backtests  |
              +----------+----------+
                         |
          +--------------+--------------+
          |                             |
          v                             v
+---------------------+       +---------------------+
| Feature Snapshots   |       | Simulation Results  |
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

## Alpaca ingest responsibilities

The ingestion layer should be deliberately simple.

Responsibilities:

```text
connect to Alpaca historical market data API
connect to Alpaca real-time websocket streams
fetch historical candles for configured symbols/timeframes
subscribe to live bars/trades/quotes where needed
normalize Alpaca payloads into internal Candle records
persist raw payload audit records where useful
persist normalized candle records
handle pagination and rate limits
handle reconnects and resubscriptions
record ingestion run metadata
```

The ingestion layer should not calculate complex features. Its job is to collect, normalize and persist clean market data.

## Alpaca data model considerations

Alpaca historical stock bar responses include fields such as:

```text
t timestamp
o open
h high
l low
c close
v volume
n trade count
vw volume-weighted price
```

The internal candle model should preserve both the standard OHLCV fields and provider-specific metadata where available.

Suggested normalized candle shape:

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
  "raw": {}
}
```

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

Use Parquet files for:

```text
large historical candle archives
feature matrices
training datasets
backtest result exports
```

Rust can process Parquet efficiently through Apache Arrow/DataFusion/Polars-style workflows.

## Rust compute-core options

There are three viable ways to integrate Rust.

## Option A: Rust CLI worker

The TypeScript worker invokes Rust binaries for batch jobs.

Example:

```text
trade-engine-features generate \
  --symbol GBPJPY \
  --timeframe 5m \
  --from 2025-01-01 \
  --to 2026-01-01 \
  --input candles.parquet \
  --output features.parquet
```

Pros:

```text
simple process boundary
easy to test
easy to run in CI
good for batch jobs
low integration complexity
```

Cons:

```text
process startup overhead
less elegant for low-latency live inference
needs file or IPC handoff
```

Recommended for the first implementation.

## Option B: Rust library called from Node

Expose Rust functions to Node via N-API.

Pros:

```text
fast direct calls from TypeScript
no separate process required
good for live feature calculation
```

Cons:

```text
more complex build/release process
native module packaging complexity
harder deployment than CLI
```

Recommended after the Rust core stabilizes.

## Option C: Rust microservice

Run the Rust compute engine as its own service over HTTP or gRPC.

Pros:

```text
language isolation
scales independently
clear deployment boundary
good for live and batch workloads
```

Cons:

```text
network overhead
service orchestration complexity
more operational moving parts
```

Recommended if feature computation and simulation workloads become heavy enough to justify independent scaling.

## Initial recommendation

Start with:

```text
Rust CLI for batch feature generation and backtesting
TypeScript/NestJS for orchestration
Postgres + Parquet for storage
Python for ML training
```

Then evolve to:

```text
Rust library or Rust microservice for live feature calculation
```

## Proposed Rust crates

Suggested crate layout:

```text
crates/
  market-core/
  feature-engine/
  policy-engine/
  backtest-engine/
  risk-engine/
  cli/
```

### `market-core`

Contains core domain types:

```text
Candle
Symbol
Timeframe
Session
SpreadSnapshot
FeatureSnapshot
RollingWindow
```

### `feature-engine`

Contains deterministic feature calculations:

```text
raw candle features
rolling returns
ATR
EMA
RSI
VWAP
range compression
swing structure
session features
spread/cost features
composite scores
```

### `policy-engine`

Contains candidate trade policy logic:

```text
policy vocabulary
candidate generation
entry rules
stop rules
target rules
management rules
```

### `backtest-engine`

Contains simulation logic:

```text
forward candle simulation
entry execution
stop/target handling
partial exits
same-candle ambiguity handling
R multiple calculation
policy scoring
walk-forward utilities
```

### `risk-engine`

Contains deterministic trade filters:

```text
spread gates
volatility gates
session gates
risk/reward gates
confidence gates
sample-size gates
```

### `cli`

Exposes commands:

```text
features generate
backtest run
labels generate
reports build
```

## Rust data pipeline

Batch feature generation:

```text
1. Load candles from Parquet or database extract.
2. Sort by symbol, timeframe and timestamp.
3. Validate no duplicates.
4. Validate missing candle gaps.
5. Run rolling feature calculators.
6. Write FeatureSnapshot output to Parquet and/or Postgres.
```

Live feature generation:

```text
1. Maintain rolling state per symbol/timeframe.
2. Receive completed live candle.
3. Update rolling state.
4. Produce feature snapshot.
5. Pass snapshot to policy selector / ML service.
6. Store snapshot and decision.
```

## Stateful rolling calculation

Feature calculation should not repeatedly scan entire histories for each candle.

Use stateful rolling structures:

```text
RollingWindow<T>
RollingMean
RollingStdDev
RollingHighLow
RollingATR
RollingEMA
RollingVWAP
RollingPercentile
SwingState
SessionState
```

This enables efficient live processing.

## Determinism requirements

The Rust feature engine must be deterministic.

Given:

```text
same input candles
same feature schema version
same config
```

It must produce:

```text
same feature snapshots
same backtest results
same policy scores
```

Avoid hidden global state, random seeds without explicit configuration, and clock-dependent logic.

## Time and session handling

All internal candle timestamps should be stored in UTC.

Session logic should be explicit and configurable:

```text
Asia
London
New York
London/New York overlap
Rollover
Out of hours
```

Session definitions should be versioned because changing them changes historical features.

## Feature schema versioning

Every feature snapshot should include:

```text
feature_schema_version
calculation_config_hash
source_data_version
created_at
```

Example:

```json
{
  "symbol": "GBPJPY",
  "timeframe": "5m",
  "timestamp": "2026-07-02T09:35:00Z",
  "feature_schema_version": "v1.0.0",
  "calculation_config_hash": "sha256:...",
  "features": {
    "atr_14": 0.21,
    "atr_percentile_100": 0.72,
    "distance_to_vwap_atr": 0.84
  }
}
```

## Recommended initial commands

```text
trade-engine ingest historical --source alpaca --symbol AAPL --timeframe 1Min --from 2025-01-01 --to 2025-12-31

trade-engine features generate --symbol AAPL --timeframe 1Min --schema v1.0.0

trade-engine backtest run --symbol AAPL --timeframe 1Min --policy-schema v1.0.0

trade-engine labels generate --symbol AAPL --timeframe 1Min --label-schema v1.0.0

trade-engine report instrument-profile --symbol AAPL --timeframe 1Min
```

## Practical first build

The first technical milestone should be:

```text
Alpaca historical ingest -> normalized candles -> Rust feature generation -> Parquet feature snapshots
```

Do not start with live trading. The initial target is a reliable research and labelling pipeline.

## Suggested first milestone scope

```text
1. TypeScript Alpaca historical bars importer
2. Normalized Candle schema
3. Candle persistence to Postgres and/or Parquet
4. Rust market-core crate
5. Rust feature-engine crate
6. Rust CLI command for feature generation
7. FeatureSnapshot v1.0.0 output
8. Unit tests for ATR, EMA, RSI, VWAP and range features
```

## Rust versus C decision

### Choose Rust if the goal is:

```text
production-grade maintainable compute core
safe concurrency
reliable feature calculation
fast backtests
clear domain modelling
team-friendly codebase
```

### Choose C if the goal is:

```text
ultra-minimal kernels
embedded-style execution
manual control over every allocation
very narrow performance-critical functions
```

For this project, Rust is the stronger default.

## Rust versus C++ decision

### Rust advantages

```text
memory safety
better modern package management
cleaner concurrency model
less legacy complexity
strong enums and pattern matching
```

### C++ advantages

```text
larger quantitative finance ecosystem
more existing numerical libraries
more mature low-latency trading examples
```

For this project, Rust is preferable unless the system later needs to integrate heavily with existing C++ quant libraries.

## Python boundary warning

Python is excellent for research and ML training, but it should not become the source of truth for production feature calculation unless performance and determinism are carefully controlled.

Preferred pattern:

```text
Rust generates canonical features.
Python trains on Rust-generated feature matrices.
Live inference uses the same Rust-generated features.
```

This avoids training/serving skew.

## Summary

Recommended stack:

```text
Alpaca ingestion: TypeScript/NestJS worker
Canonical candle store: Postgres + Parquet
Feature calculation: Rust
Backtesting: Rust
Policy simulation: Rust
ML training: Python
API/dashboard: TypeScript
```

This gives the project a practical balance of speed, reliability, explainability and implementation velocity.
