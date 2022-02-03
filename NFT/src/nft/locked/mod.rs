pub use locked_impl::*;
use crate::nft::TokenId;

pub mod locked_impl;

pub trait NonFungibleTokenLocked {
  fn nft_token_is_locked(&self, token_id: TokenId) -> bool;

  fn nft_on_lock(&mut self, token_id: TokenId, locked: bool);
}
