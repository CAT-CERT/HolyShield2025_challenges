#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
BIN="${DIR}/build/nr-gnb"
CFG="${1:-${DIR}/config/open5gs-gnb.yaml}"

if [[ ! -x "${BIN}" ]]; then
  echo "nr-gnb not built. Building now..." >&2
  cmake -S "${DIR}" -B "${DIR}/build"
  cmake --build "${DIR}/build" -j
fi

exec "${BIN}" -c "${CFG}"

