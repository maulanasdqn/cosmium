use std::path::{Path, PathBuf};
use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::network::{
    CookieParam, DeleteCookiesParams, SetCookiesParams,
};

pub async fn preseed_cookies(page: &Page, host: &str, session_dir: &Path) -> bool {
    let cookie_path = session_path(session_dir, host);
    let data = match tokio::fs::read_to_string(&cookie_path).await {
        Ok(d) => d,
        Err(_) => return false,
    };
    let cookies: Vec<SavedCookie> = match serde_json::from_str(&data) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if cookies.is_empty() {
        return false;
    }

    let params: Vec<CookieParam> = cookies
        .into_iter()
        .filter(|c| c.name.starts_with("datadome") || c.name.starts_with("_dd"))
        .map(|c| {
            let mut p = CookieParam::new(c.name, c.value);
            p.domain = Some(c.domain);
            p.path = Some(c.path);
            p.secure = Some(c.secure);
            p.http_only = Some(c.http_only);
            p
        })
        .collect();

    if params.is_empty() {
        return false;
    }

    let count = params.len();
    let cmd = SetCookiesParams::new(params);
    match page.execute(cmd).await {
        Ok(_) => {
            tracing::info!(count, "pre-seeded DataDome cookies from cache");
            true
        }
        Err(e) => {
            tracing::warn!(error = %e, "DataDome cookie pre-seed failed");
            false
        }
    }
}

pub async fn save_cookies(page: &Page, host: &str, session_dir: &Path) {
    let cookies = match crate::infrastructure::scraping::cookies::collect(page).await {
        c if c.is_empty() => return,
        c => c,
    };

    let dd_cookies: Vec<SavedCookie> = cookies
        .into_iter()
        .filter(|c| {
            let name = c.name.to_lowercase();
            name.starts_with("datadome") || name.starts_with("_dd")
        })
        .map(|c| SavedCookie {
            name: c.name.clone(),
            value: c.value.clone(),
            domain: c.domain.clone(),
            path: c.path.clone(),
            secure: c.secure,
            http_only: c.http_only,
        })
        .collect();

    if dd_cookies.is_empty() {
        return;
    }

    let cookie_path = session_path(session_dir, host);
    if let Some(parent) = cookie_path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }

    if let Ok(json) = serde_json::to_string_pretty(&dd_cookies) {
        let _ = tokio::fs::write(&cookie_path, json).await;
        tracing::debug!(
            count = dd_cookies.len(),
            path = %cookie_path.display(),
            "saved DataDome cookies"
        );
    }
}

pub async fn get_dd_cookie_value(page: &Page) -> Option<String> {
    use chromiumoxide::cdp::browser_protocol::network::GetCookiesParams;

    let timeout = Duration::from_secs(5);
    let result =
        match tokio::time::timeout(timeout, page.execute(GetCookiesParams::default())).await {
            Ok(Ok(resp)) => resp,
            _ => return None,
        };

    result
        .result
        .cookies
        .iter()
        .find(|c| c.name == "datadome")
        .map(|c| c.value.clone())
}

pub async fn clear_cookies(page: &Page, host: &str) {
    for name in &["datadome", "_dd_s"] {
        let mut params = DeleteCookiesParams::new(name.to_string());
        params.domain = Some(format!(".{host}"));
        let _ = page.execute(params).await;
    }
}

fn session_path(session_dir: &Path, host: &str) -> PathBuf {
    let safe_host = host.replace('.', "_");
    session_dir.join(format!("datadome_{safe_host}.json"))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SavedCookie {
    name: String,
    value: String,
    domain: String,
    path: String,
    secure: bool,
    http_only: bool,
}
