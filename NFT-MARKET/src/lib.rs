use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, PanicOnDefault,
    Promise, PromiseOrValue, CryptoHash, BorshStorageKey,
};

use crate::market::*;
use crate::market::base::MarketFactory;

mod market;
mod ft_callbacks;
mod nft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub market: MarketFactory
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    ByNFTTokenType,
    ByNFTTokenTypeInner { token_type_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_with_default_meta(owner_id: AccountId) -> Self {
      Self::new(
        owner_id,
      )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id: owner_id.clone().into(),
            market: MarketFactory::new(owner_id.clone(), None, None),
        };

        this
    }

    /// only owner
    pub fn add_ft_token_ids(&mut self, ft_token_ids: Vec<FungibleTokenId>) -> Vec<bool> {
      self.market.assert_owner();
      let mut added = vec![];
      for ft_token_id in ft_token_ids {
        added.push(self.market.ft_token_ids.insert(&ft_token_id));
      }
      added
    }

    /// views

    pub fn supported_ft_token_ids(&self) -> Vec<AccountId> {
    self.market.ft_token_ids.to_vec()
  }

}

impl_market_core!(Contract, market);
impl_market_enumeration!(Contract, market);
