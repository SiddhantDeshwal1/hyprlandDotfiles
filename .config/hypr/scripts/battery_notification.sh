#!/bin/bash

# Path to the battery icon
battery_icon="/home/siddhantdeshwal/.config/swaync/icons/battery-status.png"

while true; do
    # Get the battery percentage
    battery_level=$(cat /sys/class/power_supply/BAT1/capacity | tr -d '[:space:]')

    # Get the charging status
    charging_status=$(cat /sys/class/power_supply/BAT1/status | tr -d '[:space:]')

    echo "Battery Level: $battery_level"
    echo "Charging Status: $charging_status"

    # Check battery level and charging status
    if [[ "$battery_level" -lt 25 && "$charging_status" != "Charging" ]]; then
        # Show a notification with the battery icon
        notify-send -u normal -t 10000 -i "$battery_icon" "Low Battery" "Battery level is $battery_level%. Plug in the charger!"
    fi

    # Wait for 1 minute before checking again
    sleep 60
done
