#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near view $CONTRACT_NAME market_sales_by_owner_id "{ \"account_id\": \"$ACCOUNT_ID\", \"from_index\": \"0\", \"limit\": 10000 }"
