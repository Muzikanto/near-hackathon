use crate::nft::{NonFungibleToken, Token, TokenId, random_number};
use near_sdk::{AccountId, env};
use crate::nft::metadata::{TokenRarity, TokenMetadata, TokenType, TokenCollection};
use crate::nft::mint::NonFungibleTokenMint;
use crate::nft::royalty::Royalty;
use crate::SaleId;
use crate::nft::events_171::NftMint;

impl NonFungibleToken {
  fn internal_random_tokens(&mut self, sale_id: &SaleId, amount: &u32) -> Vec<TokenId> {
    let mut random_tokens = self.sale_random_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale");

    let mut index = 0;
    let mut tokens = Vec::new();

    loop {
      if &index == amount {
        break;
      }

      let rand_index = random_number(random_tokens.to_vec().len().clone());
      let token_id = random_tokens.get(rand_index).expect("Invalid token index").clone();

      random_tokens.remove(rand_index);
      self.sale_random_tokens.as_mut().unwrap().insert(&sale_id, &random_tokens);

      assert_eq!(&self.owner_by_id.get(&token_id).unwrap(), &self.owner_id, "Token already minted");

      tokens.push(token_id);
      index = index + 1;
    }

    tokens
  }
}

impl NonFungibleTokenMint for NonFungibleToken {
  fn nft_create(
    &mut self,
    token_id: TokenId,
    _receiver_id: Option<AccountId>,
    token_metadata: TokenMetadata,
    rarity: TokenRarity,
    collection: TokenCollection,
    token_type: TokenType,
    bind_to_owner: Option<bool>,
    sale_id: Option<SaleId>,
    perpetual_royalties: Option<Royalty>,
    fractionation_id: Option<TokenId>,
  ) -> Token {
    self.assert_owner();

    self.internal_create_nft(
      &token_id,
      Some(self.owner_id.clone()),
      Some(token_metadata),
      rarity,
      collection,
      bind_to_owner,
      sale_id,
      perpetual_royalties,
      fractionation_id,
      token_type,
    )
  }

  fn nft_mint(&mut self, receiver_id: AccountId, sale_id: SaleId, amount: u32) {
    self.assert_sale_not_locked(&sale_id);
    self.assert_sale_started(&sale_id);

    let sale = self.sale_by_id.as_ref().unwrap().get(&sale_id).expect("Not found sale");
    let buy_max = sale.buy_max;
    let per_transaction_min = sale.per_transaction_min;
    let per_transaction_max = sale.per_transaction_max;

    let rest_amount = self.sale_random_tokens.as_ref().unwrap().get(&sale_id).expect("Not found sale").len() as u32;
    let owner_minted = self.internal_mint_counter_by_sale(&receiver_id, &sale_id);

    if amount > rest_amount {
      env::panic_str("Insufficient amount of nft");
    }
    assert!(owner_minted + amount <= buy_max, "Mint limit");
    assert!(amount <= per_transaction_max, "Invalid mint max amount");
    assert!(amount >= per_transaction_min, "Invalid mint min amount");

    let deposit = env::attached_deposit();
    let price = sale.price.0;
    assert!(deposit >= price * (amount as u128), "Invalid attached deposit");

    let tokens = self.internal_random_tokens(&sale_id, &amount);

    tokens.iter().for_each(|token_id| {
      self.internal_transfer_unguarded(&token_id, &self.owner_id.clone(), &receiver_id);
    });

    let next_minted = u32::from(owner_minted + amount);
    self.internal_mint_counter_change(&receiver_id, &sale_id, &next_minted);

    NftMint {
      owner_id: &receiver_id,
      token_ids: &tokens,
      memo: None
    }.emit()
  }
}
