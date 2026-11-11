use std::path::PathBuf;

pub fn user_data_dir(profile_name: &str) -> PathBuf {
    let base = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("cosmium")
        .join("profiles");
    base.join(sanitize(profile_name))
}

pub fn user_data_dir_for(profile_name: &str, proxy: Option<&str>) -> PathBuf {
    proxy.map_or_else(
        || user_data_dir(profile_name),
        |url| {
            user_data_dir(&format!(
                "{profile_name}-proxy-{:016x}",
                fnv1a(url.as_bytes())
            ))
        },
    )
}

pub fn session_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("cosmium")
        .join("sessions")
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
    })
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

    #[test]
    fn direct_scrapes_keep_the_profile_dir() {
        assert_eq!(user_data_dir_for("win11", None), user_data_dir("win11"));
    }

    #[test]
    fn each_proxy_gets_a_stable_dir_of_its_own() {
        let a = user_data_dir_for("win11", Some("http://u:p@1.2.3.4:8080"));
        let again = user_data_dir_for("win11", Some("http://u:p@1.2.3.4:8080"));
        let b = user_data_dir_for("win11", Some("http://u:p@5.6.7.8:8080"));
        assert_eq!(a, again);
        assert_ne!(a, b);
        assert_ne!(a, user_data_dir("win11"));
    }

    #[test]
    fn proxy_credentials_never_reach_the_path() {
        let p = user_data_dir_for("win11", Some("http://alice:hunter2@1.2.3.4:8080"));
        let s = p.to_string_lossy();
        assert!(!s.contains("alice"));
        assert!(!s.contains("hunter2"));
    }
}
