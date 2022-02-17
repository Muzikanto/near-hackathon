use super::NonFungibleTokenEnumeration;
use crate::nft::token::Token;
use crate::nft::{NonFungibleToken};
use near_sdk::json_types::{U128};
use near_sdk::{env, require, AccountId};
use crate::nft::base::NonFungibleTokenCore;

type TokenId = String;

impl NonFungibleToken {
  /// Helper function used by a enumerations methods
  /// Note: this method is not exposed publicly to end users
  pub fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
    let metadata = self.token_metadata_by_id.as_ref().unwrap().get(&token_id);
    let approved_account_ids =
      Some(self.approvals_by_id.as_ref().unwrap().get(&token_id).unwrap_or_default());

    // custom
    let locked = self.token_locked_by_id.as_ref().unwrap().get(&token_id).unwrap_or_else(||false);
    let bind_to_owner = self.token_bind_by_id.as_ref().unwrap().get(&token_id);
    let rarity = self.token_rarity_by_id.as_ref().unwrap().get(&token_id);
    let royalty = self.token_royalty_by_id.as_ref().unwrap().get(&token_id);
    let collection = self.token_collection_by_id.as_ref().unwrap().get(&token_id);
    let token_type = self.token_type_by_id.as_ref().unwrap().get(&token_id);
    let token_sub_type = self.token_sub_type_by_id.as_ref().unwrap().get(&token_id);
    let fractionation_id = self.fractionation_token_by_id.as_ref().unwrap().get(&token_id);
    let sale_id = self.sale_by_token.as_ref().unwrap().get(&token_id);

    Token {
      token_id,
      owner_id,
      metadata,
      approved_account_ids,
      sale_id,
      royalty,
      collection,
      token_type,
      token_sub_type,
      rarity,
      bind_to_owner,
      locked: Some(locked),
      fractionation_id,
    }
  }

  pub(crate) fn internal_token_ids_for_owner(
    &self,
    account_id: &AccountId,
  ) -> Vec<TokenId> {
    let tokens_per_owner = self.tokens_per_owner.as_ref().unwrap_or_else(|| {
      env::panic_str(
        "Could not find tokens_per_owner when calling a method on the \
                enumeration standard.",
      )
    });
    let token_set = if let Some(token_set) = tokens_per_owner.get(&account_id) {
      token_set
    } else {
      return vec![];
    };

    token_set.as_vector().to_vec()
  }
}

impl NonFungibleTokenEnumeration for NonFungibleToken {
  fn nft_total_supply(&self) -> U128 {
    // An unfortunate cast from the max of TreeMap to the spec
    (self.owner_by_id.len() as u128).into()
  }

  fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
    // Get starting index, whether or not it was explicitly given.
    // Defaults to 0 based on the spec:
    // https://nomicon.io/Standards/NonFungibleToken/Enumeration.html#interface
    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    require!(
            (self.owner_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");
    self.owner_by_id
      .iter()
      .skip(start_index as usize)
      .take(limit)
      .map(|(token_id, owner_id)| self.enum_get_token(owner_id, token_id))
      .collect()
  }

  fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
    let tokens_per_owner = self.tokens_per_owner.as_ref().unwrap_or_else(|| {
      env::panic_str(
        "Could not find tokens_per_owner when calling a method on the \
                enumeration standard.",
      )
    });
    tokens_per_owner
      .get(&account_id)
      .map(|account_tokens| U128::from(account_tokens.len() as u128))
      .unwrap_or(U128(0))
  }

  fn nft_tokens_for_owner(
    &self,
    account_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<Token> {
    let keys = self.internal_token_ids_for_owner(&account_id);

    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");
    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    require!(
            keys.len() as u128 > start_index,
            "Out of bounds, please use a smaller from_index."
        );
    keys
      .iter()
      .skip(start_index as usize)
      .take(limit)
      .map(|token_id| self.enum_get_token(account_id.clone(), token_id.clone()))
      .collect()
  }

  fn nft_tokens_by_ids(&self, ids: Vec<TokenId>) -> Vec<Token> {
    ids
      .iter()
      .map(|token_id| self.nft_token( token_id.clone()).unwrap())
      .collect::<Vec<Token>>()
  }
}
