use crate::client::SoundchartsClient;
use crate::config::{self, is_sandbox, load_config, resolve_credentials};
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
            print_version();
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

    print_version();
}

fn print_version() {
    let current = env!("CARGO_PKG_VERSION");
    println!("CLI version ......... {}", current);
}
