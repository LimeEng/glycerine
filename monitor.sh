#!/bin/bash

source ./qbit.sh

trap "echo 'Exiting...'; exit 0" SIGINT SIGTERM SIGHUP SIGQUIT

if [[ -z "$QBIT_URL" || -z "$QBIT_USER" || -z "$QBIT_PASS" || -z "$FORWARDED_PORT_PATH" ]]; then
  echo "ERROR: Missing one or more required environment variables (QBIT_URL, QBIT_USER, QBIT_PASS, FORWARDED_PORT_PATH)."
  exit 1
fi

push_port () {
  local port
  port=$(cat "$FORWARDED_PORT_PATH")
  echo "Detected port change: $port"
  update_port "$QBIT_URL" "$QBIT_USER" "$QBIT_PASS" "$port"
}

while true; do
  if [ -f "$FORWARDED_PORT_PATH" ]; then
    push_port
    inotifywait -mq -e close_write,delete_self "$FORWARDED_PORT_PATH" | while read -r _file event; do
      case "$event" in
        CLOSE_WRITE,CLOSE)
          push_port  # Only read the file if it was modified
          ;;
        DELETE_SELF)
          break  # Exit monitoring if the file is deleted
          ;;
      esac
    done
  else
    echo "$FORWARDED_PORT_PATH not found, sleeping for 10 seconds"
    sleep 10
  fi
done
