FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

CMD ["/app/target/release/node-template" , "--dev" ,"--ws-port 9944", "--ws-external", "--rpc-port 9933", "--rpc-external"]
