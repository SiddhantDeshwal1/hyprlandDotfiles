use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
};

const LINKS_FILE: &str = "saved_links.txt";

fn add_link(url: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LINKS_FILE)?;
    writeln!(file, "{}", url)?;
    Ok(())
}

fn load_problem(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if url.trim().is_empty() {
        println!("No URL provided.");
        return Ok(());
    }

    // Extract the last two segments from the URL.
    let parts: Vec<&str> = url.trim().split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() < 2 {
        println!("Invalid URL.");
        return Ok(());
    }
    let url_segment = format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1]);
    fs::write(LINKS_FILE, format!("{}\n", url_segment))?;

    let resp = get(url)?.text()?;
    let document = Html::parse_document(&resp);

    // Collect input test cases from div elements with classes matching "test-example-line-<number>".
    let mut inputs: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let div_selector = Selector::parse("div").unwrap();
    let class_regex = Regex::new(r"test-example-line-(\d+)").unwrap();
    for element in document.select(&div_selector) {
        if let Some(class_attr) = element.value().attr("class") {
            for class_name in class_attr.split_whitespace() {
                if let Some(caps) = class_regex.captures(class_name) {
                    if let Ok(idx) = caps[1].parse::<usize>() {
                        let text = element
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        inputs.entry(idx).or_default().push(text);
                        break;
                    }
                }
            }
        }
    }

    // Extract outputs using a regex that finds a "title" div with text "Output" followed by a <pre> block.
    let output_regex = Regex::new(
        r#"<div[^>]*class="[^"]*\btitle\b[^"]*"[^>]*>\s*Output\s*</div>\s*<pre[^>]*>(?P<content>.*?)</pre>"#,
    )?;
    let mut outputs = Vec::new();
    for caps in output_regex.captures_iter(&resp) {
        let content = caps.name("content").unwrap().as_str().trim().to_string();
        outputs.push(content);
    }

    // Write test inputs to "input.txt".
    let mut input_file = File::create("input.txt")?;
    for (_idx, lines) in inputs.iter() {
        for line in lines {
            writeln!(input_file, "{}", line)?;
        }
    }

    // Write expected outputs to "expected.txt".
    fs::write("expected.txt", outputs.join("\n"))?;

    println!("✅ Test cases extracted.");
    Ok(())
}

fn compile_cpp() -> bool {
    let status = Command::new("g++")
        .args(&["-std=c++20", "-O2", "-o", "workspace", "workspace.cpp"])
        .status();
    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

fn run_problem() -> bool {
    if !compile_cpp() {
        println!("❌ Compilation failed.");
        return false;
    }

    if !Path::new("input.txt").exists() {
        println!("❌ input.txt not found.");
        return false;
    }

    let input_file = File::open("input.txt").unwrap();
    let output = Command::new("./workspace")
        .stdin(Stdio::from(input_file))
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            println!("Error running workspace: {}", e);
            return false;
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let output_lines: Vec<&str> = stdout.lines().map(|s| s.trim()).collect();

    if !Path::new("expected.txt").exists() {
        println!("❌ expected.txt not found.");
        return false;
    }
    let expected_content = fs::read_to_string("expected.txt").unwrap();
    let expected_lines: Vec<&str> = expected_content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    println!("\n📤 Output:");
    for line in &output_lines {
        println!("{}", line);
    }

    println!("\n✅ Checking output...\n");
    let mut all_passed = true;
    let max_cases = output_lines.len().max(expected_lines.len());
    for i in 0..max_cases {
        let out = output_lines.get(i).unwrap_or(&"");
        let exp = expected_lines.get(i).unwrap_or(&"");
        if out == exp {
            println!("\x1b[32m+ Case {}: {}\x1b[0m", i + 1, out);
        } else {
            println!("\x1b[31m- Case {}: {}\x1b[0m", i + 1, out);
            println!("\x1b[33m  Expected: {}\x1b[0m", exp);
            all_passed = false;
        }
    }
    if all_passed {
        println!("\n\x1b[32m✅ Passed\x1b[0m");
    } else {
        println!("\n\x1b[31m❌ Wrong Answer\x1b[0m");
    }
    all_passed
}

fn submit_problem() {
    if !Path::new(LINKS_FILE).exists() {
        println!("❌ No saved URLs.");
        return;
    }
    let content = fs::read_to_string(LINKS_FILE).unwrap_or_default();
    let mut lines = content.lines();
    let url_fragment = lines.next().unwrap_or("").trim();
    if url_fragment.is_empty() {
        println!("❌ Empty saved URL.");
        return;
    }
    let url = format!("https://codeforces.com/problemset/submit/{}", url_fragment);
    // Open the URL in Librewolf.
    let _ = Command::new("librewolf").arg(&url).spawn();

    if Path::new("workspace.cpp").exists() {
        let code = fs::read_to_string("workspace.cpp").unwrap_or_default();
        // Copy the code to the clipboard using arboard.
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(code) {
                    println!("❌ Failed to copy to clipboard: {}", e);
                }
            }
            Err(e) => println!("❌ Failed to access clipboard: {}", e),
        }
    } else {
        println!("❌ workspace.cpp not found.");
    }
}

