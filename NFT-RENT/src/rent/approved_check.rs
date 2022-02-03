use crate::{RentFactory, TokenId};
use near_sdk::{env};

impl RentFactory {
  pub(crate) fn assert_approved(&self, token_id: &TokenId) {
    let sender_id = env::predecessor_account_id();
    let approve_id = self.approved_owner_by_id.as_ref().unwrap().get(&token_id).expect("Not approved for sender");

    assert_eq!(approve_id, sender_id, "Not approved for sender");
  }
}
