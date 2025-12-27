#!/usr/bin/env bash
set -euo pipefail

TARGETS_CSV=${TARGETS_CSV:-"ue:4999"} #TODO change host -> ip
GNB_LAT=${GNB_LAT:-30}
GNB_LON=${GNB_LON:-120}
INTERVAL=${INTERVAL:-10}
JITTER=${JITTER:-2}

IFS=',' read -r -a TARGETS <<< "$TARGETS_CSV"

send_once() {
  local payload ok host port t
  payload=$(printf '{"lat":%s,"lon":%s}' "$GNB_LAT" "$GNB_LON")
  ok=0
  for t in "${TARGETS[@]}"; do
    host="${t%:*}"; port="${t##*:}"
    if printf '%s' "$payload" >"/dev/udp/$host/$port" 2>/dev/null; then
      ok=1
    else
      echo "[GNB-ANNOUNCER-SH] send error to $host:$port" >&2
    fi
  done
  if [ "$ok" -eq 0 ]; then
    echo "[GNB-ANNOUNCER-SH] no targets accepted packet" >&2
  fi
}

trap 'exit 0' TERM INT
echo "[GNB-ANNOUNCER-SH] targets=$TARGETS_CSV lat=$GNB_LAT lon=$GNB_LON every ${INTERVAL}s"
while :; do
  send_once
  sleep_for=$(( INTERVAL + (RANDOM % (2*JITTER + 1)) - JITTER ))
  [ "$sleep_for" -lt 1 ] && sleep_for=1
  sleep "$sleep_for"
done

