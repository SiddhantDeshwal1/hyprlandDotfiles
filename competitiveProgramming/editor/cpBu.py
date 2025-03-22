import os
import re
import subprocess
import sys

import pyperclip
import requests
from bs4 import BeautifulSoup

LINKS_FILE = "saved_links.txt"


def add_link(url):
    with open(LINKS_FILE, "a", encoding="utf-8") as file:
        file.write(url + "\n")


def load_problem(url):
    if not url:
        print("No URL provided.")
        return

    url_segment = "/".join(url.strip().split("/")[-2:])
    with open(LINKS_FILE, "w", encoding="utf-8") as file:
        file.write(url_segment + "\n")

    response = requests.get(url, headers={"User-Agent": "Mozilla/5.0"})
    soup = BeautifulSoup(response.text, "lxml")

    input_lines_by_case = {}
    for div in soup.find_all("div", class_=re.compile(r"test-example-line")):
        for c in div.get("class", []):
            match = re.match(r"test-example-line-(\d+)", c)
            if match:
                line_index = int(match.group(1))
                input_lines_by_case.setdefault(line_index, []).append(div.get_text())
                break

    outputs = []
    for title_div in soup.find_all("div", class_="title", string="Output"):
        pre = title_div.find_next("pre")
        if pre:
            outputs.append(pre.get_text().strip())

    with open("input.txt", "w", encoding="utf-8") as f:
        for idx in sorted(input_lines_by_case.keys()):
            for line in input_lines_by_case[idx]:
                f.write(line.strip() + "\n")

    with open("expected.txt", "w", encoding="utf-8") as f:
        for output in outputs:
            f.write(output + "\n")

    print("Test cases extracted.")


def run():
    # Compile C++
    if (
        subprocess.run(["g++", "-O0", "-o", "workspace", "workspace.cpp"]).returncode
        != 0
    ):
        print("Compilation failed. Exiting.")
        return

    if not os.path.exists("input.txt"):
        print("Error: input.txt not found. Exiting.")
        return

    # Run program
    with open("input.txt", "r") as fin:
        result = subprocess.run(
            ["./workspace"], stdin=fin, capture_output=True, text=True
        )
        output_lines = [line.strip() for line in result.stdout.splitlines()]

    # Read expected
    if not os.path.exists("expected.txt"):
        print("Error: expected.txt not found. Exiting.")
        return 0

    with open("expected.txt", "r") as f:
        expected_lines = [line.strip() for line in f if line.strip()]

    # Output
    print("\nProgram Output:")
    for line in output_lines:
        print(line)

    print("\nChecking if output matches expected...\n")
    all_passed = True
    max_len = max(len(output_lines), len(expected_lines))
    for i in range(max_len):
        out = output_lines[i] if i < len(output_lines) else ""
        exp = expected_lines[i] if i < len(expected_lines) else ""
        if out == exp:
            print(f"\033[32m+ Case {i+1}: {out}\033[0m")
        else:
            print(f"\033[31m- Case {i+1}: {out}\033[0m")
            print(f"\033[33m  Expected: {exp}\033[0m")
            all_passed = False

    if all_passed:
        print("\n\033[32mVerdict: ✅ Passed\033[0m")
    else:
        print("\n\033[31mVerdict: ❌ Wrong Answer\033[0m")

    return all_passed


def check_problem():

    if run():
        submit_problem()


def submit_problem():
    if not os.path.exists(LINKS_FILE):
        print("No saved URLs found.")
        return

    with open(LINKS_FILE, "r", encoding="utf-8") as f:
        url_fragment = f.readline().strip()

    if not url_fragment:
        print("No saved link found.")
        return

    subprocess.run(
        ["librewolf", f"https://codeforces.com/problemset/submit/{url_fragment}"]
    )

    path = "workspace.cpp"
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            pyperclip.copy(f.read())
    else:
        print(f"{path} not found.")


def main():
    if len(sys.argv) < 2:
        print("Usage: python cf.py <command> <link>")
        return

    command = sys.argv[1]

    if command == "add" and len(sys.argv) >= 3:
        add_link(sys.argv[2])
    elif command == "load" and len(sys.argv) >= 3:
        load_problem(sys.argv[2])
    elif command == "check":
        check_problem()
    elif command == "submit":
        submit_problem()
    else:
        print(f"Unknown or incomplete command '{command}'.")


if __name__ == "__main__":
    main()
