#[macro_export]
macro_rules! impl_rent_core {
    ($contract: ident, $tokens: ident) => {
        use $crate::{RentFactoryCore, RentFactoryResolve};
        use near_sdk::json_types::U128;
        use near_sdk::{Promise};

        #[near_bindgen]
        impl RentFactoryCore for $contract {
          //
          fn rent_token_is_locked(&self, token_id: TokenId) -> bool {
            self.$tokens.rent_token_is_locked(token_id)
          }
          #[payable]
          fn rent_add(&mut self, token_id: TokenId, account_id: AccountId, price_per_hour: U128, min_time: u64, max_time: u64) {
            self.$tokens.rent_add(token_id, account_id, price_per_hour, min_time, max_time)
          }
          #[payable]
          fn rent_remove(&mut self, token_id: TokenId, account_id: AccountId) {
            self.$tokens.rent_remove(token_id, account_id)
          }

          #[payable]
          fn rent_pay(&mut self, token_id: TokenId, time: u64, receiver_id: AccountId) -> Promise {
            self.$tokens.rent_pay(token_id, time, receiver_id)
          }
          #[payable]
          fn rent_claim(&mut self, token_id: TokenId, account_id: AccountId) -> Promise {
            self.$tokens.rent_claim(token_id, account_id)
          }
          fn rent_is_ended(&self, token_id: TokenId) -> bool {
            self.$tokens.rent_is_ended(token_id)
          }
          fn rent_total_supply(&self) -> u64 {
            self.$tokens.rent_total_supply()
          }
        }

          #[near_bindgen]
        impl RentFactoryResolve for $contract {
          //
          #[private]
          fn rent_resolve_pay(&mut self, token_id: TokenId, owner_id: AccountId, receiver_id: AccountId, time: u64, end_time: u64, price: U128) -> U128  {
            self.$tokens.rent_resolve_pay(token_id, owner_id, receiver_id, time, end_time, price)
          }
          #[private]
          fn rent_resolve_claim(&mut self, token_id: TokenId, owner_id: AccountId, renter_id: AccountId) {
            self.$tokens.rent_resolve_claim(token_id, owner_id, renter_id)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_rent_enumeration {
    ($contract: ident, $tokens: ident) => {
        use $crate::RentFactoryEnumeration;

        #[near_bindgen]
        impl RentFactoryEnumeration for $contract {
          fn rents(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
           self.$tokens.rents(from_index, limit)
          }
          fn rents_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
           self.$tokens.rents_for_account(account_id, from_index, limit)
          }
          fn rents_supply_for_account(&self, account_id: AccountId) -> U128 {
            self.$tokens.rents_supply_for_account(account_id)
          }
           fn rent(&self, token_id: TokenId) -> Option<JsonRent> {
           self.$tokens.rent(token_id)
          }
          fn rented_tokens_for_account(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRent> {
            self.$tokens.rented_tokens_for_account(account_id, from_index, limit)
          }
          fn rented_tokens_ids_for_account(&self, account_id: AccountId) -> Vec<TokenId> {
            self.$tokens.rented_tokens_ids_for_account(account_id)
          }
          fn rented_tokens_supply_for_account(&self, account_id: AccountId) -> U128 {
            self.$tokens.rented_tokens_supply_for_account(account_id)
          }
          fn rents_by_ids(&self, ids: Vec<TokenId>) -> Vec<JsonRent> {
            self.$tokens.rents_by_ids(ids)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_rent_stats {
    ($contract: ident, $tokens: ident) => {
        use $crate::RentFactoryStats;

        #[near_bindgen]
        impl RentFactoryStats for $contract {
          fn rent_avg_price(&self) -> U128 {
            self.$tokens.rent_avg_price()
          }
          fn rent_current_total_supply(&self) -> U128 {
            self.$tokens.rent_current_total_supply()
          }
          fn rent_floor_price(&self) -> U128 {
            self.$tokens.rent_floor_price()
          }
          fn rent_total_owners(&self) -> U128 {
            self.$tokens.rent_total_owners()
          }
        }
    };
}
