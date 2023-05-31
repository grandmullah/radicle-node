FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944

CMD ["./target/release/node-template", "--chain", "dev", "--port", "30333", "--ws-port", "9944" ,"--unsafe-ws-external", "--rpc-port","9933","--unsafe-rpc-external","--rpc-cors","all", "--validator"]

