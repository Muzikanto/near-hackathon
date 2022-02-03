#!/usr/bin/env bash

# exit on first error after this point to avoid redeploying with successful build
set -e

echo
echo ---------------------------------------------------------
echo "Step 0: Check for required environment variables"
echo ---------------------------------------------------------
echo

[ -z "$CONTRACT" ] && echo "Missing \$CONTRACT environment variable" && exit 1
[ -z "$CONTRACT" ] || echo "Found it! \$CONTRACT is set to [ $CONTRACT ]"

echo
echo
echo ---------------------------------------------------------
echo "Step 1: Delete $CONTRACT, setting $BENEFICIARY as beneficiary"
echo ---------------------------------------------------------
echo
near delete $CONTRACT "cleanup"

echo
echo ---------------------------------------------------------
echo "Step 2: Clean up project folders"
echo ---------------------------------------------------------
echo
rm -rf ./neardev

exit 0
