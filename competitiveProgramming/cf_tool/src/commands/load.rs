use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

pub fn load(contest_id: &str, problem_index: &str) {
    let url = format!(
        "https://codeforces.com/contest/{}/problem/{}",
        contest_id, problem_index
    );
    let res = get(&url).expect("Failed to fetch problem page");
    let body = res.text().unwrap();

    let document = Html::parse_document(&body);
    let input_selector = Selector::parse(".input pre").unwrap();
    let output_selector = Selector::parse(".output pre").unwrap();

    let mut input_file = File::create("data/input.txt").unwrap();
    let mut output_file = File::create("data/expected.txt").unwrap();

    for input in document.select(&input_selector) {
        let text = input.text().collect::<Vec<_>>().join("\n");
        writeln!(input_file, "{}", text.trim()).unwrap();
        writeln!(input_file, "---").unwrap(); // separator
    }

    for output in document.select(&output_selector) {
        let text = output.text().collect::<Vec<_>>().join("\n");
        writeln!(output_file, "{}", text.trim()).unwrap();
        writeln!(output_file, "---").unwrap(); // separator
    }

    println!("Test cases loaded.");
}
