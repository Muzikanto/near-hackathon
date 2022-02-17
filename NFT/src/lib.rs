use near_sdk::{AccountId, BorshStorageKey, env, near_bindgen, PanicOnDefault, Promise, PromiseOrValue, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, TreeMap, UnorderedSet};
use near_sdk::json_types::U128;

use crate::nft::*;
use crate::nft::metadata::{TokenType, NFT_METADATA_SPEC, NonFungibleTokenMetadataProvider, TokenMetadata, TokenSubType};
use std::collections::HashMap;

mod nft;
mod event;

const DEFAULT_ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAABmJLR0QA/wD/AP+gvaeTAAACAUlEQVRIidXVOWhVURAG4C8vxmAwcSmCCIIiWrjggmBhYyfYiQoKoiDaCGKrha2FWAiCQqxcUlmlsBEttBAUJNHGQlwCbhGekSDRkOTF4szBm+vbsMvAcA+z/LOec1no1NlC340zuIQqtmMz1uE6luMlZv8n+GmMYS74aImzfAynGoFUGsjuYgD9bSTSj5u40wDvHzostWOuxKOYCh6to6/iUBmsPIM1WItH4bShkNWysO+Mc6YZDOFa+PzARFYWS6pIQxRZvm6z5E68CR/YUfQrVrBe2o6c7UVpi0jlDwQP4V1U2osObMOTCLIYv/C9HGAQu/AW+8Ipg1/Ae6kdM/iMp9iDJQE6jY84Fv6DInqmL1iFyQDpC/lVPG/Qnt04F+cJLEJPYK0Wgkwr4ttTAnnVALys6yucV+ZDW3vbhDpaGRQDjMd3UmHNsLWJ/5bCeSJ8iQEzv0Uj6MI9aUgHQn5EWtmfJfClocv0EA+kizqdhcUteia9K1P4hL3SdvRK21KNzLqwE2elpRDBb0QVw9LTMc78HlawPwBz+ee1ntMcLkuvag52H7UMmqlWMOrGpmzUgmax0d9LOdzKr9lj9zv4Qx19FQfbSEgFt+sANPofZL6lTjvr9beG4ziJb20kNBa2J9RpTbNf5oj0BH+Vtulx2NekQfbjSiTzoo1EFij9AUQdkBPH3hCPAAAAAElFTkSuQmCC";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  owner_id: AccountId,
  tokens: NonFungibleToken,
  metadata: LazyOption<NFTContractMetadata>,

  pub contract_royalty: u32,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
  // Nft
  NonFungibleToken,
  Metadata,
  TokenMetadata,
  Enumeration,
  Approval,
  // Nft custom
  NftLocked,
  NftBindToOwner,

  // Series
  SaleByToken,
  SaleTokens,
  SaleLocked,
  SaleRandomTokens,
  SaleMintCounter,
  SalesAvailable,

  // Royalty
  TokenRoyalty,

  // Extra info
  TokenRarity,
  TokenCollection,
  TokenType,
  TokenSubType,

  // Fractionation
  Fractionations,
  FractionationTokens,
  FractionationsIds,
  // FractionationsByOwner,
  FractionationsCompleted,

  // SaleTokensInner { sale_hash: Vec<u8> },
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
      NFTContractMetadata {
        spec: NFT_METADATA_SPEC.to_string(),
        name: "Mfight NFT".to_string(),
        symbol: "NCHK".to_string(),
        icon: Some(DEFAULT_ICON.to_string()),
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    metadata.assert_valid();

    let metadata = LazyOption::new(StorageKey::Metadata, Some(&metadata));
    let tokens = NonFungibleToken::new(
      StorageKey::NonFungibleToken,
      owner_id.clone(),
      Some(StorageKey::TokenMetadata),
      Some(StorageKey::Enumeration),
      Some(StorageKey::Approval),
      Some(StorageKey::NftLocked),
      Some(StorageKey::NftBindToOwner),

      Some(StorageKey::SaleByToken),
      Some(StorageKey::SaleTokens),
      Some(StorageKey::SaleLocked),
      Some(StorageKey::SaleRandomTokens),
      Some(StorageKey::SaleMintCounter),
      Some(StorageKey::SalesAvailable),

      Some(StorageKey::TokenRarity),
      Some(StorageKey::TokenCollection),
      Some(StorageKey::TokenType),
      Some(StorageKey::TokenSubType),

      Some(StorageKey::TokenRoyalty),

      Some(StorageKey::Fractionations),
      Some(StorageKey::FractionationTokens),
      Some(StorageKey::FractionationsIds),
      Some(StorageKey::FractionationsCompleted),
    );

    Self {
      owner_id: owner_id.clone(),
      tokens,
      metadata,
      contract_royalty: 0,
    }
  }

  pub fn admin_unlock_nft(&mut self, token_id: TokenId) {
    self.tokens.assert_owner();
    self.tokens.internal_nft_set_locked(&token_id, false);
  }

  pub fn test(&mut self, token_id: TokenId, title: String) -> TokenId {
    self.tokens.assert_owner();

    let metadata_by_id = self.tokens.token_metadata_by_id.as_mut().unwrap();
    let mut metadata = metadata_by_id.get(&token_id).expect("Not found");

    metadata.title = Some(title);

    metadata_by_id.insert(&token_id, &metadata);

    token_id
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldNonFungibleToken {
      // owner of contract
      pub owner_id: AccountId,

      // The storage size in bytes for each new token
      pub extra_storage_in_bytes_per_token: StorageUsage,

      // always required
      pub owner_by_id: TreeMap<TokenId, AccountId>,

      // required by metadata extension
      pub token_metadata_by_id: Option<LookupMap<TokenId, TokenMetadata>>,

      // required by enumeration extension
      pub tokens_per_owner: Option<LookupMap<AccountId, UnorderedSet<TokenId>>>,

      // required by approval extension
      pub approvals_by_id: Option<LookupMap<TokenId, HashMap<AccountId, u64>>>,
      pub next_approval_id_by_id: Option<LookupMap<TokenId, u64>>,

      // ====== Sales ======
      pub sale_by_token: Option<LookupMap<TokenId, SaleId>>,
      pub sale_tokens: Option<LookupMap<SaleId, UnorderedSet<TokenId>>>,
      pub sales_locked: Option<UnorderedSet<SaleId>>,
      pub sales_available: Option<UnorderedSet<SaleId>>,
      pub sale_by_id: Option<HashMap<SaleId, Sale>>,
      pub sale_date_by_id: Option<HashMap<SaleId, u64>>,
      pub sale_random_tokens: Option<LookupMap<SaleId, Vec<TokenId>>>,
      pub sale_mint_counter: Option<LookupMap<SaleId, LookupMap<AccountId, u32>>>,

      // ===== Royalty =====
      pub contract_royalty: u32,
      pub token_royalty_by_id: Option<LookupMap<TokenId, Royalty>>,

      // ===== Lock =====
      pub token_locked_by_id: Option<LookupMap<TokenId, bool>>,
      pub token_bind_by_id: Option<LookupMap<TokenId, bool>>,

      // ===== Extra =====
      pub token_rarity_by_id: Option<LookupMap<TokenId, TokenRarity>>,
      pub token_collection_by_id: Option<LookupMap<TokenId, TokenCollection>>,
      pub token_type_by_id: Option<LookupMap<TokenId, TokenType>>,
      pub token_sub_type_by_id: Option<LookupMap<TokenId, TokenSubType>>,

      // ===== Fractionation feature =====
      // key part -> new token
      pub fractionation_token_by_id: Option<LookupMap<TokenId, TokenId>>,
      pub fractionation_by_id: Option<TreeMap<TokenId, UnorderedSet<TokenId>>>,
      pub fractionation_ids: Option<UnorderedSet<TokenId>>,
      pub fractionation_completed_by_id: Option<LookupMap<TokenId, u64>>,
    }
    #[derive(BorshDeserialize)]
    struct Old {
      owner_id: AccountId,
      tokens: OldNonFungibleToken,
      metadata: LazyOption<NFTContractMetadata>,

      pub contract_royalty: u32,
    }

    let old: Old = env::state_read().expect("Error");

    let tokens = NonFungibleToken {
      // owner of contract
      owner_id: old.tokens.owner_id,

      // The storage size in bytes for each new token
      extra_storage_in_bytes_per_token: old.tokens.extra_storage_in_bytes_per_token,

      // always required
      owner_by_id: old.tokens.owner_by_id,

      // required by metadata extension
      token_metadata_by_id: old.tokens.token_metadata_by_id,

      // required by enumeration extension
      tokens_per_owner: old.tokens.tokens_per_owner,

      // required by approval extension
      approvals_by_id: old.tokens.approvals_by_id,
      next_approval_id_by_id: old.tokens.next_approval_id_by_id,

      // ====== Sales ======
      sale_by_token: old.tokens.sale_by_token,
      sale_tokens: old.tokens.sale_tokens,
      sales_locked: old.tokens.sales_locked,
      sales_available: old.tokens.sales_available,
      sale_by_id: old.tokens.sale_by_id,
      sale_date_by_id: old.tokens.sale_date_by_id,
      sale_random_tokens: old.tokens.sale_random_tokens,
      sale_mint_counter: old.tokens.sale_mint_counter,

      // ===== Royalty =====
      contract_royalty: old.tokens.contract_royalty,
      token_royalty_by_id: old.tokens.token_royalty_by_id,

      // ===== Lock =====
      token_locked_by_id: old.tokens.token_locked_by_id,
      token_bind_by_id: old.tokens.token_bind_by_id,

      // ===== Extra =====
      token_rarity_by_id: old.tokens.token_rarity_by_id,
      token_collection_by_id: old.tokens.token_collection_by_id,
      token_type_by_id: old.tokens.token_type_by_id,
      token_sub_type_by_id: old.tokens.token_sub_type_by_id,

      // ===== Fractionation feature =====
      // key part -> new token
      fractionation_token_by_id: old.tokens.fractionation_token_by_id,
      fractionation_by_id: old.tokens.fractionation_by_id,
      fractionation_ids: old.tokens.fractionation_ids,
      fractionation_completed_by_id: old.tokens.fractionation_completed_by_id,
    };

    Self {
      tokens,
      owner_id: old.owner_id,
      contract_royalty: old.contract_royalty,
      metadata: old.metadata,
    }
  }
}

impl_non_fungible_token_core!(Contract, tokens);
impl_non_fungible_token_approval!(Contract, tokens);
impl_non_fungible_token_enumeration!(Contract, tokens);
impl_non_fungible_token_payout!(Contract, tokens);
impl_non_fungible_token_royalty!(Contract, tokens);
impl_non_fungible_token_sales!(Contract, tokens);
impl_non_fungible_token_burn!(Contract, tokens);
impl_non_fungible_token_locked!(Contract, tokens);
impl_non_fungible_token_bind_to_owner!(Contract, tokens);
impl_non_fungible_token_fractionation!(Contract, tokens);
impl_non_fungible_token_mint!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
    self.metadata.get().unwrap()
  }
}

