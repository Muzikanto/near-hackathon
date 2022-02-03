#!/bin/bash
source neardev/dev-account.env
FROM_INDEX="0"
LIMIT=1000
near view $CONTRACT_NAME nft_fractionations "{ \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
