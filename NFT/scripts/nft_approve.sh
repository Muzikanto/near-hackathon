#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="706"
near call $CONTRACT_NAME nft_approve --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"dev-1643272789478-89975438845226\" }" --amount "0.1" --gas 300000000000000
