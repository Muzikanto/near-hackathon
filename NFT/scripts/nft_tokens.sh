#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_tokens "{ \"limit\": 1, \"from_index\": \"50\" }"
