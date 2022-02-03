use crate::stats::RentFactoryStats;
use near_sdk::json_types::U128;
use crate::base::RentFactory;

impl RentFactoryStats for RentFactory {
  fn rent_avg_price(&self) -> U128 {
    let rents = &self.rents_pending;

    let result = rents.iter().fold(0, |sum, token_id| {
      let rent = self.rents_by_id.get(&token_id).unwrap();

      sum + rent.price_per_hour.0
    });

    U128::from(result / u128::from(rents.len()))
  }

  fn rent_current_total_supply(&self) -> U128 {
    let count = self.rents_current.len() as u128;

    U128::from(count)
  }

  fn rent_floor_price(&self) -> U128 {
    let rents = &self.rents_pending;

    let min_rent = rents
      .iter()
      .min_by(|a, b| {
        let rent1 = self.rents_by_id.get(&a).unwrap();
        let rent2 = self.rents_by_id.get(&b).unwrap();

        rent1.price_per_hour.0.cmp(&rent2.price_per_hour.0)
      });

    if let Some(min_rent) = min_rent {
      let rent = self.rents_by_id.get(&min_rent).unwrap();

      return rent.price_per_hour;
    }

    U128::from(0)
  }

  fn rent_total_owners(&self) -> U128 {
    let count = self.rents_per_account.len() as u128;

    U128::from(count)
  }
}
