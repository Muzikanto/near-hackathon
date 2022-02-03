#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME market_sum_price "{ }"
