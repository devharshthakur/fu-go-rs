use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeleterError {
    #[error("No write permission in target directory. Try running with sudo/admin privileges. Original error: {0}")]
    WritePermission(std::io::Error),
    #[error("Failed to remove directory. Original error: {0}")]
    RemoveDir(std::io::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn delete_go_installations(path_to_delete: &PathBuf) -> Result<(), DeleterError> {
    // Performing write operation check first
    let test_file = path_to_delete.join("fugors-test-file.tmp");
    fs::write(&test_file, "test").map_err(DeleterError::WritePermission)?;
    fs::remove_file(&test_file).map_err(DeleterError::WritePermission)?;

    // Delete main go directory
    fs::remove_dir_all(path_to_delete).map_err(DeleterError::RemoveDir)?;

    // Also cleanup GVM instalations, if they exist.
    if let Some(home_dir) = home::home_dir() {
        let gvm_path = home_dir.join(".gvm").join("gos");
        if gvm_path.exists() {
            let _ = fs::remove_dir_all(gvm_path);
        }
    }
    Ok(())
}
