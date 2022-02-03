pub use payout_impl::*;
use crate::nft::TokenId;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

pub mod payout_impl;

pub trait NonFungibleTokenPayout {
  fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout;

  fn nft_transfer_payout(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: u64,
    memo: String,
    balance: U128,
    max_len_payout: u32,
  ) -> Payout;
}
