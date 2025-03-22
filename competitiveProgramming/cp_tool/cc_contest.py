# TODO :: color red if upcoming is less than 12 hr

from datetime import datetime

import requests

url = "https://www.codechef.com/api/list/contests/all"
response = requests.get(url)
data = response.json()

print("📅 Upcoming CodeChef Contests:\n")

for contest in data["future_contests"]:
    name = contest["contest_name"]
    code = contest["contest_code"]
    start = datetime.strptime(contest["contest_start_date"], "%d %b %Y  %H:%M:%S")
    end = datetime.strptime(contest["contest_end_date"], "%d %b %Y  %H:%M:%S")
    duration_minutes = int(contest["contest_duration"])
    url = f"https://www.codechef.com/{code}"

    print(f"🟢 {name}")
    print(f"   🗓  Start   : {start.strftime('%A, %d %B %Y %I:%M %p')}")
    print(f"   🛑 End     : {end.strftime('%A, %d %B %Y %I:%M %p')}")
    print(f"   ⏱  Duration: {duration_minutes} minutes")
    print(f"   🔗 Link    : {url}\n")
