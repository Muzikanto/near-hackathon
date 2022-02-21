#!/bin/bash
source ../scripts/neardev/dev-account.env
near view $CONTRACT_NAME rent_is_ended "{ \"token_id\": \"2\" }"
