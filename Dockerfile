FROM rust:1.62-alpine3.15

COPY Cargo.toml Cargo.lock src tests .
RUN cargo build

RUN cargo test



