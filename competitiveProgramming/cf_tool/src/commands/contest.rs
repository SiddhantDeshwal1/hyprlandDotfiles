use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct ContestResponse {
    status: String,
    result: Vec<Contest>,
}

#[derive(Deserialize)]
struct Contest {
    name: String,
    phase: String,
    startTimeSeconds: i64,
    durationSeconds: i64,
}

pub fn run() {
    let url = "https://codeforces.com/api/contest.list?gym=false";
    let res = reqwest::blocking::get(url)
        .unwrap()
        .json::<ContestResponse>()
        .unwrap();
    if res.status != "OK" {
        println!("❌ Failed to fetch contests");
        return;
    }
    let upcoming: Vec<_> = res
        .result
        .into_iter()
        .filter(|c| c.phase == "BEFORE")
        .collect();
    if upcoming.is_empty() {
        println!("📭 No upcoming contests.");
        return;
    }

    println!("\n📅 Upcoming Contests (IST):\n");
    let ist_offset = FixedOffset::east_opt(5 * 3600 + 1800).unwrap();

    for contest in upcoming {
        let naive = NaiveDateTime::from_timestamp(contest.startTimeSeconds, 0);
        let time = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
        let ist = time.with_timezone(&ist_offset);
        println!(
            "📌 {} | 🕒 {} IST | ⏱️ {}h",
            contest.name,
            ist.format("%Y-%m-%d %H:%M"),
            contest.durationSeconds / 3600
        );
    }
}
