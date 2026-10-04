const BLOCKED_STATUSES: [i32; 3] = [403, 429, 503];

const WALL_URL_SEGMENTS: [&str; 7] = [
    "/captcha/",
    "/account-verification",
    "/negative_traffic",
    "/challenge/",
    "/verify/traffic",
    "/verify/bot",
    "/blocked/",
];

const CHALLENGE_MARKERS: [&str; 13] = [
    "just a moment",
    "checking your browser",
    "ddos protection",
    "verify you are human",
    "confirm you are human",
    "attention required",
    "acceso ha sido denegado",
    "access denied",
    "press & hold",
    "captcha-delivery.com",
    "awswaf.com",
    "please enable js and disable any ad blocker",
    "access is temporarily restricted",
];

const TITLE_BLOCK_MARKERS: [&str; 3] = ["just a moment", "access denied", "attention required"];

const SNIFF_BYTES: usize = 8192;
const SHORT_PAGE_BYTES: usize = 2048;

pub fn is_blocked(http_status: i32, body: &[u8], final_url: &str) -> bool {
    if BLOCKED_STATUSES.contains(&http_status) {
        return true;
    }
    if landed_on_wall(final_url) {
        return true;
    }
    let head = String::from_utf8_lossy(body.get(..SNIFF_BYTES).unwrap_or(body)).to_lowercase();
    is_block_page(&head, body.len())
}

fn is_block_page(html: &str, body_len: usize) -> bool {
    if let Some(title) = extract_title(html) {
        if TITLE_BLOCK_MARKERS.iter().any(|m| title.contains(m)) {
            return true;
        }
    }
    if body_len <= SHORT_PAGE_BYTES {
        let marker_count = CHALLENGE_MARKERS
            .iter()
            .filter(|m| html.contains(*m))
            .count();
        return marker_count >= 1;
    }
    let marker_count = CHALLENGE_MARKERS
        .iter()
        .filter(|m| html.contains(*m))
        .count();
    marker_count >= 2
}

fn extract_title(html: &str) -> Option<&str> {
    let start = html.find("<title")?.checked_add(6)?;
    let rest = html.get(start..)?;
    let content_start = rest.find('>')?.checked_add(1)?;
    let content = rest.get(content_start..)?;
    let end = content.find("</title")?;
    content.get(..end)
}

pub fn is_challenge_page(html: &str) -> bool {
    let lower = html.to_lowercase();
    CHALLENGE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

fn landed_on_wall(final_url: &str) -> bool {
    let url = final_url.to_lowercase();
    WALL_URL_SEGMENTS.iter().any(|seg| url.contains(seg))
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
    fn status_429_is_blocked() {
        assert!(is_blocked(429, b"<html></html>", "https://example.com"));
    }

    #[test]
    fn status_403_is_blocked() {
        assert!(is_blocked(403, b"<html></html>", "https://example.com"));
    }

    #[test]
    fn captcha_url_segment_is_blocked() {
        assert!(is_blocked(
            200,
            b"<html></html>",
            "https://example.com/captcha/wall?go=https://shop.example.com"
        ));
    }

    #[test]
    fn title_with_block_marker_is_blocked() {
        assert!(is_blocked(
            200,
            b"<html><title>Just a moment...</title><body>checking</body></html>",
            "https://example.com"
        ));
    }

    #[test]
    fn short_body_with_one_challenge_marker_is_blocked() {
        assert!(is_blocked(
            200,
            b"<html><body>checking your browser</body></html>",
            "https://example.com"
        ));
    }

    #[test]
    fn long_body_needs_two_markers() {
        let mut body = b"<html><body>".to_vec();
        body.extend(vec![b'x'; 4000]);
        body.extend(b"captcha-delivery.com");
        body.extend(b"</body></html>");
        assert!(!is_blocked(200, &body, "https://example.com"));

        let mut body2 = b"<html><body>".to_vec();
        body2.extend(vec![b'x'; 4000]);
        body2.extend(b"captcha-delivery.com verify you are human");
        body2.extend(b"</body></html>");
        assert!(is_blocked(200, &body2, "https://example.com"));
    }

    #[test]
    fn wikipedia_article_about_captchas_not_blocked() {
        let mut body = b"<html><head><title>CAPTCHA - Wikipedia</title></head><body>".to_vec();
        body.extend(b"A CAPTCHA is a type of challenge-response test used in computing ");
        body.extend(b"to determine whether the user is human. The term captcha was coined ");
        body.extend(vec![b'x'; 6000]);
        body.extend(b"</body></html>");
        assert!(!is_blocked(
            200,
            &body,
            "https://en.wikipedia.org/wiki/CAPTCHA"
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
