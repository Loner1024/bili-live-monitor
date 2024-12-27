FROM ubuntu:24.04

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
