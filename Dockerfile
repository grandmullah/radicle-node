FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

CMD ["/app/target/release/node-template" , "--dev"]
