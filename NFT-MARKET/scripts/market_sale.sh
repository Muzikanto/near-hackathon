#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="52"
NFT_CONTRACT="dev-1643810988780-10800077455219"
near view $CONTRACT_NAME market_sale "{ \"nft_contract_token\": \"$NFT_CONTRACT\", \"token_id\": \"$TOKEN_ID\" }"
