use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub name: String,
    pub lang: String,
    pub default: bool,
    #[serde(rename = "localService")]
    pub local_service: bool,
    #[serde(rename = "voiceURI")]
    pub voice_uri: String,
}
