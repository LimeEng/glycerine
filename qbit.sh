#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib.sh"

COOKIES="/tmp/cookies.txt"

login() {
  local qbit_url="$1"
  local qbit_user="$2"
  local qbit_pass="$3"
  local login_url="${qbit_url}/api/v2/auth/login"
  local login_data="username=${qbit_user}&password=${qbit_pass}"

  request "$login_url" "$login_data" --cookie-jar "$COOKIES"
}

update_preferences() {
  local qbit_url="$1"
  local port="$2"
  local preference_url="${qbit_url}/api/v2/app/setPreferences"
  local preference_data="json={\"listen_port\":\"$port\"}"

  request "$preference_url" "$preference_data" --cookie "$COOKIES"
}

update_port() {
  local qbit_url="$1"
  local qbit_user="$2"
  local qbit_pass="$3"
  local port="$4"
  local status

  if ! status=$(update_preferences "$qbit_url" "$port"); then
    return 1
  fi

  if [[ "$status" == "200" ]]; then
    echo "qBittorrent port updated to $port"
    return 0
  fi

  if ! status=$(login "$qbit_url" "$qbit_user" "$qbit_pass"); then
    echo "Failed to login to qBittorrent" >&2
    return 1
  fi

  if [[ "$status" != "200" ]]; then
    echo "Failed to update qBittorrent port" >&2
    return 1
  fi

  echo "Logged into qBittorrent"

  if ! status=$(update_preferences "$qbit_url" "$port"); then
    return 1
  fi

  if [[ "$status" == "200" ]]; then
    echo "Port updated to $port"
    return 0
  fi

  echo "Failed to update qBittorrent port" >&2
  return 1
}
