use near_sdk::serde::{Serialize};

use near_sdk::AccountId;
use crate::events::log_event;
use crate::{TokenId, stringify, Rent};
use near_sdk::json_types::U128;

const EVENT_RENT_ADD: &str = "rent_add";
const EVENT_RENT_UPDATE: &str = "rent_update";
const EVENT_RENT_REMOVE: &str = "rent_remove";
const EVENT_RENT_RESOLVE_PAY: &str = "rent_resolve_pay";
const EVENT_RENT_PAY: &str = "rent_pay";
const EVENT_RENT_CLAIM: &str = "rent_claim";

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventRentCreateData {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub price_per_hour: U128,
  pub min_time: u64,
  pub max_time: u64,
  pub created_at: u64,
}
pub(crate) fn log_rent_add(rent: &Rent) {
  log_event(&EVENT_RENT_ADD.to_string(), stringify(&EventRentCreateData {
    token_id: rent.token_id.to_string(),
    owner_id: rent.owner_id.clone(),
    price_per_hour: rent.price_per_hour.clone(),
    min_time: rent.min_time.clone(),
    max_time: rent.max_time.clone(),
    created_at: rent.created_at.clone(),
  }));
}
pub(crate) fn log_rent_update(rent: &Rent) {
  log_event(&EVENT_RENT_UPDATE.to_string(), stringify(&EventRentCreateData {
    token_id: rent.token_id.to_string(),
    owner_id: rent.owner_id.clone(),
    price_per_hour: rent.price_per_hour.clone(),
    min_time: rent.min_time.clone(),
    max_time: rent.max_time.clone(),
    created_at: rent.created_at.clone(),
  }));
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventRentRemoveData {
  pub token_id: TokenId,
  pub account_id: AccountId,
}
pub(crate) fn log_rent_remove(token_id: &TokenId, account_id: &AccountId) {
  log_event(&EVENT_RENT_REMOVE.to_string(), stringify(&EventRentRemoveData {
    token_id: token_id.to_string(),
    account_id: account_id.clone(),
  }));
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventRentPayData {
  pub token_id: TokenId,
  pub owner_id: AccountId,
}
pub(crate) fn log_rent_pay(token_id: &TokenId, owner_id: &AccountId) {
  log_event(&EVENT_RENT_PAY.to_string(), stringify(&EventRentPayData {
    token_id: token_id.to_string(),
    owner_id: owner_id.clone(),
  }));
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventRentResolvePayData {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub receiver_id: AccountId,
  pub time: u64,
  pub ended_at: u64,
  pub price: U128,
}
pub(crate) fn log_rent_resolve_pay(token_id: &TokenId, owner_id: &AccountId, receiver_id: &AccountId, time: &u64, ended_at: &u64, price: &U128) {
  log_event(&EVENT_RENT_RESOLVE_PAY.to_string(), stringify(&EventRentResolvePayData {
    token_id: token_id.to_string(),
    owner_id: owner_id.clone(),
    receiver_id: receiver_id.clone(),
    time: time.clone(),
    ended_at: ended_at.clone(),
    price: price.clone(),
  }));
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventRentClaimData {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub renter_id: AccountId,
}
pub(crate) fn log_rent_claim(token_id: &TokenId, owner_id: &AccountId, renter_id: &AccountId) {
  log_event(&EVENT_RENT_CLAIM.to_string(), stringify(&EventRentClaimData {
    token_id: token_id.to_string(),
    owner_id: owner_id.clone(),
    renter_id: renter_id.clone(),
  }));
}
