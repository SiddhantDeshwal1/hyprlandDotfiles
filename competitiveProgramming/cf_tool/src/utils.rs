use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub const LINKS_FILE: &str = "data/saved_links.txt";

pub fn read_lines(path: &str) -> Vec<String> {
    File::open(path)
        .map(BufReader::new)
        .map(|reader| reader.lines().filter_map(Result::ok).collect())
        .unwrap_or_default()
}

pub fn write_lines(path: &str, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn run_cpp() -> Option<Vec<String>> {
    if !Command::new("g++")
        .args([
            "-std=c++20",
            "-O2",
            "-o",
            "data/workspace",
            "data/workspace.cpp",
        ])
        .status()
        .ok()?
        .success()
    {
        println!("❌ Compilation failed.");
        return None;
    }

    let input = File::open("data/input.txt").ok()?;
    let output = Command::new("./data/workspace")
        .stdin(Stdio::from(input))
        .stdout(Stdio::piped())
        .spawn()
        .ok()?
        .wait_with_output()
        .ok()?;

    Some(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect(),
    )
}
