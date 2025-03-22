use crate::utils::{read_lines, run_cpp};
use colored::*;

pub fn run() {
    let expected = read_lines("data/expected.txt");
    if let Some(output) = run_cpp() {
        println!("\n📤 Output:");
        for line in &output {
            println!("{}", line);
        }

        println!("\n✅ Checking output...\n");
        let mut all_passed = true;
        for i in 0..expected.len().max(output.len()) {
            let empty = String::new();
            let out = output.get(i).unwrap_or(&empty);
            let exp = expected.get(i).unwrap_or(&empty);
            if out == exp {
                println!("{} Case {}: {}", "+".green(), i + 1, out);
            } else {
                println!("{} Case {}: {}", "-".red(), i + 1, out);
                println!("{} Expected: {}", "  Expected:".yellow(), exp);
                all_passed = false;
            }
        }
        println!(
            "
{}",
            if all_passed {
                "✅ Passed".green()
            } else {
                "❌ Wrong Answer".red()
            }
        );
    }
}
