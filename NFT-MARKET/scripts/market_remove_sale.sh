#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="1"
#NFT_CONTRACT="mfight-nft.testnet"
near call $CONTRACT_NAME market_remove_sale --accountId $CONTRACT_NAME "{ \"nft_contract_id\": \"$CONTRACT_NAME\", \"token_id\": \"$TOKEN_ID\" }" --amount "0.01"
