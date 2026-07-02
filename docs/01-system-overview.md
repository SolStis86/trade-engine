# System Overview

## Purpose

The trade engine is intended to analyse candle data and determine the best way to trade an instrument based on historical and live market features.

The system should not be limited to selecting a named strategy. Instead, it should learn a **trade policy**: a structured decision describing whether to trade, which direction to favour, how to enter, where to invalidate, how to target and how to manage the position.

## Target question

The engine should answer:

```text
Given the current market state, what trade behaviour has historically produced the best risk-adjusted result for this instrument and timeframe?
```

This is different from asking:

```text
Will the next candle go up or down?
```

The engine is not primarily a price-prediction system. It is a market-state-to-trade-policy system.

## Core responsibilities

The engine should provide:

1. **Feature extraction**  
   Convert historical and live candles into structured market-state features.

2. **Regime detection**  
   Identify whether the instrument is trending, ranging, volatile, compressed, extended, liquid, illiquid, mean-reverting or breakout-prone.

3. **Candidate policy generation**  
   Generate constrained trade behaviours that can be tested and scored.

4. **Historical simulation**  
   Evaluate what would have happened if different policies were applied at historical points.

5. **Labelling**  
   Determine the best historical action for each candle/window.

6. **Model training**  
   Train models to map market features to favourable trade behaviours.

7. **Live inference**  
   Apply the same feature pipeline to live candles and output a structured recommendation.

8. **Risk gating**  
   Prevent trades when execution conditions or market context are unsuitable.

9. **Outcome tracking**  
   Track live recommendations and actual outcomes for future retraining.

## High-level architecture

```text
Historical candles              Live candles
       |                              |
       v                              v
+----------------+            +----------------+
| Data Ingestion |            | Live Ingestion |
+----------------+            +----------------+
       |                              |
       +--------------+---------------+
                      v
              +----------------+
              | Feature Engine |
              +----------------+
                      |
                      v
            +---------------------+
            | Regime Classifier   |
            +---------------------+
                      |
                      v
        +--------------------------+
        | Candidate Policy Engine  |
        +--------------------------+
                      |
                      v
        +--------------------------+
        | Policy Scoring / ML Rank |
        +--------------------------+
                      |
                      v
              +---------------+
              | Risk Gate     |
              +---------------+
                      |
                      v
           +--------------------+
           | Trade Plan Builder |
           +--------------------+
                      |
                      v
           +--------------------+
           | Outcome Tracking  |
           +--------------------+
```

## Core engine types

### Feature Engine

The feature engine creates a versioned feature snapshot for each symbol, timeframe and candle timestamp.

Examples:

- ATR percentile
- VWAP distance in ATR units
- range compression
- session state
- higher-timeframe trend
- spread-to-ATR ratio
- close position inside candle range
- recent swing distance

### Regime Classifier

The regime classifier summarises the market state.

Examples:

```text
bullish_high_volatility_london
range_low_volatility_asia
bearish_expansion_new_york
compressed_breakout_candidate
mean_reversion_candidate
no_trade_chop
```

The first version can use deterministic rules. Later versions can use unsupervised clustering or supervised model outputs.

### Candidate Policy Engine

A policy is a structured trade behaviour.

Example:

```json
{
  "decision": "trade",
  "direction": "long",
  "entry_type": "pullback_confirmation",
  "stop_type": "structural",
  "target_type": "prior_high_low",
  "management": "partial_then_trail"
}
```

A policy is not necessarily a named strategy. Named strategies are simply pre-bundled policies.

For example:

```text
VWAP Reversion =
  direction: towards VWAP
  entry: exhaustion / reversal confirmation
  stop: beyond rejection extreme
  target: VWAP or mean
  management: fixed or partial exit
```

```text
Breakout Continuation =
  direction: with breakout
  entry: range break or retest
  stop: inside failed breakout zone
  target: fixed R or prior structure extension
  management: trail after confirmation
```

The engine should eventually learn the policy components directly.

### Policy Scoring Engine

Each candidate policy should be scored by historical and live criteria.

Suggested scoring inputs:

