use super::resolver::NonFungibleTokenResolver;
use crate::nft::base::NonFungibleTokenCore;
use crate::nft::metadata::{TokenMetadata, TokenRarity, TokenCollection, TokenType, TokenSubType};
use crate::nft::token::{Token, TokenId};
use crate::nft::utils::{
  hash_account_id, refund_approved_account_ids,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8};
use near_sdk::{assert_one_yocto, env, ext_contract, log, require, AccountId, Balance, Gas, IntoStorageKey, PromiseOrValue, PromiseResult, StorageUsage, BorshStorageKey, CryptoHash};
use std::collections::HashMap;
use crate::{SaleId, Sale};
use crate::nft::royalty::Royalty;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

const NO_DEPOSIT: Balance = 0;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  TokensPerOwner { account_hash: Vec<u8> },
  TokenPerOwnerInner { account_id_hash: CryptoHash },
}

#[ext_contract(ext_self)]
trait NFTResolver {
  fn nft_resolve_transfer(
    &mut self,
    previous_owner_id: AccountId,
    receiver_id: AccountId,
    token_id: TokenId,
    approved_account_ids: Option<HashMap<AccountId, u64>>,
  ) -> bool;
}

#[ext_contract(ext_receiver)]
pub trait NonFungibleTokenReceiver {
  /// Returns true if token should be returned to `sender_id`
  fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool>;
}

/// Implementation of the non-fungible token standard.
/// Allows to include NEP-171 compatible token to any contract.
/// There are next traits that any contract may implement:
///     - NonFungibleTokenCore -- interface with nft_transfer methods. NonFungibleToken provides methods for it.
///     - NonFungibleTokenApproval -- interface with nft_approve methods. NonFungibleToken provides methods for it.
///     - NonFungibleTokenEnumeration -- interface for getting lists of tokens. NonFungibleToken provides methods for it.
///     - NonFungibleTokenMetadata -- return metadata for the token in NEP-177, up to contract to implement.
///
/// For example usage, see examples/non-fungible-token/src/lib.rs.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleToken {
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


impl NonFungibleToken {
  pub fn new<Q, R, S, T, L1, L2, S1, S2, S3, S4, S5, S6, P1, E1, E2, E3, E4, F1, F2, F3, F4>(
    owner_by_id_prefix: Q,
    owner_id: AccountId,
    token_metadata_prefix: Option<R>,
    enumeration_prefix: Option<S>,
    approval_prefix: Option<T>,

    token_locked_prefix: Option<L1>,
    token_bind_prefix: Option<L2>,

    sale_by_token_prefix: Option<S1>,
    tokens_per_sale_prefix: Option<S2>,
    sales_locked_prefix: Option<S3>,
    random_tokens_prefix: Option<S4>,
    mint_counter_prefix: Option<S5>,
    sales_available_prefix: Option<S6>,

    token_rarity_prefix: Option<E1>,
    token_collection_prefix: Option<E2>,
    token_type_prefix: Option<E3>,
    token_sub_type_prefix: Option<E4>,

    token_royalty_prefix: Option<P1>,

    fractionation_prefix: Option<F1>,
    fractionation_tokens_prefix: Option<F2>,
    fractionation_available_prefix: Option<F3>,
    fractionation_completed_prefix: Option<F4>,
  ) -> Self
    where
      Q: IntoStorageKey,
      R: IntoStorageKey,
      S: IntoStorageKey,
      T: IntoStorageKey,
      L1: IntoStorageKey,
      L2: IntoStorageKey,

      S1: IntoStorageKey,
      S2: IntoStorageKey,
      S3: IntoStorageKey,
      S4: IntoStorageKey,
      S5: IntoStorageKey,
      S6: IntoStorageKey,

      E1: IntoStorageKey,
      E2: IntoStorageKey,
      E3: IntoStorageKey,
      E4: IntoStorageKey,

      P1: IntoStorageKey,

