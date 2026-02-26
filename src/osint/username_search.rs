use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, debug};

use super::UsernameResult;

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
];

struct PlatformCheck {
    name: &'static str,
    url_template: &'static str,
    not_found_indicators: &'static [&'static str],
}

const PLATFORMS: &[PlatformCheck] = &[
    PlatformCheck {
        name: "Instagram",
        url_template: "https://www.instagram.com/{}/",
        not_found_indicators: &["Sorry, this page isn't available", "Page Not Found"],
    },
    PlatformCheck {
        name: "Twitter/X",
        url_template: "https://x.com/{}",
        not_found_indicators: &["This account doesn't exist", "doesn't exist"],
    },
    PlatformCheck {
        name: "GitHub",
        url_template: "https://github.com/{}",
        not_found_indicators: &["Not Found"],
    },
    PlatformCheck {
        name: "Reddit",
        url_template: "https://www.reddit.com/user/{}/",
        not_found_indicators: &["nobody on Reddit goes by that name", "Sorry, nobody"],
    },
    PlatformCheck {
        name: "TikTok",
        url_template: "https://www.tiktok.com/@{}",
        not_found_indicators: &["Couldn't find this account"],
    },
    PlatformCheck {
        name: "Pinterest",
        url_template: "https://www.pinterest.com/{}/",
        not_found_indicators: &["Sorry! We couldn't find that page"],
    },
    PlatformCheck {
        name: "LinkedIn",
        url_template: "https://www.linkedin.com/in/{}/",
        not_found_indicators: &["Page not found", "this page doesn't exist"],
    },
    PlatformCheck {
        name: "Facebook",
        url_template: "https://www.facebook.com/{}",
        not_found_indicators: &["Page Not Found", "content isn't available"],
    },
    PlatformCheck {
        name: "YouTube",
        url_template: "https://www.youtube.com/@{}",
        not_found_indicators: &["404 Not Found", "This page isn't available"],
    },
    PlatformCheck {
        name: "Twitch",
        url_template: "https://www.twitch.tv/{}",
        not_found_indicators: &["Sorry. Unless you've got a time machine"],
    },
    PlatformCheck {
        name: "Medium",
        url_template: "https://medium.com/@{}",
        not_found_indicators: &["404", "Page not found"],
    },
    PlatformCheck {
        name: "Spotify",
        url_template: "https://open.spotify.com/user/{}",
        not_found_indicators: &["Page not found"],
    },
];

/// Search for a username across multiple social platforms concurrently
/// Ported from Kallisto-OSINTer's username_search module
pub async fn search_username(username: &str) -> Vec<UsernameResult> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()
        .unwrap_or_default();

    let mut handles = Vec::new();

    for platform in PLATFORMS {
        let url = platform.url_template.replace("{}", username);
        let client = client.clone();
        let name = platform.name;
        let indicators: Vec<String> = platform
            .not_found_indicators
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        let handle = tokio::spawn(async move {
            check_platform(&client, name, &url, &indicators).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    let found_count = results.iter().filter(|r| r.found).count();
    info!(
        "Username search for '{}': found on {}/{} platforms",
        username,
        found_count,
        results.len()
    );

    results
}

async fn check_platform(
    client: &Client,
    platform: &str,
    url: &str,
    not_found_indicators: &[String],
) -> UsernameResult {
    let ua_idx = url.len() % USER_AGENTS.len();
    let user_agent = USER_AGENTS[ua_idx];

    match client
        .get(url)
        .header("User-Agent", user_agent)
        .header("Accept", "text/html,application/xhtml+xml")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status().as_u16();
            let found = if status == 404 {
                false
            } else if status == 200 {
                match response.text().await {
                    Ok(body) => {
                        let body_lower = body.to_lowercase();
                        !not_found_indicators
                            .iter()
                            .any(|indicator| body_lower.contains(indicator))
                    }
                    Err(_) => false,
                }
            } else {
                status >= 200 && status < 400
            };

            debug!("{}: {} (status {})", platform, if found { "FOUND" } else { "NOT FOUND" }, status);

            UsernameResult {
                platform: platform.to_string(),
                url: url.to_string(),
                found,
                status_code: Some(status),
            }
        }
        Err(e) => {
            debug!("{}: Error - {}", platform, e);
            UsernameResult {
                platform: platform.to_string(),
                url: url.to_string(),
                found: false,
                status_code: None,
            }
        }
    }
}
