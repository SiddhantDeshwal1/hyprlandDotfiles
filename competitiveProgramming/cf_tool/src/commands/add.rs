use crate::utils::LINKS_FILE;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run(url: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LINKS_FILE)
        .unwrap();
    writeln!(file, "{}", url).unwrap();
    println!("✅ Link added.");
}
