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

    #[arg(long, env = "COSMIUM_LLM_API_KEY")]
    pub llm_api_key: Option<String>,

    #[arg(
        long,
        env = "COSMIUM_LLM_BASE_URL",
        default_value = "https://api.deepseek.com"
    )]
    pub llm_base_url: String,

    #[arg(long, env = "COSMIUM_LLM_MODEL", default_value = "deepseek-chat")]
    pub llm_model: String,

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

fn build_llm(
    key: Option<String>,
    base_url: &str,
) -> Option<Arc<dyn crate::domain::llm::LlmClient>> {
    let key = key.filter(|k| !k.trim().is_empty())?;
    let client = OpenRouterClient::new(OpenRouterConfig {
        api_key: key,
        base_url: base_url.trim_end_matches('/').to_owned(),
        referer: None,
        title: None,
    });
    Some(Arc::new(client))
}

fn host_of(url: &str) -> &str {
    url.split("://")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .unwrap_or(url)
}

pub async fn execute(args: ServeArgs, state: &CliState) -> Result<()> {
    let binary = args.binary.unwrap_or_else(|| state.binary.clone());

    let llm = build_llm(
        args.llm_api_key.or(args.deepseek_api_key),
        &args.llm_base_url,
    );
    let llm_host = host_of(&args.llm_base_url);

    let env = config::env::Env::init()?;

    let app_state = AppState {
        profile_repo: Arc::clone(&state.profile_repo),
        binary,
        profiles_dir: env.profiles_dir.clone(),
        api_key: args.api_key,
        llm,
        llm_model: args.llm_model.clone(),
        ui_dir: args.ui_dir.or_else(default_ui_dir),
    };

    println!(
        "  Cosmium server starting on http://localhost:{}",
        args.port
    );
    if app_state.llm.is_some() {
        println!("  AI features: enabled ({} via {llm_host})", args.llm_model);
    }
    println!("  Open your browser to start scraping\n");

    crate::presentation::http::serve(app_state, args.port).await
}
