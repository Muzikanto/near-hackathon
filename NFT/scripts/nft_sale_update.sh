#!/bin/bash
source neardev/dev-account.env
SALE_ID="sketches_test"
near call $CONTRACT_NAME nft_sale_update --accountId $CONTRACT_NAME "{ \"sale_id\": \"$SALE_ID\", \"date\": 1643280497042 }"
