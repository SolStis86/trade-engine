# Candle Feature Catalogue

This document lists the feature groups that can be derived from candle data and related execution context.

The initial implementation should not attempt to use every feature immediately. The purpose of this catalogue is to define the possible feature universe. The first model should use a smaller, well-understood, versioned feature schema.

## Feature design principles

1. **Prefer normalized values**  
   Raw values do not compare well across instruments. Prefer ATR-normalized, percentage, z-score or percentile values.

2. **Avoid future leakage**  
   Features available to the live model must only use information available at the candle close or decision time.

3. **Version feature definitions**  
   If a calculation changes, increment the feature schema version.

4. **Prefer scores over binary flags where possible**  
   For example, `rejection_score` is often more useful than `is_rejection_candle`.

5. **Separate live input features from training-only outcome features**  
   Forward returns, future highs/lows and target/stop outcomes are for labelling only and must never be included as live model inputs.

---

## 1. Raw candle features

Base candle fields:

```text
open
high
low
close
volume
timestamp
timeframe
symbol
```

Derived candle values:

```text
body
body_abs
range
upper_wick
lower_wick
direction
mid_price
typical_price
ohlc4
```

Normalized candle values:

```text
body_pct_of_range
upper_wick_pct_of_range
lower_wick_pct_of_range
close_position_in_range
open_position_in_range
body_to_wick_ratio
wick_imbalance
body_atr_ratio
range_atr_ratio
upper_wick_atr_ratio
lower_wick_atr_ratio
```

Example:

```text
close_position_in_range = (close - low) / (high - low)
```

This tells whether the candle closed near its high, middle or low.

---

## 2. Candle shape features

Boolean candle-shape features:

```text
is_bullish
is_bearish
is_neutral
is_doji
is_large_body
is_small_body
is_marubozu
is_pin_bar
is_hammer
is_inverted_hammer
is_shooting_star
is_spinning_top
is_inside_bar
is_outside_bar
is_engulfing_bullish
is_engulfing_bearish
is_long_upper_wick
is_long_lower_wick
is_full_body_close
is_rejection_candle
is_exhaustion_candle
```

Score-based candle-shape features:

```text
doji_score
engulfing_score
pin_bar_score
rejection_score
body_strength_score
close_strength_score
wick_rejection_score
bullish_rejection_score
bearish_rejection_score
exhaustion_score
```

Example:

```text
bullish_rejection_score = lower_wick_pct_of_range * close_position_in_range
bearish_rejection_score = upper_wick_pct_of_range * (1 - close_position_in_range)
```

---

## 3. Multi-candle pattern features

Consecutive candle features:

```text
consecutive_bullish_candles
consecutive_bearish_candles
consecutive_higher_closes
consecutive_lower_closes
consecutive_higher_highs
consecutive_lower_lows
consecutive_inside_bars
consecutive_expanding_ranges
consecutive_contracting_ranges
```

Pattern flags and scores:

```text
three_bar_reversal
three_bar_continuation
morning_star_like_pattern
evening_star_like_pattern
bullish_engulfing_sequence
bearish_engulfing_sequence
failed_breakout_pattern
liquidity_sweep_reversal
micro_pullback_pattern
compression_break_pattern
```

Rolling sequence metrics:

```text
last_3_candle_return
last_5_candle_return
last_10_candle_return
last_3_body_sum
last_5_body_sum
bullish_candle_ratio_5
bearish_candle_ratio_5
average_body_size_5
average_range_5
range_expansion_ratio_5
body_expansion_ratio_5
```

---

## 4. Return and momentum features

Simple returns:

```text
return_1
return_2
return_3
return_5
return_10
return_20
return_50
log_return_1
log_return_5
log_return_20
```

Momentum:

```text
momentum_3
momentum_5
momentum_10
momentum_20
rate_of_change_5
rate_of_change_10
rate_of_change_20
```

Acceleration:

```text
momentum_change_5
momentum_change_10
return_acceleration
roc_acceleration
price_velocity
price_acceleration
```

Normalized momentum:

