use near_sdk::{AccountId, Promise};
use near_sdk::json_types::U128;

use crate::TokenId;

mod base_impl;

mod internal;
mod events;

pub use self::base_impl::*;

pub trait RentFactoryCore {
  //
  fn rent_token_is_locked(&self, token_id: TokenId) -> bool;

  fn rent_update(&mut self, token_id: TokenId, account_id: AccountId, price_per_hour: U128, min_time: u64, max_time: u64);
  fn rent_remove(&mut self, token_id: TokenId, account_id: AccountId);

  // payable
  fn rent_pay(&mut self, token_id: TokenId, time: u64, receiver_id: AccountId) -> Promise;
  fn rent_claim(&mut self, token_id: TokenId, account_id: AccountId) -> Promise;

  fn rent_is_ended(&self, token_id: TokenId) -> bool;
  fn rent_total_supply(&self) -> u64;
}
pub trait RentFactoryResolve {
  fn rent_resolve_pay(&mut self, token_id: TokenId, owner_id: AccountId, receiver_id: AccountId, time: u64, end_time: u64, price: U128) -> U128;
  fn rent_resolve_claim(&mut self, token_id: TokenId, owner_id: AccountId, renter_id: AccountId);
}

