#!/bin/bash
source ../scripts/neardev/dev-account.env
CALLER_ID="dev-1643811241604-32051819748095"
TOKEN_ID="140"
RECEIVER_ID="$CALLER_ID"
TIME=86400000
near call $CONTRACT_NAME rent_pay --accountId $CALLER_ID "{ \"token_id\": \"$TOKEN_ID\", \"time\": $TIME, \"receiver_id\": \"$RECEIVER_ID\" }" --amount "190" --gas 300000000000000
