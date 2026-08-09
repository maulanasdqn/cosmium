use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use config::env::Env;

use crate::application::use_cases::build::{BuildChromium, BuildConfig, BuildPhase};

#[derive(Debug, Args)]
pub struct BuildCmd {
    #[arg(long)]
    pub from: Option<BuildPhase>,
    #[arg(long, conflicts_with = "from")]
    pub only: Option<BuildPhase>,
    #[arg(long)]
    pub jobs: Option<u32>,
    #[arg(long)]
    pub install_build_deps: bool,
    #[arg(long)]
    pub tag: Option<String>,
}

pub async fn execute(cmd: BuildCmd) -> Result<()> {
    let env = Env::init()?;
    let cosmium_root = std::env::var("COSMIUM_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let chromium_src = cosmium_root.join("src");
    let depot_tools = cosmium_root.join("depot_tools");
    let dist_dir = cosmium_root.join("dist");
    let args_gn = select_args_gn(&cosmium_root);
    let patches_dir = cosmium_root.join("patches");
    let patches_series = patches_dir.join("series");

    let chromium_tag = cmd.tag.map(Ok).unwrap_or_else(|| {
        std::fs::read_to_string(cosmium_root.join("VERSION"))
            .map(|s| s.trim().to_owned())
            .context("reading VERSION")
    })?;

    let phases: Vec<BuildPhase> = match (cmd.only, cmd.from) {
        (Some(p), _) => vec![p],
        (None, Some(start)) => BuildPhase::from_inclusive(start),
        (None, None) => BuildPhase::all().to_vec(),
    };

    tracing::info!(args_gn = %args_gn.display(), "selected gn config");
    let config = BuildConfig {
        cosmium_root,
        chromium_src,
        depot_tools,
        build_out: env.build_out.clone(),
        chromium_tag,
        args_gn,
        patches_dir,
        patches_series,
        dist_dir,
        jobs: cmd.jobs,
        install_build_deps: cmd.install_build_deps,
    };
    BuildChromium::new(config).execute(&phases).await
}

fn select_args_gn(root: &Path) -> PathBuf {
    let candidate = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => ".config/args.mac-arm64.gn",
        _ => ".config/args.gn",
    };
    root.join(candidate)
}
