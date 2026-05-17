use crate::domain::profile::identity::Identity;

pub(super) fn platform_of(id: &Identity) -> Option<&'static str> {
    let ua = &id.user_agent;
    if ua.contains("Windows NT") {
        Some("Windows")
    } else if ua.contains("Mac OS X") || ua.contains("Macintosh") {
        Some("macOS")
    } else if ua.contains("Linux") || ua.contains("X11") {
        Some("Linux")
    } else {
        None
    }
}
