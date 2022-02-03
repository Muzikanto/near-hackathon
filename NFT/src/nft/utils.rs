use near_sdk::{env, require, AccountId, Balance, CryptoHash, Promise};
use std::collections::HashMap;
use std::mem::size_of;

// TODO: need a way for end users to determine how much an approval will cost.
pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
  // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
  account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

pub fn refund_approved_account_ids_iter<'a, I>(
  account_id: AccountId,
  approved_account_ids: I,
) -> Promise
  where
    I: Iterator<Item = &'a AccountId>,
{
  let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();
  Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

pub fn refund_approved_account_ids(
  account_id: AccountId,
  approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
  refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

pub fn refund_deposit_to_account(storage_used: u64, account_id: AccountId) {
  let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
  let attached_deposit = env::attached_deposit();

  require!(
        required_cost <= attached_deposit,
        format!("Must attach {} yoctoNEAR to cover storage", required_cost)
    );

  let refund = attached_deposit - required_cost;
  if refund > 1 {
    Promise::new(account_id).transfer(refund);
  }
}

/// Assumes that the precedecessor will be refunded
pub fn refund_deposit(storage_used: u64) {
  refund_deposit_to_account(storage_used, env::predecessor_account_id())
}

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  let mut hash = CryptoHash::default();
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}

/// Assert that at least 1 yoctoNEAR was attached.
pub(crate) fn assert_at_least_one_yocto() {
  require!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR")
}

// custom
// https://github.com/ilblackdragon/dragonear/blob/main/src/dragon.rs
pub(crate) fn random_number(max: usize) -> usize {
  // array of 32 int (0..250), max 8000
  let seed = env::random_seed();
  let rand = usize::from(seed[0]) * usize::from(seed[1]);

  rand % max
}

pub(crate) fn date_now() -> u64 {
  env::block_timestamp() / 1000000
}
