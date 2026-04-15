use std::process::Command;

pub fn run() {
    let current = env!("SC_VERSION");
    eprintln!("Current version: {current}");

    let result = self_update::backends::github::Update::configure()
        .repo_owner("oneortwo")
        .repo_name("soundcharts-cli")
        .bin_name("sc")
        .current_version(current)
        .show_download_progress(true)
        .build();

    match result {
        Ok(updater) => match updater.update() {
            Ok(status) => {
                if status.updated() {
                    eprintln!("Updated to {}!", status.version());
                    install_completions();
                } else {
                    eprintln!("Already up to date ({current}).");
                }
            }
            Err(e) => {
                eprintln!("error: Update failed: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("error: Failed to configure updater: {e}");
            std::process::exit(1);
        }
    }
}

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
