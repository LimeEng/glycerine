#!/bin/bash

source ./qbit.sh

if [[ -z "$QBIT_URL" || -z "$QBIT_USER" || -z "$QBIT_PASS" || -z "$FORWARDED_PORT_PATH" ]]; then
  echo "ERROR: Missing one or more required environment variables (QBIT_URL, QBIT_USER, QBIT_PASS, FORWARDED_PORT_PATH)."
  exit 1
fi

push_port () {
  local port=$(cat $FORWARDED_PORT_PATH)
  echo "Detected port change: $port"
  update_port port
}

while true; do
  if [ -f $FORWARDED_PORT_PATH ]; then
    push_port
    inotifywait -mq -e close_write $FORWARDED_PORT_PATH | while read _change; do
      push_port
    done
  else
    echo "$FORWARDED_PORT_PATH not found, sleeping for 10 seconds"
    sleep 10
  fi
done
