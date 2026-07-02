# Initial Feature Selection Plan

The full feature catalogue is intentionally broad. The first implementation should use a smaller, explainable, high-signal feature set.

The goal of the initial feature schema is to support the first version of the trade policy engine without overfitting or creating unnecessary complexity.

## Initial goal

The first feature set should allow the system to answer:

```text
Is the current market state suitable for trading?
If so, is the better behaviour continuation, reversal, breakout, mean reversion or no trade?
What is the likely risk-adjusted quality of that behaviour?
```

## Why not use every feature immediately?

Using hundreds of features at the start creates several risks:

1. **Overfitting**  
   The model may learn noise instead of durable market behaviour.

2. **Debugging difficulty**  
   It becomes hard to understand why a recommendation was made.

3. **Feature leakage risk**  
   More features increase the chance that a future-looking value is accidentally included.

4. **Implementation drag**  
   The project needs a working feature/backtest/label pipeline before it needs exotic features.

5. **Poor model discipline**  
   Initial performance should be measured with a clean baseline before adding complexity.

## MVP feature groups

The first version should include the following groups:

```text
1. Raw candle shape
2. Returns and momentum
3. ATR volatility
4. VWAP distance and behaviour
5. Moving average trend
6. RSI / oscillator state
7. Range compression and breakout position
8. Session context
9. Spread/cost context
10. Higher-timeframe trend
11. Recent swing structure
12. Basic volume / relative volume
13. Composite trade-behaviour scores
```

## MVP feature schema v1.0.0

### Raw candle shape

```text
open
high
low
close
volume
body_abs
range
upper_wick
lower_wick
body_pct_of_range
upper_wick_pct_of_range
lower_wick_pct_of_range
close_position_in_range
body_atr_ratio
range_atr_ratio
```

Why included:

- detects rejection candles
- detects strong closes
- helps classify momentum versus exhaustion
- supports reversal and continuation detection

### Returns and momentum

```text
return_1
return_3
return_5
return_10
return_5_atr_normalized
return_10_atr_normalized
momentum_5
momentum_10
momentum_change_5
```

Why included:

- captures recent directional pressure
- distinguishes acceleration from exhaustion
- helps determine continuation versus reversal behaviour

### Volatility

```text
true_range
atr_14
atr_percent
atr_percentile_100
atr_change_20
range_atr_ratio
volatility_expansion_ratio
volatility_contracting
```

Why included:

- many policies depend on volatility regime
- stop and target distances should be volatility-aware
- low-volatility and extreme-volatility regimes may require no-trade decisions

### VWAP and mean value

```text
session_vwap
distance_to_vwap_atr
vwap_slope
above_vwap
below_vwap
vwap_cross
vwap_reclaim
vwap_rejection
bars_above_vwap
bars_below_vwap
```

Why included:

- VWAP is central to intraday mean-reversion and continuation logic
- distance from VWAP is a strong overextension signal
- VWAP reclaim/loss can support directional policy selection

### Moving average trend

```text
ema_20
ema_50
distance_to_ema_20_atr
distance_to_ema_50_atr
ema_20_slope
ema_50_slope
ema_20_above_ema_50
ema_alignment
```

Why included:

- helps detect trend direction and strength
- supports pullback-continuation policies
- helps avoid mean reversion against strong trend conditions

### RSI / oscillator state

```text
rsi_14
rsi_14_slope
rsi_percentile_100
overbought_score
oversold_score
```

Why included:

- identifies stretched momentum
- supports reversal/mean-reversion features
- useful as one signal among many, not as a standalone strategy

### Range compression and breakout position

```text
rolling_high_20
rolling_low_20
range_mid_20
distance_to_range_high_atr
distance_to_range_low_atr
range_compression_20
close_above_range_high
close_below_range_low
wick_above_range_high
wick_below_range_low
breakout_strength
false_breakout_score
```

Why included:

- supports breakout candidate detection
- identifies failed breakouts and liquidity sweeps
- helps classify whether price is near range edge or mid-range

### Session context

```text
hour_of_day
day_of_week
session
is_asia_session
is_london_session
is_new_york_session
is_london_ny_overlap
is_rollover_period
minutes_since_session_open
minutes_until_session_close
session_open_price
session_high
session_low
distance_to_session_high_atr
distance_to_session_low_atr
```

Why included:

- intraday behaviour varies heavily by session
- spread, volatility and breakout probability often vary by session
- certain periods should be down-weighted or avoided

### Spread and execution cost

```text
spread
spread_percent
spread_atr_ratio
spread_percentile_100
round_trip_cost_atr
expected_move_to_cost_ratio
spread_too_wide
poor_execution_environment
```

Why included:

- essential for scalping and short-duration trades
- prevents apparently profitable signals that are not executable after costs
- supports no-trade policy decisions

### Higher-timeframe context

For a 1m or 5m decision, include 15m and 1H context where possible.

```text
htf_trend_direction
htf_trend_strength
htf_ema_alignment
htf_price_above_vwap
htf_distance_to_vwap_atr
htf_atr_percentile
htf_rsi
htf_structure_bias
ltf_htf_trend_alignment
ltf_countertrend_flag
```

Why included:

- prevents lower-timeframe trades against strong higher-timeframe pressure
- identifies pullbacks inside larger trends
- helps classify countertrend reversal opportunities

### Recent swing structure

```text
recent_swing_high
recent_swing_low
bars_since_swing_high
bars_since_swing_low
distance_to_swing_high_atr
distance_to_swing_low_atr
higher_high_count
higher_low_count
lower_high_count
lower_low_count
market_structure_bias
```

