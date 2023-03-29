#!/bin/bash

# set -eux

CALLER_ADDRESS=""
CALLEE_ADDRESS=""

cargo contract build --manifest-path caller/Cargo.toml
cargo contract build --manifest-path callee/Cargo.toml

CALLER_ADDRESS=$(cargo contract instantiate --constructor default \
    --suri //Alice --salt $(date +%s) \
    --manifest-path caller/Cargo.toml \
    --output-json --skip-confirm --execute | jq .contract -r)

echo "Caller Address: $CALLER_ADDRESS"

CALLEE_ADDRESS=$(cargo contract instantiate --constructor new \
    --suri //Alice --salt $(date +%s) \
    --manifest-path callee/Cargo.toml \
    --output-json --skip-confirm --execute | jq .contract -r)

echo "Callee Address: $CALLEE_ADDRESS"

# Continuing execution after a Contract Error

cargo contract call --contract $CALLER_ADDRESS \
    --message get -s //Alice --manifest-path caller/Cargo.toml

cargo contract call --contract $CALLER_ADDRESS \
    --message contract_error --args $CALLEE_ADDRESS \
    -s //Alice --execute --skip-confirm \
    --manifest-path caller/Cargo.toml

cargo contract call --contract $CALLER_ADDRESS \
    --message get -s //Alice --manifest-path caller/Cargo.toml

# Continuing execution after a Lang Error

# cargo contract call --contract $CALLER_ADDRESS \
#     --message get -s //Alice --manifest-path caller/Cargo.toml
#
# cargo contract call --contract $CALLER_ADDRESS \
#     --message lang_error --args $CALLEE_ADDRESS \
#     -s //Alice --execute --skip-confirm \
#     --manifest-path caller/Cargo.toml
# 
# cargo contract call --contract $CALLER_ADDRESS \
#     --message get -s //Alice --manifest-path caller/Cargo.toml

# Continuing execution after a Lang Error

# cargo contract call --contract $CALLER_ADDRESS \
#     --message get -s //Alice --manifest-path caller/Cargo.toml
#
# cargo contract call --contract $CALLER_ADDRESS \
#     --message environment_error --args $CALLEE_ADDRESS \
#     -s //Alice --execute --skip-confirm \
#     --manifest-path caller/Cargo.toml
#
# cargo contract call --contract $CALLER_ADDRESS \
#     --message get -s //Alice --manifest-path caller/Cargo.toml
