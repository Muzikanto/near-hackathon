use near_sdk::{AccountId, env, BorshStorageKey};
use near_sdk::collections::{UnorderedSet, LookupMap};
use crate::{SaleId};
use crate::nft::{TokenId, NonFungibleToken, JsonSale};
use near_sdk::borsh::{self, BorshSerialize};

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  SaleTokensInner { sale_hash: Vec<u8> },
  MintCounterPerSale { sale_hash: Vec<u8> },
}

impl NonFungibleToken {
  pub(crate) fn assert_sale_not_locked(&self, sale_id: &SaleId) {
    let is_locked = self.sales_locked.as_ref().unwrap().contains(&sale_id);

    if is_locked {
      env::panic_str("Sale is locked");
    }
  }

  pub(crate) fn assert_sale_started(&self, sale_id: &SaleId) {
    let date = self.sale_date_by_id.as_ref().unwrap().get(sale_id).expect("Not found sale");
    let now = env::block_timestamp();

    if &now < date {
      env::panic_str("Sale is not started");
    }
  }

  pub(crate) fn enum_get_sale(&self, sale_id: &SaleId) -> JsonSale {
    let sale = self.sale_by_id.as_ref().unwrap().get(sale_id).expect("Not found sale");
    let locked = self.sales_locked.as_ref().unwrap().contains(sale_id);
    let date = self.sale_date_by_id.as_ref().unwrap().get(sale_id);

    let mut start_date: Option<u64> = None;
    let mut not_minted = sale.amount;
    let rand_tokens = self.sale_random_tokens.as_ref().unwrap().get(sale_id);
    if let Some(rand_tokens) = rand_tokens {
      not_minted = rand_tokens.len() as u64;
    }
    if let Some(date) = date {
      start_date = Some(date.clone());
    }

    JsonSale {
      id: sale_id.clone(),
      name: sale.name.clone(),
      price: sale.price.clone(),
      buy_max: sale.buy_max,
      per_transaction_min: sale.per_transaction_min,
      per_transaction_max: sale.per_transaction_max,
      amount: sale.amount,
      not_minted,
      locked,
      start_date,
    }
  }

  pub fn internal_sale_add_token(&mut self, sale_id: &SaleId, token_id: &TokenId) {
    let sale_by_token = self.sale_by_token.as_mut().unwrap();
    let tokens_per_sale = self.sale_tokens.as_mut().unwrap();

    let sale_tokens = &mut tokens_per_sale.get(&sale_id).unwrap_or_else(|| {
      UnorderedSet::new(StorageKey::SaleTokensInner {
        sale_hash: env::sha256(sale_id.as_bytes()),
      })
    });

    sale_tokens.insert(&token_id);
    tokens_per_sale.insert(&sale_id, &sale_tokens);

    sale_by_token.insert(&token_id, &sale_id);

    assert_ne!(sale_tokens.len(), 0, "{}", &format!("Token does not added to sale {}", &token_id.to_string()));
  }

  pub(crate) fn internal_mint_counter_change(&mut self, owner_id: &AccountId, sale_id: &SaleId, value: &u32) {
    if let Some(mint_counter) = &mut self.sale_mint_counter {
      let mut sale_accounts = mint_counter.get(&sale_id).unwrap_or_else(|| {
        LookupMap::new(StorageKey::MintCounterPerSale {
          sale_hash: env::sha256(sale_id.as_bytes()),
        })
      });
      sale_accounts.insert(&owner_id, &value);
      mint_counter.insert(&sale_id, &sale_accounts);
    }
  }

  pub(crate) fn internal_mint_counter_by_sale(&self, owner_id: &AccountId, sale_id: &SaleId) -> u32 {
    let sale_accounts = self.sale_mint_counter.as_ref().unwrap().get(&sale_id).unwrap_or_else(||
      LookupMap::new(StorageKey::MintCounterPerSale {
        sale_hash: env::sha256(sale_id.as_bytes()),
      })
    );

    sale_accounts.get(&owner_id).unwrap_or_else(|| 0)
  }

  // pub fn internal_nft_sale_burn(&mut self, sale_id: &SaleId) {
  //   self.assert_owner();
  //
  //   self.sale_tokens.as_mut().unwrap().remove(&sale_id);
  //   self.sale_random_tokens.as_mut().unwrap().remove(&sale_id);
  //   self.sales_locked.as_mut().unwrap().remove(&sale_id);
  //   self.sale_mint_counter.as_mut().unwrap().remove(&sale_id);
  //   self.sale_by_id.as_mut().unwrap().remove(sale_id);
  //   self.sale_date_by_id.as_mut().unwrap().remove(sale_id);
  // }
}
