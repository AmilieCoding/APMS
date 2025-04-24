use std::process::Command;
use std::os::unix::process::CommandExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub enum PrivilegeError {
    NotRoot,
    SudoRequired,
    SystemError(String),
}

impl std::fmt::Display for PrivilegeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivilegeError::NotRoot => write!(f, "This operation requires root privileges. Please run with sudo."),
            PrivilegeError::SudoRequired => write!(f, "Permission denied. Please run with sudo."),
            PrivilegeError::SystemError(e) => write!(f, "System error: {}", e),
        }
    }
}

pub struct PermissionChecker;

impl PermissionChecker {
    pub fn is_root() -> bool {
        unsafe { libc::geteuid() == 0 }
    }

    pub fn restart_with_sudo() -> ! {
        if !Self::is_root() {
            let args: Vec<String> = std::env::args().collect();
            let error = Command::new("sudo")
                .args(&args)
                .exec();

            eprintln!("Failed to execute sudo: {}", error);
            std::process::exit(1);
        }
        std::process::exit(1);
    }

    pub fn is_sudo() -> bool {
        std::env::var("SUDO_USER").is_ok()
    }

    pub fn ensure_root() -> Result<(), PrivilegeError> {
        if !Self::is_root() {
            return Err(PrivilegeError::NotRoot);
        }
        Ok(())
    }

    pub fn can_write_to_path(path: &Path) -> bool {
        if let Ok(metadata) = path.metadata() {
            if Self::is_root() {
                return true;
            }
            
            if metadata.uid() == unsafe { libc::geteuid() } {
                return metadata.mode() & 0o200 != 0;
            }
            
            if metadata.gid() == unsafe { libc::getegid() } {
                return metadata.mode() & 0o020 != 0;
            }

            return metadata.mode() & 0o002 != 0;
        }
        false
    }
}