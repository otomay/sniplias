use serde::Deserialize;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_OWNER: &str = "otomay";
const REPO_NAME: &str = "sniplias";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMethod {
    Manual,
    Cargo,
    Pacman,
    Unknown,
}

#[derive(Debug)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub latest_version: String,
    pub current_version: String,
    pub install_method: InstallMethod,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub fn get_current_version() -> &'static str {
    CURRENT_VERSION
}

pub fn detect_install_method() -> InstallMethod {
    // Get the path of the current executable
    if let Ok(path) = std::env::current_exe() {
        let path_str = path.to_string_lossy().to_lowercase();

        // Check if installed via cargo (in ~/.cargo/bin or target/)
        if path_str.contains(".cargo")
            || path_str.contains("target/release") && !path_str.contains("deps")
        {
            return InstallMethod::Cargo;
        }

        // Check if installed via pacman (Arch Linux AUR)
        if path_str.starts_with("/usr/bin/") || path_str.starts_with("/usr/local/bin/") {
            // Could be manual or pacman - check if we can detect pacman
            if std::path::Path::new("/var/lib/pacman").exists() {
                // Try to check if the binary is tracked by pacman
                let output = std::process::Command::new("pacman")
                    .args(["-Qo", &path.to_string_lossy()])
                    .output();

                if let Ok(output) = output {
                    if output.status.success() {
                        return InstallMethod::Pacman;
                    }
                }
            }

            // If we can't confirm pacman, assume manual (install.sh)
            return InstallMethod::Manual;
        }

        // Check for ~/.local/bin (common for manual install)
        if path_str.contains(".local/bin") {
            return InstallMethod::Manual;
        }
    }

    // Try to detect via environment
    // If CARGO_HOME is set, likely installed via cargo
    if std::env::var("CARGO_HOME").is_ok() {
        return InstallMethod::Cargo;
    }

    // Default to unknown
    InstallMethod::Unknown
}

pub fn check_for_update() -> Option<UpdateInfo> {
    let current_version = get_current_version();

    // Fetch latest release from GitHub
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("sniplias-update-checker")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let response = client.get(&url).send().ok()?;

    if !response.status().is_success() {
        return None;
    }

    let release: GitHubRelease = response.json().ok()?;

    let latest_version = release.tag_name.trim_start_matches('v');
    let has_update = latest_version != current_version;

    Some(UpdateInfo {
        has_update,
        latest_version: latest_version.to_string(),
        current_version: current_version.to_string(),
        install_method: detect_install_method(),
    })
}

#[allow(dead_code)]
pub fn get_update_command(install_method: InstallMethod) -> Option<&'static str> {
    match install_method {
        InstallMethod::Manual => Some("curl -sL https://raw.githubusercontent.com/otomay/sniplias/master/scripts/install.sh | sh"),
        InstallMethod::Cargo => Some("cargo install sniplias"),
        InstallMethod::Pacman => Some("yay -S sniplias (or your AUR helper)"),
        InstallMethod::Unknown => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version() {
        let version = get_current_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_install_method_detection() {
        let method = detect_install_method();
        assert!(matches!(
            method,
            InstallMethod::Manual
                | InstallMethod::Cargo
                | InstallMethod::Pacman
                | InstallMethod::Unknown
        ));
    }
}
