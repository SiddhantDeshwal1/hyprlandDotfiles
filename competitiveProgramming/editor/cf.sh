#!/bin/bash

WORKSPACE="workspace.cpp"
INPUT="input.txt"
EXPECTED="expected.txt"
OUTPUT="output.txt"
LINKFILE="saved_links.txt"

green='\033[0;32m'
red='\033[0;31m'
nc='\033[0m'

function add() {
    echo "$1" >> $LINKFILE
}

function load() {
    URL=$1
    echo "$URL" > $LINKFILE

    python3 <<EOF
import requests
from bs4 import BeautifulSoup
import html

url = "$URL"
res = requests.get(url)
soup = BeautifulSoup(res.text, "html.parser")

inputs = soup.find_all("div", class_="input")
outputs = soup.find_all("div", class_="output")

if not inputs or not outputs:
    print("❌ Failed to extract samples")
    exit(1)

with open("$INPUT", "w") as f_in, open("$EXPECTED", "w") as f_out:
    for i, o in zip(inputs, outputs):
        in_text = i.find("pre").get_text("\n", strip=False)
        out_text = o.find("pre").get_text("\n", strip=False)
        f_in.write(html.unescape(in_text.strip()) + "\n")
        f_out.write(html.unescape(out_text.strip()) + "\n")

print("✅ Extracted samples successfully from:", url)
EOF
}

function check() {
    g++ -std=c++20 -O2 -o workspace $WORKSPACE
    if [ $? -ne 0 ]; then
        echo -e "${red}Compilation Error${nc}"
        exit 1
    fi

    IFS=''
    pass=true
    paste -d'\n' $INPUT $EXPECTED | while read -r input && read -r expected; do
        echo "$input" | ./workspace > $OUTPUT
        output=$(cat $OUTPUT | sed 's/ *$//g')
        expected=$(echo "$expected" | sed 's/ *$//g')
        if [[ "$output" == "$expected" ]]; then
            echo -e "${green}✔ Passed${nc}"
        else
            echo -e "${red}✘ Failed${nc}"
            echo "Input: $input"
            echo "Output: $output"
            echo "Expected: $expected"
            pass=false
        fi
    done

    if $pass; then
        echo -e "${green}All tests passed. Submitting...${nc}"
        link=$(head -1 $LINKFILE)
        xdg-open "$link/submit"
        xclip -selection clipboard < $WORKSPACE
    fi
}

function submit() {
    link=$(head -1 $LINKFILE)
    xclip -selection clipboard < $WORKSPACE
    xdg-open "$link/submit"
}

function usage() {
    echo "Usage:"
    echo "./cf.sh add <url>"
    echo "./cf.sh load <url>"
    echo "./cf.sh check"
    echo "./cf.sh submit"
}

case "$1" in
    add) add "$2" ;;
    load) load "$2" ;;
    check) check ;;
    submit) submit ;;
    *) usage ;;
esac
