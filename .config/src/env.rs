use std::path::PathBuf;

use anyhow::Result;

pub struct Env {
    pub profiles_dir: PathBuf,
    pub patches_dir: PathBuf,
    pub build_out: PathBuf,
    pub binary: PathBuf,
    pub openrouter: OpenRouterEnv,
}

pub struct OpenRouterEnv {
    pub api_key: Option<String>,
    pub base_url: String,
    pub model: String,
    pub referer: Option<String>,
    pub title: Option<String>,
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
            openrouter: OpenRouterEnv {
                api_key: std::env::var("OPENROUTER_API_KEY").ok(),
                base_url: std::env::var("OPENROUTER_BASE_URL")
                    .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_owned()),
                model: std::env::var("OPENROUTER_MODEL")
                    .unwrap_or_else(|_| "anthropic/claude-sonnet-4.6".to_owned()),
                referer: std::env::var("OPENROUTER_REFERER").ok(),
                title: std::env::var("OPENROUTER_TITLE").ok(),
            },
        })
    }
}
