use super::probe::{ProbeDef, simple};
use crate::domain::profile::{MediaDeviceKind, Profile};

const UNLISTED_POOL: [&str; 15] = [
    "Cascadia Code",
    "Cascadia Mono",
    "Segoe UI Variable",
    "Segoe Fluent Icons",
    "Aptos",
    "Arial Nova",
    "Noto Sans",
    "DejaVu Sans",
    "Liberation Sans",
    "Ubuntu",
    "Roboto",
    "Helvetica Neue",
    "Menlo",
    "Monaco",
    "Lucida Grande",
];

const DETECT_FONTS: &str = "(list=>{const c=document.createElement('canvas').getContext('2d');const t='mmmmmmmmmmlli1WQ@#';const w=f=>{c.font='72px '+f;return c.measureText(t).width};const base=['monospace','serif','sans-serif'].map(b=>[b,w(b)]);return list.filter(f=>base.some(([b,bw])=>w('\"'+f+'\", '+b)!==bw));})";

pub(super) fn device_list_probes(p: &Profile) -> Vec<ProbeDef> {
    let listed = &p.fonts.installed;
    let unlisted: Vec<&str> = UNLISTED_POOL
        .into_iter()
        .filter(|f| !listed.iter().any(|l| l.eq_ignore_ascii_case(f)))
        .collect();
    let mut probes = vec![
        simple(
            "fonts_unlisted_hidden",
            &format!(
                "JSON.stringify({DETECT_FONTS}({}))",
                serde_json::to_string(&unlisted).unwrap_or_default()
            ),
            "[]",
        ),
        simple(
            "fonts_listed_visible",
            &format!(
                "String({DETECT_FONTS}({}).length>0)",
                serde_json::to_string(listed).unwrap_or_default()
            ),
            "true",
        ),
    ];
    probes.push(simple(
        "media_device_kinds",
        "JSON.stringify((await navigator.mediaDevices.enumerateDevices()).map(d=>d.kind))",
        &expected_kinds(p),
    ));
    probes
}

fn expected_kinds(p: &Profile) -> String {
    let order = [
        (MediaDeviceKind::AudioInput, "audioinput"),
        (MediaDeviceKind::VideoInput, "videoinput"),
        (MediaDeviceKind::AudioOutput, "audiooutput"),
    ];
    let kinds: Vec<&str> = order
        .into_iter()
        .filter(|(kind, _)| p.media_devices.iter().any(|d| d.kind == *kind))
        .map(|(_, name)| name)
        .collect();
    serde_json::to_string(&kinds).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::expected_kinds;
    use crate::domain::profile::{MediaDeviceKind, Profile};

    fn fixture() -> Profile {
        let raw = include_str!("../../../../profiles/win11_rtx3060_en-us.json");
        serde_json::from_str(raw).expect("fixture parses")
    }

    #[test]
    fn kinds_follow_chrome_order() {
        assert_eq!(
            expected_kinds(&fixture()),
            r#"["audioinput","videoinput","audiooutput"]"#
        );
    }

    #[test]
    fn missing_kinds_are_dropped() {
        let mut p = fixture();
        p.media_devices
            .retain(|d| d.kind != MediaDeviceKind::VideoInput);
        assert_eq!(expected_kinds(&p), r#"["audioinput","audiooutput"]"#);
    }
}
