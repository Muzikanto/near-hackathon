use crate::nft::metadata::{TokenMetadata, TokenRarity, TokenCollection, TokenType, TokenSubType};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::HashMap;
use serde_json::{to_string};

/// Note that token IDs for NFTs are strings on NEAR. It's still fine to use autoincrementing numbers as unique IDs if desired, but they should be stringified. This is to make IDs more future-proof as chain-agnostic conventions and standards arise, and allows for more flexibility with considerations like bridging NFTs across chains, etc.
pub type TokenId = String;

/// In this implementation, the Token struct takes two extensions standards (metadata and approval) as optional fields, as they are frequently used in modern NFTs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub metadata: Option<TokenMetadata>,
  pub approved_account_ids: Option<HashMap<AccountId, u64>>,

  // custom
  pub sale_id: Option<TokenId>,
  pub royalty: Option<HashMap<AccountId, u32>>,

  pub collection: Option<TokenCollection>,
  pub token_type: Option<TokenType>,
  pub token_sub_type: Option<TokenSubType>,
  pub rarity: Option<TokenRarity>,

  pub bind_to_owner: Option<bool>,
  pub locked: Option<bool>,

  pub fractionation_id: Option<TokenId>,
}

impl ToString for Token {
  fn to_string(&self) -> String {
    to_string(&Token {
      token_id: self.token_id.to_string(),
      owner_id: self.owner_id.clone(),
      metadata: self.metadata.clone(),
      approved_account_ids: self.approved_account_ids.clone(),
      sale_id: self.sale_id.clone(),
      royalty: self.royalty.clone(),
      collection: self.collection.clone(),
      token_type: self.token_type.clone(),
      token_sub_type: self.token_sub_type.clone(),
      rarity: self.rarity.clone(),
      bind_to_owner: self.bind_to_owner.clone(),
      locked: self.locked.clone(),
      fractionation_id: self.fractionation_id.clone()
    }).ok().unwrap()
  }
}
