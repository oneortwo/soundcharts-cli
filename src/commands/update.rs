pub fn run() {
    let current = env!("CARGO_PKG_VERSION");
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
