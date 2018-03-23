FROM rustlang/rust:nightly AS builder

EXPOSE 8080

ENV SOURCES=/sources
WORKDIR $SOURCES

RUN mkdir -p $SOURCES

COPY Cargo.toml $SOURCES
COPY Cargo.lock $SOURCES
RUN cargo build --lib --release

COPY src $SOURCES/src

RUN cargo build --release

FROM alpine:latest
WORKDIR /root
COPY --from=builder /sources/target/release/smartlights /sl
#CMD ["ls", "-asl", "/sl"]
CMD ["/sl", "--http_port=8080"]
