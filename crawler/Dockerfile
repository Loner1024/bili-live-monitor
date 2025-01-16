FROM ubuntu:24.04

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y tzdata && ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

RUN apt-get install -y wget
RUN wget https://extensions.duckdb.org/v1.0.0/linux_amd64_gcc4/httpfs.duckdb_extension.gz
RUN gunzip ./httpfs.duckdb_extension.gz
RUN mkdir -p /root/.duckdb/extensions/v1.0.0/linux_amd64/
RUN mv ./httpfs.duckdb_extension /root/.duckdb/extensions/v1.0.0/linux_amd64/


COPY target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "danmu" ]
