use crate::nft::{NonFungibleToken, TokenId, NonFungibleTokenBindToOwner};

impl NonFungibleToken {
  pub(crate) fn assert_bind_to_player(&self, token_id: &TokenId) {
    let is_bind = self.internal_token_is_bind_to_owner(&token_id);

    assert!(
      !&is_bind,
      "Token is bind to account"
    );
  }

  pub(crate) fn internal_token_is_bind_to_owner(&self, token_id: &TokenId) -> bool {
    self.token_bind_by_id.as_ref().unwrap().get(&token_id).unwrap_or_else(||false)
  }

  pub fn internal_token_bind_to_owner(&mut self, token_id: &TokenId, bind_to_owner: &bool) {
    self.token_bind_by_id.as_mut().unwrap().insert(&token_id, &bind_to_owner);
  }
}

impl NonFungibleTokenBindToOwner for NonFungibleToken {
  fn nft_token_is_bind_to_owner(&self, token_id: TokenId) -> bool {
    self.internal_token_is_bind_to_owner(&token_id)
  }
}
