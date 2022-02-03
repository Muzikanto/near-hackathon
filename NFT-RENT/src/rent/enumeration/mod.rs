mod enumeration_impl;

use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::meta::JsonRent;
use crate::TokenId;

pub trait RentFactoryEnumeration {
  fn rents(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent>;
  fn rents_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent>;
  fn rents_by_ids(&self, ids: Vec<TokenId>) -> Vec<JsonRent>;
  fn rents_supply_for_account(&self, account_id: AccountId) -> U128;
  fn rent(&self, token_id: TokenId) -> Option<JsonRent>;
  fn rented_tokens_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent>;
  fn rented_tokens_ids_for_account(&self, account_id: AccountId) -> Vec<TokenId>;
  fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> U128;
}
