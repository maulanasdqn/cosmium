use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::network::GetCookiesParams;

use crate::domain::scraping::page::PageCookie;

pub async fn collect(page: &Page) -> Vec<PageCookie> {
    let params = GetCookiesParams::default();
    let cookies = match page.execute(params).await {
        Ok(response) => response.result.cookies,
        Err(err) => {
            tracing::debug!(error = %err, "reading cookies failed");
            return Vec::new();
        }
    };
    cookies
        .into_iter()
        .map(|c| PageCookie {
            name: c.name,
            value: c.value,
            domain: c.domain,
            path: c.path,
            secure: c.secure,
            http_only: c.http_only,
            expires: c.expires,
        })
        .collect()
}

pub async fn user_agent(page: &Page) -> String {
    match page.evaluate("navigator.userAgent").await {
        Ok(value) => value.into_value::<String>().unwrap_or_default(),
        Err(err) => {
            tracing::debug!(error = %err, "reading user agent failed");
            String::new()
        }
    }
}
