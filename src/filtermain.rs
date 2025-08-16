use serde_json::{Value};
use std::fs;

pub fn run_advanced_filter(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(input)?;
    let mut json: Value = serde_json::from_str(&data)?;

    let bad_emails = vec!["dan@hotmail.com", "mcrib96@gmail.com"];

    if let Some(entries) = json.as_array_mut() {
        entries.retain(|entry| {
            if let Some(email) = entry["email"].as_str() {
                !bad_emails.contains(&email)
            } else {
                true
            }
        });

        entries.sort_by(|a, b| {
            let date_a = a["source"]["breach_date"].as_str().unwrap_or("");
            let date_b = b["source"]["breach_date"].as_str().unwrap_or("");
            date_b.cmp(date_a)
        });

        for entry in entries.iter_mut() {
            if let Some(password) = entry["password"].as_str() {
                if is_weak_password(password) {
                    entry["weak_password"] = Value::Bool(true);
                }
            }
        }
    }

    fs::write(output, serde_json::to_string_pretty(&json)?)?;
    println!("Filtering, sorting, and pattern detection complete. Check {output}");

    Ok(())
}

fn is_weak_password(password: &str) -> bool {
    let common_patterns = [
        "123456", "password", "qwerty", "1q2w3e4r", "letmein", "iloveyou",
        "admin", "welcome", "monkey", "football", "abc123", "passw0rd", "zaq1zaq1",
        "root", "changeme", "trustno1", "shadow", "master", "superman"
    ];
    common_patterns.iter().any(|&p| password.contains(p))
}
