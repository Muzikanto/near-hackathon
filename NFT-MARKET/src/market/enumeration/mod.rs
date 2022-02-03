mod enumeration_impl;

use near_sdk::json_types::{U64};
use near_sdk::AccountId;
use crate::market::{Sale, ContractAndTokenId};

pub trait MarketEnumeration {
  fn market_supply_sales(
    &self,
  ) -> U64;

  fn market_supply_by_owner_id(
    &self,
    account_id: AccountId,
  ) -> U64;

  fn market_sales_by_owner_id(
    &self,
    account_id: AccountId,
    from_index: U64,
    limit: u64,
  ) -> Vec<Sale>;

  fn market_supply_by_nft_contract_id(
    &self,
    nft_contract_id: AccountId,
  ) -> U64;

  fn market_sales_by_nft_contract_id(
    &self,
    nft_contract_id: AccountId,
    from_index: U64,
    limit: u64,
  ) -> Vec<Sale>;

  fn market_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale>;
}
