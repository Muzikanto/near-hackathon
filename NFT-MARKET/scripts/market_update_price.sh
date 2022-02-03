#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
TOKEN_ID="1"
near call $CONTRACT_NAME update_price --accountId $CONTRACT_NAME "{ \"nft_contract_id\": \"$NFT_CONTRACT\", \"token_id\": \"$TOKEN_ID\", \"ft_token_id\": \"near\", \"price\": \"1\" }" --amount 1
