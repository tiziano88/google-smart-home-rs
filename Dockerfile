FROM ubuntu:xenial

ADD ./target/debug/smartlights /bin/smartlights

EXPOSE 1234

CMD smartlights