```text
return_5_atr_normalized
return_10_atr_normalized
momentum_5_zscore
momentum_20_zscore
```

---

## 5. Volatility features

True range and ATR:

```text
true_range
average_true_range_5
average_true_range_14
average_true_range_20
average_true_range_50
atr_percent
atr_percentile_20
atr_percentile_50
atr_percentile_100
```

Range volatility:

```text
rolling_range_5
rolling_range_10
rolling_range_20
rolling_high_low_range_20
range_percentile_20
range_percentile_100
```

Realized volatility:

```text
realized_volatility_5
realized_volatility_10
realized_volatility_20
realized_volatility_50
realized_volatility_annualized
```

Volatility change:

```text
atr_change_5
atr_change_20
volatility_expansion_ratio
volatility_contraction_ratio
volatility_regime_change
```

Volatility labels:

```text
low_volatility_regime
normal_volatility_regime
high_volatility_regime
extreme_volatility_regime
volatility_expanding
volatility_contracting
```

High-value feature:

```text
current_atr / median_atr_100
```

---

## 6. Compression and expansion features

Compression:

```text
range_compression_5
range_compression_10
range_compression_20
atr_compression_ratio
body_compression_ratio
bollinger_band_width
bollinger_band_width_percentile
keltner_channel_width
donchian_channel_width
rolling_range_narrowness
```

Expansion:

```text
range_expansion_ratio
body_expansion_ratio
atr_expansion_ratio
bollinger_band_expansion
volume_range_expansion
breakout_expansion_score
```

Squeeze features:

```text
is_bollinger_inside_keltner
squeeze_on
squeeze_off
squeeze_duration
squeeze_release_strength
```

Example:

```text
range_compression_20 = current_20_bar_range / average_20_bar_range_over_last_100
```

---

## 7. Trend features

Moving averages:

```text
sma_5
sma_10
sma_20
sma_50
sma_100
ema_5
ema_9
ema_20
ema_50
ema_100
ema_200
```

Price relative to moving averages:

```text
close_above_sma_20
close_above_ema_20
close_above_ema_50
close_above_ema_200
distance_to_ema_20
distance_to_ema_50
distance_to_ema_200
distance_to_ema_20_atr
distance_to_ema_50_atr
distance_to_ema_200_atr
```

Moving average slope:

```text
sma_20_slope
ema_20_slope
ema_50_slope
ema_200_slope
ema_slope_alignment
```

Moving average alignment:

```text
ema_9_above_ema_20
ema_20_above_ema_50
ema_50_above_ema_200
bullish_ma_stack
bearish_ma_stack
ma_compression
ma_expansion
```

Trend strength indicators:

```text
adx_14
plus_di
minus_di
adx_slope
trend_strength_score
directional_movement_score
```

Structure trend:

```text
higher_high_count
higher_low_count
lower_high_count
lower_low_count
market_structure_bias
trend_consistency_score
```

Trend labels:

```text
strong_uptrend
weak_uptrend
range_bound
weak_downtrend
strong_downtrend
```

---

## 8. Mean reversion features

VWAP-based:

```text
vwap
session_vwap
anchored_vwap
rolling_vwap
distance_to_vwap
distance_to_vwap_atr
distance_to_vwap_percent
vwap_slope
vwap_reclaim_flag
vwap_rejection_flag
vwap_cross_count
```

Moving-average reversion:

```text
distance_to_sma_20
distance_to_ema_20
distance_to_sma_50
distance_to_rolling_mean
zscore_from_mean_20
zscore_from_mean_50
```

Bollinger features:

```text
bollinger_upper
bollinger_lower
bollinger_mid
bollinger_band_position
distance_to_upper_band
distance_to_lower_band
bollinger_zscore
bollinger_band_touch
bollinger_band_rejection
```

Oscillators:

```text
rsi_7
rsi_14
rsi_21
rsi_percentile
stochastic_k
stochastic_d
cci
williams_r
```

Overextension:

```text
overbought_score
oversold_score
extension_from_mean_score
mean_reversion_pressure
```

Labels:

```text
price_overextended_up
price_overextended_down
mean_reversion_candidate
vwap_reversion_candidate
```

