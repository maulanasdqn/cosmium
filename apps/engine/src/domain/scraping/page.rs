use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ScrapedPage {
    pub http_status: i32,
    pub html: Vec<u8>,
    pub screenshot: Vec<u8>,
    pub final_url: String,
    pub cookies: Vec<PageCookie>,
    pub user_agent: String,
    pub script_results: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub expires: f64,
}
