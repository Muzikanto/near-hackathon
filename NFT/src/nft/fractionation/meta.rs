use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::nft::{TokenId, TokenCollection, TokenRarity, TokenType};
use crate::nft::metadata::TokenMetadata;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Fractionation {
  pub token_id: TokenId,
  pub entries: Vec<TokenId>,
  pub completed_at: Option<u64>,

  // custom

  pub metadata: Option<TokenMetadata>,
  pub collection: Option<TokenCollection>,
  pub token_type: Option<TokenType>,
  pub rarity: Option<TokenRarity>,
}
