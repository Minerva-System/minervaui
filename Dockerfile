FROM rust:1.77.0-alpine3.19 AS builder
ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN apk add --no-cache ncurses ncurses-dev openssl openssl-dev musl-dev libcrypto3 pkgconfig
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ncurses-libs openssl libcrypto3 libgcc
COPY --from=builder /app/target/release/minervaui /minervaui
RUN addgroup --gid 1000 appuser &&\
    adduser --disabled-password --gecos "" --home "/" --ingroup appuser --no-create-home --uid 1000 appuser
USER appuser
ENTRYPOINT ["/minervaui"]
