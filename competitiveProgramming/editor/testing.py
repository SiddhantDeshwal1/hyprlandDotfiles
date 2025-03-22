import re

import requests
from bs4 import BeautifulSoup

url = "https://codeforces.com/problemset/problem/2035/C"
headers = {"User-Agent": "Mozilla/5.0"}  # helps bypass 403
response = requests.get(url, headers=headers)
soup = BeautifulSoup(response.text, "lxml")

# Group input lines by test case index
input_lines_by_case = {}
for div in soup.find_all("div", class_=re.compile(r"test-example-line")):
    classes = div.get("class", [])
    line_index = None
    for c in classes:
        match = re.match(r"test-example-line-(\d+)", c)
        if match:
            line_index = int(match.group(1))
            break
    if line_index is not None:
        input_lines_by_case.setdefault(line_index, []).append(div.get_text())

# Get all <pre> tags that are after an "Output" title
outputs = []
for title_div in soup.find_all("div", class_="title", string="Output"):
    pre = title_div.find_next("pre")
    if pre:
        outputs.append(pre.get_text().strip())

# Save input samples
with open("input.txt", "w", encoding="utf-8") as input_file:
    for idx in sorted(input_lines_by_case.keys()):
        for line in input_lines_by_case[idx]:
            input_file.write(line.strip() + "\n")

# Save expected outputs
with open("expected.txt", "w", encoding="utf-8") as expected_file:
    for output in outputs:
        expected_file.write(output + "\n")
