# --- Build Stage ---
FROM rustlang/rust:nightly AS builder

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Minimal Scratch Stage ---
FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crondes /crondes

ENTRYPOINT ["/crondes"]
