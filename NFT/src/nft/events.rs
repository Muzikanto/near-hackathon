use crate::event::NearEvent;
use near_sdk::AccountId;
use serde::Serialize;
use crate::nft::{TokenId, JsonSale, SaleId, Token};
use near_sdk::json_types::U128;

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftCreate<'a> {
  pub token: &'a Token,
}

impl NftCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftCreate<'_>]) {
    new_171_v1(Nep171EventKind::NftCreate(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftOnLock<'a> {
  pub token_id: &'a TokenId,
  pub locked: &'a bool,
  pub account_id: &'a AccountId,
}

impl NftOnLock<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftOnLock<'_>]) {
    new_171_v1(Nep171EventKind::NftOnLock(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct NftTransferPayout<'a> {
  pub token_id: &'a TokenId,
  pub sender_id: &'a AccountId,
  pub receiver_id: &'a AccountId,
  pub balance: &'a U128,
}

impl NftTransferPayout<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[NftTransferPayout<'_>]) {
    new_171_v1(Nep171EventKind::NftTransferPayout(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationCreate<'a> {
  pub token_id: &'a TokenId,
  pub owner_id: &'a AccountId,
}

impl FractionationCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationCreate<'_>]) {
    new_171_v1(Nep171EventKind::FractionationCreate(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationAddToken<'a> {
  pub fractionation_id: &'a TokenId,
  pub token_id: &'a TokenId,
}

impl FractionationAddToken<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationAddToken<'_>]) {
    new_171_v1(Nep171EventKind::FractionationAddToken(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct FractionationComplete<'a> {
  pub token_id: &'a TokenId,
  pub from: &'a AccountId,
  pub to: &'a AccountId,
  pub completed_at: &'a u64,
}

impl FractionationComplete<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[FractionationComplete<'_>]) {
    new_171_v1(Nep171EventKind::FractionationComplete(data)).emit()
  }
}

// #

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleCreate<'a> {
  pub sale: &'a JsonSale,
}

impl SaleCreate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleCreate<'_>]) {
    new_171_v1(Nep171EventKind::SaleCreate(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleStart<'a> {
  pub sale_id: &'a SaleId,
  pub date: &'a u64,
}

impl SaleStart<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleStart<'_>]) {
    new_171_v1(Nep171EventKind::SaleStart(data)).emit()
  }
}

#[must_use]
#[derive(Serialize, Debug, Clone)]
pub struct SaleUpdate<'a> {
  pub sale_id: &'a SaleId,
  pub date: &'a u64,
}

impl SaleUpdate<'_> {
  pub fn emit(self) {
    Self::emit_many(&[self])
  }

  pub fn emit_many(data: &[SaleUpdate<'_>]) {
    new_171_v1(Nep171EventKind::SaleUpdate(data)).emit()
  }
}

// #



#[derive(Serialize, Debug)]
pub(crate) struct Nep171Event<'a> {
  version: &'static str,
  #[serde(flatten)]
  event_kind: Nep171EventKind<'a>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Nep171EventKind<'a> {
  NftCreate(&'a [NftCreate<'a>]),
  NftTransferPayout(&'a [NftTransferPayout<'a>]),
  FractionationCreate(&'a [FractionationCreate<'a>]),
  FractionationAddToken(&'a [FractionationAddToken<'a>]),
  FractionationComplete(&'a [FractionationComplete<'a>]),
  SaleCreate(&'a [SaleCreate<'a>]),
  SaleStart(&'a [SaleStart<'a>]),
  SaleUpdate(&'a [SaleUpdate<'a>]),
  NftOnLock(&'a [NftOnLock<'a>]),
}

fn new_171<'a>(version: &'static str, event_kind: Nep171EventKind<'a>) -> NearEvent<'a> {
  NearEvent::MfightNft(Nep171Event { version, event_kind })
}

fn new_171_v1(event_kind: Nep171EventKind) -> NearEvent {
  new_171("1.0.0", event_kind)
}

