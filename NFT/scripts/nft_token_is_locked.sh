#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="2"
near view $CONTRACT_NAME nft_token_is_locked "{ \"token_id\": \"$TOKEN_ID\" }"
