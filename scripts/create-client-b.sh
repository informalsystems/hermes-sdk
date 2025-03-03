#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    client create \
    --target-chain chain-b \
    --counterparty-chain chain-a