FROM rustlang/rust:nightly AS builder

EXPOSE 8080

ENV SOURCES=/sources
WORKDIR $SOURCES

RUN mkdir -p $SOURCES

COPY Cargo.toml $SOURCES
COPY Cargo.lock $SOURCES
RUN cargo build --lib

COPY src $SOURCES/src

RUN cargo build

FROM alpine:latest

COPY --from=builder /sources/target/debug/smartlights .

CMD ./smartlights --http_port=8080
