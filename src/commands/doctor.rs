use crate::client::SoundchartsClient;
use crate::config::{self, is_sandbox, load_config, resolve_credentials};
use reqwest::Client;
use std::time::Instant;

pub async fn run(flag_app_id: Option<&str>, flag_api_key: Option<&str>) {
    let config_path = config::config_path();
    if config_path.exists() {
        let config = load_config();
        if config.auth.app_id.is_some() && config.auth.api_key.is_some() {
            println!("Config file ......... OK ({})", config_path.display());
        } else {
            println!("Config file ......... WARN (exists but missing credentials)");
        }
    } else {
        println!(
            "Config file ......... NOT FOUND ({})",
            config_path.display()
        );
    }

    let creds = match resolve_credentials(flag_app_id, flag_api_key) {
        Some(c) => {
            let env = if is_sandbox(&c.app_id) {
                "sandbox"
            } else {
                "production"
            };
            println!("Credentials ......... OK ({}, via {})", env, c.source);
            c
        }
        None => {
            println!("Credentials ......... MISSING (run 'sc auth setup')");
            println!("API connectivity .... SKIPPED (no credentials)");
            println!("Quota remaining ..... SKIPPED");
            print_version().await;
            return;
        }
    };

    let client = SoundchartsClient::new(&creds.app_id, &creds.api_key);
    let start = Instant::now();
    let response = client
        .get("/api/v2/artist/search/billie%20eilish", &[("limit", "1")])
        .await;
    let elapsed = start.elapsed();
    println!(
        "API connectivity .... OK (response: {}ms)",
        elapsed.as_millis()
    );

    if let Some(quota) = response.quota_remaining {
        let status = if quota < 100 { "LOW" } else { "OK" };
        println!("Quota remaining ..... {} ({})", quota, status);
    } else {
        println!("Quota remaining ..... unknown");
    }

    print_version().await;
}

async fn print_version() {
    let current = env!("SC_VERSION");

    let latest = fetch_latest_version().await;

    match latest {
        Some(tag) if tag == current => {
            println!("CLI version ......... {} (latest)", current);
        }
        Some(tag) => {
            println!(
                "CLI version ......... {} (update available: {})",
                current, tag
            );
            println!();
            println!("  Run 'sc update' to upgrade.");
        }
        None => {
            println!("CLI version ......... {}", current);
        }
    }
}

async fn fetch_latest_version() -> Option<String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;

    let response = client
        .get("https://api.github.com/repos/oneortwo/soundcharts-cli/releases/latest")
        .header("User-Agent", "sc-cli")
        .send()
        .await
        .ok()?;

    let body: serde_json::Value = response.json().await.ok()?;

    body.get("tag_name")
        .and_then(|v| v.as_str())
        .map(|v| v.trim_start_matches('v').to_string())
}
