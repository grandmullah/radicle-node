#!/bin/sh


cargo build --release
ls  
exec ./target/release/node-template --chain ./customSpecRaw.json --base-path /tmp/node2 --keystore-path /tmp/node2/chains/radicle_testnet/keystore  --port 30334 --ws-port 9945 --unsafe-ws-external --rpc-port 9934 --unsafe-rpc-external --rpc-cors all --validator --name node2 --bootnodes /ip4/0.0.0.0/tcp/30333/p2p/12D3KooWJMNVvPtfPCTEW9B27CvqZYensS2jgioTTqiQbWYDHu3u