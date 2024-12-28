FROM ubuntu:24.04

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

COPY target/release/statistics /usr/local/bin/statistics

ENTRYPOINT [ "statistics" ]