---

## 9. Breakout features

Range boundaries:

```text
rolling_high_5
rolling_low_5
rolling_high_10
rolling_low_10
rolling_high_20
rolling_low_20
donchian_high_20
donchian_low_20
range_high
range_low
range_mid
```

Distance to range:

```text
distance_to_range_high
distance_to_range_low
distance_to_range_mid
distance_to_range_high_atr
distance_to_range_low_atr
```

Breakout state:

```text
close_above_range_high
close_below_range_low
wick_above_range_high
wick_below_range_low
breakout_strength
breakout_body_strength
breakout_close_strength
breakout_volume_confirmation
```

False breakout:

```text
failed_breakout_up
failed_breakout_down
sweep_above_high
sweep_below_low
close_back_inside_range
false_breakout_score
```

Retest:

```text
breakout_retest_seen
distance_to_breakout_level
retest_hold_score
retest_failure_score
```

Compression before breakout:

```text
pre_breakout_compression_score
pre_breakout_volume_contraction
pre_breakout_range_tightness
setup_quality_score
```

---

## 10. Support and resistance features

Recent structure:

```text
recent_swing_high
recent_swing_low
distance_to_recent_swing_high
distance_to_recent_swing_low
nearest_structure_level
distance_to_nearest_structure
```

Pivot levels:

```text
daily_pivot
weekly_pivot
monthly_pivot
r1
r2
r3
s1
s2
s3
distance_to_daily_pivot
distance_to_r1
distance_to_s1
```

Session levels:

```text
previous_day_high
previous_day_low
previous_day_close
current_day_open
weekly_open
monthly_open
asia_high
asia_low
london_high
london_low
ny_high
ny_low
```

Level interaction:

```text
level_touch_count
level_rejection_count
level_break_count
level_hold_score
level_failure_score
support_resistance_density
```

Labels:

```text
near_support
near_resistance
at_range_extreme
at_session_high
at_session_low
inside_value_area
outside_value_area
```

---

## 11. Market structure features

Swing detection:

```text
swing_high
swing_low
last_swing_high
last_swing_low
bars_since_swing_high
bars_since_swing_low
distance_from_last_swing_high
distance_from_last_swing_low
```

Structure transitions:

```text
break_of_structure_up
break_of_structure_down
change_of_character_up
change_of_character_down
market_structure_shift
```

Higher/lower structure:

```text
higher_high
higher_low
lower_high
lower_low
inside_structure
expanding_structure
contracting_structure
```

Trend leg features:

```text
current_leg_direction
current_leg_length_bars
current_leg_return
current_leg_atr_multiple
pullback_depth
pullback_depth_percent
pullback_depth_atr
```

Derived features:

```text
impulse_to_pullback_ratio
structure_quality_score
trend_leg_maturity
bars_since_structure_break
```

---

## 12. Liquidity sweep features

Sweep detection:

```text
sweep_previous_high
sweep_previous_low
sweep_session_high
sweep_session_low
sweep_daily_high
sweep_daily_low
wick_sweep_then_close_inside
sweep_distance_atr
sweep_rejection_strength
liquidity_grab_score
```

Follow-through:

```text
post_sweep_reversal_return_1
post_sweep_reversal_return_3
post_sweep_followthrough_score
failed_sweep
successful_sweep_reversal
```

Labels:

```text
bullish_liquidity_sweep
bearish_liquidity_sweep
stop_run_detected
liquidity_grab_reversal_candidate
```

---

## 13. Volume features

Basic volume:

```text
volume
volume_sma_5
volume_sma_20
volume_sma_50
relative_volume_5
relative_volume_20
volume_percentile_20
volume_percentile_100
```

Volume change:

```text
volume_change_1
volume_change_5
volume_acceleration
volume_spike
volume_dry_up
```

Volume with price action:

```text
volume_on_bullish_candles
volume_on_bearish_candles
bullish_volume_ratio
bearish_volume_ratio
volume_price_confirmation
volume_divergence
```

Breakout volume:

```text
breakout_volume_ratio
volume_expansion_on_breakout
failed_breakout_high_volume
low_volume_breakout
```

