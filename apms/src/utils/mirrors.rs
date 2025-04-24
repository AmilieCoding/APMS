use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;
use crate::utils::permissions::PermissionChecker;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mirror {
    pub name: String,
    pub url: String,
    pub priority: u8,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorList {
    pub mirrors: Vec<Mirror>,
}

impl MirrorList {
    pub fn load() -> Result<Self, String> {
        let user_config = dirs::config_dir()
            .map(|p| p.join("apms/mirrors.conf"));
        
        let system_config = PathBuf::from("/etc/apms/mirrors.conf");

        if let Some(user_path) = user_config {
            if user_path.exists() {
                return Self::load_from_file(&user_path);
            }
        }

        if system_config.exists() {
            return Self::load_from_file(&system_config);
        }

        Ok(Self::default())
    }

    fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read mirror config: {}", e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse mirror config: {}", e))
    }

    pub fn get_mirrors(&self) -> Vec<Mirror> {
        let mut mirrors = self.mirrors.clone();
        mirrors.sort_by(|a, b| b.priority.cmp(&a.priority));
        mirrors.into_iter().filter(|m| m.enabled).collect()
    }

    pub fn save_system_config(&self) -> Result<(), String> {
        if !PermissionChecker::is_root() {
            return Err("Root privileges required to modify system configuration.".to_string());
        }

        let system_config = PathBuf::from("/etc/apms/mirrors.conf");
        let contents = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize mirror config: {}", e))?;

        fs::write(&system_config, contents)
            .map_err(|e| format!("Failed to write system mirror config: {}", e))
    }
}

impl Default for MirrorList {
    fn default() -> Self {
        Self {
            mirrors: vec![
                Mirror {
                    name: String::from("Mirror @ Local"),
                    url: String::from("http://localhost:8080"),
                    priority: 100,
                    enabled: true,
                }
            ]
        }
    }
}

pub fn ensure_config_dirs() -> Result<(), IoError> {
    let system_dir = PathBuf::from("/etc/apms");
    if !system_dir.exists() {
        fs::create_dir_all(&system_dir).ok();
    }

    if let Some(user_config) = dirs::config_dir() {
        let user_dir = user_config.join("apms");
        fs::create_dir_all(user_dir)?;
    }

    Ok(())
}