use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::error::{RuntimeError, RuntimeResult};
use crate::domain::scraping::port::{BrowserSession, CdpEndpoint};

const WS_URL_PREFIX: &str = "DevTools listening on ";
const WS_PARSE_TIMEOUT_SECS: u64 = 30;

pub struct CdpSessionRuntime {
    child: Arc<Mutex<Option<Child>>>,
}

impl CdpSessionRuntime {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for CdpSessionRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BrowserSession for CdpSessionRuntime {
    async fn launch_with_cdp(&self, binary: &Path, spec: LaunchSpec) -> RuntimeResult<CdpEndpoint> {
        if !binary.exists() {
            return Err(RuntimeError::BinaryNotFound(binary.to_path_buf()));
        }

        if let Some(dir) = spec.user_data_dir.as_ref() {
            std::fs::create_dir_all(dir).map_err(|source| RuntimeError::Spawn {
                binary: binary.to_path_buf(),
                source,
            })?;
        }

        let mut cmd = Command::new(binary);
        for (k, v) in &spec.env {
            cmd.env(k, v);
        }
        cmd.args(&spec.flags);
        cmd.arg("--remote-debugging-port=0");
        if let Some(dir) = spec.user_data_dir.as_ref() {
            cmd.arg(format!("--user-data-dir={}", dir.display()));
        }
        cmd.args(&spec.urls);
        cmd.stderr(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::null());

        let mut child = cmd.spawn().map_err(|source| RuntimeError::Spawn {
            binary: binary.to_path_buf(),
            source,
        })?;

        let stderr = child.stderr.take().ok_or_else(|| RuntimeError::Spawn {
            binary: binary.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::Other, "failed to capture stderr"),
        })?;

        let ws_url = parse_ws_url(stderr)
            .await
            .map_err(|source| RuntimeError::Spawn {
                binary: binary.to_path_buf(),
                source,
            })?;

        *self.child.lock().await = Some(child);

        Ok(CdpEndpoint { ws_url })
    }

    async fn shutdown(&self) -> RuntimeResult<()> {
        if let Some(mut child) = self.child.lock().await.take() {
            let _ = child.kill().await;
        }
        Ok(())
    }
}

impl Drop for CdpSessionRuntime {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.child.try_lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.start_kill();
            }
        }
    }
}

async fn parse_ws_url(stderr: tokio::process::ChildStderr) -> Result<String, std::io::Error> {
    let reader = BufReader::new(stderr);
    let mut lines = reader.lines();

    let result = tokio::time::timeout(
        std::time::Duration::from_secs(WS_PARSE_TIMEOUT_SECS),
        async {
            while let Some(line) = lines.next_line().await? {
                if let Some(url) = line.strip_prefix(WS_URL_PREFIX) {
                    return Ok(url.trim().to_owned());
                }
                if line.contains("DevTools listening on ws://") {
                    if let Some(pos) = line.find("ws://") {
                        return Ok(line[pos..].trim().to_owned());
                    }
                }
            }
            Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "browser exited without emitting DevTools URL",
            ))
        },
    )
    .await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "timed out waiting for DevTools URL on stderr",
        )),
    }
}
