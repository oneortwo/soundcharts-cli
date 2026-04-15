use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub app_id: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug)]
pub struct ResolvedCredentials {
    pub app_id: String,
    pub api_key: String,
    pub source: CredentialSource,
}

#[derive(Debug)]
pub enum CredentialSource {
    Flags,
    EnvVars,
    ConfigFile,
}

impl std::fmt::Display for CredentialSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialSource::Flags => write!(f, "CLI flags"),
            CredentialSource::EnvVars => write!(f, "environment variables"),
            CredentialSource::ConfigFile => write!(f, "config file"),
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Could not determine config directory")
        .join("soundcharts")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load_config() -> ConfigFile {
    let path = config_path();
    if !path.exists() {
        return ConfigFile::default();
    }
    let contents = fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&contents).unwrap_or_default()
}

pub fn save_config(config: &ConfigFile) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {e}"))?;
    let contents =
        toml::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {e}"))?;
    fs::write(config_path(), contents).map_err(|e| format!("Failed to write config: {e}"))?;
    Ok(())
}

pub fn resolve_credentials(
    flag_app_id: Option<&str>,
    flag_api_key: Option<&str>,
) -> Option<ResolvedCredentials> {
    if let (Some(app_id), Some(api_key)) = (flag_app_id, flag_api_key) {
        return Some(ResolvedCredentials {
            app_id: app_id.to_string(),
            api_key: api_key.to_string(),
            source: CredentialSource::Flags,
        });
    }

    let env_app_id = std::env::var("SOUNDCHARTS_APP_ID").ok();
    let env_api_key = std::env::var("SOUNDCHARTS_API_KEY").ok();
    if let (Some(app_id), Some(api_key)) = (env_app_id, env_api_key) {
        return Some(ResolvedCredentials {
            app_id,
            api_key,
            source: CredentialSource::EnvVars,
        });
    }

    let config = load_config();
    if let (Some(app_id), Some(api_key)) = (config.auth.app_id, config.auth.api_key) {
        return Some(ResolvedCredentials {
            app_id,
            api_key,
            source: CredentialSource::ConfigFile,
        });
    }

    None
}

pub fn is_sandbox(app_id: &str) -> bool {
    app_id == "soundcharts"
}
