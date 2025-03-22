use crate::utils::read_lines;
use std::process::Command;

pub fn run() {
    let lines = read_lines("data/saved_links.txt");
    let contest_id = lines.get(0).map(|s| s.as_str()).unwrap_or("");

    Command::new("librewolf")
        .arg(format!(
            "https://codeforces.com/contest/{}/standings/friends/true",
            contest_id
        ))
        .spawn()
        .unwrap();
}
