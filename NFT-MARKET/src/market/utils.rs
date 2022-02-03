use near_sdk::{AccountId, CryptoHash, env};
use near_sdk::serde::Serialize;
use near_sdk::serde_json::to_string;

use crate::market::FungibleTokenId;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  let mut hash = CryptoHash::default();
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}

/// Assert that at least 1 yoctoNEAR was attached.
pub(crate) fn assert_at_least_one_yocto() {
  assert!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR")
}

pub(crate) fn near_ft() -> FungibleTokenId {
  AccountId::new_unchecked("near".to_string())
}

pub(crate) fn stringify<T>(data: &T) -> String where
  T: ?Sized + Serialize,
{
  to_string(&data).ok().unwrap()
}

pub(crate) fn date_now() -> u64 {
  env::block_timestamp() / 1000000
}
