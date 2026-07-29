#!/usr/bin/env bash

set -euo pipefail

request() {
  local url="$1"
  local data="$2"
  local cookie_flag="$3"
  local status

  if ! status=$(curl --silent --show-error --location "$cookie_flag" --data "$data" "$url" --output /dev/null --write-out "%{http_code}" 2>/dev/null); then
    echo "Request failed: ${url}" >&2
    return 1
  fi

  echo "$status"
}

get_env() {
  local name="$1"
  local value="${!name:-}"

  if [[ -z "$value" ]]; then
    echo "${name} is required" >&2
    exit 1
  fi

  echo "$value"
}

read_port() {
  local path="$1"
  local port

  [[ -f "$path" ]] || { echo "$path not found" >&2; return 1; }
  port="$(<"$path")" || { echo "Failed to read $path" >&2; return 1; }
  port="${port//$'\r'/}"
  [[ -n "$port" ]] || { echo "Port file is empty: $path" >&2; return 1; }

  echo "$port"
}
