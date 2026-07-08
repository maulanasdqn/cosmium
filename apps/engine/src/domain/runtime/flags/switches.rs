use crate::domain::profile::Profile;

use super::features::{disable_features_list, webrtc_flags};

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
    f.push(format!(
        "--cosmium-languages={}",
        p.locale.languages.join(",")
    ));
    f.push(format!(
        "--cosmium-hardware-concurrency={}",
        p.hardware.hardware_concurrency
    ));
    f.push(format!(
        "--cosmium-device-memory={}",
        p.hardware.device_memory_gb
    ));
    f.push(format!("--cosmium-color-depth={}", p.screen.color_depth));
    f.push(format!(
        "--cosmium-max-touch-points={}",
        p.hardware.max_touch_points
    ));
    f.push(format!("--cosmium-webgl-vendor={}", p.gpu.vendor));
    f.push(format!("--cosmium-webgl-renderer={}", p.gpu.renderer));
    f.push(format!(
        "--cosmium-audio-base-latency={}",
        p.audio.base_latency
    ));
    f.push(format!(
        "--cosmium-audio-output-latency={}",
        p.audio.output_latency
    ));
    f.push(format!("--cosmium-canvas-seed={}", p.canvas_noise.seed));
    f.push(format!(
        "--window-size={},{}",
        p.screen.width, p.screen.height
    ));
    f.push(format!(
        "--force-device-scale-factor={}",
        p.screen.device_pixel_ratio
    ));
    f.push("--disable-blink-features=AutomationControlled".into());
    if p.strip_automation_tells {
        f.push("--cosmium-strip-automation-tells".into());
    }
    f.push(format!("--disable-features={}", disable_features_list()));
    f.push("--no-default-browser-check".into());
    f.push("--no-first-run".into());
    f.push("--no-pings".into());
    f.push("--disable-domain-reliability".into());
    f.push("--disable-component-update".into());
    f.push("--disable-search-engine-choice-screen".into());
    f.push("--password-store=basic".into());
    f.push("--use-mock-keychain".into());
    f.extend(webrtc_flags(&p.webrtc.ip_handling_policy));
    f
}

pub(super) fn primary_lang(langs: &[String]) -> &str {
    langs.first().map(String::as_str).unwrap_or("en-US")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Profile {
        let raw = include_str!("../../../../../../profiles/win11_rtx3060_en-us.json");
        serde_json::from_str(raw).expect("fixture parses")
    }

    fn mac_fixture() -> Profile {
        let raw = include_str!("../../../../../../profiles/macos_m2_en-us.json");
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
            "--cosmium-max-touch-points=0",
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

    #[test]
    fn hygiene_flags_present() {
        let flags = profile_to_flags(&fixture());
        for w in [
            "--no-default-browser-check",
            "--no-first-run",
            "--no-pings",
            "--disable-domain-reliability",
            "--disable-component-update",
            "--disable-search-engine-choice-screen",
            "--password-store=basic",
            "--use-mock-keychain",
            "--disable-blink-features=AutomationControlled",
        ] {
            assert!(flags.iter().any(|f| f == w), "missing flag: {w}");
        }
    }

    #[test]
    fn disable_features_includes_telemetry_surfaces() {
        let flags = profile_to_flags(&fixture());
        let df = flags
            .iter()
            .find(|f| f.starts_with("--disable-features="))
            .expect("--disable-features flag present");
        for feat in ["Translate", "PrivacySandboxAdsAPIs", "AcceptCHFrame"] {
            assert!(df.contains(feat), "--disable-features missing {feat}: {df}");
        }
    }

    #[test]
    fn audio_latency_switches_present() {
        let flags = profile_to_flags(&fixture());
        assert!(
            flags
                .iter()
                .any(|f| f.starts_with("--cosmium-audio-base-latency="))
        );
        assert!(
            flags
                .iter()
                .any(|f| f.starts_with("--cosmium-audio-output-latency="))
        );
    }

    #[test]
    fn canvas_seed_switch_present() {
        let flags = profile_to_flags(&fixture());
        assert!(
            flags.iter().any(|f| f.starts_with("--cosmium-canvas-seed=")),
            "canvas seed must be emitted so the deterministic-noise patch can read it"
        );
    }

    #[test]
    fn strip_automation_tells_is_opt_in() {
        let mut p = fixture();

        p.strip_automation_tells = false;
        assert!(
            !profile_to_flags(&p)
                .iter()
                .any(|f| f == "--cosmium-strip-automation-tells"),
            "flag must be absent when not opted in"
        );

        p.strip_automation_tells = true;
        assert!(
            profile_to_flags(&p)
                .iter()
                .any(|f| f == "--cosmium-strip-automation-tells"),
            "flag must be emitted when opted in"
        );
    }

    #[test]
    fn webrtc_policy_passed_through() {
        let flags = profile_to_flags(&mac_fixture());
        assert!(
            flags
                .iter()
                .any(|f| f == "--force-webrtc-ip-handling-policy=default_public_interface_only")
        );
    }
}
