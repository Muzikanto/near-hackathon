use crate::nft::{NonFungibleToken};
use std::collections::HashMap;
use near_sdk::{AccountId, Balance};
use near_sdk::json_types::U128;
use crate::nft::royalty::NonFungibleTokenRoyalty;

pub const MINTER_ROYALTY_CAP: u32 = 2000;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000;

pub type Royalty = HashMap<AccountId, u32>;

pub(crate) fn royalty_to_payout(a: u32, b: Balance) -> U128 {
  U128(a as u128 * b / 10_000u128)
}

impl NonFungibleToken {
  pub(crate) fn royalty_calculate(&self, perpetual_royalties: Option<Royalty>) -> HashMap<AccountId, u32> {
    let mut royalty = HashMap::new();
    let mut total_perpetual = 0;
    // user added perpetual_royalties (percentage paid with every transfer)
    if let Some(perpetual_royalties) = perpetual_royalties {
      assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");
      for (account, amount) in perpetual_royalties {
        royalty.insert(account, amount);
        total_perpetual += amount;
      }
    }

    assert!(total_perpetual <= MINTER_ROYALTY_CAP, "Perpetual royalties cannot be more than 20%");

    royalty

  }
}

impl NonFungibleTokenRoyalty for NonFungibleToken {
  fn set_contract_royalty(&mut self, contract_royalty: u32) {
    self.assert_owner();
    assert!(contract_royalty <= CONTRACT_ROYALTY_CAP, "Contract royalties limited to 10% for owner");
    self.contract_royalty = contract_royalty;
  }
}
