#[macro_export]
macro_rules! impl_market_stats {
    ($contract: ident, $tokens: ident) => {
        use $crate::MarketStats;

        #[near_bindgen]
        impl MarketStats for $contract {
          fn market_avg_price(&self) -> U128 {
            self.$tokens.market_avg_price()
          }
          fn market_sum_price(&self) -> U128 {
            self.$tokens.market_sum_price()
          }
          fn market_floor_price(&self) -> U128 {
            self.$tokens.market_floor_price()
          }
          fn market_total_owners(&self) -> U128 {
            self.$tokens.market_total_owners()
          }
        }
    };
}

#[macro_export]
macro_rules! impl_market_core {
    ($contract: ident, $tokens: ident) => {
        use $crate::MarketCore;

        #[near_bindgen]
        impl MarketCore for $contract {
          #[payable]
          fn market_remove_sale(&mut self, nft_contract_id: AccountId, token_id: String) {
            self.$tokens.market_remove_sale(nft_contract_id, token_id)
          }

          #[payable]
          fn market_update_price(
            &mut self,
            nft_contract_id: AccountId,
            token_id: String,
            ft_token_id: AccountId,
            price: U128,
          ) {
            self.$tokens.market_update_price(nft_contract_id, token_id, ft_token_id, price)
          }

          #[payable]
          fn market_offer(&mut self, nft_contract_id: AccountId, token_id: String) {
            self.$tokens.market_offer(nft_contract_id, token_id)
          }

          #[private]
          fn market_add_bid(
            &mut self,
            contract_and_token_id: ContractAndTokenId,
            amount: Balance,
            ft_token_id: AccountId,
            buyer_id: AccountId,
            sale: &mut Sale,
          ) {
            self.$tokens.market_add_bid(contract_and_token_id, amount, ft_token_id, buyer_id, sale)
          }

          fn market_accept_offer(
            &mut self,
            nft_contract_id: AccountId,
            token_id: String,
            ft_token_id: AccountId,
          ) {
            self.$tokens.market_accept_offer(nft_contract_id, token_id, ft_token_id)
          }

          #[private]
          fn market_process_purchase(
            &mut self,
            nft_contract_id: AccountId,
            token_id: String,
            ft_token_id: AccountId,
            price: U128,
            buyer_id: AccountId,
          ) -> Promise {
            self.$tokens.market_process_purchase(nft_contract_id, token_id, ft_token_id, price, buyer_id)
          }

          #[private]
          fn market_resolve_purchase(
            &mut self,
            ft_token_id: AccountId,
            buyer_id: AccountId,
            sale: Sale,
            price: U128,
          ) -> U128 {
            self.$tokens.market_resolve_purchase(ft_token_id, buyer_id, sale, price)
          }
        }
    };
}

#[macro_export]
macro_rules! impl_market_enumeration {
    ($contract: ident, $tokens: ident) => {
        use $crate::MarketEnumeration;

        #[near_bindgen]
        impl MarketEnumeration for $contract {
          fn market_supply_sales(&self) -> U64 {
            self.$tokens.market_supply_sales()
          }

          fn market_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
           self.$tokens.market_supply_by_owner_id(account_id)
          }

          fn market_sales_by_owner_id(&self, account_id: AccountId, from_index: U64, limit: u64) -> Vec<Sale> {
            self.$tokens.market_sales_by_owner_id(account_id, from_index, limit)
          }

          fn market_supply_by_nft_contract_id(&self, nft_contract_id: AccountId) -> U64 {
            self.$tokens.market_supply_by_nft_contract_id(nft_contract_id)
          }

          fn market_sales_by_nft_contract_id(&self, nft_contract_id: AccountId, from_index: U64, limit: u64) -> Vec<Sale> {
            self.$tokens.market_sales_by_nft_contract_id(nft_contract_id, from_index, limit)
          }

          fn market_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale> {
            self.$tokens.market_sale(nft_contract_token)
          }
        }
    }
}
