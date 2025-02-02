#!/bin/bash

COOKIES="/tmp/cookies.txt"

login () {
  local qbit_url=$1
  local qbit_user=$2
  local qbit_pass=$3

  local login_url="${qbit_url}/api/v2/auth/login"
  local login_data="username=${qbit_user}&password=${qbit_pass}"
  local status
  status=$(curl --silent --cookie-jar $COOKIES --data "${login_data}" "${login_url}" -o /dev/null -w "%{http_code}")
  echo "$status"
}

update_preferences () {
  local qbit_url=$1
  local port=$2

  local preference_url="${qbit_url}/api/v2/app/setPreferences"
  local preference_data="json={\"listen_port\":\"$port\"}"
  local status
  status=$(curl --silent --cookie $COOKIES --data "${preference_data}" "${preference_url}" -o /dev/null -w "%{http_code}")
  echo "$status"
}

update_port () {
  local qbit_url=$1
  local qbit_user=$2
  local qbit_pass=$3
  local port=$4

  local status
  status=$(update_preferences "$qbit_url" "$port")
  if [ "$status" -eq 200 ]; then
    echo "qBittorrent port updated to $port"
  else
    status=$(login "$qbit_url" "$qbit_user" "$qbit_pass" "$port")
    if [ "$status" -eq 200 ]; then
      echo "Logged into qbittorrent"
      status=$(update_preferences "$qbit_url" "$port")
      if [ "$status" -eq 200 ]; then
        echo "Port updated to $port"
      else
        echo "Failed to update qBittorrent port"
      fi
    else
      echo "Failed to login to qBittorrent"
    fi
  fi
  
  echo "qBittorrent port updated to $port"
}
