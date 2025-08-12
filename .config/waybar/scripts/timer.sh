#!/bin/bash

STATE_FILE="/tmp/timer_state"
TIME_FILE="/tmp/timer_time"

[[ ! -f "$STATE_FILE" ]] && echo "stopped" > "$STATE_FILE"
[[ ! -f "$TIME_FILE" ]] && echo 0 > "$TIME_FILE"

start() {
    state=$(cat "$STATE_FILE")
    if [[ "$state" == "paused" ]]; then
        # Resuming from paused
        paused_time=$(cat "$TIME_FILE")
        new_start=$(( $(date +%s) - paused_time ))
        echo "$new_start" > "$TIME_FILE"
    else
        # New start
        echo $(date +%s) > "$TIME_FILE"
    fi
    echo "running" > "$STATE_FILE"
}

pause() {
    state=$(cat "$STATE_FILE")
    if [[ "$state" == "running" ]]; then
        start_time=$(cat "$TIME_FILE")
        elapsed=$(( $(date +%s) - start_time ))
        echo "$elapsed" > "$TIME_FILE"
        echo "paused" > "$STATE_FILE"
    fi
}

reset() {
    echo "stopped" > "$STATE_FILE"
    echo 0 > "$TIME_FILE"
}

status() {
    state=$(cat "$STATE_FILE")
    if [[ "$state" == "running" ]]; then
        start_time=$(cat "$TIME_FILE")
        elapsed=$(( $(date +%s) - start_time ))
        printf "⏱️ %02d:%02d\n" $((elapsed/60)) $((elapsed%60))
    elif [[ "$state" == "paused" ]]; then
        elapsed=$(cat "$TIME_FILE")
        printf "⏸️ %02d:%02d\n" $((elapsed/60)) $((elapsed%60))
    else
        echo "⏱️ 00:00"
    fi
}

case "$1" in
    start) start ;;
    pause) pause ;;
    reset) reset ;;
    status) status ;;
esac
