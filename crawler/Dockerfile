FROM debian:bookworm-slim

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
