#!/usr/bin/env bash
# Stolen from https://www.advantch.com/blog/how-to-run-multiple-processes-in-a-single-docker-container/
set -Eeo pipefail
printf "READY\n";
while read -r; do
  echo -e "\e[31m Service was stopped or one of it's services crashed,
            see the logs above for more details. \e[0m" >&2
  kill -SIGTERM "$(cat supervisord.pid)"
done < /dev/stdin