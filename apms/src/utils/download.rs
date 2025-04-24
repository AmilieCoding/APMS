use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::io::Write;
use reqwest::blocking::{Client, Response};
use crate::utils::mirrors::MirrorList;
use crate::utils::permissions::PermissionChecker;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub download_url: String,
}

pub struct PackageDownloader {
    client: Client,
    packages_dir: PathBuf,
    mirrors: MirrorList,
}

impl PackageDownloader {
    pub fn new() -> Result<Self, String> {
        // Check if we have necessary permissions for system-wide installation
        if !PermissionChecker::is_root() {
            return Err("Root privileges required. Please run with sudo.".to_string());
        }

        let home_dir = dirs::home_dir()
            .ok_or("Could not find home directory")?;
        let apms_dir = home_dir.join(".apms");
        let packages_dir = apms_dir.join("packages");
        
        fs::create_dir_all(&packages_dir)
            .map_err(|e| format!("Failed to create APMS directories: {}", e))?;

        let mirrors = MirrorList::load()?;

        Ok(Self {
            client: Client::new(),
            packages_dir,
            mirrors,
        })
    }

    pub fn fetch_package_info(&self, package_name: &str) -> Result<Package, String> {
        let mirrors = self.mirrors.get_mirrors();
        if mirrors.is_empty() {
            return Err("No enabled mirrors found".to_string());
        }

        let mut last_error = String::from("All mirrors failed");

        for mirror in mirrors {
            let repo_url = format!("{}/packages/{}.json", mirror.url, package_name);
            
            match self.client.get(&repo_url).send() {
                Ok(response) if response.status().is_success() => {
                    return response.json::<Package>()
                        .map_err(|e| format!("Failed to parse package information: {}", e));
                }
                Ok(_) => {
                    last_error = format!("Package not found on mirror: {}", mirror.name);
                    continue;
                }
                Err(e) => {
                    last_error = format!("Mirror {} failed: {}", mirror.name, e);
                    continue;
                }
            }
        }

        Err(last_error)
    }

    pub fn download_package(&self, package_info: &Package) -> Result<PathBuf, String> {
        let package_path = self.packages_dir.join(&package_info.name);
        
        fs::create_dir_all(&package_path)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        let mut last_error = String::from("All mirrors failed");
        
        for mirror in self.mirrors.get_mirrors() {
            let download_url = if package_info.download_url.starts_with("http") {
                package_info.download_url.clone()
            } else {
                format!("{}/{}", mirror.url, package_info.download_url)
            };

            match self.client.get(&download_url).send() {
                Ok(response) if response.status().is_success() => {
                    return self.save_package_file(response, package_path, package_info);
                }
                Ok(_) => {
                    last_error = format!("Package not found on mirror: {}", mirror.name);
                    continue;
                }
                Err(e) => {
                    last_error = format!("Mirror {} failed: {}", mirror.name, e);
                    continue;
                }
            }
        }

        Err(last_error)
    }

    fn save_package_file(
        &self,
        response: Response,
        package_path: PathBuf,
        package_info: &Package
    ) -> Result<PathBuf, String> {
        let package_file = package_path.join(
            format!("{}-{}.tar.gz", package_info.name, package_info.version)
        );
        
        let mut file = fs::File::create(&package_file)
            .map_err(|e| format!("Failed to create package file: {}", e))?;
        
        file.write_all(&response.bytes()
            .map_err(|e| format!("Failed to read download: {}", e))?)
            .map_err(|e| format!("Failed to write package file: {}", e))?;

        Ok(package_file)
    }
}