Volume indicators:

```text
obv
obv_slope
chaikin_money_flow
money_flow_index
accumulation_distribution
volume_weighted_momentum
```

Labels:

```text
volume_confirms_move
volume_rejects_move
abnormal_volume_spike
low_liquidity_condition
```

---

## 14. VWAP and value features

VWAP variants:

```text
session_vwap
anchored_vwap_daily
anchored_vwap_weekly
anchored_vwap_from_swing
anchored_vwap_from_session_open
rolling_vwap_20
rolling_vwap_50
```

VWAP bands:

```text
vwap_upper_1std
vwap_lower_1std
vwap_upper_2std
vwap_lower_2std
vwap_band_position
distance_to_vwap_band
```

VWAP behaviour:

```text
above_vwap
below_vwap
vwap_cross
vwap_reclaim
vwap_loss
vwap_retest
vwap_rejection
bars_above_vwap
bars_below_vwap
```

VWAP scoring:

```text
vwap_trend_score
vwap_reversion_score
vwap_continuation_score
vwap_distance_atr
```

---

## 15. Session and time features

Time features:

```text
hour_of_day
minute_of_hour
day_of_week
day_of_month
week_of_month
month
quarter
is_month_start
is_month_end
is_week_start
is_week_end
```

Session features:

```text
is_asia_session
is_london_session
is_new_york_session
is_london_open
is_new_york_open
is_london_ny_overlap
is_rollover_period
minutes_since_session_open
minutes_until_session_close
```

Session behaviour:

```text
session_open_price
session_high
session_low
session_range
session_return
session_vwap
distance_to_session_open
distance_to_session_high
distance_to_session_low
```

Labels:

```text
opening_drive
mid_session_chop
session_close_volatility
london_breakout_window
ny_reversal_window
rollover_avoidance_period
```

---

## 16. Multi-timeframe features

Higher-timeframe features:

```text
htf_trend_direction
htf_trend_strength
htf_ema_alignment
htf_price_above_vwap
htf_distance_to_vwap
htf_atr_percentile
htf_rsi
htf_structure_bias
htf_break_of_structure
htf_nearest_support
htf_nearest_resistance
```

Alignment features:

```text
ltf_htf_trend_alignment
ltf_countertrend_flag
multi_timeframe_momentum_alignment
multi_timeframe_volatility_alignment
multi_timeframe_vwap_alignment
m1_direction_matches_m5
m5_direction_matches_m15
m15_direction_matches_h1
h1_bias
h4_bias
```

Labels:

```text
all_timeframes_bullish
all_timeframes_bearish
lower_timeframe_pullback_in_higher_timeframe_uptrend
lower_timeframe_reversal_against_higher_timeframe
```

---

## 17. Regime classification features

Trend/range regimes:

```text
trend_regime
range_regime
choppy_regime
breakout_regime
mean_reversion_regime
```

Volatility regimes:

```text
low_volatility
normal_volatility
high_volatility
extreme_volatility
volatility_expanding
volatility_contracting
```

Liquidity regimes:

```text
normal_liquidity
thin_liquidity
high_liquidity
spread_elevated
spread_normal
```

Behavioural regimes:

```text
momentum_regime
exhaustion_regime
compression_regime
expansion_regime
reversal_regime
continuation_regime
```

Composite regime label:

```text
market_regime_label = trend_direction + volatility_state + session_state
```

Examples:

```text
bullish_high_volatility_london
range_low_volatility_asia
bearish_expansion_ny
```

---

## 18. Spread, cost and execution features

Cost features:

```text
spread
spread_percent
spread_atr_ratio
spread_percentile_20
spread_percentile_100
commission_estimate
slippage_estimate
round_trip_cost
round_trip_cost_atr
```

Execution viability:

```text
expected_move_to_cost_ratio
target_distance_to_cost_ratio
stop_distance_to_cost_ratio
minimum_viable_r
cost_adjusted_expectancy
```

Liquidity filters:

```text
spread_too_wide
cost_too_high
illiquid_condition
poor_execution_environment
```

