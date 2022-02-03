pub use bind_to_owner_impl::*;
use crate::nft::TokenId;

pub mod bind_to_owner_impl;

pub trait NonFungibleTokenBindToOwner {
  fn nft_token_is_bind_to_owner(&self, token_id: TokenId) -> bool;
}