fn check_problem() {
    if run_problem() {
        submit_problem();
    }
}

fn friends() {
    if let Ok(content) = fs::read_to_string(LINKS_FILE) {
        if let Some(first_line) = content.lines().next() {
            let parts: Vec<&str> = first_line.split('/').collect();
            if !parts.is_empty() {
                let contest_id = parts[0];
                let url = format!(
                    "https://codeforces.com/contest/{}/standings/friends/true",
                    contest_id
                );
                let _ = Command::new("librewolf").arg(&url).spawn();
            }
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse<T> {
    status: String,
    result: T,
}

#[derive(Deserialize)]
struct Submission {
    problem: Problem,
    verdict: Option<String>,
    passedTestCount: Option<u32>,
    timeConsumedMillis: Option<u32>,
    memoryConsumedBytes: Option<u32>,
}

#[derive(Deserialize)]
struct Problem {
    contestId: u32,
    index: String,
    name: String,
}

fn check_last_submission() {
    let url = "https://codeforces.com/api/user.status?handle=worthNothing&from=1&count=1";
    if let Ok(response) = get(url) {
        if let Ok(json) = response.json::<ApiResponse<Vec<Submission>>>() {
            if json.status != "OK" || json.result.is_empty() {
                println!("❌ API error.");
                return;
            }
            let sub = &json.result[0];
            println!(
                "📘 Problem: {}{} - {}",
                sub.problem.contestId, sub.problem.index, sub.problem.name
            );
            println!("🧪 Verdict: {}", sub.verdict.as_deref().unwrap_or("N/A"));
            println!("✅ Passed: {}", sub.passedTestCount.unwrap_or(0));
            println!("⚡ Time: {} ms", sub.timeConsumedMillis.unwrap_or(0));
            println!("📦 Memory: {} bytes", sub.memoryConsumedBytes.unwrap_or(0));
        } else {
            println!("❌ Failed to parse API response.");
        }
    } else {
        println!("❌ API error.");
    }
}

#[derive(Deserialize)]
struct Contest {
    name: String,
    phase: String,
    startTimeSeconds: Option<i64>,
    durationSeconds: i64,
}

fn show_upcoming_regular_contests() {
    let url = "https://codeforces.com/api/contest.list?gym=false";
    if let Ok(response) = get(url) {
        if let Ok(json) = response.json::<ApiResponse<Vec<Contest>>>() {
            if json.status != "OK" {
                println!("❌ Failed: API status not OK.");
                return;
            }
            let mut upcoming: Vec<&Contest> =
                json.result.iter().filter(|c| c.phase == "BEFORE").collect();
            if upcoming.is_empty() {
                println!("📭 No upcoming contests.");
                return;
            }
            upcoming.sort_by_key(|c| c.startTimeSeconds.unwrap_or(0));
            println!("\n📅 Upcoming Contests (IST):\n");
            for contest in upcoming {
                let name = &contest.name;
                if let Some(start_ts) = contest.startTimeSeconds {
                    // Convert from UTC to IST (UTC+5:30).
                    let utc_time = NaiveDateTime::from_timestamp_opt(start_ts, 0).unwrap();
                    let utc_dt: DateTime<Utc> = DateTime::from_utc(utc_time, Utc);
                    let ist_offset = FixedOffset::east(5 * 3600 + 30 * 60);
                    let ist_time = utc_dt.with_timezone(&ist_offset);
                    println!(
                        "📌 {} | 🕒 {} IST | ⏱️ {}h",
                        name,
                        ist_time.format("%Y-%m-%d %H:%M"),
                        contest.durationSeconds / 3600
                    );
                }
            }
        } else {
            println!("❌ Failed to parse contest list.");
        }
    } else {
        println!("❌ Error fetching contest list.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cf <command> [link]");
        return;
    }
    let cmd = &args[1];
    let arg = if args.len() > 2 { Some(&args[2]) } else { None };

    match cmd.as_str() {
        "add" => {
            if let Some(link) = arg {
                if let Err(e) = add_link(link) {
                    println!("Error adding link: {}", e);
                }
            } else {
                println!("Missing link argument.");
            }
        }
        "load" => {
            if let Some(link) = arg {
                if let Err(e) = load_problem(link) {
                    println!("Error loading problem: {}", e);
                }
            } else {
                println!("Missing link argument.");
            }
        }
        "check" => check_problem(),
        "submit" => submit_problem(),
        "contest" => show_upcoming_regular_contests(),
        "last" => check_last_submission(),
        "friends" => friends(),
        _ => println!("❓ Unknown or incomplete command '{}'", cmd),
    }
}
