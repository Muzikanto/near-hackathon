#!/bin/bash
source neardev/dev-account.env
OWNER_ID="$CONTRACT_NAME"
near call $CONTRACT_NAME new_with_default_meta --accountId $CONTRACT_NAME "{ \"owner_id\": \"$OWNER_ID\" }"
