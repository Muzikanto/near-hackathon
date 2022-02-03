#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
TOKEN_ID="1"

near call $CONTRACT_NAME nft_fractionation_complete --accountId $OWNER_ID "{ \"token_id\": \"$TOKEN_ID\" }" --gas 300000000000000
