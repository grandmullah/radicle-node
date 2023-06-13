FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944
EXPOSE 30333

ENTRYPOINT ["./target/release/node-template",  "--chain", "./customSpecRaw.json", "--base-path", "/tmp/node2", "--port", "30333", "--ws-port", "9944" ,"--unsafe-ws-external", "--rpc-port","9933","--unsafe-rpc-external","--rpc-cors","all", "--validator","--name","node2", "--bootnodes", "/ip4/35.232.24.147/tcp/30333/p2p/12D3KooWHbXkwNqURP7HSsb7p3jiLxCYyEqjA7huT4Y1x5Q3ugSQ"]

