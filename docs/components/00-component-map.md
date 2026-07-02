# Component Map

This document maps the main components of the trade engine and explains how data moves through the system.

## Component overview

```mermaid
flowchart TD
    A[Alpaca Market Data API] --> B[Rust Ingestion Worker]
    B --> C[Raw Payload Audit]
    B --> D[Canonical Candle Store]
    D --> E[Rust Feature Engine]
    E --> F[Feature Snapshot Store]
    F --> G[Regime Classifier]
    F --> H[Policy Engine]
    G --> H
    H --> I[Backtest Engine]
    I --> J[Policy Simulation Results]
    J --> K[Labelling Engine]
    F --> K
    K --> L[Training Dataset]
    L --> M[ML Training Service]
    M --> N[Model Registry]
    F --> O[Live Inference Engine]
    N --> O
    G --> O
    O --> P[Risk Engine]
    P --> Q[Trade Plan Builder]
    Q --> R[Live Policy Decision Store]
    R --> S[Outcome Tracker]
    S --> L
    R --> T[Dashboard / API]
    J --> T
    F --> T
```

## Primary components

| Component | Runtime | Purpose |
| --- | --- | --- |
| Ingestion Worker | Rust | Pull historical and live market data from Alpaca and normalize it into canonical candles. |
| Canonical Candle Store | Postgres + Parquet | Store clean OHLCV candles and provider metadata. |
| Feature Engine | Rust | Generate deterministic market-state features from candle data. |
| Regime Classifier | Rust initially | Classify trend, range, volatility, liquidity and session states. |
| Policy Engine | Rust | Generate valid candidate trade behaviours. |
| Backtest Engine | Rust | Simulate candidate policies across historical data. |
| Labelling Engine | Rust / Python boundary | Convert simulation outcomes into supervised training labels. |
| ML Training Service | Python | Train, evaluate and calibrate models. |
| Model Registry | TypeScript + DB / MLflow later | Track model versions, feature schemas and promotion state. |
| Live Inference Engine | Rust + Python/ML service | Score current feature snapshots and recommend policy behaviour. |
| Risk Engine | Rust | Apply deterministic vetoes and hard risk controls. |
| Trade Plan Builder | Rust initially, TypeScript presentation later | Convert selected policy into a structured trade plan. |
| Outcome Tracker | Rust worker initially | Track whether live recommendations won, lost, waited or expired. |
| API / Dashboard | TypeScript later | Expose reports, decisions, features and model performance. |

## Batch research flow

```mermaid
sequenceDiagram
    participant User
    participant CLI as Rust CLI
    participant Ingest as Rust Ingestion Worker
    participant Store as Candle Store
    participant Feature as Rust Feature Engine
    participant Policy as Policy Engine
    participant Backtest as Backtest Engine
    participant Label as Labelling Engine
    participant ML as ML Training Service

    User->>CLI: Request historical profile for symbol/timeframe
    CLI->>Ingest: Fetch historical candles from Alpaca
    Ingest->>Store: Persist canonical candles
    CLI->>Feature: Generate feature snapshots
    Feature->>Store: Read candles
    Feature-->>CLI: Write feature snapshots
    CLI->>Policy: Generate candidate policies
    Policy-->>Backtest: Candidate policies
    Backtest->>Store: Read candles and features
    Backtest-->>CLI: Persist simulation results
    CLI->>Label: Generate best-policy labels
    Label-->>CLI: Persist training examples
    CLI->>ML: Train and validate model
    ML-->>CLI: Model metrics and artifacts
```

## Live flow

```mermaid
sequenceDiagram
    participant Alpaca
    participant Ingest as Rust Live Ingestion Worker
    participant Feature as Rust Feature Engine
    participant Regime as Regime Classifier
    participant Inference as Live Inference Engine
    participant Risk as Risk Engine
    participant Plan as Trade Plan Builder
    participant Store as Decision Store
    participant UI as Dashboard

    Alpaca-->>Ingest: Completed bar / live candle
    Ingest->>Feature: Normalized completed candle
    Feature-->>Regime: Feature snapshot
    Regime-->>Inference: Regime labels and scores
    Inference->>Inference: Score policy candidates
    Inference-->>Risk: Best candidate + confidence
    Risk-->>Plan: Approved, wait, or vetoed
    Plan-->>Store: Persist structured decision
    Store-->>UI: Display recommendation
```

## Component dependency direction

The architecture should avoid circular dependency between services and packages.

Recommended dependency direction:

```mermaid
flowchart LR
    A[market-core] --> B[alpaca-ingest]
    A --> C[feature-engine]
    A --> D[policy-engine]
    A --> E[backtest-engine]
    C --> E
    D --> E
    C --> F[regime-classifier]
    C --> G[risk-engine]
    D --> G
    E --> H[labelling-engine]
    H --> I[ml-training]
    I --> J[model-registry]
```

Rules:

```text
market-core should not depend on any higher-level engine.
alpaca-ingest should normalize provider data into market-core types.
feature-engine should not depend on ML models.
policy-engine should not depend on backtest results.
risk-engine should be deterministic and independently testable.
ML training should consume generated features and labels, not redefine them.
```

## Shared contracts

The most important shared contracts are:

```text
Candle
SpreadSnapshot
FeatureSnapshot
RegimeSnapshot
CandidatePolicy
PolicySimulationResult
TrainingExample
ModelRun
LivePolicyDecision
TradePlan
OutcomeResolution
```

These contracts should be versioned as schemas before implementation begins.

## Documentation index

| Document | Component |
| --- | --- |
| [`01-ingestion-worker.md`](01-ingestion-worker.md) | Alpaca historical and live data ingestion. |
| [`02-candle-store.md`](02-candle-store.md) | Canonical candle storage and raw audit. |
| [`03-rust-feature-engine.md`](03-rust-feature-engine.md) | Deterministic feature calculation. |
| [`04-regime-classifier.md`](04-regime-classifier.md) | Market regime classification. |
| [`05-policy-engine.md`](05-policy-engine.md) | Candidate policy generation. |
| [`06-backtest-engine.md`](06-backtest-engine.md) | Historical simulation and scoring. |
| [`07-labelling-engine.md`](07-labelling-engine.md) | Best-policy and no-trade labelling. |
| [`08-ml-training-service.md`](08-ml-training-service.md) | Model training and evaluation. |
| [`09-live-inference-and-risk.md`](09-live-inference-and-risk.md) | Live prediction, risk gate and trade plan flow. |
| [`10-api-dashboard.md`](10-api-dashboard.md) | API and dashboard surfaces. |
