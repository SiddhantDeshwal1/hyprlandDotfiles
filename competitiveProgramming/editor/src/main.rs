use clap::{App, Arg, SubCommand};
use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    process::{Command, Stdio},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use webbrowser;

const LINKS_FILE: &str = "saved_links.txt";

fn main() {
    let matches = App::new("cf-helper")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a problem URL")
                .arg(Arg::with_name("URL").required(true)),
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load problem test cases")
                .arg(Arg::with_name("URL").required(true)),
        )
        .subcommand(SubCommand::with_name("check").about("Check solution"))
        .subcommand(SubCommand::with_name("submit").about("Submit solution"))
        .subcommand(SubCommand::with_name("friends").about("Open friends standings"))
        .subcommand(SubCommand::with_name("last").about("Check last submission"))
        .subcommand(SubCommand::with_name("contest").about("Show upcoming contests"))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => add_link(sub_m.value_of("URL").unwrap()),
        ("load", Some(sub_m)) => load_problem(sub_m.value_of("URL").unwrap()),
        ("check", _) => check_problem(),
        ("submit", _) => submit_problem(),
        ("friends", _) => friends_cmd(),
        ("last", _) => check_last_submission(),
        ("contest", _) => show_upcoming_regular_contests(),
        _ => println!("Unknown command"),
    }
}

fn add_link(url: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LINKS_FILE)
        .expect("❌ Could not open saved_links.txt");

    writeln!(file, "{}", url).expect("❌ Could not write to saved_links.txt");
    println!("✅ Added link: {}", url);
}

fn load_problem(url: &str) {
    let client = Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .expect("❌ HTTP error");

    if !resp.status().is_success() {
        println!("❌ Non-200 response: {}", resp.status());
        return;
    }

    let body = resp.text().expect("❌ Failed to read response body");
    let document = Html::parse_document(&body);
    let re_line = Regex::new(r"test-example-line-(\d+)").unwrap();
    let mut inputs_map: HashMap<i32, Vec<String>> = HashMap::new();

    // Parse input cases
    for element in document.select(&Selector::parse("div").unwrap()) {
        if let Some(class) = element.value().attr("class") {
            if let Some(caps) = re_line.captures(class) {
                let idx = caps[1].parse::<i32>().unwrap();
                let text = element.text().collect::<String>().trim().to_string();
                inputs_map.entry(idx).or_default().push(text);
            }
        }
    }

    // Parse output cases
    let mut outputs = Vec::new();
    for element in document.select(&Selector::parse("div.title").unwrap()) {
        if element.text().any(|t| t.trim() == "Output") {
            if let Some(pre) = element.next_sibling_element() {
                outputs.push(pre.text().collect::<String>().trim().to_string());
            }
        }
    }

    // Write input.txt
    let mut sorted_keys: Vec<&i32> = inputs_map.keys().collect();
    sorted_keys.sort();

    let mut input_file = File::create("input.txt").expect("❌ Could not create input.txt");
    for key in sorted_keys {
        for line in &inputs_map[key] {
            writeln!(input_file, "{}", line).unwrap();
        }
    }

    // Write expected.txt
    let mut output_file = File::create("expected.txt").expect("❌ Could not create expected.txt");
    for out in outputs {
        writeln!(output_file, "{}", out).unwrap();
    }

    println!("✅ Test cases extracted");
}

// Remaining functions follow similar patterns...
