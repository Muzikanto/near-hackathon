#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
APPROVED_ID="dev-1643272789478-89975438845226"
TOKEN_ID="706"
near call $CONTRACT_NAME nft_approve --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$APPROVED_ID\" }" --amount "0.1" --gas 300000000000000
