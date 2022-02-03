pub use macros::*;
pub use utils::*;

pub use self::base::{RentFactoryCore, RentFactoryResolve};
pub use self::enumeration::RentFactoryEnumeration;
pub use self::stats::RentFactoryStats;
pub use self::meta::{TokenId, JsonRent, Rent};

pub mod enumeration;
pub mod stats;
pub mod base;
pub mod meta;

mod utils;
mod macros;
mod approved_check;

pub mod events;
// use self::events::*;

pub trait RentFactoryReceiver {
  fn rent_on_lock(&self, token_id: TokenId, locked: bool);
}
