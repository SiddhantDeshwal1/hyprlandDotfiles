import os
import re
import subprocess
import sys
from datetime import datetime, timedelta, timezone

import httpx
import pyperclip
from bs4 import BeautifulSoup

LINKS_FILE = "saved_links.txt"


def add_link(url):
    with open(LINKS_FILE, "a", encoding="utf-8") as f:
        f.write(url + "\n")


def load_problem(url):
    if not url:
        print("No URL provided.")
        return

    url_segment = "/".join(url.strip().split("/")[-2:])
    with open(LINKS_FILE, "w", encoding="utf-8") as f:
        f.write(url_segment + "\n")

    r = httpx.get(url, headers={"User-Agent": "Mozilla/5.0"})
    soup = BeautifulSoup(r.text, "html.parser")

    inputs, outputs = {}, []

    for div in soup.find_all("div", class_=re.compile(r"test-example-line")):
        for c in div.get("class", []):
            if m := re.match(r"test-example-line-(\d+)", c):
                idx = int(m.group(1))
                inputs.setdefault(idx, []).append(div.get_text(strip=True))
                break

    for out_div in soup.find_all("div", class_="title", string="Output"):
        pre = out_div.find_next("pre")
        if pre:
            outputs.append(pre.get_text(strip=True))

    with open("input.txt", "w", encoding="utf-8") as f:
        for idx in sorted(inputs):
            f.write("\n".join(inputs[idx]) + "\n")

    with open("expected.txt", "w", encoding="utf-8") as f:
        f.write("\n".join(outputs))

    print("✅ Test cases extracted.")


def compile_cpp():
    return (
        subprocess.run(
            ["g++", "-std=c++20", "-O2", "-o", "workspace", "workspace.cpp"]
        ).returncode
        == 0
    )


def run():
    if not compile_cpp():
        print("❌ Compilation failed.")
        return False

    if not os.path.exists("input.txt"):
        print("❌ input.txt not found.")
        return False

    with open("input.txt") as fin:
        proc = subprocess.run(
            ["./workspace"], stdin=fin, capture_output=True, text=True
        )
    output = [line.strip() for line in proc.stdout.splitlines()]

    if not os.path.exists("expected.txt"):
        print("❌ expected.txt not found.")
        return False

    with open("expected.txt") as f:
        expected = [line.strip() for line in f if line.strip()]

    print("\n📤 Output:")
    for line in output:
        print(line)

    print("\n✅ Checking output...\n")
    all_passed = True
    for i in range(max(len(output), len(expected))):
        out = output[i] if i < len(output) else ""
        exp = expected[i] if i < len(expected) else ""
        if out == exp:
            print(f"\033[32m+ Case {i+1}: {out}\033[0m")
        else:
            print(f"\033[31m- Case {i+1}: {out}\033[0m")
            print(f"\033[33m  Expected: {exp}\033[0m")
            all_passed = False

    print(
        "\n\033[32m✅ Passed\033[0m"
        if all_passed
        else "\n\033[31m❌ Wrong Answer\033[0m"
    )
    return all_passed


def check_problem():
    if run():
        submit_problem()


def submit_problem():
    if not os.path.exists(LINKS_FILE):
        print("❌ No saved URLs.")
        return

    with open(LINKS_FILE) as f:
        url_fragment = f.readline().strip()

    if not url_fragment:
        print("❌ Empty saved URL.")
        return

    subprocess.run(
        ["librewolf", f"https://codeforces.com/problemset/submit/{url_fragment}"]
    )

    if os.path.exists("workspace.cpp"):
        with open("workspace.cpp", encoding="utf-8") as f:
            pyperclip.copy(f.read())
    else:
        print("❌ workspace.cpp not found.")


def friends():
    with open(LINKS_FILE) as f:
        contest_id = f.readline().strip().split("/")[0]
    subprocess.run(
        [
            "librewolf",
            f"https://codeforces.com/contest/{contest_id}/standings/friends/true",
        ]
    )


def check_last_submission():
    handle = "worthNothing"
    url = f"https://codeforces.com/api/user.status?handle={handle}&from=1&count=1"
    res = httpx.get(url).json()

    if res["status"] != "OK" or not res["result"]:
        print("❌ API error.")
        return

    sub = res["result"][0]
    p = sub["problem"]
    print(f"📘 Problem: {p['contestId']}{p['index']} - {p['name']}")
    print(f"🧪 Verdict: {sub.get('verdict', 'N/A')}")
    print(f"✅ Passed: {sub.get('passedTestCount', 'N/A')}")
    print(f"⚡ Time: {sub.get('timeConsumedMillis', 0)} ms")
    print(f"📦 Memory: {sub.get('memoryConsumedBytes', 0)} bytes")


def show_upcoming_regular_contests():
    url = "https://codeforces.com/api/contest.list?gym=false"
    try:
        res = httpx.get(url).json()
        if res["status"] != "OK":
            print("❌ Failed:", res.get("comment", "No message"))
            return

        upcoming = [c for c in res["result"] if c["phase"] == "BEFORE"]
        if not upcoming:
            print("📭 No upcoming contests.")
            return

        print("\n📅 Upcoming Contests (IST):\n")
        for c in sorted(upcoming, key=lambda x: x["startTimeSeconds"]):
            name = c["name"]
            ist = datetime.fromtimestamp(
                c["startTimeSeconds"], timezone.utc
            ) + timedelta(hours=5, minutes=30)
            print(
                f"📌 {name} | 🕒 {ist.strftime('%Y-%m-%d %H:%M')} IST | ⏱️ {c['durationSeconds'] // 3600}h"
            )

    except Exception as e:
        print("❌ Error:", e)


def main():
    if len(sys.argv) < 2:
        print("Usage: python cf.py <command> [link]")
        return

    cmd = sys.argv[1]
    arg = sys.argv[2] if len(sys.argv) > 2 else None

    match cmd:
        case "add" if arg:
            add_link(arg)
        case "load" if arg:
            load_problem(arg)
        case "check":
            check_problem()
        case "submit":
            submit_problem()
        case "contest":
            show_upcoming_regular_contests()
        case "last":
            check_last_submission()
        case "friends":
            friends()
        case _:
            print(f"❓ Unknown or incomplete command '{cmd}'")


if __name__ == "__main__":
    main()
