use std::path::PathBuf;

use anyhow::Result;

pub struct Env {
    pub profiles_dir: PathBuf,
    pub patches_dir: PathBuf,
    pub build_out: PathBuf,
    pub binary: PathBuf,
}

impl Env {
    pub fn init() -> Result<Self> {
        let _ = dotenvy::dotenv();

        let cosmium_root = std::env::var("COSMIUM_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());

        Ok(Self {
            profiles_dir: std::env::var("COSMIUM_PROFILES_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cosmium_root.join("profiles")),
            patches_dir: std::env::var("COSMIUM_PATCHES_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cosmium_root.join("patches")),
            build_out: std::env::var("COSMIUM_BUILD_OUT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cosmium_root.join("out").join("cosmium")),
            binary: std::env::var("COSMIUM_BINARY")
                .map(PathBuf::from)
                .unwrap_or_else(|_| cosmium_root.join("out").join("cosmium").join("chrome")),
        })
    }
}
