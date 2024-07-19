FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl
RUN apt install -y ca-certificates

COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
