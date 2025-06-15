# --- Build Stage ---
FROM rust:1.77 as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Minimal Scratch Stage ---
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crondes /crondes

ENTRYPOINT ["/crondes"]
