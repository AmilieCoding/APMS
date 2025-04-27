use std::fs;
use std::path::PathBuf;
use crate::utils::permissions::PermissionChecker;

pub fn delete(package_name: &str) {
    if !PermissionChecker::is_root() {
        println!("[INFO] APMS package deletion requires root privileges");
        println!("[INFO] Restarting with sudo...");
        PermissionChecker::restart_with_sudo();
    }

    println!("[INFO] Deleting package: {}", package_name);

    let install_dir = PathBuf::from("/usr/local/lib/apms/packages").join(package_name);

    let symlink_path = PathBuf::from("/usr/local/bin").join(package_name);

    if !install_dir.exists() {
        println!("[ERROR] Package '{}' is not installed", package_name);
        return;
    }

    if symlink_path.exists() {
        println!("[INFO] Removing symlink: {}", symlink_path.display());
        if let Err(e) = fs::remove_file(&symlink_path) {
            println!("[ERROR] Failed to remove symlink: {}", e);
            return;
        }
    }

    println!("[INFO] Removing package files from: {}", install_dir.display());
    if let Err(e) = fs::remove_dir_all(&install_dir) {
        println!("[ERROR] Failed to remove package directory: {}", e);
        return;
    }
    
    println!("[INFO] Successfully deleted package: {}", package_name);
}
