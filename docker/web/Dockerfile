FROM rust:latest

COPY . /work

WORKDIR /work

RUN apt-get update
RUN apt-get install -y default-mysql-client
RUN rustup install nightly
RUN rustup default nightly
RUN cargo build