FROM alpine:latest

RUN apk update && \
    apk add --no-cache bash curl inotify-tools && \
    rm -rf /var/cache/apk/*

WORKDIR /app
COPY monitor.sh qbit.sh lib.sh /app/

RUN chmod +x /app/monitor.sh /app/qbit.sh

CMD ["/app/monitor.sh"]
