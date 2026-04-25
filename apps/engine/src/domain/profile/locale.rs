use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    pub languages: Vec<String>,
    pub accept_language: String,
    pub timezone: String,
    #[serde(default)]
    pub currency: Option<String>,
}
