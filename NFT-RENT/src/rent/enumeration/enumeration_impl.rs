use crate::enumeration::RentFactoryEnumeration;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::meta::JsonRent;
use crate::base::RentFactory;
use crate::TokenId;

impl RentFactoryEnumeration for RentFactory {
  fn rents(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
    let keys = self.rents_pending.as_vector().to_vec();
    let start = u128::from(from_index.unwrap_or(U128(0)));

    keys.iter()
      .skip(start as usize)
      .take(limit.unwrap_or(0) as usize)
      .map(|token_id| self.enum_rent(&token_id).unwrap())
      .collect()
  }

  fn rents_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
    let ids = self.internal_tokens_for_account(&account_id);

    let start = u128::from(from_index.unwrap_or(U128(0)));
    ids.iter()
      .skip(start as usize)
      .take(limit.unwrap_or(0) as usize)
      .map(|token_id| self.enum_rent(&token_id).unwrap())
      .collect()
  }

  fn rents_by_ids(&self, ids: Vec<TokenId>) -> Vec<JsonRent> {
    ids.iter()
      .map(|token_id| self.enum_rent(&token_id).unwrap())
      .collect()
  }

  fn rents_supply_for_account(&self, account_id: AccountId) -> U128 {
    self.rents_per_account
      .get(&account_id)
      .map(|account_renst| U128::from(account_renst.len() as u128))
      .unwrap_or(U128(0))
  }

  fn rent(&self, token_id: TokenId) -> Option<JsonRent> {
    self.enum_rent(&token_id)
  }

  fn rented_tokens_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
    let ids = self.internal_rent_token_ids_for_account(&account_id);

    let start = u128::from(from_index.unwrap_or(U128(0)));
    ids.iter()
      .skip(start as usize)
      .take(limit.unwrap_or(0) as usize)
      .map(|token_id| self.enum_rent(&token_id).unwrap())
      .collect()
  }

  fn rented_tokens_ids_for_account(&self, account_id: AccountId) -> Vec<TokenId> {
    let ids = self.internal_rent_token_ids_for_account(&account_id);

    ids
  }

  fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> U128 {
    U128::from(self.internal_rent_token_ids_for_account(&account_id).len() as u128)
  }
}
