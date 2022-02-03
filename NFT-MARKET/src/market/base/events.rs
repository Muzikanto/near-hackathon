use near_sdk::serde::{Serialize};
use near_sdk::{AccountId};

use crate::{TokenId};
use crate::market::{log_event, stringify, Sale};
use near_sdk::json_types::U128;
use crate::market::meta::Payout;

const EVENT_MARKET_CREATE_SALE: &str = "market_create_sale";
const EVENT_MARKET_UPDATE_SALE: &str = "market_update_sale";
const EVENT_MARKET_REMOVE_SALE: &str = "market_remove_sale";
const EVENT_MARKET_OFFER: &str = "market_offer";

//

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMarketCreateSaleData {
  pub owner_id: AccountId,

  pub nft_contract_id: AccountId,
  pub token_id: TokenId,
  pub sale: Sale,
}
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMarketUpdateSaleData {
  pub owner_id: AccountId,

  pub nft_contract_id: AccountId,
  pub token_id: TokenId,
  pub ft_token_id: AccountId,
  pub price: U128,
}
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMarketRemoveSaleData {
  pub owner_id: AccountId,

  pub nft_contract_id: AccountId,
  pub token_id: TokenId,
}
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMarketOfferData {
  pub owner_id: AccountId,

  pub receiver_id: AccountId,
  pub nft_contract_id: AccountId,
  pub token_id: TokenId,
  pub payout: Payout,
  pub ft_token_id: AccountId,
  pub price: U128,
}

pub(crate) fn log_market_create_sale(owner_id: &AccountId, nft_contract_id: &AccountId, token_id: &TokenId, sale: &Sale) {
  log_event(&EVENT_MARKET_CREATE_SALE.to_string(), stringify(&EventMarketCreateSaleData {
    owner_id: owner_id.clone(),
    nft_contract_id: nft_contract_id.clone(),
    token_id: token_id.clone(),
    sale: sale.clone(),
  }));
}
pub(crate) fn log_market_update_sale(owner_id: &AccountId, nft_contract_id: &AccountId, token_id: &TokenId, ft_token_id: &AccountId, price: &U128) {
  log_event(&EVENT_MARKET_UPDATE_SALE.to_string(), stringify(&EventMarketUpdateSaleData {
    owner_id: owner_id.clone(),
    nft_contract_id: nft_contract_id.clone(),
    token_id: token_id.clone(),
    ft_token_id: ft_token_id.clone(),
    price: price.clone(),
  }));
}
pub(crate) fn log_market_remove_sale(owner_id: &AccountId, nft_contract_id: &AccountId, token_id: &TokenId) {
  log_event(&EVENT_MARKET_REMOVE_SALE.to_string(), stringify(&EventMarketRemoveSaleData {
    owner_id: owner_id.clone(),
    nft_contract_id: nft_contract_id.clone(),
    token_id: token_id.clone(),
  }));
}
pub(crate) fn log_market_offer(owner_id: &AccountId, receiver_id: &AccountId, nft_contract_id: &AccountId, token_id: &TokenId, ft_token_id: &AccountId, price: &U128, payout: &Payout) {
  log_event(&EVENT_MARKET_OFFER.to_string(), stringify(&EventMarketOfferData {
    owner_id: owner_id.clone(),
    receiver_id: receiver_id.clone(),
    nft_contract_id: nft_contract_id.clone(),
    token_id: token_id.clone(),
    payout: payout.clone(),
    ft_token_id: ft_token_id.clone(),
    price: price.clone(),
  }));
}



