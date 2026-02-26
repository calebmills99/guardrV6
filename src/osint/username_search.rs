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
    /// Status codes that mean "anti-bot block" — treat as likely exists
    auth_wall_codes: &'static [u16],
    /// If true, a 200 with login/auth page likely means profile exists but is gated
    auth_gated: bool,
}

const PLATFORMS: &[PlatformCheck] = &[
    PlatformCheck {
        name: "Instagram",
        url_template: "https://www.instagram.com/{}/",
        not_found_indicators: &["Page Not Found"],
        auth_wall_codes: &[],
        auth_gated: true, // Instagram redirects to login for valid profiles when not authenticated
    },
    PlatformCheck {
        name: "Twitter/X",
        url_template: "https://x.com/{}",
        not_found_indicators: &["This account doesn't exist", "doesn't exist", "account is suspended"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "GitHub",
        url_template: "https://github.com/{}",
        not_found_indicators: &["Not Found"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "Reddit",
        url_template: "https://www.reddit.com/user/{}/about.json",
        not_found_indicators: &["\"error\": 404"],
        auth_wall_codes: &[403], // Reddit 403 = auth wall, profile likely exists
        auth_gated: false,
    },
    PlatformCheck {
        name: "Pinterest",
        url_template: "https://www.pinterest.com/{}/",
        not_found_indicators: &["Sorry! We couldn't find that page"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "LinkedIn",
        url_template: "https://www.linkedin.com/in/{}/",
        not_found_indicators: &["Page not found", "this page doesn't exist", "profile is not available"],
        auth_wall_codes: &[999, 403], // 999 = LinkedIn's anti-bot code, profile likely exists
        auth_gated: true,
    },
    PlatformCheck {
        name: "Facebook",
        url_template: "https://www.facebook.com/{}",
        not_found_indicators: &["Page Not Found", "content isn't available", "This content isn't available"],
        auth_wall_codes: &[400], // Facebook returns 400 for non-logged-in, profile likely exists
        auth_gated: true,
    },
    PlatformCheck {
        name: "YouTube",
        url_template: "https://www.youtube.com/@{}",
        not_found_indicators: &["404 Not Found", "This page isn't available"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "Twitch",
        url_template: "https://www.twitch.tv/{}",
        not_found_indicators: &["Sorry. Unless you've got a time machine"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "Medium",
        url_template: "https://medium.com/@{}",
        not_found_indicators: &["404", "Page not found"],
        auth_wall_codes: &[],
        auth_gated: false,
    },
    PlatformCheck {
        name: "Spotify",
        url_template: "https://open.spotify.com/user/{}",
        not_found_indicators: &["Page not found"],
        auth_wall_codes: &[],
        auth_gated: false,
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
        let auth_wall_codes: Vec<u16> = platform.auth_wall_codes.to_vec();
        let auth_gated = platform.auth_gated;

        let handle = tokio::spawn(async move {
            check_platform(&client, name, &url, &indicators, &auth_wall_codes, auth_gated).await
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
    auth_wall_codes: &[u16],
    auth_gated: bool,
) -> UsernameResult {
    let ua_idx = url.len() % USER_AGENTS.len();
    let user_agent = USER_AGENTS[ua_idx];

    match client
        .get(url)
        .header("User-Agent", user_agent)
        .header("Accept", "text/html,application/xhtml+xml,application/json")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status().as_u16();

            // Auth wall codes = platform is blocking unauthenticated access
            // This typically means the profile EXISTS but requires login to view
            if auth_wall_codes.contains(&status) {
                debug!("{}: FOUND (auth wall, status {})", platform, status);
                return UsernameResult {
                    platform: platform.to_string(),
                    url: url.to_string(),
                    found: true,
                    status_code: Some(status),
                };
            }

            let found = if status == 404 {
                false
            } else if status == 200 {
                match response.text().await {
                    Ok(body) => {
                        let body_lower = body.to_lowercase();
                        let has_not_found = not_found_indicators
                            .iter()
                            .any(|indicator| body_lower.contains(indicator));

                        if has_not_found {
                            false
                        } else if auth_gated {
                            // Auth-gated platforms (Instagram, TikTok, Facebook, LinkedIn):
                            // a 200 with a login page usually means the profile exists
                            // but the platform is forcing login to view it
                            let login_indicators = ["log in", "sign in", "login", "signin", "create an account"];
                            let has_login = login_indicators
                                .iter()
                                .any(|ind| body_lower.contains(ind));
                            // If we see login prompts without not-found text, profile likely exists
                            has_login || !has_not_found
                        } else {
                            true
                        }
                    }
                    Err(_) => false,
                }
            } else {
                // 3xx redirects likely mean profile exists
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
