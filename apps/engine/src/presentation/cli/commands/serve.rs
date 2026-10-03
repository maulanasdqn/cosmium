use std::sync::Arc;

use anyhow::Result;
use clap::Args;

use crate::infrastructure::llm::OpenRouterClient;
use crate::infrastructure::llm::openrouter::OpenRouterConfig;
use crate::presentation::cli::state::CliState;
use crate::presentation::http::AppState;

#[derive(Debug, Args)]
pub struct ServeArgs {
    #[arg(long, default_value = "3000", env = "COSMIUM_PORT")]
    pub port: u16,

    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<std::path::PathBuf>,

    #[arg(long, default_value = "dev-key", env = "COSMIUM_API_KEY")]
    pub api_key: String,

    #[arg(long, env = "DEEPSEEK_API_KEY")]
    pub deepseek_api_key: Option<String>,

    #[arg(long, env = "COSMIUM_UI_DIR")]
    pub ui_dir: Option<std::path::PathBuf>,
}

fn default_ui_dir() -> Option<std::path::PathBuf> {
    let root = std::env::var_os("COSMIUM_ROOT")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::current_dir().ok())?;
    Some(root.join("apps").join("ui").join("dist"))
}

pub async fn execute(args: ServeArgs, state: &CliState) -> Result<()> {
    let binary = args.binary.unwrap_or_else(|| state.binary.clone());

    let llm = args
        .deepseek_api_key
        .filter(|k| !k.trim().is_empty())
        .map(|key| {
            let client = OpenRouterClient::new(OpenRouterConfig {
                api_key: key,
                base_url: "https://api.deepseek.com".into(),
                referer: None,
                title: None,
            });
            Arc::new(client) as Arc<dyn crate::domain::llm::LlmClient>
        });

    let env = config::env::Env::init()?;

    let app_state = AppState {
        profile_repo: Arc::clone(&state.profile_repo),
        binary,
        profiles_dir: env.profiles_dir.clone(),
        api_key: args.api_key,
        llm,
        llm_model: "deepseek-chat".into(),
        ui_dir: args.ui_dir.or_else(default_ui_dir),
    };

    println!(
        "  Cosmium server starting on http://localhost:{}",
        args.port
    );
    if app_state.llm.is_some() {
        println!("  AI profile generation: enabled (DeepSeek)");
    }
    println!("  Open your browser to start scraping\n");

    crate::presentation::http::serve(app_state, args.port).await
}
