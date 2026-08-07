use std::path::Path;

use anyhow::{Context, Result, bail};
use tokio::fs;

use super::BuildConfig;
use super::exec;

const LINUX_FILES: &[&str] = &[
    "chrome",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "chrome_crashpad_handler",
    "chrome_sandbox",
    "icudtl.dat",
    "resources.pak",
    "v8_context_snapshot.bin",
    "libEGL.so",
    "libGLESv2.so",
    "libvk_swiftshader.so",
    "vk_swiftshader_icd.json",
    "ANGLE",
    "locales",
];

const MAC_FILES: &[&str] = &[
    "Chromium.app",
    "icudtl.dat",
    "v8_context_snapshot.arm64.bin",
    "v8_context_snapshot.x86_64.bin",
];

pub async fn run(c: &BuildConfig) -> Result<()> {
    let staged = if cfg!(target_os = "macos") {
        MAC_FILES
    } else {
        LINUX_FILES
    };
    let probe = if cfg!(target_os = "macos") {
        c.build_out.join("Chromium.app")
    } else {
        c.build_out.join("chrome")
    };
    if !probe.exists() {
        bail!("{} not found — run compile first", probe.display());
    }
    fs::create_dir_all(&c.dist_dir).await.ok();
    let stage = c.dist_dir.join(format!("cosmium-{}", c.chromium_tag));
    if stage.exists() {
        fs::remove_dir_all(&stage).await.ok();
    }
    fs::create_dir_all(&stage).await?;

    for f in staged {
        let src = c.build_out.join(f);
        if !src.exists() {
            tracing::warn!(file = %f, "missing — skipped");
            continue;
        }
        let dst = stage.join(f);
        if src.is_dir() {
            copy_dir(&src, &dst).await?;
        } else {
            fs::copy(&src, &dst).await?;
        }
    }

    let archive = c
        .dist_dir
        .join(format!("cosmium-{}.tar.zst", c.chromium_tag));
    exec::run(
        &c.dist_dir,
        "tar",
        &[
            "--use-compress-program=zstd",
            "-cf",
            archive.to_str().context("archive utf-8")?,
            "-C",
            c.dist_dir.to_str().context("dist_dir utf-8")?,
            &format!("cosmium-{}", c.chromium_tag),
        ],
        &c.depot_tools,
    )
    .await?;
    tracing::info!(archive = %archive.display(), "packaged");
    Ok(())
}

async fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).await?;
    let mut entries = fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            Box::pin(copy_dir(&path, &target)).await?;
        } else {
            fs::copy(&path, &target).await?;
        }
    }
    Ok(())
}
