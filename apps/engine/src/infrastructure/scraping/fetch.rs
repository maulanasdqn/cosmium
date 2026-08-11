use std::time::Duration;

use chromiumoxide::Page;

pub async fn in_page_fetch(page: &Page, target_url: &str) -> Option<String> {
    let safe = target_url.replace('\'', "\\'");
    let js = format!(
        r#"
(async () => {{
    try {{
        const resp = await fetch('{safe}', {{
            credentials: 'include',
            headers: {{
                'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                'Accept-Language': 'en-US,en;q=0.9,id;q=0.8',
                'Upgrade-Insecure-Requests': '1'
            }}
        }});
        const status = resp.status;
        const text = await resp.text();
        const hasChallenge = text.toLowerCase().includes('captcha-delivery.com')
            || text.toLowerCase().includes('access is temporarily restricted')
            || text.toLowerCase().includes('just a moment');
        return JSON.stringify({{ status, len: text.length, hasChallenge }});
    }} catch(e) {{
        return JSON.stringify({{ error: e.message }});
    }}
}})()
"#
    );
    tracing::info!("probing target URL via in-page fetch");
    let probe_timeout = Duration::from_secs(15);
    let probe = match tokio::time::timeout(probe_timeout, page.evaluate(js.as_str())).await {
        Ok(Ok(v)) => v.into_value::<String>().unwrap_or_default(),
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "in-page fetch probe failed");
            return None;
        }
        Err(_) => {
            tracing::warn!("in-page fetch probe timed out (15s)");
            return None;
        }
    };
    tracing::debug!(result = %probe, "in-page fetch probe result");

    if !probe.contains("\"status\":200") || probe.contains("\"hasChallenge\":true") {
        tracing::info!("in-page fetch probe blocked, falling back to navigation");
        return None;
    }

    tracing::info!("in-page fetch probe passed, fetching full HTML");
    let fetch_js = format!(
        r#"
(async () => {{
    const resp = await fetch('{safe}', {{
        credentials: 'include',
        headers: {{
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
            'Accept-Language': 'en-US,en;q=0.9,id;q=0.8',
            'Upgrade-Insecure-Requests': '1'
        }}
    }});
    return await resp.text();
}})()
"#
    );
    let fetch_timeout = Duration::from_secs(20);
    match tokio::time::timeout(fetch_timeout, page.evaluate(fetch_js.as_str())).await {
        Ok(Ok(v)) => {
            let html = v.into_value::<String>().unwrap_or_default();
            if html.is_empty() {
                tracing::warn!("in-page fetch returned empty response");
                None
            } else {
                tracing::info!(bytes = html.len(), "in-page fetch succeeded");
                Some(html)
            }
        }
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "in-page fetch failed");
            None
        }
        Err(_) => {
            tracing::warn!("in-page fetch timed out (20s)");
            None
        }
    }
}
