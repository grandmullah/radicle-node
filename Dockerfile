FROM paritytech/ci-linux:production


WORKDIR /app/

COPY . .

RUN cargo build --release

RUN ["/app/target/release/node-template" , "--dev"]
