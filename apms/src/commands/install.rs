use std::fs;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::utils::download::PackageDownloader;
use crate::utils::permissions::PermissionChecker;

pub fn install(package_name: &str) {
    if !PermissionChecker::is_root() {
        println!("[INFO] APMS package installation requires root privileges");
        println!("[INFO] Restarting with sudo...");
        PermissionChecker::restart_with_sudo();
    }

    println!("[INFO] Searching for package: {}", package_name);
    
    let downloader = match PackageDownloader::new() {
        Ok(d) => d,
        Err(e) => {
            println!("[ERROR] Failed to initialize package downloader: {}", e);
            return;
        }
    };

    println!("[INFO] Fetching package information...");
    let package_info = match downloader.fetch_package_info(package_name) {
        Ok(info) => info,
        Err(e) => {
            println!("[ERROR] {}", e);
            return;
        }
    };

    println!("[INFO] Downloading {} version {}...", package_info.name, package_info.version);
    let package_file = match downloader.download_package(&package_info) {
        Ok(file) => file,
        Err(e) => {
            println!("[ERROR] {}", e);
            return;
        }
    };

    println!("[INFO] Installing {}...", package_info.name);
    if let Err(e) = install_package(&package_file, &package_info.name) {
        println!("[ERROR] Installation failed: {}", e);
        if let Err(cleanup_err) = cleanup_failed_install(&package_file) {
            println!("[WARNING] Cleanup failed: {}", cleanup_err);
        }
        return;
    }

    println!("[INFO] Successfully installed {}", package_info.name);
}

fn install_package(package_file: &PathBuf, package_name: &str) -> Result<(), String> {
    let install_dir = PathBuf::from("/usr/local/lib/apms/packages").join(package_name);
    fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Failed to create installation directory: {}", e))?;

    let tar_gz = fs::File::open(package_file)
        .map_err(|e| format!("Failed to open package file: {}", e))?;
    
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    
    archive.unpack(&install_dir)
        .map_err(|e| format!("Failed to extract package: {}", e))?;

    let neofetch_path = install_dir.join("neofetch-7.1.0").join("neofetch");
    let target_path = PathBuf::from("/usr/local/bin/neofetch");

    fs::create_dir_all("/usr/local/bin")
        .map_err(|e| format!("Failed to create /usr/local/bin: {}", e))?;

    if target_path.exists() {
        fs::remove_file(&target_path)
            .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
    }

    println!("[INFO] Creating symlink: {} -> {}", target_path.display(), neofetch_path.display());
    std::os::unix::fs::symlink(&neofetch_path, &target_path)
        .map_err(|e| format!("Failed to create symlink: {}", e))?;

    Ok(())
}

fn create_symlinks(bin_dir: &PathBuf) -> Result<(), String> {
    println!("[DEBUG] Creating symlinks from: {}", bin_dir.display());
    let target_dir = PathBuf::from("/usr/local/bin");

    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;
    
    for entry in fs::read_dir(bin_dir)
        .map_err(|e| format!("Failed to read bin directory: {}", e))? 
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        println!("[DEBUG] Processing file: {}", path.display());
        
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let target_path = target_dir.join(file_name);
                println!("[DEBUG] Creating symlink: {} -> {}", target_path.display(), path.display());

                if target_path.exists() {
                    println!("[DEBUG] Removing existing file: {}", target_path.display());
                    fs::remove_file(&target_path)
                        .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
                }

                std::os::unix::fs::symlink(&path, &target_path)
                    .map_err(|e| format!("Failed to create symlink: {}", e))?;
                
                println!("[INFO] Created symlink for {}", file_name.to_string_lossy());
            }
        }
    }
    
    Ok(())
}

fn cleanup_failed_install(package_file: &PathBuf) -> Result<(), String> {
    if let Err(e) = fs::remove_file(package_file) {
        println!("[WARNING] Failed to remove package file: {}", e);
    }

    let package_name = package_file
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Failed to get package name from file path")?;
        
    let install_dir = PathBuf::from("/usr/local/lib/apms/packages").join(package_name);
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir)
            .map_err(|e| format!("Failed to remove installation directory: {}", e))?;
    }

    Ok(())
}