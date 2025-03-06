#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    channel create \
    --target-chain-id chain-b \
    --counterparty-chain-id chain-a \
    --target-client-id 07-tendermint-0 \
    --counterparty-client-id 07-tendermint-0 \
    --target-connection-id connection-0 \
    --target-port-id transfer \
    --counterparty-port-id transfer \
    --ordering unordered \
    --version ics20-1