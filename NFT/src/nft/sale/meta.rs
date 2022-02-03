use near_sdk::serde::{Deserialize, Serialize};
use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
  pub name: String,
  pub amount: u64,
  pub price: U128,
  pub buy_max: u32,
  pub per_transaction_min: u32,
  pub per_transaction_max: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonSale {
  pub id: SaleId,
  pub name: String,
  pub amount: u64,
  pub price: U128,
  pub buy_max: u32,
  pub per_transaction_min: u32,
  pub per_transaction_max: u32,
  pub not_minted: u64,
  pub locked: bool,
  pub start_date: Option<u64>,
}

pub type SaleId = String;
