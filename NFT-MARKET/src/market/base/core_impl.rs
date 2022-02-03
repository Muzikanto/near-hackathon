use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, TreeMap};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize};
use near_sdk::{AccountId, Balance, Gas, CryptoHash, BorshStorageKey, Promise, promise_result_as_success, env, ext_contract};
use crate::market::meta::{ContractAndTokenId, TokenId, Payout};
use crate::market::{Sale, FungibleTokenId, near_ft, MarketCore, Bid, assert_at_least_one_yocto};
use crate::market::base::events::{log_market_remove_sale, log_market_update_sale};
use crate::market::base::log_market_offer;

// TODO check seller supports storage_deposit at ft_token_id they want to post sale in

pub(crate) const GAS_FOR_FT_TRANSFER: Gas = Gas(5_000_000_000_000);
/// greedy max Tgas for resolve_purchase
pub(crate) const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);
pub(crate) const GAS_FOR_NFT_TRANSFER: Gas = Gas(16_000_000_000_000);
pub(crate) const BID_HISTORY_LENGTH_DEFAULT: u8 = 1;
pub(crate) const NO_DEPOSIT: Balance = 0;
// const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;
pub(crate) static DELIMETER: &str = "||";

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalanceBounds {
    pub min: U128,
    pub max: Option<U128>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MarketFactory {
    pub owner_id: AccountId,
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
    pub by_owner_id: TreeMap<AccountId, UnorderedSet<ContractAndTokenId>>,
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub ft_token_ids: UnorderedSet<AccountId>,
    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub bid_history_length: u8,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
}

impl MarketFactory {
   pub fn new(owner_id: AccountId, ft_token_ids:Option<Vec<FungibleTokenId>>, bid_history_length:Option<u8>) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            sales: UnorderedMap::new(StorageKey::Sales),
            by_owner_id: TreeMap::new(StorageKey::ByOwnerId),
            by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
            ft_token_ids: UnorderedSet::new(StorageKey::FTTokenIds),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            bid_history_length: bid_history_length.unwrap_or(BID_HISTORY_LENGTH_DEFAULT),
        };
        // support NEAR by default
        this.ft_token_ids.insert(&near_ft());

        if let Some(ft_token_ids) = ft_token_ids {
            for ft_token_id in ft_token_ids {
                this.ft_token_ids.insert(&ft_token_id);
            }
        }

        this
    }
}

impl MarketCore for MarketFactory {
  /// for add sale see: nft_callbacks.rs

  /// TODO remove without redirect to wallet? panic reverts
  // #[payable]
  fn market_remove_sale(&mut self, nft_contract_id: AccountId, token_id: String) {
    assert_at_least_one_yocto();
    let sale = self.internal_remove_sale(&nft_contract_id, &token_id);
    let owner_id = env::predecessor_account_id();
    assert_eq!(owner_id, sale.owner_id, "Must be sale owner");
    self.refund_all_bids(&sale.bids);

    log_market_remove_sale(&owner_id, &nft_contract_id, &token_id);
  }

  // #[payable]
  fn market_update_price(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
    price: U128,
  ) {
    assert_at_least_one_yocto();
    let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
    let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
    assert_eq!(
      env::predecessor_account_id(),
      sale.owner_id,
      "Must be sale owner"
    );

    if !self.ft_token_ids.contains(&ft_token_id) {
      env::panic_str(&format!("Token {} not supported by this market", ft_token_id).to_string());
    }
    sale.sale_conditions.insert(ft_token_id.clone(), price);
    self.sales.insert(&contract_and_token_id, &sale);

    log_market_update_sale(&sale.owner_id, &nft_contract_id, &token_id, &ft_token_id, &price);
  }

  // #[payable]
  fn market_offer(&mut self, nft_contract_id: AccountId, token_id: String) {
    let contract_id: AccountId = nft_contract_id.into();
    let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id);
    let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
    let buyer_id = env::predecessor_account_id();
    assert_ne!(sale.owner_id, buyer_id, "Cannot bid on your own sale.");
    let ft_token_id = near_ft();
    let price = sale
      .sale_conditions
      .get(&ft_token_id)
      .expect("Not for sale in NEAR")
      .0;

    let deposit = env::attached_deposit();
    assert!(deposit > 0, "Attached deposit must be greater than 0");

