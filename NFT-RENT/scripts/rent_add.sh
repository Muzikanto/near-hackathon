#!/bin/bash
source ../scripts/neardev/dev-account.env
TOKEN_ID="1"
ACCOUNT_ID="muzikant.testnet"
PRICE=1
TIME=900000000000
near call $CONTRACT_NAME rent_add --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$ACCOUNT_ID\", \"price\": $PRICE, \"time\": $TIME }" --amount "0.01"
