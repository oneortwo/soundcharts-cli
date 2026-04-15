use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::process;

const BASE_URL: &str = "https://customer.api.soundcharts.com";

pub struct SoundchartsClient {
    client: Client,
    base_url: String,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub body: Value,
    pub quota_remaining: Option<u64>,
}

impl SoundchartsClient {
    pub fn new(app_id: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-app-id",
            HeaderValue::from_str(app_id).expect("Invalid app_id"),
        );
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(api_key).expect("Invalid api_key"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn get(&self, path: &str, params: &[(&str, &str)]) -> ApiResponse {
        let url = format!("{}{}", self.base_url, path);

        let response = match self.client.get(&url).query(params).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error: Failed to connect to Soundcharts API: {e}");
                process::exit(1);
            }
        };

        let quota_remaining = response
            .headers()
            .get("x-quota-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            eprintln!("error: Invalid credentials. Run 'sc auth setup' to configure.");
            process::exit(2);
        }

        if status == reqwest::StatusCode::NOT_FOUND {
            eprintln!("error: Resource not found.");
            process::exit(3);
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            eprintln!("error: API quota exceeded. Check your plan at soundcharts.com.");
            process::exit(4);
        }

        if !status.is_success() {
            eprintln!("error: API returned {status}");
            process::exit(1);
        }

        let body: Value = match response.json().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("error: Failed to parse API response: {e}");
                process::exit(1);
            }
        };

        if let Some(errors) = body.get("errors").and_then(|e| e.as_array()) {
            if !errors.is_empty() {
                let msg = errors[0]
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown API error");
                let code = errors[0].get("code").and_then(|c| c.as_u64()).unwrap_or(0);
                if code == 404 {
                    eprintln!("error: Resource not found.");
                    process::exit(3);
                }
                if code == 403 {
                    eprintln!("error: Forbidden: {msg}");
                    process::exit(1);
                }
                eprintln!("error: API error: {msg}");
                process::exit(1);
            }
        }

        ApiResponse {
            body,
            quota_remaining,
        }
    }
}
