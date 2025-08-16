// src/main.rs

mod filter;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: guardr <input_file> <output_file>");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    filter::run_filter(input, output)?;
    Ok(())
}
