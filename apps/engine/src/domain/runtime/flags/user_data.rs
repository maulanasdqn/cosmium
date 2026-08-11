use std::path::PathBuf;

pub fn user_data_dir(profile_name: &str) -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("cosmium")
        .join("profiles");
    base.join(sanitize(profile_name))
}

pub fn session_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("cosmium")
        .join("sessions")
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_data_dir_is_isolated_per_profile() {
        let a = user_data_dir("macos_m2_en-us");
        let b = user_data_dir("win11_rtx3060_en-us");
        assert_ne!(a, b);
        assert!(a.ends_with("macos_m2_en-us"));
    }

    #[test]
    fn user_data_dir_sanitizes_unsafe_chars() {
        let p = user_data_dir("../../etc/passwd");
        let s = p.to_string_lossy();
        assert!(!s.contains(".."));
    }
}
