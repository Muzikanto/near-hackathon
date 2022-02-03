use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;

pub type TokenId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rent {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub price_per_hour: U128,
  pub min_time: u64,
  pub max_time: u64,
  pub created_at: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonRent {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub price_per_hour: U128,
  pub min_time: u64,
  pub max_time: u64,
  pub created_at: u64,
  pub ended_at: Option<u64>,
  pub renter_id: Option<AccountId>,
}
