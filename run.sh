#!/bin/sh


cargo build --release
ls  
exec ./target/release/node-template --chain ./customSpecRaw.json --base-path /tmp/node1 --keystore-path /tmp/node1/chains/radicle_testnet/keystore  --port 30333 --ws-port 9944 --unsafe-ws-external --rpc-port 9933 --unsafe-rpc-external --rpc-cors all --validator--name node1