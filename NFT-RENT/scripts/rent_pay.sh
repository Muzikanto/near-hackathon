#!/bin/bash
source ../scripts/neardev/dev-account.env
TOKEN_ID="2"
RECEIVER_ID="mfight.testnet"
TIME=900000000000
near call $CONTRACT_NAME rent_pay --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\", \"time\": $TIME, \"receiver_id\": \"$RECEIVER_ID\" }" --amount "0.1" --gas 300000000000000
