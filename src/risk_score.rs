use serde_json::Value;
use std::collections::HashSet;

pub fn calculate_risk_score(entry: &Value, weak_passwords: &HashSet<String>) -> u8 {
    let mut score = 0;

    // Check for weak password (worth up to 40 points)
    if let Some(password) = entry["password"].as_str() {
        if weak_passwords.contains(password) {
            score += 40;
        }
    }

    // Check if email appears multiple times (20 points)
    if entry["occurrences"].as_u64().unwrap_or(1) > 1 {
        score += 20;
    }

    // Recent breach? (within past year - 40 points)
    if let Some(date) = entry["source"]["breach_date"].as_str() {
        if is_recent_breach(date) {
            score += 40;
        }
    }

    score.min(100) // cap at 100
}

fn is_recent_breach(date: &str) -> bool {
    use chrono::{NaiveDate, Utc, Duration};

    if let Ok(breach_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        let one_year_ago = Utc::now().naive_utc().date() - Duration::days(365);
        breach_date > one_year_ago
    } else {
        false
    }
}
