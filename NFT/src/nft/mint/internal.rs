use crate::nft::{NonFungibleToken, TokenRarity, TokenCollection, SaleId, Royalty, TokenId, TokenType, Token};
use crate::nft::metadata::{TokenMetadata, TokenSubType};
use near_sdk::{AccountId, env};
use near_sdk::collections::UnorderedSet;
use crate::nft::base::StorageKey;
use std::collections::HashMap;
use crate::nft::events::NftCreate;

impl NonFungibleToken {
  pub fn internal_create_nft(
    &mut self,
    token_id: &TokenId,
    token_owner_id: Option<AccountId>,
    token_metadata: Option<TokenMetadata>,
    rarity: TokenRarity,
    collection: TokenCollection,
    bind_to_owner: Option<bool>,
    sale_id: Option<SaleId>,
    perpetual_royalties: Option<Royalty>,
    fractionation_id: Option<TokenId>,
    token_type: TokenType,
    token_sub_type: Option<TokenSubType>,
  ) -> Token {
    let token = self.internal_create_nft_with_refund(
      token_id,
      token_owner_id,
      token_metadata,
      rarity,
      collection,
      bind_to_owner,
      sale_id,
      perpetual_royalties,
      fractionation_id,
      token_type,
      token_sub_type,
      Some(env::predecessor_account_id()),
    );

    token
  }

  /// Mint a new token without checking:
 /// * Whether the caller id is equal to the `owner_id`
 /// * `refund_id` will transfer the left over balance after storage costs are calculated to the provided account.
 ///   Typically the account will be the owner. If `None`, will not refund. This is useful for delaying refunding
 ///   until multiple tokens have been minted.
 ///
 /// Returns the newly minted token
  pub fn internal_create_nft_with_refund(
    &mut self,
    token_id: &TokenId,
    token_owner_id: Option<AccountId>,
    token_metadata: Option<TokenMetadata>,
    rarity: TokenRarity,
    collection: TokenCollection,
    bind_to_owner: Option<bool>,
    sale_id: Option<SaleId>,
    perpetual_royalties: Option<Royalty>,
    fractionation_id: Option<TokenId>,
    token_type: TokenType,
    token_sub_type: Option<TokenSubType>,
    _refund_id: Option<AccountId>,
  ) -> Token {
    // let prev_storage = env::storage_usage();
    // Remember current storage usage if refund_id is Some
    // let _initial_storage_usage = refund_id.map(|account_id| (account_id, env::storage_usage()));

    // assert!(sale_id.is_some(), "Require sale id");

    if self.token_metadata_by_id.is_some() && token_metadata.is_none() {
      env::panic_str("Must provide metadata");
    }
    if self.owner_by_id.get(&token_id).is_some() {
      env::panic_str("token_id must be unique");
    }
    if let Some(fractionation_id) = &fractionation_id {
      if sale_id.is_some() && &fractionation_id == &token_id {
        env::panic_str("Dont provide sale_id for fractionation token");
      }
    }
    if let Some(fractionation_id) = &fractionation_id {
      if &fractionation_id == &token_id {
        assert_eq!(self.fractionation_by_id.as_mut().unwrap().contains_key(&token_id), false, "Fractionation already exists");
      } else {
        assert!(self.fractionation_by_id.as_mut().unwrap().contains_key(&fractionation_id), "Not found fractionation");
      }
    }

    let mut owner_id = self.owner_id.clone();

    if let Some(token_owner_id) = token_owner_id {
      owner_id = token_owner_id;
    }

    // Core behavior: every token must have an owner
    self.owner_by_id.insert(&token_id, &owner_id);

    // Metadata extension: Save metadata, keep variable around to return later.
    // Note that check above already panicked if metadata extension in use but no metadata
    // provided to call.
    self.token_metadata_by_id
      .as_mut()
      .and_then(|by_id| by_id.insert(&token_id, token_metadata.as_ref().unwrap()));

    // custom
    self.token_rarity_by_id.as_mut().unwrap().insert(&token_id, &rarity);
    self.token_collection_by_id.as_mut().unwrap().insert(&token_id, &collection);
    self.token_type_by_id.as_mut().unwrap().insert(&token_id, &token_type);
    if let Some(bind_to_owner) = &bind_to_owner {
      self.internal_token_bind_to_owner(&token_id, &bind_to_owner);
    }
    if let Some(sale_id) = &sale_id {
      self.internal_sale_add_token(&sale_id, &token_id);
    }
    let royalty = self.royalty_calculate(perpetual_royalties);

    // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
        UnorderedSet::new(StorageKey::TokensPerOwner {
          account_hash: env::sha256(owner_id.as_bytes()),
        })
      });
      token_ids.insert(&token_id);
      tokens_per_owner.insert(&owner_id, &token_ids);
    }

    // Approval Management extension: return empty HashMap as part of Token
    let approved_account_ids =
      if self.approvals_by_id.is_some() { Some(HashMap::new()) } else { None };

    // if let Some((id, storage_usage)) = initial_storage_usage {
    // refund_deposit_to_account(env::storage_usage() - storage_usage, id)
    // }
    // Return any extra attached deposit not used for storage

    let token = Token {
      token_id: token_id.clone(),
      owner_id,
      metadata: token_metadata,
      approved_account_ids,
      sale_id,
      royalty: Some(royalty),
      collection: Some(collection),
      token_type: Some(token_type),
      token_sub_type: token_sub_type,
      rarity: Some(rarity),
      bind_to_owner,
      locked: None,
      fractionation_id: fractionation_id.clone(),
    };

    NftCreate {
      token: &token,
    }.emit();

    // after log
    if let Some(fractionation_id) = &fractionation_id {
      if &fractionation_id == &token_id {
        self.internal_create_fractionation(&token_id);
      } else {
        self.internal_add_token_to_fractionation(&token_id, &fractionation_id);
      }
    }

    // env::log_str(&format!("{}{}{}", &prev_storage.to_string(), ", ".to_string(), &env::storage_usage().to_string()));

    token
  }
}
