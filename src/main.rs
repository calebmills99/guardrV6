mod filter;
mod filtermain;
mod fetch_dumps;
mod weak_pass;
mod risk_score;

use std::env;
use weak_pass::load_password_list;
use serde_json::{Value};
use std::fs;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  guardr basic <input_file> <output_file>");
    eprintln!("  guardr advanced <input_file> <output_file>");
    eprintln!("  guardr fetch <url> <output_file>");
    eprintln!("  guardr check-pass <password_list> <password>");
    eprintln!("  guardr risk-score <password_list> <input_json>");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];
    let input = &args[2];
    let output = &args[3];

    match command.as_str() {
        "basic" => filter::run_filter(input, output)?,
        "advanced" => filtermain::run_advanced_filter(input, output)?,
        "fetch" => fetch_dumps::download_dump(input, output)?,
        "check-pass" => {
            let passwords = weak_pass::load_password_list(input)?;
            if weak_pass::is_password_weak(output, &passwords) {
                println!("🚨 \"{}\" is WEAK. Pick a better password, sweetie!", output);
            } else {
                println!("✅ \"{}\" is strong enough. Work it!", output);
            }
        },
        "risk-score" => {
            let passwords = load_password_list(input)?;
            let json_str = fs::read_to_string(output)?;
            let json_data: Value = serde_json::from_str(&json_str)?;
            let risk = risk_score::calculate_risk_score(&json_data, &passwords);
            println!("🔍 Risk score: {} / 100", risk);
        },
        _ => {
            eprintln!("Unknown command: '{}'", command);
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}