For scalping and short-duration policies, this group is critical.

---

## 19. Risk and trade viability features

Risk geometry:

```text
distance_to_stop_candidate
distance_to_target_candidate
risk_reward_ratio
expected_move_atr
stop_distance_atr
target_distance_atr
room_to_nearest_resistance
room_to_nearest_support
```

Trade quality:

```text
setup_quality_score
risk_reward_score
invalidation_clarity_score
target_clarity_score
trade_room_score
```

Avoidance features:

```text
too_close_to_resistance_for_long
too_close_to_support_for_short
stop_too_tight
stop_too_wide
target_too_close
risk_reward_unfavourable
```

---

## 20. Indicator features

Trend and momentum:

```text
macd_line
macd_signal
macd_histogram
macd_histogram_slope
adx
plus_di
minus_di
aroon_up
aroon_down
trix
```

Mean reversion:

```text
rsi
stochastic_k
stochastic_d
cci
williams_r
ultimate_oscillator
```

Volatility:

```text
atr
bollinger_band_width
keltner_width
standard_deviation
historical_volatility
chaikin_volatility
```

Volume:

```text
obv
mfi
chaikin_money_flow
accumulation_distribution
vwap
```

Channel/location:

```text
donchian_channel_position
bollinger_band_position
keltner_channel_position
price_channel_position
```

---

## 21. Divergence features

RSI divergence:

```text
bullish_rsi_divergence
bearish_rsi_divergence
hidden_bullish_rsi_divergence
hidden_bearish_rsi_divergence
```

MACD divergence:

```text
bullish_macd_divergence
bearish_macd_divergence
macd_momentum_failure
```

Volume divergence:

```text
price_up_volume_down
price_down_volume_down
obv_divergence
mfi_divergence
```

Structure divergence:

```text
higher_high_with_weaker_momentum
lower_low_with_weaker_momentum
```

Score-based versions:

```text
bullish_divergence_score
bearish_divergence_score
momentum_exhaustion_score
```

---

## 22. Gap features

Useful for equities, indices and futures.

```text
gap_up
gap_down
gap_size
gap_size_atr
gap_fill_percent
distance_to_gap_fill
gap_remains_open
gap_filled
```

Session gaps:

```text
open_gap_vs_previous_close
open_gap_vs_previous_day_high
open_gap_vs_previous_day_low
```

Gap behaviour:

```text
gap_and_go_candidate
gap_fade_candidate
gap_exhaustion_candidate
```

---

## 23. News and event proximity features

These are not candle-only, but they should eventually be included.

```text
minutes_until_high_impact_news
minutes_since_high_impact_news
news_event_today
central_bank_event_today
inflation_data_today
employment_data_today
earnings_today
inventory_report_today
fomc_day
cpi_day
nfp_day
```

Impact features:

```text
pre_news_volatility_compression
post_news_volatility_expansion
news_avoidance_window
event_risk_score
```

Initial implementation can use a simple economic-calendar flag and an avoidance window.

---

## 24. Correlation and intermarket features

For FX:

```text
dxy_return
dxy_trend
dxy_volatility
yield_return
risk_on_risk_off_score
```

For gold:

```text
dxy_inverse_correlation
real_yield_proxy
silver_correlation
oil_correlation
spx_correlation
```

For crypto:

```text
btc_market_direction
eth_market_direction
total_crypto_market_trend
funding_rate
open_interest_change
```

For indices:

```text
vix_level
vix_change
sector_breadth
advance_decline_ratio
bond_yield_change
```

Correlation metrics:

```text
rolling_correlation_20
rolling_correlation_50
correlation_breakdown
beta_to_reference_asset
relative_strength_vs_reference
```

---

## 25. Statistical features

Rolling statistics:

```text
rolling_mean_20
rolling_std_20
rolling_skew_20
rolling_kurtosis_20
rolling_median_20
rolling_iqr_20
```

Z-scores:

```text
price_zscore_20
return_zscore_20
volume_zscore_20
range_zscore_20
atr_zscore_100
```

Percentiles:

