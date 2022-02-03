#!/bin/bash
source neardev/dev-account.env
sh build.sh && near deploy --accountId mfight-market.testnet --wasmFile ../res/market.wasm --helperUrl https://near-contract-helper.onrender.com