    if !sale.is_auction && deposit == price {
      self.market_process_purchase(
        contract_id,
        token_id,
        ft_token_id,
        U128(deposit),
        buyer_id,
      );
    } else {
      if sale.is_auction && price > 0 {
        assert!(deposit >= price, "Attached deposit must be greater than reserve price");
      }
      self.market_add_bid(
        contract_and_token_id,
        deposit,
        ft_token_id,
        buyer_id,
        &mut sale,
      );
    }
  }

  // #[private]
  fn market_add_bid(
    &mut self,
    contract_and_token_id: ContractAndTokenId,
    amount: Balance,
    ft_token_id: AccountId,
    buyer_id: AccountId,
    sale: &mut Sale,
  ) {
    unimplemented!();

    // store a bid and refund any current bid lower
    let new_bid = Bid {
      owner_id: buyer_id,
      price: U128(amount),
    };

    let bids_for_token_id = sale.bids.entry(ft_token_id.clone()).or_insert_with(Vec::new);

    if !bids_for_token_id.is_empty() {
      let current_bid = &bids_for_token_id[bids_for_token_id.len()-1];
      assert!(
        amount > current_bid.price.0,
        "Can't pay less than or equal to current bid price: {}",
        current_bid.price.0
      );
      if ft_token_id == near_ft() {
        Promise::new(current_bid.owner_id.clone()).transfer(u128::from(current_bid.price));
      } else {
        ext_contract::ft_transfer(
          current_bid.owner_id.clone(),
          current_bid.price,
          None,
          ft_token_id.clone(),
          1,
          GAS_FOR_FT_TRANSFER,
        );
      }
    }

    bids_for_token_id.push(new_bid);
    if bids_for_token_id.len() > self.bid_history_length as usize {
      bids_for_token_id.remove(0);
    }

    self.sales.insert(&contract_and_token_id, &sale);
  }

  fn market_accept_offer(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
  ) {
    unimplemented!();

    let contract_id: AccountId = nft_contract_id.into();
    let contract_and_token_id = format!("{}{}{}", contract_id.clone(), DELIMETER, token_id.clone());
    // remove bid before proceeding to process purchase
    let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
    let bids_for_token_id = sale.bids.remove(&ft_token_id).expect("No bids");
    let bid = &bids_for_token_id[bids_for_token_id.len()-1];
    self.sales.insert(&contract_and_token_id, &sale);
    // panics at `self.internal_remove_sale` and reverts above if predecessor is not sale.owner_id
    self.market_process_purchase(
      contract_id,
      token_id,
      ft_token_id.into(),
      bid.price,
      bid.owner_id.clone(),
    );
  }

  // #[private]
  fn market_process_purchase(
    &mut self,
    nft_contract_id: AccountId,
    token_id: String,
    ft_token_id: AccountId,
    price: U128,
    buyer_id: AccountId,
  ) -> Promise {
    let sale = self.internal_remove_sale(&nft_contract_id, &token_id);

    ext_contract::nft_transfer_payout(
      buyer_id.clone(),
      token_id,
      sale.approval_id,
      "payout from market".to_string(),
      price,
      10,
      nft_contract_id.clone(),
      1,
      GAS_FOR_NFT_TRANSFER,
    )
      .then(ext_self::market_resolve_purchase(
        ft_token_id,
        buyer_id,
        sale,
        price,
        env::current_account_id().clone(),
        NO_DEPOSIT,
        GAS_FOR_ROYALTIES,
      ))
  }

  /// self callback

  // #[private]
  fn market_resolve_purchase(
    &mut self,
    ft_token_id: FungibleTokenId,
    buyer_id: AccountId,
    sale: Sale,
    price: U128,
  ) -> U128 {

    // checking for payout information
    let payout_option = promise_result_as_success().and_then(|value| {
      // None means a bad payout from bad NFT contract
      near_sdk::serde_json::from_slice::<Payout>(&value)
        .ok()
        .and_then(|payout| {
          // gas to do 10 FT transfers (and definitely 10 NEAR transfers)
          if payout.len() + sale.bids.len() > 10 || payout.is_empty() {
            env::log_str(&format!("Cannot have more than 10 royalties and sale.bids refunds").to_string());
            None
          } else {
            // TODO off by 1 e.g. payouts are fractions of 3333 + 3333 + 3333
            let mut remainder = price.0;
            for &value in payout.values() {
              remainder = remainder.checked_sub(value.0)?;
            }
            if remainder == 0 || remainder == 1 {
              Some(payout)
            } else {
              None
            }
          }
        })
    });
    // is payout option valid?
    let payout = if let Some(payout_option) = payout_option {
      payout_option
    } else {
      if ft_token_id == AccountId::new_unchecked("near".to_string()) {
        Promise::new(buyer_id).transfer(u128::from(price));
      }
      // leave function and return all FTs in ft_resolve_transfer
      return price;
    };
    // Going to payout everyone, first return all outstanding bids (accepted offer bid was already removed)
    self.refund_all_bids(&sale.bids);

    // NEAR payouts
    if ft_token_id == near_ft() {
      for (receiver_id, amount) in payout.clone() {
        Promise::new(receiver_id).transfer(amount.0);
      }

      log_market_offer(&sale.owner_id, &buyer_id, &sale.nft_contract_id, &sale.token_id, &ft_token_id, &price, &payout);
      // refund all FTs (won't be any)
      price
    } else {
      // FT payouts
      for (receiver_id, amount) in payout.clone() {
        ext_contract::ft_transfer(
          receiver_id,
          amount,
          None,
          ft_token_id.clone(),
          1,
          GAS_FOR_FT_TRANSFER,
        );
      }

      log_market_offer(&sale.owner_id, &buyer_id, &sale.nft_contract_id, &sale.token_id, &ft_token_id, &price, &payout);

      // keep all FTs (already transferred for payouts)
      U128(0)
    }
  }
}

#[ext_contract(ext_contract)]
trait ExtContract {
  fn nft_transfer_payout(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: u64,
    memo: String,
    balance: U128,
    max_len_payout: u32,
  );
  fn ft_transfer(
    &mut self,
    receiver_id: AccountId,
    amount: U128,
    memo: Option<String>
  );
}

/// self call

#[ext_contract(ext_self)]
trait ExtSelf {
  fn market_resolve_purchase(
    &mut self,
    ft_token_id: AccountId,
    buyer_id: AccountId,
    sale: Sale,
    price: U128,
  ) -> Promise;
}
