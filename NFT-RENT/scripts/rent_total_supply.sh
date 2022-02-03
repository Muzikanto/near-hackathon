#!/bin/bash
source ../scripts/neardev/dev-account.env
near view $CONTRACT_NAME rent_total_supply "{}"
