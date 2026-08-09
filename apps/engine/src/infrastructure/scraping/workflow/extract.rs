use chromiumoxide::Page;

use super::js_string;

pub async fn extract(
    page: &Page,
    selector: &str,
    attribute: Option<&str>,
    limit: u32,
) -> Vec<String> {
    let code = format!(
        "(() => {{ \
            const attr = {attr}; \
            return Array.from(document.querySelectorAll({sel})) \
                .map(el => attr \
                    ? (el.getAttribute(attr) ?? '') \
                    : (el.textContent ?? '').trim()); \
        }})()",
        attr = attr_expr(attribute),
        sel = js_string(selector)
    );
    apply_limit(eval_strings(page, code).await, limit)
}

pub async fn collect_urls(
    page: &Page,
    selector: &str,
    attribute: Option<&str>,
    limit: u32,
) -> Vec<String> {
    let code = format!(
        "(() => {{ \
            const attr = {attr}; \
            return Array.from(document.querySelectorAll({sel})) \
                .map(el => {{ \
                    const raw = el.getAttribute(attr); \
                    if (!raw) return null; \
                    try {{ return new URL(raw, location.href).href; }} \
                    catch (e) {{ return null; }} \
                }}).filter(Boolean); \
        }})()",
        attr = attr_expr(attribute.or(Some("href"))),
        sel = js_string(selector)
    );
    apply_limit(eval_strings(page, code).await, limit)
}

fn attr_expr(attribute: Option<&str>) -> String {
    attribute.map_or_else(|| "null".to_owned(), js_string)
}

fn apply_limit(mut items: Vec<String>, limit: u32) -> Vec<String> {
    if limit > 0 && items.len() > limit as usize {
        items.truncate(limit as usize);
    }
    items
}

async fn eval_strings(page: &Page, code: String) -> Vec<String> {
    match page.evaluate(code).await {
        Ok(result) => result.into_value::<Vec<String>>().unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
