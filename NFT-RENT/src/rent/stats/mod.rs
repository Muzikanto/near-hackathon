mod stats_impl;

use near_sdk::json_types::U128;

pub trait RentFactoryStats {
  fn rent_avg_price(&self) -> U128;
  fn rent_current_total_supply(&self) -> U128;
  fn rent_floor_price(&self) -> U128;
  fn rent_total_owners(&self) -> U128;
}
