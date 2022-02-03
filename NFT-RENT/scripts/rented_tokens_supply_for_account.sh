#!/bin/bash
source ../scripts/neardev/dev-account.env
#ACCOUNT_ID=$CONTRACT_NAME
ACCOUNT_ID="muzikant.testnet"
near view $CONTRACT_NAME rented_tokens_supply_for_account "{ \"account_id\": \"$ACCOUNT_ID\" }"
