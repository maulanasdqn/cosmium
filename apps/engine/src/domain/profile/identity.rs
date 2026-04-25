use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub user_agent: String,
    pub client_hints: ClientHints,
    pub navigator_platform: String,
    pub navigator_app_version: String,
    #[serde(default = "default_vendor")]
    pub navigator_vendor: String,
    #[serde(default = "default_product")]
    pub navigator_product: String,
    #[serde(default = "default_product_sub")]
    pub navigator_product_sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHints {
    pub brands: Vec<Brand>,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub bitness: String,
    #[serde(default)]
    pub model: String,
    pub mobile: bool,
    pub wow64: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    pub brand: String,
    pub version: String,
}

fn default_vendor() -> String {
    "Google Inc.".to_owned()
}
fn default_product() -> String {
    "Gecko".to_owned()
}
fn default_product_sub() -> String {
    "20030107".to_owned()
}
