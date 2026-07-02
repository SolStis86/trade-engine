# Policy Labelling and Training Plan

This document describes how historical candle data should be converted into training examples for a trade policy engine.

The goal is to train a model that maps market features to the most favourable trade behaviour.

```text
market features -> best trade policy
```

Not:

```text
market features -> next candle prediction
```

## Core idea

For each candle/window in the historical dataset:

1. generate a feature snapshot using only information available at that time
2. generate candidate trade policies
3. simulate each candidate policy forward
4. score the policy outcomes
5. label the best policy, or label `no_trade`
6. train a model to reproduce that selection from features alone

## Policy vocabulary

The initial constrained vocabulary should be:

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

This controlled space prevents unconstrained strategy invention while still allowing the engine to compose flexible trade behaviours.

## Candidate policy example

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

## Historical simulation process

For each feature snapshot at time `t`:

```text
1. Build the available market context at t.
2. Generate valid candidate policies.
3. For each policy:
   - determine entry conditions
   - determine stop placement
   - determine target placement
   - simulate forward candle by candle
   - include spread, commission and slippage assumptions
   - record outcome
4. Score each policy.
5. Select the best policy if it passes minimum quality thresholds.
6. Otherwise label the state as no_trade.
```

## Entry simulation

### Immediate entry

Entry occurs at the next valid executable price after the signal candle closes.

Use this to model momentum/continuation behaviour where waiting for a pullback may reduce opportunity.

### Pullback confirmation

Entry requires price to pull back into a valid area and then confirm direction.

Examples:

```text
pullback to EMA/VWAP
pullback to prior breakout level
pullback to recent structure
pullback not exceeding allowed depth
bullish/bearish confirmation candle
```

### Breakout confirmation

Entry requires a break of a defined range or structure level.

Examples:

```text
close above rolling high
close below rolling low
range break with body close
breakout plus retest
breakout with spread and volatility filters
```

### Reversal confirmation

Entry requires evidence of exhaustion or failed continuation.

Examples:

```text
liquidity sweep
rejection wick
close back inside range
RSI/extension condition
VWAP reversion condition
```

## Stop simulation

### ATR-based stop

Example options:

```text
0.5 ATR
1.0 ATR
1.5 ATR
2.0 ATR
```

These should be converted into actual price distances using the ATR available at time `t`.

### Structural stop

Example options:

```text
below recent swing low for long
above recent swing high for short
outside consolidation range
beyond liquidity sweep wick
beyond VWAP rejection extreme
```

Structural stops should include a small buffer:

```text
buffer = max(2 * spread, 0.05 * ATR)
```

## Target simulation

### Fixed R

Example options:

```text
1R
1.5R
2R
3R
```

### Prior high/low

Use relevant levels:

```text
recent swing high
recent swing low
session high
session low
previous day high
previous day low
range high
range low
```

### VWAP or mean

For mean-reversion behaviour:

```text
session VWAP
rolling VWAP
EMA 20
rolling mean
range mid
```

## Management simulation

### Fixed exit

Exit fully at target or stop.

### Partial then trail

Example management rule:

```text
take 50% profit at 1R
move stop to breakeven after 1R
trail remainder behind structure or ATR
exit remainder at target 2 or trailing stop
```

This should be simulated explicitly so that average R reflects the actual management behaviour.

## Cost model

Every simulation should include costs.

Minimum cost inputs:

```text
spread
commission
slippage estimate
round_trip_cost
```

Suggested cost-adjusted entry/exit modelling:

```text
long entry = ask
long exit = bid
short entry = bid
short exit = ask
```

If bid/ask data is unavailable, approximate using spread around the mid/close.

Scalping policies must be rejected when:

```text
expected_move_to_cost_ratio < minimum threshold
spread_atr_ratio too high
spread_percentile too high
```

## Policy scoring

A policy should not be selected purely because it produced the largest single gain.

Suggested policy score:

```text
policy_score =
  expectancy_score
  + profit_factor_score
  + win_probability_score
  + regime_consistency_score
  + sample_size_score
  - drawdown_penalty
  - adverse_excursion_penalty
  - cost_penalty
  - duration_penalty
  - overfitting_penalty
```

Core metrics:

```text
total_trades
wins
losses
win_rate
loss_rate
average_win
average_loss
average_win_loss_ratio
win_loss_ratio
expectancy
profit_factor
max_drawdown
max_adverse_excursion
max_favourable_excursion
largest_losing_streak
average_duration_bars
```

## Expectancy

The primary metric should be expectancy:

```text
expectancy = (win_rate * average_win) - (loss_rate * average_loss)
```

Where `average_loss` should be a positive magnitude in the formula.

Example:

```text
win_rate = 0.56
loss_rate = 0.44
average_win = 1.2R
average_loss = 0.8R

expectancy = (0.56 * 1.2) - (0.44 * 0.8)
expectancy = 0.672 - 0.352
expectancy = +0.32R
```

## No-trade labelling

`no_trade` should be selected when no policy passes minimum quality thresholds.

Example no-trade conditions:

