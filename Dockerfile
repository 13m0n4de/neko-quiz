FROM rust:1.80.0-alpine AS builder

WORKDIR /app
COPY . .

RUN set -xe && \
    apk add --no-cache binaryen musl-dev curl && \
    curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && cp tailwindcss-linux-x64 /usr/local/bin/tailwindcss && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-binstall && \
    cargo binstall -y trunk

RUN cd frontend && \
    CARGO_TARGET_DIR=../target-trunk trunk build --release && \
    cd ..
RUN cargo build --bin server --release

FROM scratch

COPY --from=builder /app/dist/ /dist
COPY --from=builder /app/target/release/server /neko-quiz
COPY --from=builder /app/config.toml /config.toml

ENTRYPOINT ["/neko-quiz"]

CMD ["-a", "0.0.0.0:3000"]
