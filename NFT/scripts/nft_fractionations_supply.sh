#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME nft_fractionations_supply "{}"
