#!/bin/bash
source ../scripts/neardev/dev-account.env
FROM_INDEX=0
LIMIT=100
near view $CONTRACT_NAME rents "{ \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
