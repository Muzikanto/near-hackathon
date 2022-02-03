pub use royalty_impl::*;

pub mod royalty_impl;

pub trait NonFungibleTokenRoyalty {
  fn set_contract_royalty(&mut self, contract_royalty: u32);
}
