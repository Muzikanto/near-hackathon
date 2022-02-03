#!/bin/bash
source neardev/dev-account.env

# migrate state
near call $CONTRACT_NAME migrate --accountId $CONTRACT_NAME "{ }"
