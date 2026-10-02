use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;

use crate::domain::profile::Profile;

use super::flags::session_cache_dir;

const FONT_DIRS: [&str; 2] = ["/usr/share/fonts", "/usr/local/share/fonts"];

const SUBSTITUTES: [(&str, &[&str]); 11] = [
    ("Liberation Sans", &["Arial", "Helvetica"]),
    ("Liberation Serif", &["Times New Roman", "Times"]),
    ("Liberation Mono", &["Courier New", "Courier"]),
    ("Liberation Sans Narrow", &["Arial Narrow"]),
    ("Carlito", &["Calibri"]),
    ("Caladea", &["Cambria"]),
    ("Gelasio", &["Georgia"]),
    ("Arimo", &["Arial", "Helvetica"]),
    ("Tinos", &["Times New Roman", "Times"]),
    ("Cousine", &["Courier New", "Courier"]),
    ("Noto Color Emoji", &["Segoe UI Emoji", "Apple Color Emoji"]),
];

const HIDDEN_FAMILY: &str = "cosmium-unavailable";

const GENERICS: [(&str, &[&str]); 3] = [
    ("sans-serif", &["Arial", "Helvetica"]),
    ("serif", &["Times New Roman", "Times"]),
    ("monospace", &["Courier New", "Courier", "Consolas"]),
];

#[derive(Debug, Clone)]
pub struct FontConfig {
    pub dir: PathBuf,
    pub xml: String,
}

pub fn fontconfig_for(p: &Profile) -> FontConfig {
    let installed = &p.fonts.installed;
    let mut hasher = DefaultHasher::new();
    installed.hash(&mut hasher);
    let dir = session_cache_dir()
        .join("fontconfig")
        .join(format!("{:016x}", hasher.finish()));
    let xml = render(installed, &dir.join("cache").display().to_string());
    FontConfig { dir, xml }
}

fn first_installed<'a>(installed: &[String], candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .find(|c| installed.iter().any(|i| i == c))
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render(installed: &[String], cache_dir: &str) -> String {
    let mut lines = vec![
        "<?xml version=\"1.0\"?>".to_owned(),
        "<!DOCTYPE fontconfig SYSTEM \"urn:fontconfig:fonts.dtd\">".to_owned(),
        "<fontconfig>".to_owned(),
    ];
    lines.extend(FONT_DIRS.iter().map(|d| format!("  <dir>{d}</dir>")));
    lines.push(format!("  <cachedir>{}</cachedir>", esc(cache_dir)));
    lines.extend(rename_rules(installed));
    lines.extend(hide_rules(installed));
    lines.extend(generic_rules(installed));
    lines.push("  <selectfont>".to_owned());
    lines.push("    <rejectfont><pattern><patelt name=\"scalable\"><bool>true</bool></patelt></pattern><pattern><patelt name=\"scalable\"><bool>false</bool></patelt></pattern></rejectfont>".to_owned());
    lines.push("    <acceptfont>".to_owned());
    lines.extend(installed.iter().map(|name| {
        format!(
            "      <pattern><patelt name=\"family\"><string>{}</string></patelt></pattern>",
            esc(name)
        )
    }));
    lines.push("    </acceptfont>".to_owned());
    lines.push("  </selectfont>".to_owned());
    lines.push("</fontconfig>".to_owned());
    lines.join("\n") + "\n"
}

fn rename_rules(installed: &[String]) -> Vec<String> {
    let mut taken = BTreeSet::new();
    SUBSTITUTES
        .iter()
        .filter_map(|(source, targets)| {
            let target = first_installed(installed, targets)?;
            taken.insert(target).then(|| {
                format!(
                    "  <match target=\"scan\"><test name=\"family\"><string>{}</string></test><edit name=\"family\" mode=\"assign\" binding=\"same\"><string>{}</string></edit></match>",
                    esc(source),
                    esc(target)
                )
            })
        })
        .collect()
}

fn hide_rules(installed: &[String]) -> Vec<String> {
    let sans = first_installed(installed, &["Arial", "Helvetica"]);
    let mono = first_installed(installed, &["Courier New", "Courier"]);
    SUBSTITUTES
        .iter()
        .filter(|(_, targets)| first_installed(installed, targets).is_some())
        .map(|(source, targets)| {
            let is_mono = targets.iter().any(|t| t.starts_with("Courier"));
            let decoy = if is_mono { sans } else { mono }
                .map(|d| format!("<string>{}</string>", esc(d)))
                .unwrap_or_default();
            format!(
                "  <match target=\"pattern\"><test name=\"family\"><string>{}</string></test><edit name=\"family\" mode=\"assign_replace\" binding=\"strong\"><string>{HIDDEN_FAMILY}</string>{decoy}</edit></match>",
                esc(source)
            )
        })
        .collect()
}

fn generic_rules(installed: &[String]) -> Vec<String> {
    let mut rules: Vec<String> = GENERICS
        .iter()
        .filter_map(|(generic, prefs)| {
            first_installed(installed, prefs).map(|target| {
                format!(
                    "  <match target=\"pattern\"><test name=\"family\"><string>{generic}</string></test><edit name=\"family\" mode=\"prepend\" binding=\"strong\"><string>{}</string></edit></match>",
                    esc(target)
                )
            })
        })
        .collect();
    if let Some(sans) = first_installed(installed, &["Arial", "Helvetica"]) {
        rules.push(format!(
            "  <match target=\"pattern\"><edit name=\"family\" mode=\"append_last\"><string>{}</string></edit></match>",
            esc(sans)
        ));
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn renames_only_to_installed_targets_and_allowlists_profile_fonts() {
        let installed = vec!["Arial".to_owned(), "Segoe UI Emoji".to_owned()];
        let xml = render(&installed, "/c");
        assert!(xml.contains("<string>Liberation Sans</string></test><edit name=\"family\" mode=\"assign\" binding=\"same\"><string>Arial</string>"));
        assert!(xml.contains("<string>Segoe UI Emoji</string></edit>"));
        assert!(!xml.contains("Times New Roman"));
        assert!(
            !xml.contains("<string>Arimo</string></test><edit name=\"family\" mode=\"assign\"")
        );
        assert!(xml.contains(
            "<string>Liberation Sans</string></test><edit name=\"family\" mode=\"assign_replace\""
        ));
        assert!(xml.contains("<patelt name=\"family\"><string>Arial</string></patelt>"));
    }

    #[test]
    fn escapes_xml_in_font_names() {
        let xml = render(&["A&B <x>".to_owned()], "/c");
        assert!(xml.contains("A&amp;B &lt;x&gt;"));
    }
}
