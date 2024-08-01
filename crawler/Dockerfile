FROM debian:bookworm-slim

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

RUN apt install -y openssl
RUN apt install -y ca-certificates

COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
