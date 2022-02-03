use crate::nft::{NonFungibleToken, TokenId, refund_approved_account_ids};
use near_sdk::json_types::U128;
use crate::nft::payout::{NonFungibleTokenPayout};
use crate::nft::royalty::royalty_to_payout;
use crate::royalty::{MINTER_ROYALTY_CAP, CONTRACT_ROYALTY_CAP};
use near_sdk::{env, assert_one_yocto, AccountId};
use std::collections::HashMap;
use crate::nft::events::NftPayout;

pub type Payout = HashMap<AccountId, U128>;

impl NonFungibleTokenPayout for NonFungibleToken {
  fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
    let royalty = self.token_royalty_by_id.as_ref().unwrap().get(&token_id).expect("No token");
    let owner_id = self.owner_by_id.get(&token_id).expect("No token");

    // compute payouts based on balance option
    // adds in contract_royalty and computes previous owner royalty from remainder
    let mut total_perpetual = 0;
    let balance_u128 = u128::from(balance);
    let mut payout: Payout = HashMap::new();

    assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

    for (k, v) in royalty.iter() {
      let key = k.clone();
      if key != owner_id {
        payout.insert(key, royalty_to_payout(*v, balance_u128));
        total_perpetual += *v;
      }
    }

    // payout to contract owner - may be previous token owner, they get remainder of balance
    if self.contract_royalty > 0 && self.owner_id != owner_id {
      payout.insert(self.owner_id.clone(), royalty_to_payout(self.contract_royalty, balance_u128));
      total_perpetual += self.contract_royalty;
    }
    assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
    // payout to previous owner
    payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

    payout
  }

  fn nft_transfer_payout(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: u64,
    memo: String,
    balance: U128,
    max_len_payout: u32,
  ) -> Payout {
    assert_one_yocto();
    let sender_id = env::predecessor_account_id();
    let (owner_id, approved_account_ids) = self.internal_transfer(
      &sender_id,
      &receiver_id,
      &token_id,
      Some(approval_id),
      Some(memo),
    );

    if let Some(approved_account_ids) = approved_account_ids {
      refund_approved_account_ids(
        owner_id.clone(),
        &approved_account_ids,
      );
    }

    // compute payouts based on balance option
    // adds in contract_royalty and computes previous owner royalty from remainder
    let mut total_perpetual = 0;
    let balance_u128 = u128::from(balance);
    let mut payout: Payout = HashMap::new();
    let royalty = self.token_royalty_by_id.as_ref().unwrap().get(&token_id);

    if let Some(royalty) = royalty {
      assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

      for (k, v) in royalty.iter() {
        let key = k.clone();
        if key != owner_id {
          payout.insert(key, royalty_to_payout(*v, balance_u128));
          total_perpetual += *v;
        }
      }

      // payout to contract owner - may be previous token owner, they get remainder of balance
      if self.contract_royalty > 0 && self.owner_id != owner_id {
        payout.insert(self.owner_id.clone(), royalty_to_payout(self.contract_royalty, balance_u128));
        total_perpetual += self.contract_royalty;
      }
      assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
      // payout to previous owner
      payout.insert(owner_id.clone(), royalty_to_payout(10000 - total_perpetual, balance_u128));
    } else {
      payout.insert(owner_id.clone(), U128::from(balance_u128));
    }

    NftPayout {
      token_id: &token_id,
      sender_id: &sender_id,
      receiver_id: &receiver_id,
      balance: &balance
    }.emit();

    payout
  }
}
