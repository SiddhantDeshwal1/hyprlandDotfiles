use crate::utils::{read_lines, LINKS_FILE};
use clipboard::ClipboardProvider;
use std::process::Command;

pub fn run() {
    let url = read_lines(LINKS_FILE).get(0).cloned().unwrap_or_default();
    if url.is_empty() {
        println!("❌ No saved URL.");
        return;
    }
    Command::new("librewolf")
        .arg(format!("https://codeforces.com/problemset/submit/{}", url))
        .spawn()
        .unwrap();

    if let Ok(contents) = std::fs::read_to_string("data/workspace.cpp") {
        clipboard::ClipboardContext::new()
            .unwrap()
            .set_contents(contents)
            .unwrap();
    } else {
        println!("❌ workspace.cpp not found.");
    }
}
