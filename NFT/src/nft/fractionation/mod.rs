pub use fractionation_impl::*;
pub use meta::*;
use crate::TokenId;
use near_sdk::json_types::U128;

pub mod meta;
pub mod fractionation_impl;
mod internal;

pub trait NonFungibleTokenFractionation {
  fn nft_fractionation(&self, token_id: TokenId) -> Fractionation;
  fn nft_fractionations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation>;
  fn nft_fractionations_supply(&self) -> U128;
  fn nft_fractionation_complete(&mut self, token_id: TokenId);
}
