use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

use tokio::sync::Mutex;

const LOOKUP_TIMEOUT: Duration = Duration::from_secs(6);

static CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub async fn exit_timezone(proxy: Option<&str>) -> Option<String> {
    let key = proxy.unwrap_or_default().to_owned();
    if let Some(tz) = CACHE.lock().await.get(&key) {
        return Some(tz.clone());
    }
    let client = client(proxy)?;
    let tz = match ipwho(&client).await {
        Some(tz) => tz,
        None => ip_api(&client).await?,
    };
    if !is_valid_tz(&tz) {
        tracing::warn!(tz, "geo lookup returned an invalid timezone");
        return None;
    }
    tracing::info!(tz, "geo-synced timezone from exit IP");
    CACHE.lock().await.insert(key, tz.clone());
    Some(tz)
}

fn client(proxy: Option<&str>) -> Option<reqwest::Client> {
    let mut builder = reqwest::Client::builder().timeout(LOOKUP_TIMEOUT);
    if let Some(url) = proxy {
        builder = builder.proxy(reqwest::Proxy::all(url).ok()?);
    }
    builder.build().ok()
}

async fn fetch_json(client: &reqwest::Client, url: &str) -> Option<serde_json::Value> {
    client.get(url).send().await.ok()?.json().await.ok()
}

async fn ipwho(client: &reqwest::Client) -> Option<String> {
    let v = fetch_json(client, "https://ipwho.is/").await?;
    v.get("timezone")?
        .get("id")?
        .as_str()
        .map(ToOwned::to_owned)
}

async fn ip_api(client: &reqwest::Client) -> Option<String> {
    let v = fetch_json(client, "http://ip-api.com/json/?fields=status,timezone").await?;
    v.get("timezone")?.as_str().map(ToOwned::to_owned)
}

fn is_valid_tz(tz: &str) -> bool {
    !tz.is_empty()
        && tz.len() < 64
        && tz
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '_' | '-' | '+'))
}

#[cfg(test)]
mod tests {
    use super::is_valid_tz;

    #[test]
    fn accepts_iana_names_and_rejects_injection() {
        assert!(is_valid_tz("Asia/Jakarta"));
        assert!(is_valid_tz("America/Argentina/Buenos_Aires"));
        assert!(is_valid_tz("Etc/GMT+7"));
        assert!(!is_valid_tz("Asia/Jakarta --no-sandbox"));
        assert!(!is_valid_tz(""));
    }
}
