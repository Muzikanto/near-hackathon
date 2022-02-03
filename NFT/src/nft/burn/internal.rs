use crate::nft::{NonFungibleToken, TokenId};
use near_sdk::{env, AccountId};
use crate::nft::events_171::NftBurn;

impl NonFungibleToken {
  pub(crate) fn assert_available_burn(&self, token_id: &TokenId) {
    self.assert_nft_not_locked(&token_id);
    self.assert_token_not_fractionation(&token_id);
  }

  pub fn internal_burn_token(&mut self, sender_id: &AccountId, token_id: &TokenId)  {
    self.assert_token_holder(&token_id);
    self.assert_available_burn(&token_id);

    self.internal_burn_token_unguarded(&sender_id, &token_id);
  }

  pub fn internal_burn_token_unguarded(&mut self, sender_id: &AccountId, token_id: &TokenId) {
    let tokens_per_owner = self.tokens_per_owner.as_mut().unwrap();

    let mut owner_tokens = tokens_per_owner.get(&sender_id).unwrap_or_else(|| {
      env::panic_str("Unable to access tokens per owner.")
    });
    owner_tokens.remove(token_id);
    if owner_tokens.is_empty() {
      tokens_per_owner.remove(&sender_id);
    } else {
      tokens_per_owner.insert(&sender_id, &owner_tokens);
    }

    self.owner_by_id.remove(&token_id);

    self.token_collection_by_id.as_mut().unwrap().remove(&token_id);
    self.token_type_by_id.as_mut().unwrap().remove(&token_id);
    self.token_rarity_by_id.as_mut().unwrap().remove(&token_id);

    self.token_royalty_by_id.as_mut().unwrap().remove(&token_id);

    self.token_metadata_by_id.as_mut().unwrap().remove(&token_id);

    self.fractionation_token_by_id.as_mut().unwrap().remove(&token_id);

    self.sale_by_token.as_mut().unwrap().remove(&token_id);

    self.approvals_by_id.as_mut().unwrap().remove(&token_id);

    self.token_locked_by_id.as_mut().unwrap().remove(&token_id);
    self.token_bind_by_id.as_mut().unwrap().remove(&token_id);

    NftBurn {
      owner_id: &sender_id,
      token_ids: &[token_id],
      authorized_id: None,
      memo: None
    }.emit();
  }
}
