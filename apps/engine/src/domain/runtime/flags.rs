use crate::domain::profile::{IpHandlingPolicy, Profile};

pub fn profile_to_flags(p: &Profile) -> Vec<String> {
    let mut f = Vec::new();
    f.push(format!("--user-agent={}", p.identity.user_agent));
    f.push(format!("--lang={}", primary_lang(&p.locale.languages)));
    f.push(format!("--accept-lang={}", p.locale.accept_language));
    f.push(format!(
        "--cosmium-platform={}",
        p.identity.navigator_platform
    ));
    f.push(format!(
        "--cosmium-ua-platform={}",
        p.identity.client_hints.platform
    ));
    f.push(format!("--cosmium-languages={}", p.locale.languages.join(",")));
    f.push(format!(
        "--cosmium-hardware-concurrency={}",
        p.hardware.hardware_concurrency
    ));
    f.push(format!(
        "--cosmium-device-memory={}",
        p.hardware.device_memory_gb
    ));
    f.push(format!("--cosmium-color-depth={}", p.screen.color_depth));
    f.push(format!("--cosmium-webgl-vendor={}", p.gpu.vendor));
    f.push(format!("--cosmium-webgl-renderer={}", p.gpu.renderer));
    f.push(format!(
        "--window-size={},{}",
        p.screen.width, p.screen.height
    ));
    f.push(format!(
        "--force-device-scale-factor={}",
        p.screen.device_pixel_ratio
    ));
    f.push("--disable-blink-features=AutomationControlled".into());
    f.push("--disable-features=Translate,InterestFeedContentSuggestions".into());
    f.push("--no-default-browser-check".into());
    f.push("--no-first-run".into());
    f.extend(webrtc_flags(&p.webrtc.ip_handling_policy));
    f
}

fn primary_lang(langs: &[String]) -> &str {
    langs.first().map(String::as_str).unwrap_or("en-US")
}

fn webrtc_flags(policy: &IpHandlingPolicy) -> Vec<String> {
    let value = match policy {
        IpHandlingPolicy::Default => "default",
        IpHandlingPolicy::DefaultPublicInterfaceOnly => "default_public_interface_only",
        IpHandlingPolicy::DefaultPublicAndPrivateInterfaces => {
            "default_public_and_private_interfaces"
        }
        IpHandlingPolicy::DisableNonProxiedUdp => "disable_non_proxied_udp",
    };
    vec![format!("--force-webrtc-ip-handling-policy={value}")]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Profile {
        let raw = include_str!("../../../../../profiles/win11_rtx3060_en-us.json");
        serde_json::from_str(raw).expect("fixture parses")
    }

    #[test]
    fn emits_all_cosmium_switches() {
        let flags = profile_to_flags(&fixture());
        let want = [
            "--cosmium-platform=Win32",
            "--cosmium-ua-platform=Windows",
            "--cosmium-languages=en-US,en",
            "--cosmium-hardware-concurrency=12",
            "--cosmium-device-memory=8",
            "--cosmium-color-depth=24",
            "--cosmium-webgl-vendor=Google Inc. (NVIDIA)",
        ];
        for w in want {
            assert!(flags.iter().any(|f| f == w), "missing flag: {w}");
        }
    }

    #[test]
    fn webgl_renderer_passed_through() {
        let flags = profile_to_flags(&fixture());
        assert!(
            flags
                .iter()
                .any(|f| f.starts_with("--cosmium-webgl-renderer=ANGLE (NVIDIA"))
        );
    }
}
