use crate::market::{SaleConditions, TokenId, Sale, hash_account_id};
use near_sdk::json_types::{U64};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedSet;
use crate::market::base::{StorageKey, log_market_create_sale};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::borsh::{BorshSerialize};
use crate::*;
use near_sdk::serde_json::from_str;

static DELIMETER: &str = "||";

/// approval callbacks from NFT Contracts

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub sale_conditions: SaleConditions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auction: Option<bool>,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    /// where we add the sale because we know nft owner can only call nft_approve

    fn nft_on_approve(
      &mut self,
      token_id: TokenId,
      owner_id: AccountId,
      approval_id: u64,
      msg: String,
    ) {
      let nft_contract_id = env::predecessor_account_id();
      let signer_id = env::signer_account_id();
      assert_ne!(
          nft_contract_id,
          signer_id,
          "nft_on_approve should only be called via cross-contract call"
      );
      assert_eq!(
          &owner_id,
          &signer_id,
          "owner_id should be signer_id"
      );

      let SaleArgs { sale_conditions, is_auction } = from_str(&msg).expect("Not valid SaleArgs");

      for (ft_token_id, _price) in sale_conditions.clone() {
          if !self.market.ft_token_ids.contains(&ft_token_id) {
              env::panic_str(
                  &format!("Token {} not supported by this market", ft_token_id).to_string(),
              );
          }
      }

      let bids = HashMap::new();

      let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
      let sale = Sale {
        owner_id: owner_id.clone().into(),
        approval_id,
        nft_contract_id: nft_contract_id.clone(),
        token_id: token_id.clone(),
        sale_conditions,
        bids,
        created_at: date_now(),
        is_auction: is_auction.unwrap_or(false),
      };
      self.market.sales.insert(
          &contract_and_token_id,
          &sale,
      );

      // extra for views

      let mut by_owner_id = self.market.by_owner_id.get(&owner_id).unwrap_or_else(|| {
          UnorderedSet::new(
              StorageKey::ByOwnerIdInner {
                  account_id_hash: hash_account_id(&owner_id),
              }
              .try_to_vec()
              .unwrap(),
          )
      });

      by_owner_id.insert(&contract_and_token_id);
      self.market.by_owner_id.insert(&owner_id, &by_owner_id);

      let mut by_nft_contract_id = self.market
          .by_nft_contract_id
          .get(&nft_contract_id)
          .unwrap_or_else(|| {
              UnorderedSet::new(
                  StorageKey::ByNFTContractIdInner {
                      account_id_hash: hash_account_id(&nft_contract_id),
                  }
                  .try_to_vec()
                  .unwrap(),
              )
          });
      by_nft_contract_id.insert(&token_id);
      self.market.by_nft_contract_id
          .insert(&nft_contract_id, &by_nft_contract_id);

      log_market_create_sale(&owner_id, &nft_contract_id, &token_id, &sale);
    }
}
