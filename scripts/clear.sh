#!/usr/bin/env bash

cargo run --bin hermes -- \
    --config test-data/config.toml \
    start \
    --chain-id-a chain-a \
    --chain-id-b chain-b \
    --client-id-a 07-tendermint-0 \
    --client-id-b 07-tendermint-0 \
    --clear-past-blocks 60s \
    --stop-after-blocks 0s