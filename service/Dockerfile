FROM debian

ENV TZ=Asia/Shanghai \
    DEBIAN_FRONTEND=noninteractive

COPY target/release/service /usr/local/bin/service

EXPOSE 8080

ENTRYPOINT [ "service" ]
