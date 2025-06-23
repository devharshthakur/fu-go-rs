use std::env;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("Could not determine user home directory")]
    HomeDir,
    #[error("I/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No Go installation found")]
    NotFound,
}

/// Represents a found go installation
pub struct GoInstallation {
    pub versions: Vec<String>,
    pub path_to_delete: PathBuf,
}

pub fn find_go_installations() -> Result<GoInstallation, FinderError> {
    let mut versions = Vec::new();
    let mut search_paths = Vec::new();

    // --- Platform-specific path detection --- //

    #[cfg(target_os = "windows")]
    {
        if let Ok(user_profile) = env::var("USERPROFILE") {
            search_paths.push(PathBuf::from(&user_profile).join("go"));
        }

        if let Ok(program_files) = env::var("ProgramFiles") {
            search_paths.push(PathBuf::from(&program_files).join("Go"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        search_paths.push(PathBuf::from("/usr/local/go"));
        search_paths.push(PathBuf::from("/usr/local/Cellar/go"));
    }

    let go_path = search_paths.into_iter().find(|p| p.exists() && p.is_dir());
    let path_to_delete = match go_path {
        Some(path) => path,
        None => return Err(FinderError::NotFound),
    };

    if path_to_delete.exists() {
        if let Ok(output) = Command::new("go").arg("version").output() {
            if output.status.success() {
                let go_version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                versions.push(go_version_str);
            }
        }
    }
    //  ------ Check for GVM (Go Version Manager) installations ------ //
    if let Some(home_dir) = home::home_dir() {
        //home crate api is not intended for external use
        let gvm_path = home_dir.join(".gvm").join("gos");
        if gvm_path.exists() {
            if let Ok(entries) = fs::read_dir(gvm_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(dir_name) = entry.file_name().to_str() {
                            if dir_name.starts_with("go") {
                                versions.push(format!("go {}", dir_name));
                            }
                        }
                    }
                }
            }
        }
    }

    if versions.is_empty() {
        Err(FinderError::NotFound)
    } else {
        Ok(GoInstallation {
            versions,
            path_to_delete,
        })
    }
}
