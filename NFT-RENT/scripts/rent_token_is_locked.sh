#!/bin/bash
source ../scripts/neardev/dev-account.env
near view $CONTRACT_NAME rent_token_is_locked "{ \"token_id\": \"test\" }"
