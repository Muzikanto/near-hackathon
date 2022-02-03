use crate::*;
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub price_per_hour: U128,
    pub min_time: u64,
    pub max_time: u64,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        assert_ne!(
            nft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(
            &owner_id,
            &signer_id,
            "owner_id should be signer_id"
        );

        let SaleArgs { max_time, min_time, price_per_hour } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid SaleArgs");

        self.rent.approved_owner_by_id.as_mut().unwrap().insert(&token_id, &owner_id);

        self.rent.assert_valid_time(&min_time);
        self.rent.assert_valid_time(&max_time);

        self.rent.internal_rent_add(&token_id, &owner_id, &price_per_hour, &min_time, &max_time);
    }
}
