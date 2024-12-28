FROM ubuntu:24.04

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y tzdata && ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

COPY target/release/service /usr/local/bin/service

EXPOSE 8080

ENTRYPOINT [ "service" ]
