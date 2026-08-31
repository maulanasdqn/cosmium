use crate::domain::profile::Profile;
use crate::infrastructure::scraping::stealth::StealthConfig;

pub fn build_stealth_config(p: &Profile) -> StealthConfig {
    let ch = &p.identity.client_hints;
    let full_ver = p
        .chrome_version
        .clone()
        .unwrap_or_else(|| extract_chrome_version(&p.identity.user_agent));

    let brands: Vec<serde_json::Value> = ch
        .brands
        .iter()
        .map(|b| serde_json::json!({"brand": b.brand, "version": b.version}))
        .collect();

    let fvl: Vec<serde_json::Value> = ch
        .brands
        .iter()
        .map(|b| {
            let ver = if b.brand == "Google Chrome" || b.brand == "Chromium" {
                full_ver.clone()
            } else {
                format!("{}.0.0.0", b.version)
            };
            serde_json::json!({"brand": b.brand, "version": ver})
        })
        .collect();

    let seed_hex: String = p.canvas_noise.seed.chars().take(8).collect();
    let noise_seed = u32::from_str_radix(&seed_hex, 16).unwrap_or(0xDEAD_BEEF);

    let locale = p.locale.languages.first().cloned().unwrap_or_default();
    let ua = match &p.chrome_version {
        Some(ver) => {
            let major = ver.split('.').next().unwrap_or(ver);
            let re = regex::Regex::new(r"Chrome/\d+\.\d+\.\d+\.\d+").unwrap();
            re.replace(&p.identity.user_agent, format!("Chrome/{major}.0.0.0"))
                .into_owned()
        }
        None => p.identity.user_agent.clone(),
    };

    let ext_entries: Vec<serde_json::Value> = p
        .browser_state
        .extensions
        .iter()
        .map(|e| {
            serde_json::json!({
                "id": e.id, "name": e.name, "version": e.version,
                "type": "extension", "enabled": true, "mayDisable": true,
            })
        })
        .collect();

    StealthConfig {
        brands_json: serde_json::to_string(&brands).unwrap_or_default(),
        full_version_list_json: serde_json::to_string(&fvl).unwrap_or_default(),
        platform: ch.platform.clone(),
        platform_version: ch.platform_version.clone(),
        architecture: ch.architecture.clone(),
        bitness: ch.bitness.clone(),
        model: ch.model.clone(),
        mobile: ch.mobile,
        wow64: ch.wow64,
        full_version: full_ver,
        screen_width: p.screen.width,
        screen_height: p.screen.height,
        avail_width: p.screen.avail_width,
        avail_height: p.screen.avail_height,
        avail_left: p.screen.avail_left,
        avail_top: p.screen.avail_top,
        color_depth: p.screen.color_depth,
        device_pixel_ratio: p.screen.device_pixel_ratio,
        locale,
        timezone: p.locale.timezone.clone(),
        noise_seed,
        user_agent: ua,
        accept_language: p.locale.accept_language.clone(),
        history_length: p.browser_state.history_length,
        download_count: p.browser_state.download_count,
        extensions_json: serde_json::to_string(&ext_entries).unwrap_or_default(),
        languages_json: serde_json::to_string(&p.locale.languages).unwrap_or_default(),
        hardware_concurrency: p.hardware.hardware_concurrency,
        device_memory: p.hardware.device_memory_gb,
    }
}

fn extract_chrome_version(ua: &str) -> String {
    ua.split("Chrome/")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .unwrap_or("135.0.0.0")
        .to_owned()
}
