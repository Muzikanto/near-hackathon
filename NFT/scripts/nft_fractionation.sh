#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="4019"
near view $CONTRACT_NAME nft_fractionation "{ \"token_id\": \"$TOKEN_ID\" }"
