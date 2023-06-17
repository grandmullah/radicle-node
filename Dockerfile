FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944
EXPOSE 30333

CMD ["./target/release/node-template",  "--chain", "./customSpecRaw.json", "--base-path", "/tmp/node1", "--port", "30333", "--ws-port", "9944" ,"--unsafe-ws-external", "--rpc-port","9933","--unsafe-rpc-external","--rpc-cors","all", "--validator","--name","node1","--password-interactive" ]

