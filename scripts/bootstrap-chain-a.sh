#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    bootstrap chain \
    --chain-id chain-a \
    --chain-store-dir test-data/chain-a