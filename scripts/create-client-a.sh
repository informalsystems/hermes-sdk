#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    client create \
    --target-chain-id chain-a \
    --counterparty-chain-id chain-b