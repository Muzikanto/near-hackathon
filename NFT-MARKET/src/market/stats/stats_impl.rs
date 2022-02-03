use crate::stats::MarketStats;
use near_sdk::json_types::U128;
use crate::market::base::{MarketFactory};
use crate::market::near_ft;

impl MarketStats for MarketFactory {
  fn market_avg_price(&self) -> U128 {
    let sales = &self.sales;

    let result = sales.iter().fold(0, |sum, token_id| {
      let near_price = token_id.1.sale_conditions.get(&near_ft()).unwrap().0;

      sum + near_price
    });

    U128::from(result / u128::from(sales.len()))
  }

  fn market_sum_price(&self) -> U128 {
    let sales = &self.sales;

    let result = sales.iter().fold(0, |sum, token_id| {
      let near_price = token_id.1.sale_conditions.get(&near_ft()).unwrap().0;

      sum + near_price
    });

    U128::from(result)
  }

  fn market_floor_price(&self) -> U128 {
    let min_sale = self.sales
      .iter()
      .min_by(|a, b| {
        let price1 = a.1.sale_conditions.get(&near_ft()).unwrap().0;
        let price2 = b.1.sale_conditions.get(&near_ft()).unwrap().0;

        price1.cmp(&price2)
      });

    let min = min_sale.unwrap().1.sale_conditions.get(&near_ft()).unwrap().0;

    U128::from(min)
  }

  fn market_total_owners(&self) -> U128 {
    let count = self.by_owner_id.len() as u128;

    U128::from(count)
  }
}
