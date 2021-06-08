FROM rust:latest

COPY . /work

WORKDIR /work

RUN rustup install nightly
RUN rustup default nightly
RUN cargo build
RUN cargo install diesel_cli --no-default-features --features mysql