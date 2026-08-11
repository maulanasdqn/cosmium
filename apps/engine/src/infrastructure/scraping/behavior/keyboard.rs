use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::input::{DispatchKeyEventParams, DispatchKeyEventType};
use rand::Rng;

const BASE_DELAY_MIN_MS: u64 = 45;
const BASE_DELAY_MAX_MS: u64 = 130;
const PAUSE_CHANCE: f64 = 0.08;
const PAUSE_MIN_MS: u64 = 200;
const PAUSE_MAX_MS: u64 = 500;

fn char_to_code(ch: char) -> (String, i64) {
    match ch {
        'a'..='z' => (
            format!("Key{}", ch.to_ascii_uppercase()),
            ch.to_ascii_uppercase() as i64,
        ),
        'A'..='Z' => (format!("Key{ch}"), ch as i64),
        '0'..='9' => (format!("Digit{ch}"), ch as i64),
        ' ' => ("Space".into(), 32),
        '.' => ("Period".into(), 190),
        ',' => ("Comma".into(), 188),
        '-' | '_' => ("Minus".into(), 189),
        '=' => ("Equal".into(), 187),
        '/' => ("Slash".into(), 191),
        '@' => ("Digit2".into(), 50),
        ':' | ';' => ("Semicolon".into(), 186),
        '\'' | '"' => ("Quote".into(), 222),
        _ => ("Unidentified".into(), 0),
    }
}

fn key_evt(
    kind: DispatchKeyEventType,
    key: Option<String>,
    text: Option<String>,
    code: String,
    vk: i64,
    commands: Option<Vec<String>>,
) -> DispatchKeyEventParams {
    DispatchKeyEventParams {
        r#type: kind,
        text,
        unmodified_text: None,
        code: Some(code),
        key,
        windows_virtual_key_code: Some(vk),
        native_virtual_key_code: Some(vk),
        modifiers: None,
        timestamp: None,
        key_identifier: None,
        auto_repeat: None,
        is_keypad: None,
        is_system_key: None,
        location: None,
        commands,
    }
}

async fn type_char(page: &Page, ch: char) {
    let (code, vk) = char_to_code(ch);
    let key_str = ch.to_string();

    let down = key_evt(
        DispatchKeyEventType::KeyDown,
        Some(key_str.clone()),
        Some(key_str.clone()),
        code.clone(),
        vk,
        None,
    );
    let _ = page.execute(down).await;

    let chr = key_evt(
        DispatchKeyEventType::Char,
        Some(key_str.clone()),
        Some(key_str.clone()),
        code.clone(),
        vk,
        None,
    );
    let _ = page.execute(chr).await;

    let up = key_evt(
        DispatchKeyEventType::KeyUp,
        Some(key_str),
        None,
        code,
        vk,
        None,
    );
    let _ = page.execute(up).await;
}

fn plan_delays(len: usize) -> Vec<u64> {
    let mut rng = rand::rng();
    (0..len)
        .map(|_| {
            let base = rng.random_range(BASE_DELAY_MIN_MS..=BASE_DELAY_MAX_MS);
            let extra = if rng.random_bool(PAUSE_CHANCE) {
                rng.random_range(PAUSE_MIN_MS..=PAUSE_MAX_MS)
            } else {
                0
            };
            base + extra
        })
        .collect()
}

pub async fn type_text(page: &Page, text: &str) {
    let chars: Vec<char> = text.chars().collect();
    let delays = plan_delays(chars.len());
    for (ch, delay) in chars.into_iter().zip(delays) {
        type_char(page, ch).await;
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
}

pub async fn clear_field(page: &Page, selector: &str) {
    let js = format!(
        "(() => {{ const el = document.querySelector({}); if (el) {{ el.focus(); el.select(); }} }})()",
        serde_json::to_string(selector).unwrap_or_default()
    );
    let _ = page.evaluate(js).await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    let del = key_evt(
        DispatchKeyEventType::KeyDown,
        Some("Backspace".into()),
        None,
        "Backspace".into(),
        8,
        Some(vec!["deleteBackward".into()]),
    );
    let _ = page.execute(del).await;

    let del_up = key_evt(
        DispatchKeyEventType::KeyUp,
        Some("Backspace".into()),
        None,
        "Backspace".into(),
        8,
        None,
    );
    let _ = page.execute(del_up).await;
    tokio::time::sleep(Duration::from_millis(30)).await;
}
