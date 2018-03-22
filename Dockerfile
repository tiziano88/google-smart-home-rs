FROM rustlang/rust:nightly

EXPOSE 8080

ENV SOURCES=/sources

RUN mkdir -p $SOURCES

COPY ./ $SOURCES

WORKDIR $SOURCES

CMD cargo run -- --http_port=8080