      F1: IntoStorageKey,
      F2: IntoStorageKey,
      F3: IntoStorageKey,
      F4: IntoStorageKey,
  {
    let (approvals_by_id, next_approval_id_by_id) = if let Some(prefix) = approval_prefix {
      let prefix: Vec<u8> = prefix.into_storage_key();
      (
        Some(LookupMap::new(prefix.clone())),
        Some(LookupMap::new([prefix, "n".into()].concat())),
      )
    } else {
      (None, None)
    };
    let mut this = Self {
      owner_id,
      extra_storage_in_bytes_per_token: 0,
      owner_by_id: TreeMap::new(owner_by_id_prefix),
      token_metadata_by_id: token_metadata_prefix.map(LookupMap::new),
      tokens_per_owner: enumeration_prefix.map(LookupMap::new),
      approvals_by_id,
      next_approval_id_by_id,

      // custom
      token_locked_by_id: token_locked_prefix.map(LookupMap::new),
      token_bind_by_id: token_bind_prefix.map(LookupMap::new),

      // series
      sale_by_token: sale_by_token_prefix.map(LookupMap::new),
      sale_tokens: tokens_per_sale_prefix.map(LookupMap::new),
      sales_locked: sales_locked_prefix.map(UnorderedSet::new),
      sale_by_id: Some(HashMap::new()),
      sale_date_by_id: Some(HashMap::new()),
      sale_random_tokens: random_tokens_prefix.map(LookupMap::new),
      sale_mint_counter: mint_counter_prefix.map(LookupMap::new),
      sales_available: sales_available_prefix.map(UnorderedSet::new),
      // royalty
      contract_royalty: 0,
      token_royalty_by_id: token_royalty_prefix.map(LookupMap::new),

      token_rarity_by_id: token_rarity_prefix.map(LookupMap::new),
      token_collection_by_id: token_collection_prefix.map(LookupMap::new),
      token_type_by_id: token_type_prefix.map(LookupMap::new),
      token_sub_type_by_id: token_sub_type_prefix.map(LookupMap::new),

      // fractionation feature
      fractionation_by_id: fractionation_prefix.map(TreeMap::new),
      fractionation_token_by_id: fractionation_tokens_prefix.map(LookupMap::new),
      fractionation_ids: fractionation_available_prefix.map(UnorderedSet::new),
      fractionation_completed_by_id: fractionation_completed_prefix.map(LookupMap::new),
    };
    this.measure_min_token_storage_cost();
    this
  }

  // TODO: does this seem reasonable?
  fn measure_min_token_storage_cost(&mut self) {
    let initial_storage_usage = env::storage_usage();
    // 64 Length because this is the max account id length
    let tmp_token_id = "a".repeat(64);
    let tmp_owner_id = AccountId::new_unchecked("a".repeat(64));

    // 1. set some dummy data
    self.owner_by_id.insert(&tmp_token_id, &tmp_owner_id);
    if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
      token_metadata_by_id.insert(
        &tmp_token_id,
        &TokenMetadata {
          title: Some("a".repeat(64)),
          description: Some("a".repeat(64)),
          media: Some("a".repeat(64)),
          media_hash: Some(Base64VecU8::from("a".repeat(64).as_bytes().to_vec())),
          copies: Some(1),
          issued_at: None,
          expires_at: None,
          starts_at: None,
          updated_at: None,
          extra: None,
          reference: None,
          reference_hash: None,
        },
      );
    }
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      let u = &mut UnorderedSet::new(StorageKey::TokensPerOwner {
        account_hash: env::sha256(tmp_owner_id.as_bytes()),
      });
      u.insert(&tmp_token_id);
      tokens_per_owner.insert(&tmp_owner_id, u);
    }
    if let Some(approvals_by_id) = &mut self.approvals_by_id {
      let mut approvals = HashMap::new();
      approvals.insert(tmp_owner_id.clone(), 1u64);
      approvals_by_id.insert(&tmp_token_id, &approvals);
    }
    if let Some(next_approval_id_by_id) = &mut self.next_approval_id_by_id {
      next_approval_id_by_id.insert(&tmp_token_id, &1u64);
    }
    let u = UnorderedSet::new(
      StorageKey::TokenPerOwnerInner { account_id_hash: hash_account_id(&tmp_owner_id) }
        .try_to_vec()
        .unwrap(),
    );
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      tokens_per_owner.insert(&tmp_owner_id, &u);
    }

    // 2. see how much space it took
    self.extra_storage_in_bytes_per_token = env::storage_usage() - initial_storage_usage;

    // 3. roll it all back
    if let Some(next_approval_id_by_id) = &mut self.next_approval_id_by_id {
      next_approval_id_by_id.remove(&tmp_token_id);
    }
    if let Some(approvals_by_id) = &mut self.approvals_by_id {
      approvals_by_id.remove(&tmp_token_id);
    }
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      tokens_per_owner.remove(&tmp_owner_id);
    }
    if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
      token_metadata_by_id.remove(&tmp_token_id);
    }
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      tokens_per_owner.remove(&tmp_owner_id);
    }
    self.owner_by_id.remove(&tmp_token_id);
  }
}

