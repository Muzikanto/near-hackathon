use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{AccountId, env, CryptoHash, BorshStorageKey};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U128;
use crate::meta::{Rent, JsonRent};
use crate::base::RentFactory;
use crate::{TokenId, hash_account_id, date_now};
use crate::base::events::{log_rent_add, log_rent_remove};

pub const RENT_TIME_MIN: u64 = 899999; // min 15 min (900000)
pub const RENT_TIME_NAX: u64 = 8639999999; // max 100 days (8640000000)

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  RentTokensPerAccountInner { account_id_hash: CryptoHash },
  RentsPerAccountInner { account_id_hash: CryptoHash },
}

impl RentFactory {
  pub(crate) fn assert_owner(&self) {
    assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
  }

  pub(crate) fn assert_valid_time(&self, time: &u64) {
    assert!(time > &RENT_TIME_MIN, "minimum 15 min.");
    assert!(time < &RENT_TIME_NAX, "maximum 100 days.");
  }

  pub(crate) fn internal_rent_add(&mut self, token_id: &TokenId, account_id: &AccountId, price_per_hour: &U128, time: &u64, max_time: &u64) {
    let is_paid = self.rents_current.get(&token_id).is_some();

    assert!(!is_paid, "Token is already in rent");

    let rent = Rent {
      owner_id: account_id.clone(),
      token_id: token_id.clone(),
      price_per_hour: price_per_hour.clone(),
      min_time: time.clone(),
      max_time: max_time.clone(),
      created_at: date_now(),
    };

    self.rents_pending.insert(&token_id);
    self.rents_by_id.insert(&token_id, &rent);
    self.internal_add_rent_to_account(&account_id, &token_id);

    log_rent_add(&rent);
  }

  pub(crate) fn internal_rent_remove(&mut self, token_id: &TokenId, account_id: &AccountId) {
    let is_paid = self.rents_current.get(&token_id).is_some();

    assert!(!is_paid, "Token is already in rent");

    let rent = self.rents_by_id.get(&token_id).expect("Not found rent");

    assert_eq!(rent.owner_id, account_id.clone(), "Only owner can remove rent");

    self.rents_by_id.remove(&token_id);
    self.rents_pending.remove(&token_id);
    self.internal_remove_rent_from_account(&account_id, &token_id);

    log_rent_remove(&token_id, &account_id);
  }

  pub(crate) fn internal_add_token_to_account(
    &mut self,
    account_id: &AccountId,
    token_id: &TokenId,
  ) {
    let mut tokens_set = self.rent_tokens_per_account.get(&account_id).unwrap_or_else(|| {
      UnorderedSet::new(
        StorageKey::RentTokensPerAccountInner {
          account_id_hash: hash_account_id(&account_id),
        }
          .try_to_vec()
          .unwrap(),
      )
    });
    tokens_set.insert(&token_id);
    self.rent_tokens_per_account.insert(&account_id, &tokens_set);
  }

  pub(crate) fn internal_remove_token_from_account(
    &mut self,
    account_id: &AccountId,
    token_id: &TokenId,
  ) {
    let mut tokens_set = self
      .rent_tokens_per_account

      .get(&account_id)
      .expect("Token should be owned by the sender");

    tokens_set.remove(&token_id);

    if tokens_set.is_empty() {
      self.rent_tokens_per_account.remove(&account_id);
    } else {
      self.rent_tokens_per_account.insert(&account_id, &tokens_set);
    }
  }

  pub(crate) fn internal_add_rent_to_account(
    &mut self,
    account_id: &AccountId,
    token_id: &TokenId,
  ) {
    let mut rents_set = self.rents_per_account.get(account_id).unwrap_or_else(|| {
      UnorderedSet::new(
        StorageKey::RentsPerAccountInner {
          account_id_hash: hash_account_id(&account_id),
        }
          .try_to_vec()
          .unwrap(),
      )
    });
    rents_set.insert(token_id);
    self.rents_per_account.insert(account_id, &rents_set);
  }

  pub(crate) fn internal_remove_rent_from_account(
    &mut self,
    account_id: &AccountId,
    token_id: &TokenId,
  ) {
    self.rents_by_id.remove(&token_id);
    self.rents_current.remove(&token_id);

    let mut rents_set = self
      .rents_per_account

      .get(account_id)
      .expect("Rent should be owned by the sender");

    rents_set.remove(&token_id);

    if rents_set.is_empty() {
      self.rents_per_account.remove(account_id);
    } else {
      self.rents_per_account.insert(account_id, &rents_set);
    }

    self.rents_end_by_id.remove(&token_id);
  }

  pub(crate) fn internal_rent_is_ended(&self, token_id: &TokenId) -> bool {
    let rent_end_at = self.rents_end_by_id.get(&token_id).expect("Not found");
    let now = date_now();

    now > rent_end_at
  }

  pub(crate) fn enum_rent(&self, token_id: &TokenId) -> Option<JsonRent> {
    if let Some(rent) = self.rents_by_id.get(&token_id) {
      Some(JsonRent {
        token_id: rent.token_id,
        owner_id: rent.owner_id,
        price_per_hour: rent.price_per_hour,
        min_time: rent.min_time,
        max_time: rent.max_time,
        ended_at: self.rents_end_by_id.get(&token_id),
        renter_id: self.rents_current.get(&token_id),
        created_at: rent.created_at,
      })
    } else {
      None
    }
  }

  pub(crate) fn internal_rent_token_is_locked(&self, token_id: &TokenId) -> bool {
    self.rents_current.get(&token_id).is_some()
  }

  pub(crate) fn internal_rent_token_ids_for_account(&self, account_id: &AccountId) -> Vec<TokenId> {
    let tokens_account = self.rent_tokens_per_account.get(&account_id);
    let tokens = if let Some(tokens_account) = tokens_account {
      tokens_account
    } else {
      return vec![];
    };

    tokens
      .iter()
      .filter(|token_id| {
        !self.internal_rent_is_ended(&token_id)
      })
      .collect()
  }

  pub(crate) fn internal_tokens_for_account(&self, account_id: &AccountId) -> Vec<TokenId> {
    let rents_account = self.rents_per_account.get(&account_id);
    let rents = if let Some(rents_account) = rents_account {
      rents_account
    } else {
      return vec![];
    };

    rents.as_vector().to_vec()
  }
}
