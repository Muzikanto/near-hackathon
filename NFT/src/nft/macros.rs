// Core

#[macro_export]
macro_rules! impl_non_fungible_token_core {
    ($contract: ident, $token: ident) => {
        use $crate::nft::base::NonFungibleTokenCore;
        use $crate::nft::base::NonFungibleTokenResolver;

        #[near_bindgen]
        impl NonFungibleTokenCore for $contract {
            #[payable]
            fn nft_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
            ) {
                self.$token.nft_transfer(receiver_id, token_id, approval_id, memo)
            }

            #[payable]
            fn nft_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                self.$token.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
            }

            fn nft_token(&self, token_id: TokenId) -> Option<Token> {
                self.$token.nft_token(token_id)
            }
        }

        #[near_bindgen]
        impl NonFungibleTokenResolver for $contract {
            #[private]
            fn nft_resolve_transfer(
                &mut self,
                previous_owner_id: AccountId,
                receiver_id: AccountId,
                token_id: TokenId,
                approved_account_ids: Option<std::collections::HashMap<AccountId, u64>>,
            ) -> bool {
                self.$token.nft_resolve_transfer(
                    previous_owner_id,
                    receiver_id,
                    token_id,
                    approved_account_ids,
                )
            }
        }
    };
}

// Approval

#[macro_export]
macro_rules! impl_non_fungible_token_approval {
    ($contract: ident, $token: ident) => {
        use $crate::NonFungibleTokenApproval;

        #[near_bindgen]
        impl NonFungibleTokenApproval for $contract {
            #[payable]
            fn nft_approve(
                &mut self,
                token_id: TokenId,
                account_id: AccountId,
                msg: Option<String>,
            ) -> Option<Promise> {
                self.$token.nft_approve(token_id, account_id, msg)
            }

            #[payable]
            fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                self.$token.nft_revoke(token_id, account_id)
            }

            #[payable]
            fn nft_revoke_all(&mut self, token_id: TokenId) {
                self.$token.nft_revoke_all(token_id)
            }

            fn nft_is_approved(
                &self,
                token_id: TokenId,
                approved_account_id: AccountId,
                approval_id: Option<u64>,
            ) -> bool {
                self.$token.nft_is_approved(token_id, approved_account_id, approval_id)
            }
        }
    };
}

// Enumeration

#[macro_export]
macro_rules! impl_non_fungible_token_enumeration {
    ($contract: ident, $token: ident) => {
        use $crate::NonFungibleTokenEnumeration;

        #[near_bindgen]
        impl NonFungibleTokenEnumeration for $contract {
            fn nft_total_supply(&self) -> near_sdk::json_types::U128 {
                self.$token.nft_total_supply()
            }

            fn nft_tokens(
                &self,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.nft_tokens(from_index, limit)
            }

            fn nft_supply_for_owner(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
                self.$token.nft_supply_for_owner(account_id)
            }

            fn nft_tokens_for_owner(
                &self,
                account_id: AccountId,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.nft_tokens_for_owner(account_id, from_index, limit)
            }

            fn nft_tokens_by_ids(
                &self,
                ids: Vec<TokenId>,
            ) -> Vec<Token> {
                self.$token.nft_tokens_by_ids(ids)
            }
        }
    };
}

// Market Payout

#[macro_export]
macro_rules! impl_non_fungible_token_payout {
    ($contract: ident, $token: ident) => {
        use $crate::{NonFungibleTokenPayout, Payout};

        #[near_bindgen]
        impl NonFungibleTokenPayout for $contract {
          #[payable]
          fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, memo: String, balance: U128, max_len_payout: u32) -> Payout {
              self.$token.nft_transfer_payout(receiver_id, token_id, approval_id, memo, balance, max_len_payout)
          }

          fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
              self.$token.nft_payout(token_id, balance, max_len_payout)
          }
        }
    };
}

// Royalty

#[macro_export]
macro_rules! impl_non_fungible_token_royalty {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenRoyalty};

        #[near_bindgen]
        impl NonFungibleTokenRoyalty for $contract {
          fn set_contract_royalty(&mut self, contract_royalty: u32) {
            self.$tokens.set_contract_royalty(contract_royalty);
          }
        }
    };
}

// Sales

