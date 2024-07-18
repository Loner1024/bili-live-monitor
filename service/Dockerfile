FROM gcr.io/distroless/cc

COPY target/release/service /usr/local/bin/service

EXPOSE 8080

ENTRYPOINT [ "service" ]
