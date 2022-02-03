#!/bin/bash
source ../scripts/neardev/dev-account.env
near view $CONTRACT_NAME rent_avg_price --accountId $CONTRACT_NAME "{ }"
