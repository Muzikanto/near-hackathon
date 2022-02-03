use crate::nft::{NonFungibleToken, TokenId, Fractionation};
use near_sdk::collections::UnorderedSet;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{BorshStorageKey, env};
use crate::nft::events::{FractionationCreate, FractionationAddToken};

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  FractionationTokensInner { token_hash: Vec<u8> },
}

impl NonFungibleToken {
  pub(crate) fn assert_token_not_fractionation(&self, token_id: &TokenId) {
    let fractionation = self.fractionation_token_by_id.as_ref().unwrap().get(&token_id);

    if let Some(_fractionation) = fractionation {
      env::panic_str("The token belongs to fractionation");
    }
  }

  pub fn enum_fractionation(&self, token_id: &TokenId) -> Fractionation {
    let entries = self.fractionation_by_id.as_ref().unwrap().get(&token_id).expect("Not found fractionation");
    let completed_at = self.fractionation_completed_by_id.as_ref().unwrap().get(&token_id);

    let collection = self.token_collection_by_id.as_ref().unwrap().get(&token_id);
    let token_type = self.token_type_by_id.as_ref().unwrap().get(&token_id);
    let rarity = self.token_rarity_by_id.as_ref().unwrap().get(&token_id);
    let metadata = self.token_metadata_by_id.as_ref().unwrap().get(&token_id);

    Fractionation {
      token_id: token_id.clone(),
      entries: entries.to_vec(),
      completed_at,
      metadata,
      collection,
      token_type,
      rarity,
    }
  }

  pub fn internal_remove_fractionation(&mut self, token_id: &TokenId) {
    self.fractionation_ids.as_mut().unwrap().remove(&token_id);
  }

  pub fn internal_add_token_to_fractionation(&mut self, token_id: &TokenId, fractionation_id: &TokenId) {
    let fractionation_by_id = self.fractionation_by_id.as_mut().unwrap();

    let mut fractionation = fractionation_by_id.get(&fractionation_id).expect("Not found fractionation");

    fractionation.insert(&token_id);
    fractionation_by_id.insert(&fractionation_id, &fractionation);
    self.fractionation_token_by_id.as_mut().unwrap().insert(&token_id, &fractionation_id);

    FractionationAddToken {
      fractionation_id,
      token_id
    }.emit();
  }

  pub fn internal_create_fractionation(&mut self, token_id: &TokenId) {
    let fractionation_by_id = self.fractionation_by_id.as_mut().unwrap();

    assert_eq!(fractionation_by_id.contains_key(&token_id), false, "Fractionation already exists");

    let fractionation = UnorderedSet::new(StorageKey::FractionationTokensInner {
      token_hash: env::sha256(token_id.as_bytes()),
    });

    fractionation_by_id.insert(&token_id, &fractionation);
    self.fractionation_ids.as_mut().unwrap().insert(&token_id);

    FractionationCreate {
      token_id,
      owner_id: &env::current_account_id(),
    }.emit();
  }

  // pub(crate) fn internal_nft_fractionation_burn(&mut self, fractionation_id: &TokenId) {
  //   self.assert_owner();
  //
  //   let fractionation_by_id = self.fractionation_by_id.as_mut().unwrap();
  //   let fractionation_token_by_id = self.fractionation_token_by_id.as_mut().unwrap();
  //   let fractionation = fractionation_by_id.get(&fractionation_id).unwrap();
  //
  //   fractionation.iter().for_each(|token_id| {
  //     fractionation_token_by_id.remove(&token_id.clone());
  //   });
  //
  //   fractionation_token_by_id.remove(&fractionation_id);
  //   fractionation_by_id.remove(&fractionation_id);
  // }
}
