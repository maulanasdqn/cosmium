use anyhow::Result;
use config::logger;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    engine::cli::run().await
}
