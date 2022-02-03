#!/bin/bash
source neardev/dev-account.env
OWNER_ID="mfight-nft.testnet"
SALE_ID="badges"

near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 1\",
    \"description\": \"badge 1 text\",
    \"media\": \"https://mfight.io/static/nft/badge/1.png\"
  },
  \"token_id\": \"1\",
  \"sale_id\": \"$SALE_ID\",
  \"rarity\": \"Common\",
  \"collection\": \"Unknown\",
  \"bind_to_owner\": false,
  \"royalty\": {}
}"
