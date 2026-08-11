use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::error::{RuntimeError, RuntimeResult};
use crate::domain::scraping::port::{BrowserSession, CdpEndpoint};

pub struct CdpSessionRuntime {
    state: Arc<Mutex<Option<SessionState>>>,
}

struct SessionState {
    _handler: JoinHandle<()>,
}

impl CdpSessionRuntime {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(None)),
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

        let mut builder = BrowserConfig::builder()
            .disable_default_args()
            .chrome_executable(binary);

        if let Some(dir) = spec.user_data_dir.as_ref() {
            std::fs::create_dir_all(dir).map_err(|source| RuntimeError::Spawn {
                binary: binary.to_path_buf(),
                source,
            })?;
            builder = builder.user_data_dir(dir);
        }

        for (k, v) in &spec.env {
            builder = builder.env(k, v);
        }

        for flag in &spec.flags {
            let stripped = flag.strip_prefix("--").unwrap_or(flag);
            builder = builder.arg(stripped);
        }

        let config = builder.build().map_err(|e| RuntimeError::Spawn {
            binary: binary.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        })?;

        let (browser, mut handler) =
            Browser::launch(config)
                .await
                .map_err(|e| RuntimeError::Spawn {
                    binary: binary.to_path_buf(),
                    source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
                })?;

        let handler_task = tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                if event.is_err() {
                    break;
                }
            }
        });

        *self.state.lock().await = Some(SessionState {
            _handler: handler_task,
        });

        Ok(CdpEndpoint { browser })
    }

    async fn shutdown(&self) -> RuntimeResult<()> {
        if let Some(_state) = self.state.lock().await.take() {
            drop(_state);
        }
        Ok(())
    }
}
