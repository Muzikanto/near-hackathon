#!/bin/bash
source neardev/dev-account.env

near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{ \"token_id\": \"707\" }"


#near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{
#  \"sale_id\":\"jewelry\"
#}"
#
#near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{
#  \"sale_id\":\"shield\"
#}"
#
#near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{
#  \"sale_id\":\"weapon\"
#}"
#
#near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{
#  \"sale_id\":\"pet\"
#}"
#
#near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{
#  \"sale_id\":\"armor\"
#}"
