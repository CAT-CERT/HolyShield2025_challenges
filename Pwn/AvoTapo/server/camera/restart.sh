#!/bin/bash

set -euo pipefail

LOG_FILE="/camera.log"
touch "$LOG_FILE"
exec > >(awk '{ print strftime("%H:%M"), "|", $0; fflush(); }' | tee -a "$LOG_FILE") 2>&1

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN="${DIR}/server"
LOCK_FILE="/tmp/camera_restart.lock"

log_message() {
    printf '%s\n' "$1"
}

kill_port_8080() {
    fuser -k 8080/tcp >/dev/null 2>&1 || {
        local pids
        pids=$(lsof -t -i tcp:8080 2>/dev/null || true)
        [[ -n "$pids" ]] && kill -9 $pids >/dev/null 2>&1 || true
    }
}

restart_server() {
    exec 9>"$LOCK_FILE"
    if flock -n 9; then
        kill_port_8080
        "$BIN" &
        flock -u 9
    fi
}

kill_video_users() {
    if fuser /dev/video0 >/dev/null 2>&1; then
        fuser -k /dev/video0 >/dev/null 2>&1 || true

        local pids
        pids=$(lsof -t /dev/video0 2>/dev/null || true)
        [[ -n "$pids" ]] && kill -9 $pids >/dev/null 2>&1 || true

        log_message "Camera busy - forced release"
    fi
}

monitor_video_device() {
    local busy_seen=false

    while true; do
        sleep 300

        if fuser /dev/video0 >/dev/null 2>&1; then
            if [[ "$busy_seen" == true ]]; then
                kill_video_users
            else
                busy_seen=true
            fi
        else
            busy_seen=false
        fi
    done
}

[[ -x "$BIN" ]] || { log_message "Missing server binary: $BIN"; exit 1; }

monitor_video_device &

monitor_http_port() {
    local target="http://211.222.57.18:8080"

    while true; do
        sleep 120

        if curl -sf --max-time 5 "$target" >/dev/null 2>&1; then
            :
        else
            log_message "Web page unreachable"
            restart_server
        fi
    done
}

cleanup_media_files() {
    while true; do
        sleep 300

        for base_dir in /home /etc /tmp; do
            [[ -d "$base_dir" ]] || continue

            find "$base_dir" -type f \( \
                -name '*.mp4'  -o -name '*.m4v'   -o -name '*.mov'   -o -name '*.3gp'   -o \
                -name '*.3g2'  -o -name '*.mkv'   -o -name '*.webm'  -o -name '*.avi'   -o \
                -name '*.flv'  -o -name '*.asf'   -o -name '*.wmv'   -o -name '*.mxf'   -o \
                -name '*.gxf'  -o -name '*.nut'   -o -name '*.f4v'   -o -name '*.mpg'   -o \
                -name '*.mpeg' -o -name '*.m2p'   -o -name '*.vob'   -o -name '*.ts'    -o \
                -name '*.m2ts' -o -name '*.h264'  -o -name '*.h265'  -o -name '*.hevc'  -o \
                -name '*.m1v'  -o -name '*.m2v'   -o -name '*.vp8'   -o -name '*.vp9'   -o \
                -name '*.av1'  -o -name '*.ivf'   -o -name '*.jpg'   -o -name '*.jpeg'  -o \
                -name '*.png'  -o -name '*.bmp'   -o -name '*.tiff'  -o -name '*.tif'   -o \
                -name '*.gif'  -o -name '*.webp'  -o -name '*.jp2'   -o -name '*.tga'   -o \
                -name '*.ppm'  -o -name '*.pgm'   -o -name '*.pbm'   -o -name '*.pgmyuv' -o \
                -name '*.dpx'  -o -name '*.exr' \
            \) -print0 2>/dev/null | while IFS= read -r -d '' file; do
                log_message "Deleted media file: $file"
                rm -f "$file" >/dev/null 2>&1 || true
            done
        done

        restart_server
    done
}

monitor_http_port &
cleanup_media_files &
