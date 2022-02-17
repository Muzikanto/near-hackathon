#!/bin/bash
source neardev/dev-account.env
NFT_CONTRACT="dev-1643810988780-10800077455219"
near view $CONTRACT_NAME market_sales_by_nft_contract_id "{ \"nft_contract_id\": \"$NFT_CONTRACT\", \"from_index\": \"0\", \"limit\": 10000 }"