impl NonFungibleTokenCore for NonFungibleToken {
  fn nft_transfer(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
  ) {
    // assert_one_yocto();
    let sender_id = env::predecessor_account_id();

    self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
  }

  fn nft_transfer_call(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
    msg: String,
  ) -> PromiseOrValue<bool> {
    assert_one_yocto();
    require!(
            env::prepaid_gas() > GAS_FOR_NFT_TRANSFER_CALL + GAS_FOR_RESOLVE_TRANSFER,
            "More gas is required"
        );
    let sender_id = env::predecessor_account_id();
    let (old_owner, old_approvals) =
      self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    // Initiating receiver's call and the callback
    ext_receiver::nft_on_transfer(
      sender_id,
      old_owner.clone(),
      token_id.clone(),
      msg,
      receiver_id.clone(),
      NO_DEPOSIT,
      env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
    )
      .then(ext_self::nft_resolve_transfer(
        old_owner,
        receiver_id,
        token_id,
        old_approvals,
        env::current_account_id(),
        NO_DEPOSIT,
        GAS_FOR_RESOLVE_TRANSFER,
      ))
      .into()
  }

  fn nft_token(&self, token_id: TokenId) -> Option<Token> {
    let owner_id = self.owner_by_id.get(&token_id)?;
    let token = self.enum_get_token(owner_id.clone(), token_id.clone());

    Some(token)
  }
}

impl NonFungibleTokenResolver for NonFungibleToken {
  /// Returns true if token was successfully transferred to `receiver_id`.
  fn nft_resolve_transfer(
    &mut self,
    previous_owner_id: AccountId,
    receiver_id: AccountId,
    token_id: TokenId,
    approved_account_ids: Option<HashMap<AccountId, u64>>,
  ) -> bool {
    // Get whether token should be returned
    let must_revert = match env::promise_result(0) {
      PromiseResult::NotReady => env::abort(),
      PromiseResult::Successful(value) => {
        if let Ok(yes_or_no) = near_sdk::serde_json::from_slice::<bool>(&value) {
          yes_or_no
        } else {
          true
        }
      }
      PromiseResult::Failed => true,
    };

    // if call succeeded, return early
    if !must_revert {
      return true;
    }

    // OTHERWISE, try to set owner back to previous_owner_id and restore approved_account_ids

    // Check that receiver didn't already transfer it away or burn it.
    if let Some(current_owner) = self.owner_by_id.get(&token_id) {
      if current_owner != receiver_id {
        // The token is not owned by the receiver anymore. Can't return it.
        return true;
      }
    } else {
      // The token was burned and doesn't exist anymore.
      // Refund storage cost for storing approvals to original owner and return early.
      if let Some(approved_account_ids) = approved_account_ids {
        refund_approved_account_ids(previous_owner_id, &approved_account_ids);
      }
      return true;
    };

    log!("Return token {} from @{} to @{}", token_id, receiver_id, previous_owner_id);

    self.internal_transfer_unguarded(&token_id, &receiver_id, &previous_owner_id);

    // If using Approval Management extension,
    // 1. revert any approvals receiver already set, refunding storage costs
    // 2. reset approvals to what previous owner had set before call to nft_transfer_call
    if let Some(by_id) = &mut self.approvals_by_id {
      if let Some(receiver_approvals) = by_id.get(&token_id) {
        refund_approved_account_ids(receiver_id, &receiver_approvals);
      }
      if let Some(previous_owner_approvals) = approved_account_ids {
        by_id.insert(&token_id, &previous_owner_approvals);
      }
    }

    false
  }
}
