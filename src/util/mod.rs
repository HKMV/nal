use std::env;
use std::path::{Path, PathBuf};

pub mod cmd;
pub mod logs;
pub mod rc4;
pub mod service;

/// app所在目录
pub fn app_dir() -> String {
    if cfg!(target_os = "windows") {
        // env::current_dir()?.to_str().unwrap_or(".").to_string()
        env::current_exe()
            .unwrap_or(PathBuf::new())
            .parent()
            .unwrap_or(Path::new("."))
            .to_str()
            .unwrap_or(".")
            .to_string()
    } else {
        ".".to_string()
    }
}