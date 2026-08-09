mod actions;
mod extract;

use std::collections::HashMap;
use std::time::Duration;

use chromiumoxide::Page;
use serde_json::Value;

use crate::domain::scraping::workflow::WorkflowStep;

const DEFAULT_SCRIPT_TIMEOUT: u64 = 30;
const FOLLOW_SETTLE_MS: u64 = 400;

pub async fn run(page: &Page, steps: &[WorkflowStep]) -> HashMap<String, String> {
    let mut results = HashMap::new();
    for step in steps {
        match step {
            WorkflowStep::Delay { duration_ms } => {
                tokio::time::sleep(Duration::from_millis(u64::from(*duration_ms))).await;
            }
            WorkflowStep::Script {
                name,
                code,
                timeout_seconds,
            } => {
                let seconds = if *timeout_seconds == 0 {
                    DEFAULT_SCRIPT_TIMEOUT
                } else {
                    u64::from(*timeout_seconds)
                };
                if let Some(output) = evaluate(page, code, seconds, name).await {
                    results.insert(name.clone(), output);
                }
            }
            WorkflowStep::Click { selector } => {
                actions::click(page, selector).await;
            }
            WorkflowStep::Input { selector, text } => {
                actions::input(page, selector, text).await;
            }
            WorkflowStep::Scroll {
                infinite,
                selector,
                times,
            } => {
                actions::scroll(page, *infinite, selector.as_deref(), *times).await;
            }
            WorkflowStep::Extract {
                name,
                selector,
                attribute,
                limit,
            } => {
                let items = extract::extract(page, selector, attribute.as_deref(), *limit).await;
                results.insert(name.clone(), json_strings(&items));
            }
            WorkflowStep::FollowUrls {
                name,
                selector,
                attribute,
                limit,
                workflow,
            } => {
                let followed =
                    follow_urls(page, selector, attribute.as_deref(), *limit, workflow).await;
                results.insert(name.clone(), followed);
            }
        }
    }
    results
}

fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_owned())
}

async fn follow_urls(
    page: &Page,
    selector: &str,
    attribute: Option<&str>,
    limit: u32,
    steps: &[WorkflowStep],
) -> String {
    let urls = extract::collect_urls(page, selector, attribute, limit).await;
    let mut out = Vec::with_capacity(urls.len());
    for url in urls {
        if page.goto(&url).await.is_err() {
            continue;
        }
        tokio::time::sleep(Duration::from_millis(FOLLOW_SETTLE_MS)).await;
        let fields = Box::pin(run(page, steps)).await;
        out.push(detail_object(url, fields));
    }
    Value::Array(out).to_string()
}

fn detail_object(url: String, fields: HashMap<String, String>) -> Value {
    let mut obj = serde_json::Map::with_capacity(fields.len() + 1);
    obj.insert("url".to_owned(), Value::String(url));
    for (key, raw) in fields {
        obj.insert(
            key,
            serde_json::from_str(&raw).unwrap_or(Value::String(raw)),
        );
    }
    Value::Object(obj)
}

fn json_strings(items: &[String]) -> String {
    Value::Array(items.iter().map(|s| Value::String(s.clone())).collect()).to_string()
}

async fn evaluate(page: &Page, code: &str, seconds: u64, name: &str) -> Option<String> {
    let evaluation = tokio::time::timeout(Duration::from_secs(seconds), page.evaluate(code)).await;
    match evaluation {
        Ok(Ok(result)) => Some(serialise(result.into_value::<Value>().ok())),
        Ok(Err(err)) => {
            tracing::warn!(
                script = %name,
                error = %err,
                "workflow script failed"
            );
            None
        }
        Err(_) => {
            tracing::warn!(
                script = %name,
                seconds,
                "workflow script timed out"
            );
            None
        }
    }
}

fn serialise(value: Option<Value>) -> String {
    value
        .as_ref()
        .map_or_else(|| "null".to_owned(), ToString::to_string)
}
