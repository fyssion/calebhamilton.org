# syntax=docker/dockerfile:1

FROM rustlang/rust:nightly AS prepare
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos

FROM prepare AS builder
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo leptos build --release

FROM debian:buster-slim
COPY ./posts /usr/local/share/fyssion_zone/posts
COPY --from=builder ./target/site /usr/local/share/fyssion_zone/site
COPY --from=builder ./target/server/release/fyssion_zone /usr/local/bin/fyssion_zone
WORKDIR /usr/local/share/fyssion_zone
ENV LEPTOS_OUTPUT_NAME="fyssion_zone"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="/usr/local/share/fyssion_zone/site"
ENV LEPTOS_PKG_PATH="/usr/local/share/fyssion_zone/site/pkg"
EXPOSE 3000
CMD ["/usr/local/bin/fyssion_zone"]
