use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

const TRESTLE_PHONE_URL: &str = "https://api.trestleiq.com/3.2/phone";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneLookupResult {
    pub valid: bool,
    pub number: String,
    pub country_code: Option<String>,
    pub carrier: Option<String>,
    pub line_type: Option<String>,
    pub is_prepaid: Option<bool>,
    pub is_commercial: Option<bool>,
    pub owner_name: Option<String>,
    pub owner_age_range: Option<String>,
    pub owner_gender: Option<String>,
    pub owner_city: Option<String>,
    pub owner_state: Option<String>,
    pub associated_emails: Vec<String>,
}

/// Look up a phone number using Trestle Reverse Phone API (v3.2)
pub async fn lookup_phone(phone: &str, api_key: &str) -> Result<PhoneLookupResult> {
    let client = crate::http_client::build_default_client();

    let response = client
        .get(TRESTLE_PHONE_URL)
        .header("x-api-key", api_key)
        .query(&[("phone", phone)])
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;

            if let Some(err) = body.get("error") {
                let msg = err.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                warn!("Trestle error: {}", msg);
                return Err(anyhow::anyhow!("Trestle: {}", msg));
            }

            let phone_info = &body;
            let is_valid = phone_info.get("is_valid").and_then(|v| v.as_bool()).unwrap_or(false);
            let line_type = phone_info.get("line_type").and_then(|v| v.as_str()).map(String::from);
            let carrier = phone_info.get("carrier").and_then(|v| v.as_str()).map(String::from);
            let is_prepaid = phone_info.get("is_prepaid").and_then(|v| v.as_bool());
            let is_commercial = phone_info.get("is_commercial").and_then(|v| v.as_bool());
            let country_code = phone_info
                .get("country_calling_code")
                .and_then(|v| v.as_str())
                .map(String::from);

            // Extract owner info from belongs_to array
            let belongs_to = phone_info
                .get("belongs_to")
                .and_then(|b| b.as_array())
                .and_then(|arr| arr.first());

            let owner_name = belongs_to
                .and_then(|b| b.get("name"))
                .and_then(|v| v.as_str())
                .map(String::from);

            let owner_age_range = belongs_to
                .and_then(|b| b.get("age_range"))
                .and_then(|v| v.as_str())
                .map(String::from);

            let owner_gender = belongs_to
                .and_then(|b| b.get("gender"))
                .and_then(|v| v.as_str())
                .map(String::from);

            // Extract address
            let current_address = phone_info
                .get("current_addresses")
                .and_then(|a| a.as_array())
                .and_then(|arr| arr.first());

            let owner_city = current_address
                .and_then(|a| a.get("city"))
                .and_then(|v| v.as_str())
                .map(String::from);

            let owner_state = current_address
                .and_then(|a| a.get("state_code"))
                .and_then(|v| v.as_str())
                .map(String::from);

            // Extract associated emails
            let associated_emails = phone_info
                .get("associated_emails")
                .and_then(|e| e.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            item.get("address")
                                .or_else(|| item.as_str().map(serde_json::Value::from).as_ref().map(|_| item))
                                .and_then(|v| v.as_str())
                                .map(String::from)
                        })
                        .collect()
                })
                .unwrap_or_default();

            info!(
                "Trestle phone lookup: {} valid={} type={} carrier={} owner={}",
                phone,
                is_valid,
                line_type.as_deref().unwrap_or("unknown"),
                carrier.as_deref().unwrap_or("unknown"),
                owner_name.as_deref().unwrap_or("unknown"),
            );

            Ok(PhoneLookupResult {
                valid: is_valid,
                number: phone.to_string(),
                country_code,
                carrier,
                line_type,
                is_prepaid,
                is_commercial,
                owner_name,
                owner_age_range,
                owner_gender,
                owner_city,
                owner_state,
                associated_emails,
            })
        }
        401 => Err(anyhow::anyhow!("Trestle: Invalid API key")),
        429 => Err(anyhow::anyhow!("Trestle: Rate limit exceeded")),
        status => {
            warn!("Trestle: status {}", status);
            Err(anyhow::anyhow!("Trestle returned status {}", status))
        }
    }
}

/// Assess risk from phone lookup data
pub fn assess_phone_risk(result: &PhoneLookupResult) -> (f32, String) {
    if !result.valid {
        return (80.0, "Invalid phone number — could be fake or disposable".to_string());
    }

    let line_type = result.line_type.as_deref().unwrap_or("unknown");
    let carrier = result.carrier.as_deref().unwrap_or("");
    let is_prepaid = result.is_prepaid.unwrap_or(false);

    let mut score: f32;
    let mut details = Vec::new();

    match line_type {
        "Mobile" => {
            score = 12.0;
            details.push(format!("Valid mobile on {}", if carrier.is_empty() { "unknown carrier" } else { carrier }));
        }
        "Landline" => {
            score = 18.0;
            details.push("Landline — unusual for dating but real".to_string());
        }
        "FixedVOIP" => {
            score = 35.0;
            details.push("Fixed VoIP (business-grade) — moderate risk".to_string());
        }
        "NonFixedVOIP" => {
            score = 65.0;
            details.push("Non-fixed VoIP — commonly used for burner/disposable numbers".to_string());
        }
        "TollFree" => {
            score = 75.0;
            details.push("Toll-free number — not a personal phone".to_string());
        }
        _ => {
            score = 40.0;
            details.push(format!("Line type: {}", line_type));
        }
    };

    if is_prepaid {
        score += 15.0;
        details.push("Prepaid phone — harder to trace".to_string());
    }

    if result.owner_name.is_some() {
        score = (score - 10.0).max(5.0);
        details.push(format!("Registered owner identified"));
    }

    if !result.associated_emails.is_empty() {
        score = (score - 5.0).max(5.0);
        details.push(format!("{} associated email(s) found", result.associated_emails.len()));
    }

    if let Some(ref city) = result.owner_city {
        if let Some(ref state) = result.owner_state {
            details.push(format!("Location: {}, {}", city, state));
        }
    }

    (score.min(100.0), details.join(". "))
}
