use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs;

use crate::domain::profile::{Profile, ProfileError, ProfileRepository, ProfileResult};

pub struct FsJsonProfileRepository {
    root: PathBuf,
}

impl FsJsonProfileRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

#[async_trait]
impl ProfileRepository for FsJsonProfileRepository {
    async fn load(&self, path: &Path) -> ProfileResult<Profile> {
        let resolved = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };

        let resolved = if resolved.extension().is_none() {
            resolved.with_extension("json")
        } else {
            resolved
        };

        let bytes = fs::read(&resolved).await.map_err(|source| {
            if source.kind() == std::io::ErrorKind::NotFound {
                ProfileError::NotFound(resolved.clone())
            } else {
                ProfileError::Io {
                    path: resolved.clone(),
                    source,
                }
            }
        })?;

        serde_json::from_slice::<Profile>(&bytes).map_err(|source| ProfileError::Json {
            path: resolved,
            source,
        })
    }

    async fn list(&self) -> ProfileResult<Vec<String>> {
        let mut entries = fs::read_dir(&self.root).await.map_err(|source| ProfileError::Io {
            path: self.root.clone(),
            source,
        })?;

        let mut out = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|source| ProfileError::Io {
            path: self.root.clone(),
            source,
        })? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if stem == "schema" {
                continue;
            }
            out.push(stem.to_owned());
        }
        out.sort();
        Ok(out)
    }
}