```text
all candidate policies have negative expectancy
expected move is too small relative to cost
spread is too wide
volatility is too low
volatility is too extreme
price is mid-range with poor risk/reward
historical sample size is too small
policy confidence is too low
major event risk is present
```

`no_trade` should not be a fallback afterthought. It should be treated as a successful decision when trading would historically have been unfavourable.

## Training example shape

```json
{
  "symbol": "GBPJPY",
  "timeframe": "5m",
  "timestamp": "2026-07-02T09:35:00Z",
  "feature_schema_version": "v1.0.0",
  "label_schema_version": "v1.0.0",
  "features": {
    "atr_percentile_100": 0.72,
    "distance_to_vwap_atr": 0.84,
    "range_compression_20": 0.42,
    "spread_atr_ratio": 0.08,
    "session": "LONDON",
    "htf_trend_direction": "bullish"
  },
  "best_policy": {
    "decision": "trade",
    "direction": "long",
    "entry_type": "pullback_confirmation",
    "stop_type": "structural",
    "target_type": "prior_high_low",
    "management": "partial_then_trail"
  },
  "outcome": {
    "expected_r": 0.42,
    "win_probability": 0.58,
    "stopout_probability": 0.34,
    "max_drawdown_r": -0.72,
    "average_duration_bars": 9,
    "sample_size": 184
  }
}
```

## Model options

### Phase 1: tabular models

Start with robust tabular models:

```text
LightGBM
XGBoost
CatBoost
Random Forest
Logistic regression baselines
```

These are easier to debug than deep sequence models and typically perform well on engineered trading features.

### Phase 2: ranking models

Once candidate policy simulations are reliable, train a model to rank candidate policies directly.

Shape:

```text
input: feature snapshot + candidate policy encoding
output: expected policy score
```

This lets the engine score many possible behaviours and choose the highest-ranking valid option.

### Phase 3: sequence models

Only after the tabular pipeline is stable, consider:

```text
Temporal CNN
LSTM / GRU
Transformer encoder
```

These can consume recent candle sequences directly, but they should not be the starting point.

## Recommended first model structure

For the first version, use separate targets:

```text
trade_decision_model:
  trade / no_trade / wait

direction_model:
  long / short

entry_type_model:
  immediate / pullback / breakout / reversal

stop_type_model:
  atr_based / structural

target_type_model:
  fixed_r / prior_high_low / vwap_or_mean

management_model:
  fixed_exit / partial_then_trail

expected_r_model:
  regression

win_probability_model:
  calibrated probability

stopout_probability_model:
  calibrated probability
```

This is easier to inspect than one black-box model.

## Validation

Use walk-forward validation.

Example:

```text
Train: Jan -> Mar
Test:  Apr

Train: Jan -> Apr
Test:  May

Train: Jan -> May
Test:  Jun

Train: Jan -> Jun
Test:  Jul
```

Never use random train/test splits for time-series trading decisions.

## Leakage prevention

Avoid these leakage errors:

```text
using future highs/lows as input features
using future session high/low before session completion
normalising with full-dataset statistics
using revised data not available at decision time
training and testing across overlapping windows incorrectly
selecting features after seeing test performance repeatedly
```

Use rolling calculations only.

Example:

```text
valid: ATR percentile calculated from the previous 100 completed candles
invalid: ATR percentile calculated from the entire dataset including future candles
```

## Calibration

Win probabilities must be calibrated.

Track:

```text
predicted probability bucket
actual win rate in bucket
calibration error
```

Example:

```text
Predicted 60-70% win probability bucket
Actual historical win rate: 62%
```

If the model says 70% but actual is 52%, confidence should be reduced.

## Model promotion criteria

A model should only be promoted to live use if it improves over deterministic baselines on walk-forward tests.

Promotion checks:

```text
higher expectancy than baseline
higher profit factor than baseline
acceptable max drawdown
reasonable sample size
stable performance across time windows
stable performance across instruments
calibrated win probability
no obvious leakage
```

## Live shadow mode

Before using a model for actual recommendations, run it in shadow mode.

In shadow mode:

```text
model produces decisions
system records decisions
no live trading action is taken
actual market outcome is later evaluated
model decisions are compared with deterministic baselines
```

Shadow mode should track:

```text
would-have-traded count
would-have-won count
would-have-lost count
expected R versus actual R
policy decision accuracy
no-trade quality
false-positive trade rate
missed-opportunity rate
```

## Output tracking

Every live recommendation should store:

```text
symbol
timeframe
timestamp
feature snapshot id
feature schema version
model version
policy selected
confidence
expected_r
win_probability
risk flags
final decision
actual outcome
actual_r
max_adverse_excursion
max_favourable_excursion
```

This creates the feedback loop for retraining.

## Summary

The labelling pipeline is the heart of the system.

The correct sequence is:

```text
feature snapshot
  -> candidate policy simulation
  -> cost-adjusted outcome scoring
  -> best policy / no-trade label
  -> walk-forward training
  -> live shadow validation
  -> production promotion
```

Do not start with a model that predicts price. Start with a model that learns which trading behaviour has historically been most favourable under similar conditions.
