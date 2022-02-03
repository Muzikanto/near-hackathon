#!/bin/bash
source neardev/dev-account.env
OWNER_ID="$CONTRACT_NAME"
ACCOUNT_ID="muzikant.testnet"
SALE_ID="badges2"
SALE2_ID="hero2"
FRACTIONATION_ID="1"

# create sale
near call $CONTRACT_NAME nft_sale_add --accountId $OWNER_ID "{ \"id\": \"$SALE_ID\", \"name\": \"$SALE_ID\", \"amount\": 4, \"price\": \"2000000000000000000000000\", \"per_transaction_min\": 1, \"per_transaction_max\": 2, \"buy_max\": 4 }"
near call $CONTRACT_NAME nft_sale_add --accountId $OWNER_ID "{ \"id\": \"$SALE2_ID\", \"name\": \"$SALE2_ID\", \"amount\": 3, \"price\": \"2000000000000000000000000\", \"per_transaction_min\": 1, \"per_transaction_max\": 3, \"buy_max\": 3 }"

# create nft
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 1\",
    \"description\": \"badge 1 text\",
    \"media\": \"https://mfight.io/static/nft/badge/1.png\"
  },
  \"rarity\": \"Common\",
  \"collection\": \"Fantasy\",
  \"token_type\": \"Badge\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"1\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 2\",
    \"description\": \"badge 2 text\",
    \"media\": \"https://mfight.io/static/nft/badge/2.png\"
  },
  \"sale_id\": \"$SALE_ID\",
  \"rarity\": \"Rare\",
  \"collection\": \"Medieval\",
  \"token_type\": \"Badge\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"2\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 3\",
    \"description\": \"badge 3 text\",
    \"media\": \"https://mfight.io/static/nft/badge/3.png\"
  },
  \"sale_id\": \"$SALE_ID\",
  \"rarity\": \"Uniq\",
  \"collection\": \"Nordic\",
  \"token_type\": \"Badge\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"3\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 4\",
    \"description\": \"badge 4 text\",
    \"media\": \"https://mfight.io/static/nft/badge/4.png\"
  },
  \"sale_id\": \"$SALE_ID\",
  \"rarity\": \"Epic\",
  \"collection\": \"PostApoc\",
  \"token_type\": \"Badge\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"4\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Badge 5\",
    \"description\": \"badge 5 text\",
    \"media\": \"https://mfight.io/static/nft/badge/5.png\"
  },
  \"sale_id\": \"$SALE_ID\",
  \"rarity\": \"Legendary\",
  \"collection\": \"SteamPunk\",
  \"token_type\": \"Badge\",
  \"bind_to_owner\": true,
  \"royalty\": {},
  \"token_id\": \"5\"
}"

near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Hero 1\",
    \"description\": \"hero 1 text\",
    \"media\": \"https://mfight.io/static/nft/hero/1.png\"
  },
  \"sale_id\": \"$SALE2_ID\",
  \"rarity\": \"Uniq\",
  \"collection\": \"Nordic\",
  \"token_type\": \"Hero\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"6\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Hero 2\",
    \"description\": \"hero 2 text\",
    \"media\": \"https://mfight.io/static/nft/hero/2.png\"
  },
  \"sale_id\": \"$SALE2_ID\",
  \"rarity\": \"Epic\",
  \"collection\": \"PostApoc\",
  \"token_type\": \"Hero\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"7\"
}"
near call $CONTRACT_NAME nft_create --accountId $OWNER_ID "{
  \"token_metadata\": {
    \"title\": \"Hero 3\",
    \"description\": \"hero 3 text\",
    \"media\": \"https://mfight.io/static/nft/hero/3.png\"
  },
  \"sale_id\": \"$SALE2_ID\",
  \"rarity\": \"Legendary\",
  \"collection\": \"Medieval\",
  \"token_type\": \"Hero\",
  \"bind_to_owner\": false,
  \"royalty\": {},
  \"token_id\": \"8\"
}"

# start sale
near call $CONTRACT_NAME nft_sale_start --accountId $OWNER_ID "{ \"sale_id\": \"$SALE_ID\", \"date\": 1642594586098000000 }"
near call $CONTRACT_NAME nft_sale_start --accountId $OWNER_ID "{ \"sale_id\": \"$SALE2_ID\", \"date\": 1642594586098000000 }"

near call $CONTRACT_NAME nft_mint --accountId $OWNER_ID "{ \"receiver_id\": \"$ACCOUNT_ID\", \"sale_id\": \"$SALE_ID\", \"amount\": 2 }" --amount 4
near call $CONTRACT_NAME nft_mint --accountId $OWNER_ID "{ \"receiver_id\": \"$ACCOUNT_ID\", \"sale_id\": \"$SALE_ID\", \"amount\": 2 }" --amount 4
near call $CONTRACT_NAME nft_mint --accountId $OWNER_ID "{ \"receiver_id\": \"$ACCOUNT_ID\", \"sale_id\": \"$SALE2_ID\", \"amount\": 3 }" --amount 6

