#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="muzikant.testnet"
near view $CONTRACT_NAME market_sale "{ \"nft_contract_token\": \"TOKEN_ID\" }"
