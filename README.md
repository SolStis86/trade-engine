# Trade Engine

A machine-learning assisted trade policy engine for analysing historical and live candle data, extracting market-state features, and determining the most statistically favourable way to trade a given instrument.

The goal is not simply to pick a named strategy such as `breakout`, `VWAP reversion`, or `mean reversion`. The long-term goal is to learn the best **trade behaviour** from market features:

- whether to trade or avoid the market
- long, short, or neutral bias
- entry behaviour
- stop/invalidation behaviour
- target behaviour
- trade management behaviour
- expected R
- win/loss profile
- confidence
- regime fit

## Core concept

Traditional systems usually work like this:

```text
Market data -> Strategy rules -> Signal
```

This project aims for a richer structure:

```text
Historical + live candles
  -> Feature engine
  -> Regime classifier
  -> Candidate policy generator
  -> Policy scoring / ML ranking
  -> Risk gate
  -> Structured trade plan
  -> Outcome tracking
  -> Retraining dataset
```

The system should eventually answer:

```text
Given this instrument, timeframe, session, volatility, trend, spread, structure and live candle behaviour, what is the best way to trade right now?
```

Example output:

```json
{
  "decision": "trade",
  "direction": "long",
  "entry_type": "pullback_confirmation",
  "stop_type": "structural_low",
  "target_type": "prior_session_high",
  "management": "partial_then_trail",
  "expected_r": 0.42,
  "win_probability": 0.58,
  "confidence": 0.71,
  "regime": "bullish_high_volatility_london_pullback",
  "reason": [
    "Price is above session VWAP",
    "Higher timeframe trend is bullish",
    "ATR percentile is elevated but not extreme",
    "Spread is inside normal range",
    "Recent pullback held above structure"
  ]
}
```

## Documentation

| Document | Purpose |
| --- | --- |
| [`docs/01-system-overview.md`](docs/01-system-overview.md) | Product concept, engine responsibilities and architecture. |
| [`docs/02-feature-catalogue.md`](docs/02-feature-catalogue.md) | Extensive candle feature catalogue. |
| [`docs/03-initial-feature-selection.md`](docs/03-initial-feature-selection.md) | Initial MVP feature set and phased selection plan. |
| [`docs/04-policy-labelling-and-training.md`](docs/04-policy-labelling-and-training.md) | How to create labels, simulate policies and train the selector. |
| [`docs/05-implementation-roadmap.md`](docs/05-implementation-roadmap.md) | Practical build plan and suggested repository modules. |
| [`docs/06-technical-architecture.md`](docs/06-technical-architecture.md) | Alpaca ingest, Rust compute core and service/language boundaries. |

## Key principles

1. **Do not optimise for win rate alone**  
   A strategy with a high win rate can still lose money if its losses are larger than its wins. The system should optimise for expectancy, profit factor, drawdown control and execution viability.

2. **Make `no_trade` a first-class decision**  
   The engine must be able to say that current conditions do not match a historically favourable setup.

3. **Use candle features first**  
   Start with robust candle-derived features before adding news, macro, intermarket or order-book data.

4. **Use walk-forward validation**  
   Never use random train/test splits for time-series trading systems.

5. **Keep policies constrained and explainable**  
   The model should select from known entry, stop, target and management behaviours rather than inventing unconstrained trades.

6. **Version everything**  
   Feature schemas, label definitions, simulation assumptions and model versions must all be tracked.

## Technical direction

The initial technical direction is:

```text
Alpaca ingestion: TypeScript/NestJS worker
Canonical candle store: Postgres + Parquet
Feature calculation: Rust
Backtesting: Rust
Policy simulation: Rust
ML training: Python
API/dashboard: TypeScript
```

The first technical milestone should be:

```text
Alpaca historical ingest
  -> normalized candles
  -> Rust feature generation
  -> Parquet/Postgres feature snapshots
```

## Initial MVP goal

Build a first version that can:

1. ingest historical candle data for a small number of instruments
2. generate a versioned feature snapshot per candle
3. simulate a constrained set of trade policies
4. label the best historical behaviour per candle/window
5. rank policy behaviours by expectancy, win rate, drawdown and profit factor
6. run live candle snapshots through the same feature engine
7. output a structured trade/no-trade recommendation

## MVP decision space

The first version should use a constrained policy space:

```text
decision:
  trade
  no_trade
  wait

direction:
  long
  short

entry_type:
  immediate
  pullback_confirmation
  breakout_confirmation
  reversal_confirmation

stop_type:
  atr_based
  structural

target_type:
  fixed_r
  prior_high_low
  vwap_or_mean

management:
  fixed_exit
  partial_then_trail
```

This gives enough combinations to be useful, but not so many that labelling, debugging and validation become unmanageable.

## Repository status

This repository currently contains the documentation foundation for the trade engine. Implementation packages and services should be added once the feature schema and labelling design are stable.
