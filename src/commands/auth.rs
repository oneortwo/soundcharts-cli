use crate::client::SoundchartsClient;
use crate::config::{self, is_sandbox, resolve_credentials, save_config, AuthConfig, ConfigFile};
use console::Term;
use dialoguer::{Input, Password};

pub async fn setup(no_input: bool) {
    let (app_id, api_key) = if no_input || !Term::stdout().is_term() {
        let app_id = std::env::var("SOUNDCHARTS_APP_ID").unwrap_or_else(|_| {
            eprintln!(
                "error: SOUNDCHARTS_APP_ID not set. Use --no-input with env vars or run interactively."
            );
            std::process::exit(2);
        });
        let api_key = std::env::var("SOUNDCHARTS_API_KEY").unwrap_or_else(|_| {
            eprintln!(
                "error: SOUNDCHARTS_API_KEY not set. Use --no-input with env vars or run interactively."
            );
            std::process::exit(2);
        });
        (app_id, api_key)
    } else {
        eprintln!("Soundcharts API Setup\n");
        let app_id: String = Input::new()
            .with_prompt("App ID")
            .interact_text()
            .expect("Failed to read App ID");
        let api_key: String = Password::new()
            .with_prompt("API Key")
            .interact()
            .expect("Failed to read API Key");
        (app_id, api_key)
    };

    eprint!("Verifying credentials... ");

    let client = SoundchartsClient::new(&app_id, &api_key);
    let response = client
        .get("/api/v2/artist/search/test", &[("limit", "1")])
        .await;

    if let Some(quota) = response.quota_remaining {
        eprintln!("OK (quota remaining: {})", quota);
    } else {
        eprintln!("OK");
    }

    let config = ConfigFile {
        auth: AuthConfig {
            app_id: Some(app_id),
            api_key: Some(api_key),
        },
    };

    match save_config(&config) {
        Ok(()) => eprintln!("Credentials saved to {}", config::config_path().display()),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn status(flag_app_id: Option<&str>, flag_api_key: Option<&str>) {
    let creds = match resolve_credentials(flag_app_id, flag_api_key) {
        Some(c) => c,
        None => {
            eprintln!("error: No credentials configured. Run 'sc auth setup'.");
            std::process::exit(2);
        }
    };

    let env = if is_sandbox(&creds.app_id) {
        "sandbox"
    } else {
        "production"
    };

    println!("Source:      {}", creds.source);
    println!("Environment: {}", env);
    println!("Config file: {}", config::config_path().display());

    let client = SoundchartsClient::new(&creds.app_id, &creds.api_key);
    let response = client
        .get("/api/v2/artist/search/test", &[("limit", "1")])
        .await;

    if let Some(quota) = response.quota_remaining {
        println!("Quota:       {}", quota);
    }
}
