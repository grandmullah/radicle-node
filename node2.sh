#!/bin/sh


cargo build --release
ls  
exec ./target/release/node-template --chain ./customSpecRaw.json --base-path /tmp/node2 --keystore-path /tmp/node2/chains/radicle_testnet/keystore  --port 30334 --ws-port 9945 --unsafe-ws-external --rpc-port 9934 --unsafe-rpc-external --rpc-cors all --validator --name node2 --bootnodes /ip4/172.18.0.2/tcp/30333/p2p/12D3KooWT2inG4BAD4fHNEGGFGsy2hvdFRZo3nG2cSV6jzWeWKvy