#!/bin/bash
source ../scripts/neardev/dev-account.env
TOKEN_ID="test"
near view $CONTRACT_NAME rent --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\" }"
