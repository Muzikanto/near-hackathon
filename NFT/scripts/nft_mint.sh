#!/bin/bash
source neardev/dev-account.env
RECEIVER_ID="muzikant.testnet"
SALE_ID="badges"

near call $CONTRACT_NAME nft_mint --accountId $RECEIVER_ID "{ \"receiver_id\": \"$RECEIVER_ID\", \"sale_id\": \"$SALE_ID\", \"amount\": 1 }" --amount 2
