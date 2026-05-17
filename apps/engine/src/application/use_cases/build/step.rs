use anyhow::{Context, Result};
use tokio::fs;

use super::BuildConfig;
use super::exec;
use super::pkg;

pub async fn prereqs(c: &BuildConfig) -> Result<()> {
    if !c.depot_tools.exists() {
        let parent = c.depot_tools.parent().unwrap_or(&c.cosmium_root);
        fs::create_dir_all(parent).await.ok();
        exec::run(
            parent,
            "git",
            &[
                "clone",
                "https://chromium.googlesource.com/chromium/tools/depot_tools.git",
                c.depot_tools.to_str().context("depot_tools utf-8")?,
            ],
            &c.depot_tools,
        )
        .await?;
    }
    Ok(())
}

pub async fn fetch(c: &BuildConfig) -> Result<()> {
    fs::create_dir_all(&c.cosmium_root).await.ok();
    if !c.chromium_src.exists() {
        exec::run(
            &c.cosmium_root,
            "fetch",
            &["--nohooks", "--no-history", "chromium"],
            &c.depot_tools,
        )
        .await?;
    } else {
        exec::run(
            &c.chromium_src,
            "gclient",
            &["sync", "--nohooks", "--with_branch_heads", "--with_tags"],
            &c.depot_tools,
        )
        .await?;
    }
    if c.install_build_deps {
        exec::run(
            &c.chromium_src,
            "./build/install-build-deps.sh",
            &[
                "--no-prompt",
                "--no-android",
                "--no-chromeos-fonts",
                "--no-arm",
                "--no-nacl",
            ],
            &c.depot_tools,
        )
        .await?;
    }
    Ok(())
}

pub async fn checkout(c: &BuildConfig) -> Result<()> {
    let tag_ref = format!("tags/{}", c.chromium_tag);
    let branch = format!("cosmium-{}", c.chromium_tag);
    let tag_fetch = format!("refs/tags/{0}:refs/tags/{0}", c.chromium_tag);
    exec::run(
        &c.chromium_src,
        "git",
        &["fetch", "--tags", "origin", &tag_fetch],
        &c.depot_tools,
    )
    .await
    .ok();
    exec::run(
        &c.chromium_src,
        "git",
        &["checkout", &tag_ref, "-B", &branch],
        &c.depot_tools,
    )
    .await?;
    exec::run(
        &c.chromium_src,
        "gclient",
        &["sync", "--with_branch_heads", "--with_tags", "--reset"],
        &c.depot_tools,
    )
    .await?;
    exec::run(&c.chromium_src, "gclient", &["runhooks"], &c.depot_tools).await?;
    Ok(())
}

pub async fn apply_patches(c: &BuildConfig) -> Result<()> {
    let series = fs::read_to_string(&c.patches_series)
        .await
        .with_context(|| format!("reading {}", c.patches_series.display()))?;
    for line in series.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let patch = c.patches_dir.join(trimmed);
        let patch_str = patch.to_str().context("patch path utf-8")?;
        exec::run(
            &c.chromium_src,
            "git",
            &["apply", "--3way", "--whitespace=nowarn", patch_str],
            &c.depot_tools,
        )
        .await
        .with_context(|| format!("applying {}", trimmed))?;
        tracing::info!(patch = %trimmed, "applied");
    }
    Ok(())
}

pub async fn compile(c: &BuildConfig) -> Result<()> {
    fs::create_dir_all(&c.build_out).await.ok();
    let args_content = fs::read_to_string(&c.args_gn).await?;
    let inline = args_content
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let build_out = c.build_out.to_str().context("build_out utf-8")?;
    exec::run(
        &c.chromium_src,
        "gn",
        &["gen", build_out, &format!("--args={inline}")],
        &c.depot_tools,
    )
    .await?;
    let mut autoninja_args: Vec<String> = vec!["-C".into(), build_out.into()];
    if let Some(j) = c.jobs {
        autoninja_args.push("-j".into());
        autoninja_args.push(j.to_string());
    }
    autoninja_args.push("chrome".into());
    let autoninja_refs: Vec<&str> = autoninja_args.iter().map(String::as_str).collect();
    exec::run(
        &c.chromium_src,
        "autoninja",
        &autoninja_refs,
        &c.depot_tools,
    )
    .await?;
    Ok(())
}

pub async fn package(c: &BuildConfig) -> Result<()> {
    pkg::run(c).await
}
