mod id;
mod record;
mod retention;

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use serde_json::Value;

pub use id::is_valid_id;
pub use record::{ResultStats, ResultSummary, StoredResult};
pub use retention::Retention;

pub struct NewResult {
    pub title: Option<String>,
    pub payload: Value,
    pub result: Value,
    pub ai_request: Option<Value>,
}

pub struct ResultStore {
    dir: PathBuf,
    retention: Retention,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

#[cfg(unix)]
async fn restrict(dir: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = tokio::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)).await;
}

#[cfg(not(unix))]
async fn restrict(_dir: &std::path::Path) {}

impl ResultStore {
    pub const fn new(dir: PathBuf, retention: Retention) -> Self {
        Self { dir, retention }
    }

    pub fn default_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cosmium")
            .join("results")
    }

    fn path(&self, id: &str, suffix: &str) -> PathBuf {
        self.dir.join(format!("{id}{suffix}"))
    }

    async fn write(&self, stored: &StoredResult) -> Result<()> {
        tokio::fs::create_dir_all(&self.dir)
            .await
            .with_context(|| format!("creating {}", self.dir.display()))?;
        restrict(&self.dir).await;
        tokio::fs::write(self.path(&stored.id, ".json"), serde_json::to_vec(stored)?).await?;
        let summary = serde_json::to_vec(&stored.summary())?;
        tokio::fs::write(self.path(&stored.id, ".summary.json"), summary).await?;
        Ok(())
    }

    pub async fn create(&self, new: NewResult) -> Result<StoredResult> {
        let stored = StoredResult {
            id: id::new_id(),
            created_at_ms: now_ms(),
            title: new.title,
            payload: new.payload,
            result: new.result,
            ai_request: new.ai_request,
            ai: None,
        };
        self.write(&stored).await?;
        match self.prune().await {
            Ok(0) => {}
            Ok(removed) => tracing::info!(removed, "pruned old results"),
            Err(e) => tracing::warn!(error = %e, "pruning results failed"),
        }
        Ok(stored)
    }

    pub async fn get(&self, id: &str) -> Result<Option<StoredResult>> {
        if !is_valid_id(id) {
            return Ok(None);
        }
        match tokio::fs::read(self.path(id, ".json")).await {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn set_ai(&self, id: &str, ai: Value) -> Result<Option<StoredResult>> {
        let Some(mut stored) = self.get(id).await? else {
            return Ok(None);
        };
        stored.ai = Some(ai);
        self.write(&stored).await?;
        Ok(Some(stored))
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        if self.get(id).await?.is_none() {
            return Ok(false);
        }
        tokio::fs::remove_file(self.path(id, ".json")).await?;
        let _ = tokio::fs::remove_file(self.path(id, ".summary.json")).await;
        Ok(true)
    }

    pub async fn delete_many(&self, ids: &[String]) -> usize {
        let mut removed = 0;
        for id in ids {
            if self.delete(id).await.unwrap_or(false) {
                removed += 1;
            }
        }
        removed
    }

    pub async fn prune(&self) -> Result<usize> {
        let entries: Vec<(String, u64)> = self
            .list()
            .await?
            .into_iter()
            .map(|summary| (summary.id, summary.created_at_ms))
            .collect();
        let expired = retention::expired_ids(&entries, self.retention, now_ms());
        Ok(self.delete_many(&expired).await)
    }

    pub async fn stats(&self) -> Result<ResultStats> {
        let summaries = self.list().await?;
        let mut bytes = 0_u64;
        if let Ok(mut entries) = tokio::fs::read_dir(&self.dir).await {
            while let Some(entry) = entries.next_entry().await? {
                if let Ok(meta) = entry.metadata().await {
                    bytes = bytes.saturating_add(meta.len());
                }
            }
        }
        Ok(ResultStats {
            count: summaries.len(),
            bytes,
            oldest_ms: summaries.last().map(|s| s.created_at_ms),
            newest_ms: summaries.first().map(|s| s.created_at_ms),
            max_results: self.retention.max_results,
            max_age_days: self.retention.max_age_days,
        })
    }

    pub async fn list(&self) -> Result<Vec<ResultSummary>> {
        let mut out = Vec::new();
        let Ok(mut entries) = tokio::fs::read_dir(&self.dir).await else {
            return Ok(out);
        };
        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".summary.json") {
                continue;
            }
            if let Ok(summary) = serde_json::from_slice(&tokio::fs::read(entry.path()).await?) {
                out.push(summary);
            }
        }
        out.sort_by_key(|summary: &ResultSummary| std::cmp::Reverse(summary.created_at_ms));
        Ok(out)
    }
}
