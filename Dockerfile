FROM alpine:latest

RUN apk update
RUN apk add --no-cache curl inotify-tools bash

COPY ./monitor.sh ./monitor.sh
COPY ./qbit.sh ./qbit.sh
RUN chmod +x ./monitor.sh
RUN chmod +x ./qbit.sh

CMD ["./monitor.sh"]
