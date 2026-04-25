mod exec;
mod pkg;
mod step;

use std::path::PathBuf;

use anyhow::Result;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BuildPhase {
    Prereqs,
    Fetch,
    Checkout,
    Apply,
    Compile,
    Package,
}

impl BuildPhase {
    pub fn all() -> &'static [BuildPhase] {
        &[
            Self::Prereqs,
            Self::Fetch,
            Self::Checkout,
            Self::Apply,
            Self::Compile,
            Self::Package,
        ]
    }
    pub fn from_inclusive(start: BuildPhase) -> Vec<BuildPhase> {
        let all = Self::all();
        let idx = all.iter().position(|p| *p == start).unwrap_or(0);
        all[idx..].to_vec()
    }
}

pub struct BuildConfig {
    pub cosmium_root: PathBuf,
    pub chromium_src: PathBuf,
    pub depot_tools: PathBuf,
    pub build_out: PathBuf,
    pub chromium_tag: String,
    pub args_gn: PathBuf,
    pub patches_dir: PathBuf,
    pub patches_series: PathBuf,
    pub dist_dir: PathBuf,
    pub jobs: Option<u32>,
    pub install_build_deps: bool,
}

pub struct BuildChromium {
    config: BuildConfig,
}

impl BuildChromium {
    pub fn new(config: BuildConfig) -> Self {
        Self { config }
    }

    pub async fn execute(&self, phases: &[BuildPhase]) -> Result<()> {
        for p in phases {
            tracing::info!(phase = ?p, "running phase");
            match p {
                BuildPhase::Prereqs => step::prereqs(&self.config).await?,
                BuildPhase::Fetch => step::fetch(&self.config).await?,
                BuildPhase::Checkout => step::checkout(&self.config).await?,
                BuildPhase::Apply => step::apply_patches(&self.config).await?,
                BuildPhase::Compile => step::compile(&self.config).await?,
                BuildPhase::Package => step::package(&self.config).await?,
            }
            tracing::info!(phase = ?p, "ok");
        }
        Ok(())
    }
}
