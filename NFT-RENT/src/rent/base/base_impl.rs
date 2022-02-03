use near_sdk::{AccountId, env, IntoStorageKey, ext_contract, Gas, Balance, Promise, is_promise_success};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, UnorderedSet, TreeMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::base::{RentFactoryCore, RentFactoryResolve};
use crate::meta::Rent;
use crate::{TokenId, assert_at_least_one_yocto, time_get_minutes, date_now};
use crate::base::events::{log_rent_pay, log_rent_claim};

// const GAS_FOR_RENT_PAY: Gas = Gas(5_000_000_000_000);
const GAS_FOR_RENT_RESOLVE_PAY: Gas = Gas(20_000_000_000_000);
const GAS_FOR_NFT_LOCK: Gas = Gas(4_000_000_000_000);
const GAS_FOR_RENT_CLAIM: Gas = Gas(18_000_000_000_000);
// const GAS_FOR_RENT_RESOLVE_CLAIM: Gas = Gas(20_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_locked_receiver)]
pub trait NonFungibleTokenLockedReceiver {
  fn nft_on_lock(&mut self, token_id: TokenId, locked: bool);
}
#[ext_contract(ext_self)]
trait ExtSelf {
  fn rent_resolve_pay(&mut self, token_id: TokenId, owner_id: AccountId, receiver_id: AccountId, time: u64, end_time: u64, price: U128);
  fn rent_resolve_claim(&mut self, token_id: TokenId, owner_id: AccountId, renter_id: AccountId);
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct RentFactory {
  pub owner_id: AccountId,
  // nft contract
  pub nft_contract_id: AccountId,

  // approved nft tokens
  pub approved_owner_by_id: Option<LookupMap<TokenId, AccountId>>,

  // paid rents
  pub rents_current: TreeMap<TokenId, AccountId>,
  // available rents for pay
  pub rents_pending: UnorderedSet<TokenId>,
  // rents info
  pub rents_by_id: TreeMap<TokenId, Rent>,
  // rents per account
  pub rents_per_account: TreeMap<AccountId, UnorderedSet<TokenId>>,
  // rents start time
  pub rents_end_by_id: LookupMap<TokenId, u64>,
  // rented nft tokens per account
  pub rent_tokens_per_account: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

impl RentFactory {
  pub fn new<R0, R1, R2, R3, R4, R5, R6>(
    owner_id: AccountId,
    nft_contract_id: AccountId,
    approved_owner_prefix: Option<R0>,
    rents_current_prefix: R1,
    rents_pending_prefix: R2,
    rents_by_id_prefix: R3,
    rent_tokens_per_account_prefix: R4,
    rents_per_account_prefix: R5,
    rents_at_prefix: R6,
  ) -> Self
    where
      R0: IntoStorageKey,
      R1: IntoStorageKey,
      R2: IntoStorageKey,
      R3: IntoStorageKey,
      R4: IntoStorageKey,
      R5: IntoStorageKey,
      R6: IntoStorageKey,
  {
    let this = Self {
      owner_id,
      nft_contract_id,
      approved_owner_by_id: approved_owner_prefix.map(LookupMap::new),
      rents_current: TreeMap::new(rents_current_prefix),
      rents_pending:UnorderedSet::new(rents_pending_prefix),
      rents_by_id: TreeMap::new(rents_by_id_prefix),
      rent_tokens_per_account: LookupMap::new(rent_tokens_per_account_prefix),
      rents_per_account: TreeMap::new(rents_per_account_prefix),
      rents_end_by_id: LookupMap::new(rents_at_prefix)
    };

    this
  }
}

impl RentFactoryCore for RentFactory {
  fn rent_token_is_locked(&self, token_id: TokenId) -> bool {
    self.internal_rent_token_is_locked(&token_id)
  }

  fn rent_add(&mut self, token_id: TokenId, account_id: AccountId, price_per_hour: U128, min_time: u64, max_time: u64) {
    assert_at_least_one_yocto();
    self.assert_approved(&token_id);
    self.assert_valid_time(&min_time);
    self.assert_valid_time(&max_time);

    self.internal_rent_add(&token_id, &account_id, &price_per_hour, &min_time, &max_time);
  }

  fn rent_remove(&mut self, token_id: TokenId, account_id: AccountId) {
    assert_at_least_one_yocto();
    self.assert_approved(&token_id);

    self.internal_rent_remove(&token_id, &account_id)
  }

  // #[payable]
  fn rent_pay(&mut self, token_id: TokenId, time: u64, receiver_id: AccountId) -> Promise {
    let is_paid = self.rents_current.get(&token_id).is_some();
    let rent = self.rents_by_id.get(&token_id).expect("Token is not available for rent");

    assert!(!is_paid, "Token is already in rent");
    assert_ne!(
      &receiver_id,
      &rent.owner_id,
      "Not rent owned token"
    );

    let deposit = env::attached_deposit();
    let minutes = time_get_minutes(time) as u128;
    let price = ((rent.price_per_hour.0) * minutes / 60) as u128;

    env::log_str(&format!("{}{}{}{}", "Minutes: ", minutes, " Price ", price).to_string());

    assert!(deposit >= price, "Invalid attached deposit {}, price {}", deposit.to_string(), price.to_string());

    let now = date_now();
    let end_time = now + time;

    assert!(time >= rent.min_time && time <= rent.max_time, "Invalid rent time");

    ext_locked_receiver::nft_on_lock(
      token_id.clone(),
      true,

      self.nft_contract_id.clone(),
      NO_DEPOSIT,
      GAS_FOR_NFT_LOCK,
    ).then(ext_self::rent_resolve_pay(
      token_id,
      rent.owner_id,
      receiver_id,
      time,
      end_time,
      U128(deposit),

      env::current_account_id().clone(),
      NO_DEPOSIT,
      GAS_FOR_RENT_RESOLVE_PAY,
    ))
  }

  fn rent_claim(&mut self, token_id: TokenId, account_id: AccountId) -> Promise {
    assert_at_least_one_yocto();
    assert_eq!(&env::predecessor_account_id(), &account_id, "Not authorized");

    let rent = self.rents_by_id.get(&token_id).expect("Not found rent");

    assert_eq!(&rent.owner_id, &account_id, "Not authorized");

    let renter_id = self.rents_current.get(&token_id).expect("Not found renter");

    let is_ended = self.internal_rent_is_ended(&token_id);

    assert!(is_ended, "Rent is not expired");

    ext_locked_receiver::nft_on_lock(
      token_id.clone(),
      false,

      self.nft_contract_id.clone(),
      NO_DEPOSIT,
      GAS_FOR_NFT_LOCK,
    )
      .then(ext_self::rent_resolve_claim(
      token_id,
      account_id,
      renter_id,

      env::current_account_id().clone(),
      NO_DEPOSIT,
      env::prepaid_gas() - GAS_FOR_RENT_CLAIM,
    ))
  }

  fn rent_is_ended(&self, token_id: TokenId) -> bool {
    self.internal_rent_is_ended(&token_id)
  }

  fn rent_total_supply(&self) -> u64 {
    self.rents_pending.len()
  }
}

impl RentFactoryResolve for RentFactory {
  fn rent_resolve_pay(&mut self, token_id: TokenId, owner_id: AccountId, receiver_id: AccountId, time: u64, end_time: u64, price: U128) -> U128 {
    if !is_promise_success() {
      Promise::new(env::signer_account_id()).transfer(u128::from(price));
      return price;
    }

    self.rents_end_by_id.insert(&token_id, &end_time);
    self.rents_current.insert(&token_id, &receiver_id);
    self.rents_pending.remove(&token_id);
    self.internal_add_token_to_account(&receiver_id, &token_id);

    log_rent_pay(&token_id, &owner_id, &receiver_id, &time, &end_time, &price);

    Promise::new(owner_id).transfer(u128::from(price));

    U128(0)
  }

  fn rent_resolve_claim(&mut self, token_id: TokenId, owner_id: AccountId, renter_id: AccountId) {
    let is_success = is_promise_success();

    if !is_success {
      env::panic_str("Error during unlock nft")
    }

    self.internal_remove_token_from_account(&renter_id, &token_id);
    self.internal_remove_rent_from_account(&owner_id, &token_id);

    log_rent_claim(&token_id, &owner_id, &renter_id);
  }
}
