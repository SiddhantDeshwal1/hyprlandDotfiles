import time
from datetime import datetime, timedelta

import requests

# Prepare the query
query = {
    "query": """
    query {
      allContests {
        title
        titleSlug
        startTime
        duration
      }
    }
    """
}

# Optional headers (sometimes required)
headers = {
    "Referer": "https://leetcode.com/contest/",
    "Content-Type": "application/json",
}

# Send request
response = requests.post("https://leetcode.com/graphql", json=query, headers=headers)
data = response.json()

# Get current time in Unix timestamp
now = int(time.time())

print("📅 Upcoming LeetCode Contests:\n")

for contest in data["data"]["allContests"]:
    if contest["startTime"] > now:
        start_time = datetime.fromtimestamp(contest["startTime"])
        duration = str(timedelta(seconds=contest["duration"]))
        print(f"{contest['title']}")
        print(f"  ➤ Start: {start_time}")
        print(f"  ➤ Duration: {duration}")
        print(f"  ➤ URL: https://leetcode.com/contest/{contest['titleSlug']}/\n")
