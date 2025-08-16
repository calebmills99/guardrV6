use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
// use colored::*; // temporarily commented out

pub fn run_filter(input: &str, output: &str) -> io::Result<()> {
    let mut file = File::open(input)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut json_data: Value = serde_json::from_str(&data)?;

    if let Some(entries) = json_data.get_mut("result").and_then(|r| r.as_array_mut()) {
        entries.retain(|entry| {
            if let Some(email) = entry.get("email").and_then(|e| e.as_str()) {
                !email.contains("dan@hotmail.com")
            } else {
                true
            }
        });
    }

    let cleaned_data = serde_json::to_string_pretty(&json_data)?;
    let mut output_file = File::create(output)?;
    output_file.write_all(cleaned_data.as_bytes())?;

    println!("Filtered JSON saved to {}", output);

    Ok(())  // <- explicitly return OK
}
