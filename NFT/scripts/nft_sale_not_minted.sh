#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_sale_not_minted "{ \"sale_id\": \"badges\" }"
