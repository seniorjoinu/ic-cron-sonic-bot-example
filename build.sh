#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package ic-cron-sonic-bot-example && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/ic_cron_sonic_bot_example.wasm -o ./target/wasm32-unknown-unknown/release/ic-cron-sonic-bot-example-opt.wasm