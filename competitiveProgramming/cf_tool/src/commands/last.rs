use chrono::{TimeZone, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    status: String,
    result: Vec<Submission>,
}

#[derive(Deserialize)]
struct Submission {
    problem: Problem,
    verdict: Option<String>,
    passedTestCount: Option<u32>,
    timeConsumedMillis: u32,
    memoryConsumedBytes: u32,
}

#[derive(Deserialize)]
struct Problem {
    contestId: u32,
    index: String,
    name: String,
}

pub fn run() {
    let handle = "worthNothing";
    let url = format!(
        "https://codeforces.com/api/user.status?handle={}&from=1&count=1",
        handle
    );

    if let Ok(resp) = reqwest::blocking::get(&url).unwrap().json::<ApiResponse>() {
        if let Some(sub) = resp.result.first() {
            println!(
                "📘 Problem: {}{} - {}",
                sub.problem.contestId, sub.problem.index, sub.problem.name
            );
            println!(
                "🧪 Verdict: {}",
                sub.verdict.clone().unwrap_or("N/A".into())
            );
            println!("✅ Passed: {}", sub.passedTestCount.unwrap_or(0));
            println!("⚡ Time: {} ms", sub.timeConsumedMillis);
            println!("📦 Memory: {} bytes", sub.memoryConsumedBytes);
        }
    } else {
        println!("❌ API error.");
    }
}
