#!/usr/bin/env bash
set -euo pipefail

/wait

npm run start &
WEBUI_PID=$!

TARGET_URL=${BASE_URL:-http://open5gs-webui:9999}

until curl -fsS "${TARGET_URL}/api/auth/csrf" >/dev/null; do
  sleep 2
done

BASE_URL="${TARGET_URL}" /usr/local/bin/seed-subscriber.sh || true

wait "${WEBUI_PID}"
