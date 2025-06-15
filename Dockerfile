ARG BINARY_NAME_DEFAULT=crondes

FROM clux/muslrust:stable AS builder
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release && mkdir -p /out && cp target/x86_64-unknown-linux-musl/release/crondes /out/

FROM scratch
# Default values for config
ENV CF_API_TOKEN=
ENV CF_ZONE_ID=
ENV CF_RECORD_ID=
ENV UPDATE_INTERVAL_SECS=300
COPY --from=builder /out/crondes /
CMD ["/crondes"]
