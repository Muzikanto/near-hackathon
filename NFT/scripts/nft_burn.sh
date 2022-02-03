#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="3"
near call $CONTRACT_NAME nft_burn --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\" }"
