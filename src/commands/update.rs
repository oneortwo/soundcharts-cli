use std::path::Path;

#[cfg(unix)]
use std::process::Command;

const REPO: &str = "oneortwo/soundcharts-cli";

#[cfg(windows)]
const ARCHIVE_EXT: &str = "zip";
#[cfg(unix)]
const ARCHIVE_EXT: &str = "tar.gz";

fn binary_name() -> &'static str {
    if cfg!(windows) { "sc.exe" } else { "sc" }
}

pub async fn run() {
    let current = env!("SC_VERSION");
    eprintln!("Current version: {current}");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    // Fetch latest release
    eprint!("Checking latest release... ");
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let response = match client.get(&url).header("User-Agent", "sc-cli").send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed");
            eprintln!("error: Could not reach GitHub: {e}");
            std::process::exit(1);
        }
    };

    let body: serde_json::Value = match response.json().await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("failed");
            eprintln!("error: Invalid response from GitHub: {e}");
            std::process::exit(1);
        }
    };

    let latest_tag = body.get("tag_name").and_then(|v| v.as_str()).unwrap_or("");
    let latest = latest_tag.trim_start_matches('v');

    if latest == current {
        eprintln!("already up to date ({current}).");
        return;
    }

    eprintln!("{latest}");

    // Find the right asset
    let target = detect_target();
    let asset_name = format!("sc-{target}.{ARCHIVE_EXT}");

    let download_url = body
        .get("assets")
        .and_then(|a| a.as_array())
        .and_then(|assets| {
            assets.iter().find_map(|a| {
                let name = a.get("name").and_then(|n| n.as_str()).unwrap_or("");
                if name == asset_name {
                    a.get("browser_download_url")
                        .and_then(|u| u.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                }
            })
        });

    let download_url = match download_url {
        Some(u) => u,
        None => {
            eprintln!("error: No binary found for {target}. Available assets:");
            if let Some(assets) = body.get("assets").and_then(|a| a.as_array()) {
                for a in assets {
                    if let Some(name) = a.get("name").and_then(|n| n.as_str()) {
                        eprintln!("  - {name}");
                    }
                }
            }
            std::process::exit(1);
        }
    };

    // Download
    eprintln!("Downloading sc {latest} for {target}...");
    let bytes = match client.get(&download_url).send().await {
        Ok(r) => match r.bytes().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("error: Download failed: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("error: Download failed: {e}");
            std::process::exit(1);
        }
    };

    // Extract and replace binary
    let current_exe = std::env::current_exe().expect("Could not determine current executable path");
    let tmp_dir = std::env::temp_dir().join("sc-update");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).expect("Failed to create temp dir");

    let archive_path = tmp_dir.join(format!("sc.{ARCHIVE_EXT}"));
    std::fs::write(&archive_path, &bytes).expect("Failed to write archive");

    if let Err(e) = extract_archive(&archive_path, &tmp_dir) {
        eprintln!("error: Failed to extract update: {e}");
        std::process::exit(1);
    }

    let new_binary = tmp_dir.join(binary_name());
    if !new_binary.exists() {
        eprintln!(
            "error: Extracted archive does not contain '{}' binary",
            binary_name()
        );
        std::process::exit(1);
    }

    // Replace current binary
    let backup = current_exe.with_extension("old");
    let _ = std::fs::remove_file(&backup);
    if let Err(e) = std::fs::rename(&current_exe, &backup) {
        eprintln!("error: Could not back up current binary: {e}");
        std::process::exit(1);
    }
    if let Err(e) = std::fs::rename(&new_binary, &current_exe) {
        // Try to restore backup
        let _ = std::fs::rename(&backup, &current_exe);
        eprintln!("error: Could not install new binary: {e}");
        std::process::exit(1);
    }
    let _ = std::fs::remove_file(&backup);
    let _ = std::fs::remove_dir_all(&tmp_dir);

    eprintln!("Updated to {latest}!");

    if let Some(notes) = body.get("body").and_then(|b| b.as_str()) {
        let notes = notes.trim();
        if !notes.is_empty() && !notes.starts_with("**Full Changelog**") {
            eprintln!();
            eprintln!("What's new:");
            eprintln!("{notes}");
        }
    }

    install_completions();
}

fn detect_target() -> &'static str {
    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "aarch64-apple-darwin"
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        "x86_64-apple-darwin"
    } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
        "x86_64-pc-windows-msvc"
    } else if cfg!(target_os = "windows") && cfg!(target_arch = "aarch64") {
        "aarch64-pc-windows-msvc"
    } else {
        eprintln!("error: Unsupported platform for self-update");
        std::process::exit(1);
    }
}

#[cfg(unix)]
fn extract_archive(archive_path: &Path, dest: &Path) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(dest).map_err(|e| e.to_string())
}

#[cfg(windows)]
fn extract_archive(archive_path: &Path, dest: &Path) -> Result<(), String> {
    let file = std::fs::File::open(archive_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    archive.extract(dest).map_err(|e| e.to_string())
}

#[cfg(windows)]
fn install_completions() {
    // Shell completions target bash/zsh/fish, none of which are standard on Windows.
    // PowerShell users can generate completions with `sc completions powershell`.
}

#[cfg(unix)]
fn install_completions() {
    let sc = std::env::current_exe().unwrap_or_default();

    if let Some(fish_dir) = dirs::config_dir().map(|d| d.join("fish/completions")) {
        if fish_dir.parent().map(|p| p.exists()).unwrap_or(false) {
            let _ = std::fs::create_dir_all(&fish_dir);
            if let Ok(output) = Command::new(&sc).args(["completions", "fish"]).output() {
                if output.status.success() {
                    let _ = std::fs::write(fish_dir.join("sc.fish"), output.stdout);
                    eprintln!("Fish completions updated.");
                }
            }
        }
    }

    let bash_dir = dirs::data_dir()
        .or_else(dirs::home_dir)
        .map(|d| d.join(".local/share/bash-completion/completions"));
    if let Some(dir) = bash_dir {
        if dir.parent().map(|p| p.exists()).unwrap_or(false) {
            let _ = std::fs::create_dir_all(&dir);
            if let Ok(output) = Command::new(&sc).args(["completions", "bash"]).output() {
                if output.status.success() {
                    let _ = std::fs::write(dir.join("sc"), output.stdout);
                    eprintln!("Bash completions updated.");
                }
            }
        }
    }

    if let Some(home) = dirs::home_dir() {
        let zsh_dir = home.join(".zfunc");
        if zsh_dir.exists() {
            if let Ok(output) = Command::new(&sc).args(["completions", "zsh"]).output() {
                if output.status.success() {
                    let _ = std::fs::write(zsh_dir.join("_sc"), output.stdout);
                    eprintln!("Zsh completions updated.");
                }
            }
        }
    }
}
