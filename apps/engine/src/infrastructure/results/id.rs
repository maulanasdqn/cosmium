use std::fmt::Write;

use rand::Rng;

pub(super) fn new_id() -> String {
    let mut bytes: [u8; 16] = rand::rng().random();
    if let Some(byte) = bytes.get_mut(6) {
        *byte = (*byte & 0x0f) | 0x40;
    }
    if let Some(byte) = bytes.get_mut(8) {
        *byte = (*byte & 0x3f) | 0x80;
    }
    let hex = bytes.iter().fold(String::with_capacity(32), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    });
    [0..8, 8..12, 12..16, 16..20, 20..32]
        .iter()
        .filter_map(|range| hex.get(range.clone()))
        .collect::<Vec<_>>()
        .join("-")
}

pub fn is_valid_id(id: &str) -> bool {
    id.len() == 36
        && id.char_indices().all(|(index, c)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                c == '-'
            } else {
                c.is_ascii_hexdigit() && !c.is_ascii_uppercase()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::{is_valid_id, new_id};

    #[test]
    fn generates_valid_v4_ids() {
        let id = new_id();
        assert!(is_valid_id(&id), "{id}");
        assert_eq!(id.chars().nth(14), Some('4'));
        assert_ne!(new_id(), id);
    }

    #[test]
    fn rejects_traversal_and_malformed_ids() {
        assert!(!is_valid_id("../../etc/passwd"));
        assert!(!is_valid_id("not-a-uuid"));
        assert!(!is_valid_id("00000000-0000-4000-8000-00000000000G"));
        assert!(is_valid_id("00000000-0000-4000-8000-000000000000"));
    }
}