#[macro_export]
macro_rules! impl_non_fungible_token_sales {
    ($contract: ident, $tokens: ident) => {
        use $crate::{SaleCore, SaleEnumeration, SaleId, JsonSale};

        #[near_bindgen]
        impl SaleCore for $contract {
          fn nft_sale_add(&mut self, id: String, name: String, amount: u64, price: U128, per_transaction_min: u32, per_transaction_max: u32, buy_max: u32) -> JsonSale {
            self.$tokens.nft_sale_add(id, name, amount, price, per_transaction_min, per_transaction_max, buy_max)
          }

          fn nft_sale_start(&mut self, sale_id: SaleId, date: u64) -> JsonSale {
            self.$tokens.nft_sale_start(sale_id, date)
          }

          fn nft_sale_update(&mut self, sale_id: SaleId, date: u64) -> JsonSale {
            self.$tokens.nft_sale_update(sale_id, date)
          }
        }

          #[near_bindgen]
        impl SaleEnumeration for $contract {
          fn nft_sale_tokens(&self, sale_id: SaleId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<Token> {
            self.$tokens.nft_sale_tokens(sale_id, from_index, limit)
          }

          fn nft_sales(&self) -> Vec<JsonSale> {
            self.$tokens.nft_sales()
          }

          fn nft_sale(&self, sale_id: SaleId) -> JsonSale {
            self.$tokens.nft_sale(sale_id)
          }

          fn nft_sale_not_minted(&self, sale_id: SaleId) -> u64 {
            self.$tokens.nft_sale_not_minted(sale_id)
          }

          fn nft_sales_locked(&self) -> Vec<String> {
            self.$tokens.nft_sales_locked()
          }

          fn nft_sale_token_locked(&self, token_id: TokenId) -> bool {
            self.$tokens.nft_sale_token_locked(token_id)
          }

          fn nft_sale_token_ids(&self, sale_id: SaleId, from_index: Option<near_sdk::json_types::U128>, limit: Option<u64>) -> Vec<TokenId> {
             self.$tokens.nft_sale_token_ids(sale_id, from_index, limit)
          }

          fn nft_sale_account_minted(&self, sale_id: SaleId, account_id: AccountId) -> u32 {
            self.$tokens.nft_sale_account_minted(sale_id, account_id)
          }
        }
    };
}

// Fractionation

#[macro_export]
macro_rules! impl_non_fungible_token_fractionation {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenFractionation, Fractionation};

        #[near_bindgen]
        impl NonFungibleTokenFractionation for $contract {
          fn nft_fractionation(&self, token_id: TokenId) -> Fractionation {
            self.$tokens.nft_fractionation(token_id)
          }
          fn nft_fractionations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Fractionation> {
            self.$tokens.nft_fractionations(from_index, limit)
          }
          fn nft_fractionations_supply(&self) -> U128 {
            self.$tokens.nft_fractionations_supply()
          }
          fn nft_fractionation_complete(&mut self, token_id: TokenId) {
            self.$tokens.nft_fractionation_complete(token_id)
          }
        }
    };
}

// Burn

#[macro_export]
macro_rules! impl_non_fungible_token_burn {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenBurn};

        #[near_bindgen]
        impl NonFungibleTokenBurn for $contract {
          #[payable]
          fn nft_burn(&mut self, token_id: &TokenId) {
            self.$tokens.nft_burn(token_id)
          }
        }
    };
}

// Locked

#[macro_export]
macro_rules! impl_non_fungible_token_locked {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenLocked};

        #[near_bindgen]
        impl NonFungibleTokenLocked for $contract {
          fn nft_token_is_locked(&self, token_id: TokenId) -> bool {
            self.$tokens.nft_token_is_locked(token_id)
          }

          fn nft_on_lock(&mut self, token_id: TokenId, locked: bool) {
            self.$tokens.nft_on_lock(token_id, locked)
          }
        }
    };
}

// Bind To Owner

#[macro_export]
macro_rules! impl_non_fungible_token_bind_to_owner {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenBindToOwner};

        #[near_bindgen]
        impl NonFungibleTokenBindToOwner for $contract {
          fn nft_token_is_bind_to_owner(&self, token_id: TokenId) -> bool {
            self.$tokens.nft_token_is_bind_to_owner(token_id)
          }
        }
    };
}

// Mint

#[macro_export]
macro_rules! impl_non_fungible_token_mint {
    ($contract: ident, $tokens: ident) => {
        use $crate::{NonFungibleTokenMint};

        #[near_bindgen]
        impl NonFungibleTokenMint for $contract {
          fn nft_create(
            &mut self,
            token_id: TokenId,
            receiver_id: Option<AccountId>,
            token_metadata: TokenMetadata,
            rarity: TokenRarity,
            collection: TokenCollection,
            token_type: TokenType,
            bind_to_owner: Option<bool>,
            sale_id: Option<SaleId>,
            perpetual_royalties: Option<Royalty>,
            fractionation_id: Option<TokenId>,
          ) -> Token {
            self.$tokens.nft_create(
              token_id,
              receiver_id,
              token_metadata,
              rarity,
              collection,
              token_type,
              bind_to_owner,
              sale_id,
              perpetual_royalties,
              fractionation_id
            )
          }
          #[payable]
          fn nft_mint(&mut self, receiver_id: AccountId, sale_id: SaleId, amount: u32) {
            self.$tokens.nft_mint(receiver_id, sale_id, amount)
          }
        }
    };
}
