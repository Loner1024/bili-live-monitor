FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