```text
expectancy
win probability
average win
average loss
profit factor
max adverse excursion
max drawdown
sample size
regime consistency
execution cost
spread condition
session suitability
```

Suggested score shape:

```text
policy_score =
  expectancy_score
  + profit_factor_score
  + consistency_score
  + sample_size_score
  + regime_fit_score
  - drawdown_penalty
  - spread_penalty
  - slippage_penalty
  - overfitting_penalty
```

### Risk Gate

The risk gate should be deterministic and conservative.

Example hard blocks:

```text
spread too high
round-trip cost too high
ATR too low
ATR extreme / chaotic regime
high-impact news nearby
rollover period
poor risk/reward
conflicting higher-timeframe signals
model confidence below threshold
insufficient historical sample size
```

The model may recommend a trade, but the risk gate can still veto it.

## Trade plan output

A live recommendation should be structured and explainable:

```json
{
  "symbol": "GBPJPY",
  "timeframe": "5m",
  "timestamp": "2026-07-02T09:35:00Z",
  "decision": "trade",
  "direction": "long",
  "entry_type": "pullback_confirmation",
  "entry_zone": {
    "lower": 182.42,
    "upper": 182.58
  },
  "stop_type": "structural_low",
  "stop_price": 182.08,
  "target_type": "prior_session_high",
  "targets": [
    {
      "price": 183.10,
      "r": 1.2
    },
    {
      "price": 183.74,
      "r": 2.1
    }
  ],
  "management": {
    "take_partial_at": "1R",
    "move_stop_to_breakeven_at": "1R",
    "trail_after": "target_1"
  },
  "expected_r": 0.42,
  "win_probability": 0.58,
  "confidence": 0.71,
  "regime": "bullish_high_volatility_london_pullback",
  "risk_flags": [],
  "reason": [
    "Price is above session VWAP",
    "Higher timeframe trend is bullish",
    "Spread is within normal range",
    "Pullback held above recent structure"
  ],
  "feature_schema_version": "v1.0.0",
  "model_version": "policy-selector-v0.1.0"
}
```

## Optimisation target

The system should not optimise only for win rate.

Core metrics:

```text
win_rate = wins / total_trades
loss_rate = losses / total_trades
win_loss_ratio = wins / losses
average_win_loss_ratio = average_win_size / average_loss_size
expectancy = (win_rate * average_win) - (loss_rate * average_loss)
profit_factor = gross_profit / gross_loss
```

Primary optimisation should be based on:

```text
expectancy
profit factor
max drawdown
sample size
execution viability
regime consistency
```

A high win-rate system with poor average loss control can still be negative expectancy.

## First-class no-trade state

`no_trade` must be treated as a valid policy.

Examples:

```text
No trade because spread is too high.
No trade because volatility is too low.
No trade because price is mid-range and risk/reward is poor.
No trade because current regime has historically produced negative expectancy.
No trade because model confidence is below threshold.
```

This is one of the most important safeguards in the system.

## Versioning requirements

The following must be versioned:

- feature schema
- feature calculation logic
- policy vocabulary
- label definition
- simulation assumptions
- cost model
- training dataset
- model version
- inference configuration

Example feature snapshot:

```json
{
  "symbol": "GBPJPY",
  "timeframe": "5m",
  "timestamp": "2026-07-02T09:35:00Z",
  "feature_schema_version": "v1.0.0",
  "features": {
    "body_pct_of_range": 0.64,
    "close_position_in_range": 0.82,
    "atr_14": 0.21,
    "atr_percentile_100": 0.72,
    "distance_to_vwap_atr": 0.84,
    "rsi_14": 61.2,
    "range_compression_20": 0.42,
    "spread_atr_ratio": 0.08,
    "session": "LONDON",
    "htf_trend_direction": "bullish"
  }
}
```

## Initial philosophy

Build the deterministic components first:

1. candle ingestion
2. feature snapshots
3. candidate policy simulation
4. historical outcome tracking
5. leaderboard and metrics

Only then introduce machine learning.

The ML model should earn its place by improving walk-forward performance compared with deterministic baselines.
