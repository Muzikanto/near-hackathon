use crate::{NonFungibleToken, TokenId};
use crate::nft::fractionation::{NonFungibleTokenFractionation, Fractionation};
use near_sdk::json_types::U128;
use near_sdk::{require, env};
use crate::nft::date_now;
use crate::nft::events::FractionationComplete;

impl NonFungibleTokenFractionation for NonFungibleToken {
  fn nft_fractionation(&self, token_id: TokenId) -> Fractionation {
    self.enum_fractionation(&token_id)
  }

  fn nft_fractionations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation> {
    let arr = self.fractionation_by_id.as_ref().unwrap();

    let start_index: u128 = from_index.map(From::from).unwrap_or_default();

    if (arr.len() as u128) <= start_index {
      return vec![];
    }

    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");

    let res = arr
      .iter()
      .skip(start_index as usize)
      .take(limit)
      .map(|(token_id, _entries)| self.enum_fractionation(&token_id))
      .collect();

    res
  }

  fn nft_fractionations_supply(&self) -> U128 {
    let count = self.fractionation_by_id.as_ref().unwrap().len();

    U128::from(count as u128)
  }

  fn nft_fractionation_complete(&mut self, token_id: TokenId) {
    let from = env::current_account_id();

    self.assert_available_transfer(&token_id);

    let fractionation = self.fractionation_by_id.as_ref().unwrap().get(&token_id).expect("Not found fractionation");

    fractionation.iter()
      .for_each(|token_id| {
        self.assert_token_holder(&token_id);
      });

    let sender_id = env::predecessor_account_id();

    // burn items
    fractionation.iter()
      .for_each(|token_id| {
        self.internal_burn_token_unguarded(&sender_id, &token_id);
      });

    // lock fractionation
    self.internal_remove_fractionation(&token_id);
    // transfer new token
    self.internal_transfer_unguarded(&token_id, &from, &sender_id);

    let date = date_now();
    self.fractionation_completed_by_id.as_mut().unwrap().insert(&token_id, &date);

    FractionationComplete {
      token_id: &token_id,
      from: &from,
      to: &sender_id,
      completed_at: &date
    }.emit();
  }
}
