use crate::nft::{NonFungibleToken, TokenId};
use crate::nft::locked::NonFungibleTokenLocked;
use crate::nft::events::NftOnLock;

impl NonFungibleToken {
  pub(crate) fn internal_nft_set_locked(&mut self, token_id: &TokenId, locked: bool) {
    if locked {
      // TODO need remove approval
      self.assert_approved(&token_id, false);
    } else {
      self.assert_approved(&token_id, true);
    }

    self.token_locked_by_id.as_mut().unwrap().insert(&token_id, &locked);

    NftOnLock {
      token_id,
      locked: &locked,
      account_id: &self.owner_by_id.get(&token_id).expect("Not found token owner"),
    }.emit();
  }

  pub(crate) fn internal_nft_is_locked(&self, token_id: &TokenId) -> bool {
    self.token_locked_by_id.as_ref().unwrap().get(&token_id).unwrap_or_else(||false)
  }

  pub(crate) fn assert_nft_not_locked(&self, token_id: &TokenId) {
    let is_locked = self.internal_nft_is_locked(token_id);

    assert!(!is_locked, "Token is locked");
  }
}

impl NonFungibleTokenLocked for NonFungibleToken {
  fn nft_token_is_locked(&self, token_id: TokenId) -> bool {
    self.internal_nft_is_locked(&token_id)
  }

  fn nft_on_lock(&mut self, token_id: TokenId, locked: bool) {
    self.internal_nft_set_locked(&token_id, locked);
  }
}
