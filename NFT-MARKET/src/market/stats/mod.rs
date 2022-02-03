mod stats_impl;

use near_sdk::json_types::U128;

pub trait MarketStats {
  fn market_avg_price(&self) -> U128;
  fn market_sum_price(&self) -> U128;
  fn market_floor_price(&self) -> U128;
  fn market_total_owners(&self) -> U128;
}
