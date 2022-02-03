#!/bin/bash
source ../scripts/neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="4"
near call $CONTRACT_NAME rent_remove --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$ACCOUNT_ID\" }" --amount "0.01"
