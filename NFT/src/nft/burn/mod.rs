pub use burn_impl::*;
use crate::TokenId;

pub mod burn_impl;
mod internal;

pub trait NonFungibleTokenBurn {
  fn nft_burn(&mut self, token_id: &TokenId);
}
