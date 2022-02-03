use near_sdk::{env, require, AccountId};
use near_sdk::json_types::{U128};
use crate::{TokenId, NonFungibleToken, Token};
use crate::{SaleId, Sale, SaleCore, SaleEnumeration};
use crate::nft::base::NonFungibleTokenCore;
use crate::nft::JsonSale;
use crate::nft::events::{SaleCreate, SaleStart, SaleUpdate};

impl SaleCore for NonFungibleToken {
  fn nft_sale_add(&mut self, id: String, name: String, amount: u64, price: U128, per_transaction_min: u32, per_transaction_max: u32, buy_max: u32) -> JsonSale {
    self.assert_owner();

    assert!(self.sales_locked.as_mut().unwrap().insert(&id), "Token type should not be locked");

    let sale = Sale {
      name,
      amount,
      price,
      per_transaction_min,
      per_transaction_max,
      buy_max,
    };
    assert!(self.sale_by_id.as_mut().unwrap().insert(id.clone(), sale).is_none(), "Token type exists");

    let json_sale = self.enum_get_sale(&id);

    SaleCreate {
      sale: &json_sale,
    }.emit();

    json_sale
  }

  fn nft_sale_start(&mut self, sale_id: SaleId, date: u64) -> JsonSale {
    self.assert_owner();

    let sale = self.sale_by_id.as_ref().unwrap().get(&sale_id).expect("Not found sale");

    assert!(self.sales_locked.as_ref().unwrap().contains(&sale_id), "Sale already unlocked");

    let tokens_per_sale = self.sale_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale tokens").as_vector().to_vec();

    assert_eq!(u64::from(sale.amount) as usize, tokens_per_sale.len(), "Not all of tokens are minted");

    self.sales_available.as_mut().unwrap().insert(&sale_id);
    self.sale_random_tokens.as_mut().unwrap().insert(&sale_id, &tokens_per_sale);
    self.sales_locked.as_mut().unwrap().remove(&sale_id);
    self.sale_date_by_id.as_mut().unwrap().insert(sale_id.clone(), date);

    SaleStart {
      sale_id: &sale_id,
      date: &date
    }.emit();

    self.enum_get_sale(&sale_id)
  }

  fn nft_sale_update(&mut self, sale_id: SaleId, date: u64) -> JsonSale {
    self.assert_owner();
    self.assert_sale_not_locked(&sale_id);
    self.sale_date_by_id.as_mut().unwrap().insert(sale_id.clone(), date);

    SaleUpdate {
      sale_id: &sale_id,
      date: &date
    }.emit();

    self.enum_get_sale(&sale_id)
  }
}

impl SaleEnumeration for NonFungibleToken {
    fn nft_sale_tokens(&self, sale_id: SaleId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
    let ids = self.nft_sale_token_ids(sale_id, from_index, limit);

    ids
      .iter()
      .map(|token_id| self.nft_token(token_id.clone()).unwrap())
      .collect()
  }

  fn nft_sales(&self) -> Vec<JsonSale> {
    self.sales_available.as_ref().unwrap()
      .iter()
      .map(|sale_id| {
        self.enum_get_sale(&sale_id)
      })
      .collect()
  }

  fn nft_sale(&self, sale_id: SaleId) -> JsonSale {
    self.enum_get_sale(&sale_id)
  }

  fn nft_sale_not_minted(&self, sale_id: SaleId) -> u64 {
    let rand_tokens = self.sale_random_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale");

    rand_tokens.len() as u64
  }

  fn nft_sales_locked(&self) -> Vec<String> {
    self.sales_locked.as_ref().unwrap().to_vec()
  }

  fn nft_sale_token_locked(&self, token_id: TokenId) -> bool {
    let sale_id = self.sale_by_token.as_ref().unwrap().get(&token_id).expect("Not found token");
    // assert!(token.token_type.is_some(), "Token must have type");
    self.sales_locked.as_ref().unwrap().contains(&sale_id)
  }

  fn nft_sale_token_ids(
    &self,
    sale_id: SaleId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<TokenId> {
    let tokens_per_sale = self.sale_tokens.as_ref().unwrap_or_else(|| {
      env::panic_str(
        "Could not find tokens_per_sale when calling a method on the \
                enumeration standard.",
      )
    });
    let token_set = if let Some(token_set) = tokens_per_sale.get(&sale_id) {
      token_set
    } else {
      return vec![];
    };
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    require!(limit != 0, "Cannot provide limit of 0.");
    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    require!(
            token_set.len() as u128 > start_index,
            "Out of bounds, please use a smaller from_index."
        );
    token_set
      .iter()
      // .skip(start_index as usize)
      // .take(limit)
      // .map(|token_id| self.enum_get_token(self.owner_by_id.get(&token_id).unwrap(), token_id))
      .collect()
  }

  fn nft_sale_account_minted(&self, sale_id: SaleId, account_id: AccountId) -> u32 {
    let owner_minted = self.internal_mint_counter_by_sale(&account_id, &sale_id);

    owner_minted
  }
}
