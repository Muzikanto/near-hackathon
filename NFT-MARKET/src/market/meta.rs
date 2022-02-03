use near_sdk::AccountId;
use std::collections::HashMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::to_string;

pub type SaleConditions = HashMap<FungibleTokenId, U128>;
pub type Bids = HashMap<FungibleTokenId, Vec<Bid>>;
pub type TokenId = String;
// pub type SaleId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;
// TODO: Capital U128
pub type Payout = HashMap<AccountId, U128>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
  pub owner_id: AccountId,
  pub price: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
  pub owner_id: AccountId,
  pub approval_id: u64,
  pub nft_contract_id: AccountId,
  pub token_id: String,
  pub sale_conditions: SaleConditions,
  pub bids: Bids,
  pub created_at: u64,
  pub is_auction: bool,
}

impl ToString for Sale {
  fn to_string(&self) -> String {
    to_string(&Sale {
      owner_id: self.owner_id.clone(),
      approval_id: self.approval_id.clone(),
      nft_contract_id: self.nft_contract_id.clone(),
      token_id: self.token_id.clone(),
      sale_conditions: self.sale_conditions.clone(),
      bids: self.bids.clone(),
      created_at: self.created_at.clone(),
      is_auction: self.is_auction.clone(),
    }).ok().unwrap()
  }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseArgs {
  pub nft_contract_id: AccountId,
  pub token_id: TokenId,
}
