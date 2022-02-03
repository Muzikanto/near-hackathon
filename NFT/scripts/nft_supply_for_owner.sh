#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near view $CONTRACT_NAME nft_supply_for_owner "{ \"account_id\": \"$ACCOUNT_ID\" }"
