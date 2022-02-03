#!/bin/bash
source ../scripts/neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="706"
near call $CONTRACT_NAME rent_claim --accountId $ACCOUNT_ID "{ \"account_id\": \"$ACCOUNT_ID\", \"token_id\": \"$TOKEN_ID\" }" --gas 300000000000000 --amount "0.01"
#3000000000000
#2428050684172
#2500000000000
#2428050684172
