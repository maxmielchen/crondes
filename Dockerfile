ARG BINARY_NAME_DEFAULT=crondes

FROM clux/muslrust:stable AS builder
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release && mkdir -p /out && cp target/x86_64-unknown-linux-musl/release/crondes /out/

FROM scratch
# Default values for config
ENV RUST_LOG=info
ENV CF_API_TOKEN=
ENV CF_ZONE_ID=
ENV CF_RECORD_ID=
ENV CF_RECORD_NAME=@
ENV UPDATE_INTERVAL_SECS=60
COPY --from=builder /out/crondes /
CMD ["/crondes"]
