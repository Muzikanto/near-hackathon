pub use mint_impl::*;
use crate::nft::{Token, TokenId};
use near_sdk::AccountId;
use crate::SaleId;
use crate::nft::royalty::Royalty;
use crate::nft::metadata::{TokenType, TokenCollection, TokenRarity, TokenMetadata, TokenSubType};

pub mod mint_impl;
mod internal;

pub trait NonFungibleTokenMint {
  fn nft_create(
    &mut self,
    token_id: TokenId,
    receiver_id: Option<AccountId>,
    token_metadata: TokenMetadata,
    rarity: TokenRarity,
    collection: TokenCollection,
    token_type: TokenType,
    token_sub_type: Option<TokenSubType>,
    bind_to_owner: Option<bool>,
    sale_id: Option<SaleId>,
    perpetual_royalties: Option<Royalty>,
    fractionation_id: Option<TokenId>,
  ) -> Token;

  fn nft_mint(
    &mut self,
    receiver_id: AccountId, sale_id: SaleId, amount: u32
  );
}
