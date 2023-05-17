FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944

CMD ["/app/target/release/node-template" , "--chain local --ws-port 9944 --unsafe-ws-external --rpc-port 9933 --unsafe-rpc-port 9933 --validator" ]
