use std::path::PathBuf;

use anyhow::Result;

pub fn build_json_output(
    result: &crate::application::use_cases::scrape_page::ScrapePageOutput,
    output_dir: &Option<PathBuf>,
) -> Result<String> {
    let mut obj = serde_json::Map::new();
    obj.insert(
        "url".into(),
        serde_json::Value::String(result.page.final_url.clone()),
    );
    obj.insert(
        "status".into(),
        serde_json::Value::Number(result.page.http_status.into()),
    );
    obj.insert(
        "html_length".into(),
        serde_json::Value::Number(result.page.html.len().into()),
    );
    obj.insert("blocked".into(), serde_json::Value::Bool(result.blocked));
    obj.insert(
        "user_agent".into(),
        serde_json::Value::String(result.page.user_agent.clone()),
    );
    obj.insert(
        "cookies_count".into(),
        serde_json::Value::Number(result.page.cookies.len().into()),
    );

    if !result.page.script_results.is_empty() {
        let mut extracted = serde_json::Map::new();
        for (k, v) in &result.page.script_results {
            let parsed: serde_json::Value =
                serde_json::from_str(v).unwrap_or(serde_json::Value::String(v.clone()));
            extracted.insert(k.clone(), parsed);
        }
        obj.insert("extracted".into(), serde_json::Value::Object(extracted));
    }

    if !result.page.screenshot.is_empty() {
        if let Some(dir) = output_dir {
            obj.insert(
                "screenshot_path".into(),
                serde_json::Value::String(dir.join("screenshot.jpg").display().to_string()),
            );
        } else {
            obj.insert(
                "screenshot_bytes".into(),
                serde_json::Value::Number(result.page.screenshot.len().into()),
            );
        }
    }

    Ok(serde_json::to_string_pretty(&serde_json::Value::Object(
        obj,
    ))?)
}
