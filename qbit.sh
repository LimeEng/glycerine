#!/bin/bash

COOKIES="/tmp/cookies.txt"

login () {
  local login_url="${QBIT_URL}/api/v2/auth/login"
  local login_data="username=$QBIT_USER&password=$QBIT_PASS"
  local status
  status=$(curl --silent --cookie-jar $COOKIES --data ${login_data} ${login_url} -o /dev/null -w "%{http_code}")
  echo $status
}

update_preferences () {
  local preference_url="${QBIT_URL}/api/v2/app/setPreferences"
  local preference_data="json={\"listen_port\":\"$1\"}"
  local status
  status=$(curl --silent --cookie $COOKIES --data "${preference_data}" ${preference_url} -o /dev/null -w "%{http_code}")
  echo $status
}

update_port () {
  local port=$1

  local status
  status=$(update_preferences $port)
  if [ "$status" -eq 200 ]; then
    echo "qBittorrent port updated to $port"
  else
    status=$(login)
    if [ "$status" -eq 200 ]; then
      echo "Logged into qbittorrent"
      status=$(update_preferences $port)
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
