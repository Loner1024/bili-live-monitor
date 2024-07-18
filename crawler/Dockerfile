FROM gcr.io/distroless/cc

COPY --from=builder /target/release/danmu /usr/local/bin/danmu

ENTRYPOINT [ "./danmu" ]
