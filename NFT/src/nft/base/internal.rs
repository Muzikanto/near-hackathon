use crate::nft::{NonFungibleToken, TokenId};
use near_sdk::{AccountId, env, require};
use std::collections::HashMap;
use near_sdk::collections::UnorderedSet;
use crate::nft::base::StorageKey;
use crate::nft::events_171::NftTransfer;

impl NonFungibleToken {
  // custom

  pub(crate) fn assert_owner(&self) {
    assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
  }

  pub(crate) fn assert_token_holder(&self, token_id: &TokenId) {
    let sender_id = env::predecessor_account_id();
    let owner_id = self.owner_by_id.get(token_id).unwrap_or_else(|| env::panic_str("Not found token"));

    assert_eq!(&owner_id, &sender_id, "Unauthorized")
  }

  pub(crate) fn assert_available_transfer(&self, token_id: &TokenId) {
    self.assert_bind_to_player(&token_id);
    self.assert_nft_not_locked(&token_id);
  }

  pub(crate) fn assert_available_approve(&self, token_id: &TokenId) {
    self.assert_available_transfer(&token_id);
  }

  /// Transfer token_id from `from` to `to`
  ///
  /// Do not perform any safety checks or do any logging
  pub fn internal_transfer_unguarded(
    &mut self,
    #[allow(clippy::ptr_arg)] token_id: &TokenId,
    from: &AccountId,
    to: &AccountId,
  ) {
    // update owner
    self.owner_by_id.insert(token_id, to);

    // if using Enumeration standard, update old & new owner's token lists
    if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
      // owner_tokens should always exist, so call `unwrap` without guard
      let mut owner_tokens = tokens_per_owner.get(from).unwrap_or_else(|| {
        env::panic_str("Unable to access tokens per owner in unguarded call.")
      });
      owner_tokens.remove(token_id);
      if owner_tokens.is_empty() {
        tokens_per_owner.remove(from);
      } else {
        tokens_per_owner.insert(from, &owner_tokens);
      }

      let mut receiver_tokens = tokens_per_owner.get(to).unwrap_or_else(|| {
        UnorderedSet::new(StorageKey::TokensPerOwner {
          account_hash: env::sha256(to.as_bytes()),
        })
      });
      receiver_tokens.insert(token_id);
      tokens_per_owner.insert(to, &receiver_tokens);
    }

    NftTransfer {
      old_owner_id: &from,
      new_owner_id: &to,
      token_ids: &[token_id],
      authorized_id: None,
      memo: None
    }.emit();
  }

  /// Transfer from current owner to receiver_id, checking that sender is allowed to transfer.
  /// Clear approvals, if approval extension being used.
  /// Return previous owner and approvals.
  pub fn internal_transfer(
    &mut self,
    sender_id: &AccountId,
    receiver_id: &AccountId,
    #[allow(clippy::ptr_arg)] token_id: &TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
  ) -> (AccountId, Option<HashMap<AccountId, u64>>) {
    // custom
    self.assert_available_transfer(&token_id);

    let owner_id =
      self.owner_by_id.get(token_id).unwrap_or_else(|| env::panic_str("Token not found"));

    // clear approvals, if using Approval Management extension
    // this will be rolled back by a panic if sending fails
    let approved_account_ids =
      self.approvals_by_id.as_mut().and_then(|by_id| by_id.remove(token_id));

    // check if authorized
    if sender_id != &owner_id {
      // if approval extension is NOT being used, or if token has no approved accounts
      let app_acc_ids =
        approved_account_ids.as_ref().unwrap_or_else(|| env::panic_str("Unauthorized"));

      // Approval extension is being used; get approval_id for sender.
      let actual_approval_id = app_acc_ids.get(sender_id);

      // Panic if sender not approved at all
      if actual_approval_id.is_none() {
        env::panic_str("Sender not approved");
      }

      // If approval_id included, check that it matches
      require!(
                approval_id.is_none() || actual_approval_id == approval_id.as_ref(),
                format!(
                    "The actual approval_id {:?} is different from the given approval_id {:?}",
                    actual_approval_id, approval_id
                )
            );
    }

    require!(&owner_id != receiver_id, "Current and next owner must differ");

    self.internal_transfer_unguarded(token_id, &owner_id, receiver_id);

    // return previous owner & approvals
    (owner_id, approved_account_ids)
  }
}
