#!/bin/bash
source neardev/dev-account.env
SALE_ID="badges"
near view $CONTRACT_NAME nft_sale_token_ids "{ \"sale_id\": \"$SALE_ID\" }"
