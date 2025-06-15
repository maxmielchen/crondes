ARG BINARY_NAME_DEFAULT=crondes

FROM clux/muslrust:stable AS builder
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release && mkdir -p /out && cp target/x86_64-unknown-linux-musl/release/crondes /out/

FROM scratch
COPY --from=builder /out/crondes /
CMD ["/crondes"]
