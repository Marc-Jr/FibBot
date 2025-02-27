FROM rust:latest
WORKDIR /usr/src/fibbit
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/fibbot"]
