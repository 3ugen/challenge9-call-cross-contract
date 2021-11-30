#!/bin/bash

./build.sh && \
export NEAR_ACCT=challenge9-b.3ugen.testnet && \
near delete $NEAR_ACCT 3ugen.testnet && \
sleep 1 && \
near create-account $NEAR_ACCT --masterAccount 3ugen.testnet --initialBalance 11 && \
sleep 1 && \
near deploy $NEAR_ACCT --wasmFile ./res/contract_b.wasm && \
sleep 1 && \
echo "!!! call get_balance" && \
near view $NEAR_ACCT get_balance