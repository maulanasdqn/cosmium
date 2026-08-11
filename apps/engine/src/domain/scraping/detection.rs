const BLOCKED_STATUSES: [i16; 3] = [403, 429, 503];

const BLOCKED_MARKERS: [&str; 6] = [
    "captcha",
    "are you a robot",
    "access denied",
    "cf-browser-verification",
    "just a moment",
    "unusual traffic",
];

const WALL_URL_MARKERS: [&str; 7] = [
    "captcha",
    "account-verification",
    "negative_traffic",
    "/challenge",
    "verify/traffic",
    "/verify/bot",
    "/blocked",
];

const CHALLENGE_MARKERS: [&str; 11] = [
    "just a moment",
    "checking your browser",
    "ddos protection",
    "verify you are human",
    "attention required",
    "acceso ha sido denegado",
    "access denied",
    "press & hold",
    "captcha-delivery.com",
    "please enable js and disable any ad blocker",
    "access is temporarily restricted",
];

const SNIFF_BYTES: usize = 4096;

pub fn is_blocked(http_status: i16, body: &[u8], final_url: &str) -> bool {
    let head = String::from_utf8_lossy(&body[..body.len().min(SNIFF_BYTES)]).to_lowercase();
    if BLOCKED_STATUSES.contains(&http_status) {
        if is_challenge_page(&head) {
            return true;
        }
        return !looks_like_real_content(&head);
    }
    if landed_on_wall(final_url) {
        return true;
    }
    BLOCKED_MARKERS.iter().any(|marker| head.contains(marker))
}

fn looks_like_real_content(html: &str) -> bool {
    html.len() > SNIFF_BYTES && !BLOCKED_MARKERS.iter().any(|m| html.contains(m))
}

pub fn is_challenge_page(html: &str) -> bool {
    let lower = html.to_lowercase();
    CHALLENGE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

fn landed_on_wall(final_url: &str) -> bool {
    let url = final_url.to_lowercase();
    WALL_URL_MARKERS.iter().any(|marker| url.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_page_not_blocked() {
        assert!(!is_blocked(
            200,
            b"<html><body>products</body></html>",
            "https://example.com/shop"
        ));
    }

    #[test]
    fn status_403_is_blocked() {
        assert!(is_blocked(403, b"<html></html>", "https://example.com"));
    }

    #[test]
    fn captcha_url_is_blocked() {
        assert!(is_blocked(
            200,
            b"<html></html>",
            "https://example.com/captcha/wall?go=https://shop.example.com"
        ));
    }

    #[test]
    fn body_marker_is_blocked() {
        assert!(is_blocked(
            200,
            b"<html>Just a moment...</html>",
            "https://example.com"
        ));
    }

    #[test]
    fn challenge_page_detected() {
        assert!(is_challenge_page("<title>Just a moment...</title>"));
        assert!(is_challenge_page("Checking your browser before accessing"));
    }

    #[test]
    fn normal_page_not_challenge() {
        assert!(!is_challenge_page("<html><body>Hello world</body></html>"));
    }
}
