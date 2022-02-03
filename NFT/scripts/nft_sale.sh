#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_sale "{ \"sale_id\": \"badges\" }"
