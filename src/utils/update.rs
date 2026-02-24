use serde::Deserialize;
use std::process::Command;

const GITHUB_API_URL: &str = "https://api.github.com/repos/otomay/sniplias/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMethod {
    Manual,
    Cargo,
    Yay,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub local_version: String,
    pub remote_version: Option<String>,
    pub update_available: bool,
    pub install_method: InstallMethod,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
}

impl UpdateInfo {
    pub fn new() -> Self {
        let install_method = detect_install_method();

        Self {
            local_version: CURRENT_VERSION.to_string(),
            remote_version: None,
            update_available: false,
            install_method,
        }
    }

    pub fn check_update(&mut self) {
        // Get remote version from GitHub API
        if let Ok(remote) = get_remote_version() {
            self.remote_version = Some(remote.clone());
            self.update_available = remote > self.local_version;
        }
    }

    #[allow(dead_code)]
    pub fn version_display(&self) -> String {
        if self.update_available {
            if let Some(ref remote) = self.remote_version {
                format!("{} -> {}", self.local_version, remote)
            } else {
                self.local_version.clone()
            }
        } else {
            format!("{} (latest)", self.local_version)
        }
    }
}

fn get_remote_version() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .args([
            "-s",
            "-H",
            "Accept: application/vnd.github+json",
            GITHUB_API_URL,
        ])
        .output()?;

    if !output.status.success() {
        return Err("Failed to fetch release info".into());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let release: GithubRelease = serde_json::from_str(&json_str)?;

    // Remove 'v' prefix if present (e.g., "v0.3.9" -> "0.3.9")
    Ok(release.tag_name.trim_start_matches('v').to_string())
}

fn detect_install_method() -> InstallMethod {
    // Check if installed via yay (AUR)
    let yay_check = Command::new("pacman").args(["-Q", "sniplias"]).output();

    if let Ok(output) = yay_check {
        if output.status.success() {
            return InstallMethod::Yay;
        }
    }

    // Check if installed via cargo
    // Look for binary in common cargo install locations
    let cargo_paths = [
        std::path::PathBuf::from(".cargo/bin/sniplias"),
        std::path::PathBuf::from(".local/bin/sniplias"),
        std::path::PathBuf::from("/usr/local/bin/sniplias"),
        std::path::PathBuf::from("/usr/bin/sniplias"),
    ];

    for path in &cargo_paths {
        if path.exists() {
            // Try to determine if it was installed via cargo
            // by checking if cargo knows about it
            let cargo_check = Command::new("cargo").args(["install", "--list"]).output();

            if let Ok(output) = cargo_check {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("sniplias") {
                    return InstallMethod::Cargo;
                }
            }
        }
    }

    // Check if it's in PATH but not from package manager
    let which_check = Command::new("which").arg("sniplias").output();

    if let Ok(output) = which_check {
        if output.status.success() {
            // It's installed but not via yay, likely manual
            return InstallMethod::Manual;
        }
    }

    InstallMethod::Unknown
}

impl Default for UpdateInfo {
    fn default() -> Self {
        Self::new()
    }
}
