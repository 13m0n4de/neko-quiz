FROM rust:1.75.0 as builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y binaryen musl-tools && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add wasm32-unknown-unknown && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install trunk

RUN cd frontend && CARGO_TARGET_DIR=../target-trunk trunk build --release --public-url / && \
    cd .. && \
    cd backend && cargo build --bin backend --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/dist/ /dist
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /neko-quiz-server

ENTRYPOINT ["/neko-quiz-server"]

CMD ["-a", "0.0.0.0"]
