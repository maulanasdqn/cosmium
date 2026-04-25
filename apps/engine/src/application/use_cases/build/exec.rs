use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use tokio::process::Command;

pub async fn run(
    cwd: &Path,
    program: &str,
    args: &[&str],
    depot_tools: &Path,
) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(cwd);
    cmd.args(args);
    cmd.env("PATH", path_with_depot(depot_tools));
    cmd.env("DEPOT_TOOLS_UPDATE", "1");
    let status = cmd.status().await?;
    if !status.success() {
        bail!(
            "command failed [{}]: {} {}",
            status.code().unwrap_or(-1),
            program,
            args.join(" ")
        );
    }
    Ok(())
}

fn path_with_depot(depot_tools: &Path) -> std::ffi::OsString {
    let sep = if cfg!(windows) { ";" } else { ":" };
    let existing = std::env::var_os("PATH").unwrap_or_default();
    let mut combined = PathBuf::from(depot_tools).into_os_string();
    combined.push(sep);
    combined.push(existing);
    combined
}
