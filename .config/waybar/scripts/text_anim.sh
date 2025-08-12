#!/bin/bash

texts=("rch" "nalyze" "im" "ttack")
index=0

while true; do
    text="${texts[$index]}"
    echo "{\"text\": \"$text\"}"
    sleep 1
    index=$(( (index + 1) % ${#texts[@]} ))
done
