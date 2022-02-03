#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_tokens_by_ids "{ \"ids\": [\"2\", \"3\"] }"
