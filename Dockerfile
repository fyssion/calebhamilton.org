# syntax=docker/dockerfile:1

# Modified from
# https://github.com/rust-lang/docker-rust-nightly/blob/3a50bf3769500fcdad5aadc68f280d45fdf4075d/debian/Dockerfile
# for rust nightly 2024-11-02
FROM buildpack-deps:buster AS rust-nightly
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/dist/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly-2024-11-02; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

FROM rust-nightly AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos@0.2.20

COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo leptos build --release

FROM debian:buster-slim
COPY ./posts /usr/local/share/calebhamilton_org/posts
COPY ./vendor /usr/local/share/calebhamilton_org/vendor
COPY --from=builder ./target/site /usr/local/share/calebhamilton_org/site
COPY --from=builder ./target/release/calebhamilton_org /usr/local/bin/calebhamilton_org
WORKDIR /usr/local/share/calebhamilton_org
ENV LEPTOS_OUTPUT_NAME="calebhamilton_org"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="/usr/local/share/calebhamilton_org/site"
ENV LEPTOS_PKG_PATH="/usr/local/share/calebhamilton_org/site/pkg"
EXPOSE 3000
CMD ["/usr/local/bin/calebhamilton_org"]