Why included:

- supports structural stops and invalidation
- helps determine whether price is trending or ranging
- identifies room to nearest support/resistance

### Basic volume

```text
volume
volume_sma_20
relative_volume_20
volume_percentile_100
volume_change_5
volume_spike
volume_dry_up
volume_price_confirmation
```

Why included:

- confirms breakouts and continuation
- identifies weak moves and exhaustion
- supports liquidity/participation context

Note: for FX or CFD data, volume quality may vary by provider. Treat volume features as provider-specific.

### Composite trade-behaviour scores

These should be deterministic and explainable.

```text
trend_score
mean_reversion_score
breakout_score
compression_score
exhaustion_score
continuation_score
reversal_score
liquidity_score
execution_quality_score
risk_reward_score
setup_quality_score
```

Why included:

- gives the model domain-aware summaries
- provides explainability in the UI
- allows deterministic baselines before ML

Composite scores should always be backed by their underlying feature values.

## Recommended initial feature count

Target approximately **60 to 90 features** for the first schema.

This is enough to represent the market state without making the first model unnecessarily noisy.

## Initial policy decision space

The first feature set should support this controlled policy vocabulary:

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

## Initial model targets

The first ML models should predict:

```text
trade_decision
best_direction
best_entry_type
best_stop_type
best_target_type
best_management_type
expected_r
win_probability
stopout_probability
no_trade_probability
```

This can be implemented either as:

1. separate models per target, or
2. a multi-output model, or
3. a ranking model that scores candidate policies.

For the first version, separate models or a candidate-policy ranking model will be easier to debug than a single monolithic model.

## Initial deterministic baselines

Before training ML models, build deterministic baselines using the composite scores.

Example:

```text
If breakout_score is high
and spread_atr_ratio is acceptable
and htf trend aligns
and range_compression_20 is strong
then rank breakout-confirmation policies highly.
```

Example:

```text
If distance_to_vwap_atr is extreme
and trend_score is weak
and rejection_score is high
then rank mean-reversion/reversal policies highly.
```

The ML model should be compared against these baselines.

## Phased feature rollout

### Phase 1: Core candle and volatility

Implement:

```text
raw candle shape
returns
ATR
ATR percentile
range and body normalization
```

Purpose:

- establish feature snapshots
- test basic data integrity
- support simple momentum/volatility baselines

### Phase 2: VWAP, trend and session

Implement:

```text
session VWAP
VWAP distance
EMA trend
session labels
time-of-day features
```

Purpose:

- support intraday policy logic
- classify trend versus mean-reversion environments

### Phase 3: Range, breakout and structure

Implement:

```text
rolling highs/lows
range compression
breakout position
recent swing highs/lows
market structure bias
```

Purpose:

- support breakout and failed-breakout policies
- enable structural stop/target construction

### Phase 4: Spread and execution viability

Implement:

```text
spread_atr_ratio
spread_percentile
round_trip_cost_atr
expected_move_to_cost_ratio
poor_execution_environment
```

Purpose:

- prevent unrealistic scalping outputs
- make `no_trade` decisions more realistic

### Phase 5: Higher-timeframe context

Implement:

```text
15m context for 1m/5m decisions
1H context for 5m/15m decisions
trend alignment
HTF VWAP distance
HTF ATR percentile
```

Purpose:

- improve directional filtering
- identify pullbacks in larger trends

### Phase 6: Composite scoring

Implement:

```text
trend_score
breakout_score
mean_reversion_score
reversal_score
continuation_score
liquidity_score
risk_reward_score
setup_quality_score
```

Purpose:

- produce explainable recommendations
- create deterministic strategy-ranking baselines

## Feature selection evaluation

Every feature group should be evaluated by whether it improves walk-forward performance.

Track:

```text
expectancy
profit factor
win rate
max drawdown
largest losing streak
sample size
calibration of win probability
no-trade accuracy
policy ranking accuracy
```

When a new feature group is added, compare:

```text
baseline without feature group
baseline with feature group
ML model without feature group
ML model with feature group
```

## Avoid in the initial version

Do not include these in the first version unless the core pipeline is already stable:

```text
complex divergence detection
news NLP
intermarket correlations
order book features
funding/open interest
sequence neural networks
reinforcement learning
hundreds of lagged features
unconstrained strategy generation
```

These can be valuable later, but they will slow down the first working system.

## Recommended first instruments and timeframes

Start small:

```text
Instruments:
  GBPJPY
  XAUUSD
  BTCUSD or SOLUSD

Timeframes:
  1m
  5m
  15m
```

Reasoning:

- enough variation across FX, metal and crypto behaviour
- enough intraday data for backtesting
- aligns well with scalping and short-duration trade-policy discovery

## First success criteria

The first version is successful if it can produce a historical report like:

```text
Instrument: GBPJPY
Timeframe: 5m
Best overall behaviour: long/short pullback continuation during London
Win rate: 56.8%
Average win/loss ratio: 1.34
Expectancy: +0.28R
Profit factor: 1.52
Max drawdown: -7.6R
Best regime: bullish or bearish trend with normal/high volatility
Worst regime: low-volatility Asia range
No-trade accuracy: 63%
```

And a live output like:

```text
Decision: Wait
Reason: price is mid-range, spread is normal, volatility is contracting, but no clear entry behaviour has positive historical expectancy in similar conditions.
```

## Summary

Initial feature selection should prioritise:

```text
volatility
VWAP distance
trend context
range compression
session context
execution cost
higher-timeframe alignment
recent structure
```

These provide a strong foundation for determining the best way to trade from candle data while keeping the first model understandable and testable.
