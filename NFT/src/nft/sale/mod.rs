pub use sale_impl::*;
pub use meta::*;
use crate::nft::{TokenId, Token};
use near_sdk::json_types::U128;
use near_sdk::AccountId;

mod internal;
pub mod meta;
pub mod sale_impl;

pub trait SaleCore {
  fn nft_sale_add(&mut self, id: String, name: String, amount: u64, price: U128, per_transaction_min: u32, per_transaction_max: u32, buy_max: u32) -> JsonSale;

  fn nft_sale_start(&mut self, sale_id: SaleId, date: u64) -> JsonSale;

  fn nft_sale_update(&mut self, sale_id: SaleId, date: u64) -> JsonSale;
}

pub trait SaleEnumeration {
  fn nft_sale_tokens(
    &self,
    sale_id: SaleId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<Token>;

  fn nft_sales(&self) -> Vec<JsonSale>;

  fn nft_sale(&self, sale_id: SaleId) -> JsonSale;

  fn nft_sale_not_minted(&self, sale_id: SaleId) -> u64;

  fn nft_sales_locked(&self) -> Vec<String>;

  fn nft_sale_token_locked(&self, token_id: TokenId) -> bool;

  fn nft_sale_token_ids(
    &self,
    sale_id: SaleId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<TokenId>;

  fn nft_sale_account_minted(&self, sale_id: SaleId, account_id: AccountId) -> u32;
}
