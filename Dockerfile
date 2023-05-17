FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

EXPOSE 9933
EXPOSE 9944

CMD ["/app/target/release/node-template", "--chain", "local", "--unsafe-ws-external"]

