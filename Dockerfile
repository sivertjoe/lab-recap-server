FROM rust:latest AS builder
ARG build_mode

WORKDIR .
COPY . .

RUN cargo install --path .

CMD ["labserver"]
