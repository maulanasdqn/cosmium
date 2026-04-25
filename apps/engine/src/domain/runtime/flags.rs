use crate::domain::profile::{IpHandlingPolicy, Profile};

pub fn profile_to_flags(p: &Profile) -> Vec<String> {
    let mut f = Vec::new();
    f.push(format!("--user-agent={}", p.identity.user_agent));
    f.push(format!("--lang={}", primary_lang(&p.locale.languages)));
    f.push(format!("--accept-lang={}", p.locale.accept_language));
    f.push(format!(
        "--cosmium-ua-platform={}",
        p.identity.client_hints.platform
    ));
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
