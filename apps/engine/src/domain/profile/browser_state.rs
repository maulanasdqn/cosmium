use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserState {
    #[serde(default = "default_history_length")]
    pub history_length: u32,

    #[serde(default = "default_download_count")]
    pub download_count: u32,

    #[serde(default = "default_extensions")]
    pub extensions: Vec<FakeExtension>,
}

impl Default for BrowserState {
    fn default() -> Self {
        Self {
            history_length: default_history_length(),
            download_count: default_download_count(),
            extensions: default_extensions(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeExtension {
    pub id: String,
    pub name: String,
    pub version: String,
}

const fn default_history_length() -> u32 {
    5
}

const fn default_download_count() -> u32 {
    3
}

fn default_extensions() -> Vec<FakeExtension> {
    vec![
        FakeExtension {
            id: "cjpalhdlnbpafiamejdnhcphjbkeiagm".into(),
            name: "uBlock Origin".into(),
            version: "1.62.0".into(),
        },
        FakeExtension {
            id: "hdokiejnpimakedhajhdlcegeplioahd".into(),
            name: "LastPass: Free Password Manager".into(),
            version: "4.133.0".into(),
        },
    ]
}
