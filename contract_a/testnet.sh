#!/bin/bash

./build.sh && \
export NEAR_ACCT=challenge9-a.3ugen.testnet && \
near delete $NEAR_ACCT 3ugen.testnet && \
sleep 1 && \
near create-account $NEAR_ACCT --masterAccount 3ugen.testnet --initialBalance 8 && \
sleep 1 && \
near deploy $NEAR_ACCT --wasmFile ./res/contract_a.wasm && \
sleep 1 && \
echo "!!! call get_balance" && \
near view $NEAR_ACCT get_balance
sleep 1 && \
near call $NEAR_ACCT call_balance_ext '{"receiver_id": "challenge9-b.3ugen.testnet"}' --accountId 3ugen.testnet