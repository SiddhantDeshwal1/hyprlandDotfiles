#!/bin/bash

# Configurable time (in seconds)
work_time=$((25 * 60))

# State file
state_file="/tmp/pomodoro_state"

# Initialize state file if it doesn't exist
if [ ! -f "$state_file" ]; then
    echo "paused" > "$state_file"          # status
    echo "$work_time" >> "$state_file"     # remaining time in seconds
fi

status=$(sed -n '1p' "$state_file")
remaining=$(sed -n '2p' "$state_file")

case $1 in
    toggle)
        if [ "$status" = "running" ]; then
            sed -i '1s/.*/paused/' "$state_file"
        else
            sed -i '1s/.*/running/' "$state_file"
        fi
        ;;
    reset)
        sed -i '1s/.*/paused/' "$state_file"
        sed -i "2s/.*/$work_time/" "$state_file"
        ;;
esac

# If running, decrement remaining time
if [ "$status" = "running" ]; then
    remaining=$((remaining - 1))
    if [ "$remaining" -le 0 ]; then
        remaining=0
        sed -i '1s/.*/paused/' "$state_file"
        notify-send "Pomodoro Timer" "Time's up!"
    fi
    sed -i "2s/.*/$remaining/" "$state_file"
fi

# Format MM:SS
mins=$((remaining / 60))
secs=$((remaining % 60))
time_str=$(printf "%02d:%02d" "$mins" "$secs")

# Output for Waybar
echo "$time_str"
