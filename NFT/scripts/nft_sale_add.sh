#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
SALE_ID="first"
near call $CONTRACT_NAME nft_sale_add --accountId $CONTRACT_NAME "{ \"id\": \"$SALE_ID\", \"name\": \"$SALE_ID\", \"amount\": 5, \"price\": \"2000000000000000000000000\", \"per_transaction_min\": 1, \"per_transaction_max\": 2, \"buy_max\": 4 }"
