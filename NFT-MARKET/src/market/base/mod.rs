mod core_impl;
mod external;
mod internal;
mod events;
pub use self::events::*;

pub use self::core_impl::*;
use near_sdk::{AccountId, Promise, Balance};
use near_sdk::json_types::{U128};
use crate::market::{Sale, ContractAndTokenId};

pub trait MarketCore {
  fn market_remove_sale(&mut self, nft_contract_id: AccountId, token_id: String);

  // #[payable]
  fn market_update_price(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
    price: U128,
  ) ;

  // #[payable]
  fn market_offer(&mut self, nft_contract_id: AccountId, token_id: String) ;

  // #[private]
  fn market_add_bid(
    &mut self,
    contract_and_token_id: ContractAndTokenId,
    amount: Balance,
    ft_token_id: AccountId,
    buyer_id: AccountId,
    sale: &mut Sale,
  ) ;

  fn market_accept_offer(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
  );

  // #[private]
  fn market_process_purchase(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
    price: U128,
    buyer_id: AccountId,
  ) -> Promise;

  // #[private]
  fn market_resolve_purchase(
    &mut self,
    ft_token_id: AccountId,
    buyer_id: AccountId,
    sale: Sale,
    price: U128,
  ) -> U128;
}
