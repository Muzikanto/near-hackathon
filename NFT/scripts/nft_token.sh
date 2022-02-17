#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_token "{ \"token_id\": \"356\" }"
