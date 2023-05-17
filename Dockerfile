FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944

CMD ["/app/target/release/node-template", "--base-path /tmp/alice", "--chain local","--port 30333", "--ws-port 9945", "--validator", "--unsafe-ws-external" ]
