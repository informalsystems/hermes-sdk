#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    client create \
    --target-chain-id chain-b \
    --counterparty-chain-id chain-a