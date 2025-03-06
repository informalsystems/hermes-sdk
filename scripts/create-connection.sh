#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    connection create \
    --target-chain-id chain-b \
    --counterparty-chain-id chain-a \
    --target-client-id 07-tendermint-0 \
    --counterparty-client-id 07-tendermint-0