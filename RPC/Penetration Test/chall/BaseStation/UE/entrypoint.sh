#!/usr/bin/env bash
set -euo pipefail

CFG="${CFG:-/app/config/open5gs-ue.yaml}"
BIN="/app/build/nr-ue"
INTERVAL="${UE_RESTART_INTERVAL:-30}"

# TUN cleanup options
TUN_IFACE="${UE_TUN_IFACE:-uesimtun0}"
TUN_CLEAN="${UE_TUN_CLEAN:-1}"

python3 /app/request_handler.py &
HANDLER_PID=$!

cleanup_tun() {
  [ "${TUN_CLEAN}" = "1" ] || return 0
  if ip link show "${TUN_IFACE}" >/dev/null 2>&1; then
    ip addr flush dev "${TUN_IFACE}" 2>/dev/null || true
    ip link set "${TUN_IFACE}" down 2>/dev/null || true
    ip link del "${TUN_IFACE}" 2>/dev/null || true
    echo "[UE-ENTRY] cleaned ${TUN_IFACE}"
  fi
}

term() {
  kill -TERM "$HANDLER_PID" 2>/dev/null || true
  kill -TERM "$UE_PID" 2>/dev/null || true
  # give some time to exit
  for _ in {1..10}; do
    kill -0 "$UE_PID" 2>/dev/null || break
    sleep 0.2
  done
  kill -KILL "$UE_PID" 2>/dev/null || true
  cleanup_tun
  exit 0
}
trap term TERM INT

echo "[UE-ENTRY] restarting nr-ue every ${INTERVAL}s (cfg=${CFG})"

while :; do
  # ensure stale TUN from previous run is removed
  cleanup_tun
  set +e
  "$BIN" -c "$CFG" &
  UE_PID=$!
  set -e

  elapsed=0
  while [ "$elapsed" -lt "$INTERVAL" ]; do
    if ! kill -0 "$UE_PID" 2>/dev/null; then
      # nr-ue exited early; restart immediately
      break
    fi
    sleep 1
    elapsed=$((elapsed + 1))
  done

  # if still running after interval, stop it gracefully
  if kill -0 "$UE_PID" 2>/dev/null; then
    kill -TERM "$UE_PID" 2>/dev/null || true
    for _ in {1..10}; do
      kill -0 "$UE_PID" 2>/dev/null || break
      sleep 0.2
    done
    kill -KILL "$UE_PID" 2>/dev/null || true
  fi
  # cleanup TUN after each cycle
  cleanup_tun
done

