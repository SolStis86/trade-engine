# Implementation Roadmap

This document proposes a practical build sequence for the trade engine.

The priority is to build a reliable deterministic foundation before introducing machine learning.

## Proposed repository structure

```text
trade-engine/
  README.md
  docs/
    01-system-overview.md
    02-feature-catalogue.md
    03-initial-feature-selection.md
    04-policy-labelling-and-training.md
    05-implementation-roadmap.md

  apps/
    api/
    worker/
    ml-service/
    dashboard/

  packages/
    market-core/
    feature-engine/
    policy-engine/
    backtest-engine/
    risk-engine/
    model-registry/
    shared-types/
```

## Suggested services

### `apps/api`

Responsibilities:

```text
instrument management
historical data query API
feature snapshot API
policy recommendation API
backtest report API
model run metadata API
live decision API
```

Suggested technology:

```text
NestJS
Postgres
Prisma
OpenAPI
```

### `apps/worker`

Responsibilities:

```text
historical candle ingestion
live candle ingestion
feature generation jobs
policy simulation jobs
backtest jobs
training dataset generation
live outcome tracking
```

Suggested technology:

```text
Node/NestJS worker
BullMQ
Redis
Postgres
```

### `apps/ml-service`

Responsibilities:

```text
train models
serve predictions
explain feature importance
calibrate probabilities
track experiments
export model versions
```

Suggested technology:

```text
Python
FastAPI
LightGBM / XGBoost / CatBoost
scikit-learn
MLflow or simple model registry initially
```

### `apps/dashboard`

Responsibilities:

```text
instrument profiles
feature-state viewer
regime viewer
policy leaderboard
backtest reports
live recommendation view
model performance view
```

Suggested technology:

```text
React or Vue
TypeScript
```

## Suggested packages

### `packages/market-core`

Domain types:

```text
Candle
Instrument
Timeframe
Session
SpreadSnapshot
MarketDataSource
```

Responsibilities:

```text
timeframe utilities
session utilities
candle validation
market calendar helpers
instrument metadata
```

### `packages/feature-engine`

Responsibilities:

```text
raw candle features
rolling features
ATR/volatility features
VWAP features
EMA/trend features
range/breakout features
swing structure features
session features
spread/cost features
higher-timeframe features
composite scores
```

Output:

```text
FeatureSnapshot
```

### `packages/policy-engine`

Responsibilities:

```text
policy vocabulary
candidate policy generation
entry behaviour definitions
stop behaviour definitions
target behaviour definitions
management behaviour definitions
policy constraints
```

Output:

```text
CandidatePolicy[]
```

### `packages/backtest-engine`

Responsibilities:

```text
historical simulation
entry/stop/target execution
partial-exit simulation
cost model
slippage model
policy outcome scoring
walk-forward validation helpers
```

Output:

```text
PolicySimulationResult
BacktestReport
TrainingExample
```

### `packages/risk-engine`

Responsibilities:

```text
spread gates
volatility gates
session gates
news/event gates
risk/reward gates
confidence gates
sample-size gates
```

Output:

```text
RiskDecision
```

### `packages/model-registry`

Responsibilities:

```text
model metadata
feature schema version tracking
label schema version tracking
training run metadata
model promotion status
model inference configuration
```

### `packages/shared-types`

Responsibilities:

```text
shared TypeScript types
schemas
DTOs
enums
validation helpers
```

## Core database entities

### Candle

```text
id
symbol
timeframe
timestamp
open
high
low
close
volume
source
created_at
```

### SpreadSnapshot

```text
id
symbol
timestamp
bid
ask
spread
source
created_at
```

### FeatureSnapshot

```text
id
symbol
timeframe
timestamp
feature_schema_version
features_json
created_at
```

### CandidatePolicy

```text
id
policy_schema_version
decision
direction
entry_type
stop_type
target_type
management
params_json
```

### PolicySimulationResult

```text
id
feature_snapshot_id
candidate_policy_id
entry_price
stop_price
target_prices_json
outcome
r_multiple
max_adverse_excursion_r
max_favourable_excursion_r
duration_bars
cost_r
score
created_at
```

### TrainingExample

```text
id
feature_snapshot_id
label_schema_version
best_policy_id
expected_r
win_probability
stopout_probability
no_trade_probability
sample_size
created_at
```

### ModelRun

```text
id
model_name
model_version
feature_schema_version
label_schema_version
training_start
training_end
validation_start
validation_end
metrics_json
artifact_uri
status
created_at
```

### LivePolicyDecision

```text
id
symbol
timeframe
timestamp
feature_snapshot_id
model_run_id
selected_policy_json
confidence
expected_r
win_probability
risk_flags_json
final_decision
actual_outcome
actual_r
created_at
resolved_at
```

## Build phases

## Phase 0: Documentation and contracts

Deliverables:

```text
feature schema v1.0.0 draft
policy vocabulary v1.0.0
label schema v1.0.0
cost model assumptions
initial database schema
```

Success criteria:

```text
developers can implement feature snapshots without ambiguity
candidate policy structure is fixed enough for simulation
```

## Phase 1: Candle ingestion and storage

Deliverables:

```text
historical candle import
candle validation
symbol/timeframe storage
basic query API
```

Initial instruments:

```text
GBPJPY
XAUUSD
BTCUSD or SOLUSD
```

Initial timeframes:

```text
1m
5m
15m
```

Success criteria:

```text
historical candles can be imported and queried consistently
missing candles and duplicate candles are detected
```

## Phase 2: Feature engine v1.0.0

