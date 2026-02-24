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

        // First check if installed in system paths (not a dev build)
        // These indicate either Manual or Pacman install
        if path_str.starts_with("/usr/bin/") || path_str.starts_with("/usr/local/bin/") {
            // Check if tracked by pacman
            if std::path::Path::new("/var/lib/pacman").exists() {
                let output = std::process::Command::new("pacman")
                    .args(["-Qo", &path.to_string_lossy()])
                    .output();

                if let Ok(output) = output {
                    if output.status.success() {
                        return InstallMethod::Pacman;
                    }
                }
            }

            // If in system path but not pacman, it's Manual
            return InstallMethod::Manual;
        }

        // Check for ~/.local/bin (common for manual install)
        if path_str.contains(".local/bin") {
            return InstallMethod::Manual;
        }

        // Now check for cargo - only if in ~/.cargo/bin explicitly
        // We check this AFTER system paths to avoid false positives from dev builds
        if path_str.contains(".cargo/bin") {
            return InstallMethod::Cargo;
        }

        // target/release without deps is likely cargo install but could be dev build
        // Only mark as cargo if we're sure it's not a dev build
        // Check if CARGO_MANIFEST_DIR is set (dev build indicator)
        if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
            // We're in a dev build, don't assume cargo install
            return InstallMethod::Unknown;
        }

        // If in target/release and no other indicators, could be cargo install
        if path_str.contains("target/release") && !path_str.contains("deps") {
            return InstallMethod::Cargo;
        }
    }

    // Try to detect via environment
    if std::env::var("CARGO_HOME").is_ok() && std::env::var("CARGO_MANIFEST_DIR").is_err() {
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
