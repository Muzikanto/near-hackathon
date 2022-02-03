#!/bin/bash
source neardev/dev-account.env
SALE_ID="badges"
near call $CONTRACT_NAME nft_sale_start --accountId $CONTRACT_NAME "{ \"sale_id\": \"$SALE_ID\", \"date\": 1642594586098 }"