Deliverables:

```text
raw candle features
returns and momentum
ATR features
VWAP features
EMA trend features
range compression features
session features
spread features
recent swing features
higher-timeframe features
composite scores
```

Success criteria:

```text
each candle can produce a deterministic FeatureSnapshot
features use only data available at the snapshot timestamp
feature calculations are unit tested
```

## Phase 3: Deterministic policy engine

Deliverables:

```text
policy vocabulary
candidate generation
entry calculators
stop calculators
target calculators
management calculators
```

Initial policies:

```text
long immediate
short immediate
long pullback confirmation
short pullback confirmation
long breakout confirmation
short breakout confirmation
long reversal confirmation
short reversal confirmation
no_trade
wait
```

Success criteria:

```text
for a feature snapshot, the engine can generate valid candidate policies
invalid policies are filtered before simulation
```

## Phase 4: Backtest and simulation engine

Deliverables:

```text
forward candle simulation
entry/stop/target checking
partial-exit handling
cost-adjusted R calculation
policy outcome scoring
backtest reports
```

Success criteria:

```text
candidate policies can be tested across historical data
results include win rate, expectancy, profit factor and drawdown
```

## Phase 5: Labelling engine

Deliverables:

```text
training example generator
best policy selection
no-trade labelling
label schema versioning
sample-size thresholds
```

Success criteria:

```text
historical features can be converted into labelled training examples
future-derived values are excluded from live feature inputs
```

## Phase 6: Baseline reports

Deliverables:

```text
instrument profile reports
policy leaderboard
regime performance report
session performance report
feature distribution report
```

Example report output:

```text
Instrument: GBPJPY
Timeframe: 5m
Best behaviour: pullback continuation
Best session: London
Win rate: 56.8%
Expectancy: +0.28R
Profit factor: 1.52
Max drawdown: -7.6R
Worst regime: low-volatility Asia range
```

Success criteria:

```text
system can identify which behaviours historically worked best per instrument/timeframe/regime
```

## Phase 7: First ML selector

Deliverables:

```text
training dataset export
trade/no-trade model
direction model
entry type model
expected R model
win probability model
model evaluation report
```

Recommended first models:

```text
LightGBM or XGBoost
Random Forest baseline
Logistic regression baseline for trade/no-trade
```

Success criteria:

```text
model improves over deterministic baseline in walk-forward validation
probabilities are reasonably calibrated
```

## Phase 8: Live shadow mode

Deliverables:

```text
live candle ingestion
live feature generation
live policy recommendation
shadow outcome tracking
model-versus-baseline comparison
```

Success criteria:

```text
model can produce live decisions without taking action
actual outcomes are captured and compared against predictions
```

## Phase 9: Dashboard

Deliverables:

```text
instrument profile view
live recommendation view
policy leaderboard
feature-state inspection
regime classification display
model performance panel
```

Recommended live recommendation card:

```text
Decision: Wait
Regime: compressed low-volatility London pre-breakout
Best policy: no trade / wait for breakout confirmation
Confidence: 64%
Reason: range compression is high, but no breakout has occurred and price is mid-range
Risk flags: none
```

## Phase 10: Expand feature universe

Only after the v1 pipeline works:

```text
news/event proximity
intermarket correlations
advanced divergence
sequence models
order book data
funding/open interest
reinforcement learning experiments
```

## Testing priorities

### Feature tests

```text
ATR is calculated correctly
VWAP resets correctly by session
rolling highs/lows exclude future candles
session labels are correct
spread ratios are calculated correctly
HTF features align to the correct lower-timeframe candle
```

### Simulation tests

```text
long stop/target ordering
short stop/target ordering
same-candle stop/target ambiguity handling
partial exit calculation
spread-adjusted entry and exit
structural stop buffering
```

### Leakage tests

```text
future high/low not present in live features
session high/low only uses completed data up to timestamp
rolling percentiles only use prior candles
train/test windows do not overlap incorrectly
```

## Same-candle ambiguity

When both stop and target are touched within the same candle, the simulator needs a defined policy.

Options:

```text
conservative: assume stop hit first
optimistic: assume target hit first
intrabar: use lower timeframe data if available
```

Recommendation:

```text
Use conservative handling unless lower-timeframe data can resolve the sequence.
```

This assumption must be stored in simulation metadata.

## Initial success criteria

The first useful release should be able to:

```text
import candles
calculate feature snapshots
simulate candidate policies
create labels
produce instrument/timeframe policy leaderboards
train a baseline model
run walk-forward validation
produce live shadow recommendations
```

## Avoid building too early

Avoid these until the core system is stable:

```text
real-money execution
fully autonomous trading
reinforcement learning
large deep-learning models
complex news interpretation
unbounded strategy generation
optimising hundreds of parameters at once
```

## Suggested first milestone

Milestone name:

```text
MVP Feature + Policy Labelling Foundation
```

Scope:

```text
Candle ingestion
Feature schema v1.0.0
Candidate policy schema v1.0.0
Historical simulation
Policy scoring
No-trade labelling
Instrument profile report
```

Out of scope:

```text
live trading
broker integration
production ML promotion
news/event modelling
advanced UI
```

## Summary

The implementation should progress in this order:

```text
1. document contracts
2. ingest candles
3. generate features
4. simulate policies
5. label historical best behaviour
6. produce deterministic reports
7. train first models
8. validate walk-forward
9. run live shadow mode
10. expand gradually
```

The highest-risk part is not the model. The highest-risk part is the correctness of features, labels, cost assumptions and validation methodology.
