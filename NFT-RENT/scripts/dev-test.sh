#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
PRICE="100000000000000000000000"
TIME=900000

near call $CONTRACT_NAME rent_add --accountId $CONTRACT_NAME "{ \"token_id\": \"1\", \"account_id\": \"$ACCOUNT_ID\", \"price\": \"$PRICE\", \"time\": $TIME }"
near call $CONTRACT_NAME rent_add --accountId $CONTRACT_NAME "{ \"token_id\": \"2\", \"account_id\": \"$ACCOUNT_ID\", \"price\": \"$PRICE\", \"time\": $TIME }"
near call $CONTRACT_NAME rent_remove --accountId $CONTRACT_NAME "{ \"token_id\": \"2\", \"account_id\": \"$ACCOUNT_ID\" }"
