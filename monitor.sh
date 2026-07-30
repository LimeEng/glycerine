#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib.sh"
source "${SCRIPT_DIR}/qbit.sh"

trap 'echo "Exiting..."; exit 0' SIGINT SIGTERM SIGHUP SIGQUIT

QBIT_URL="$(get_env QBIT_URL)"
QBIT_USER="$(get_env QBIT_USER)"
QBIT_PASS="$(get_env QBIT_PASS)"
FORWARDED_PORT_PATH="$(get_env FORWARDED_PORT_PATH)"

push_port() {
  local port

  port="$(read_port "${FORWARDED_PORT_PATH}")" || return 1
  echo "Detected port change: ${port}"
  update_port "${QBIT_URL}" "${QBIT_USER}" "${QBIT_PASS}" "${port}"
}

while true; do
  if ! push_port; then
    echo "Initial port update failed; sleeping for 10 seconds" >&2
    sleep 10
    continue
  fi

  if ! inotifywait -mq -e close_write,delete_self "${FORWARDED_PORT_PATH}" | while IFS= read -r _file event; do
    case "${event}" in
      CLOSE_WRITE,CLOSE) push_port || echo "Port update failed after file change" >&2 ;;
      DELETE_SELF) break ;;
    esac
  done; then
    echo "Monitor loop interrupted; sleeping for 10 seconds" >&2
    sleep 10
  fi
done
