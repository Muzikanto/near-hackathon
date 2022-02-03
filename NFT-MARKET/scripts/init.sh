#!/bin/bash
source neardev/dev-account.env
near call $CONTRACT_NAME new_with_default_meta --accountId $CONTRACT_NAME "{ \"owner_id\": \"$CONTRACT_NAME\" }"
