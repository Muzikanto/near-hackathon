#!/bin/bash
source ../scripts/neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
FROM_INDEX=0
LIMIT=1000
near view $CONTRACT_NAME rents_for_account "{ \"account_id\": \"$ACCOUNT_ID\", \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
