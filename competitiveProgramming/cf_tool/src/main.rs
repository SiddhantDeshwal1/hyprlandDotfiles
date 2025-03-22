mod commands;
mod utils;

use commands::{add, check, contest, friends, last, load, submit};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cf_tool <command> [args]");
        return;
    }

    let command = &args[1];
    let arg = if args.len() > 2 { Some(&args[2]) } else { None };

    match command.as_str() {
        "add" => {
            if let Some(link) = arg {
                add::run(link);
            } else {
                println!("Please provide a Codeforces problem link.");
            }
        }
        "load" => {
            if let Some(link) = arg {
                load::run(link);
            } else {
                println!("Please provide a Codeforces problem link.");
            }
        }
        "check" => check::run(),
        "submit" => submit::run(),
        "contest" => contest::run(),
        "last" => last::run(),
        "friends" => friends::run(),
        _ => println!("Unknown command: {}", command),
    }
}
