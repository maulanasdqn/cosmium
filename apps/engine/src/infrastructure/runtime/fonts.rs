use crate::domain::runtime::FontConfig;

pub(super) fn fontconfig_env(fc: Option<&FontConfig>) -> Option<(String, String)> {
    let fc = fc?;
    let path = fc.dir.join("fonts.conf");
    let current = std::fs::read_to_string(&path).ok();
    if current.as_deref() != Some(fc.xml.as_str()) {
        if let Err(e) =
            std::fs::create_dir_all(&fc.dir).and_then(|()| std::fs::write(&path, &fc.xml))
        {
            tracing::warn!(error = %e, "could not write fontconfig, using system fonts");
            return None;
        }
    }
    Some(("FONTCONFIG_FILE".into(), path.display().to_string()))
}