```text
close_percentile_20
range_percentile_20
volume_percentile_20
return_percentile_20
atr_percentile_100
```

Distribution behaviour:

```text
fat_tail_score
outlier_candle_score
abnormal_return_flag
abnormal_range_flag
```

---

## 26. Lagged features

Simple lagged values:

```text
close_lag_1
close_lag_2
close_lag_3
return_lag_1
return_lag_2
return_lag_3
rsi_lag_1
rsi_lag_3
atr_lag_1
vwap_distance_lag_1
volume_lag_1
spread_lag_1
```

Rolling lag windows:

```text
return_sequence_5
body_sequence_5
range_sequence_5
close_position_sequence_5
volume_sequence_5
```

For tree models, explicit lags are useful. For sequence models, the sequence may be fed directly.

---

## 27. Normalized instrument-specific features

ATR-normalized:

```text
body_atr_ratio
range_atr_ratio
wick_atr_ratio
distance_to_vwap_atr
distance_to_ema_atr
distance_to_support_atr
distance_to_resistance_atr
spread_atr_ratio
return_atr_ratio
```

Percentile-normalized by symbol:

```text
atr_percentile_by_symbol
volume_percentile_by_symbol
spread_percentile_by_symbol
range_percentile_by_symbol
trend_strength_percentile_by_symbol
```

These features help models generalise across instruments.

---

## 28. Training-only outcome features

These must not be used as live model inputs. They are for labelling and training dataset generation only.

Forward returns:

```text
future_return_1
future_return_3
future_return_5
future_return_10
future_return_20
future_max_high_10
future_min_low_10
future_max_favourable_excursion
future_max_adverse_excursion
```

Trade outcome simulation:

```text
hit_target_before_stop
hit_stop_before_target
time_to_target
time_to_stop
max_r_available
min_r_adverse
best_possible_r
worst_possible_r
```

Policy labelling:

```text
best_direction
best_entry_type
best_stop_type
best_target_type
best_management_type
best_policy_score
```

---

## 29. Composite scoring features

Composite scores are handcrafted summaries built from lower-level features.

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

Example:

```text
breakout_score =
  compression_score
  + range_boundary_pressure
  + volume_expansion_score
  + trend_alignment_score
  - spread_penalty
```

These should be transparent, versioned and backed by lower-level feature values.

---

## 30. Features grouped by trade behaviour

### Long continuation

```text
htf_bullish_bias
price_above_vwap
ema_stack_bullish
pullback_depth_normal
higher_low_formed
bullish_rejection_from_support
volume_confirms_upmove
room_to_resistance
```

### Short continuation

```text
htf_bearish_bias
price_below_vwap
ema_stack_bearish
pullback_depth_normal
lower_high_formed
bearish_rejection_from_resistance
volume_confirms_downmove
room_to_support
```

### Long reversal

```text
price_oversold
sweep_below_recent_low
bullish_rejection_candle
rsi_bullish_divergence
distance_below_vwap_atr
bearish_momentum_exhaustion
support_nearby
```

### Short reversal

```text
price_overbought
sweep_above_recent_high
bearish_rejection_candle
rsi_bearish_divergence
distance_above_vwap_atr
bullish_momentum_exhaustion
resistance_nearby
```

### Breakout long

```text
range_compression
close_near_range_high
volume_expansion
atr_expansion
breakout_above_range_high
retest_hold
htf_bullish_alignment
```

### Breakout short

```text
range_compression
close_near_range_low
volume_expansion
atr_expansion
breakout_below_range_low
retest_hold
htf_bearish_alignment
```

### No trade

```text
spread_too_high
volatility_too_low
volatility_too_extreme
price_mid_range
conflicting_htf_signals
near_major_news
poor_risk_reward
low_liquidity_session
choppy_market_score_high
```

---

## Storage recommendation

Store feature snapshots in a versioned structure:

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

## Highest-value initial feature families

The first implementation should focus on:

```text
ATR percentile
VWAP distance in ATR units
range compression
spread/cost ratio
session
higher-timeframe trend
recent swing structure
close position in candle range
relative volume
distance to support/resistance
```
