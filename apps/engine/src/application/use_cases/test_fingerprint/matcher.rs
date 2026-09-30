pub(crate) enum Matcher {
    Exact(String),
    Contains(&'static str),
}

impl Matcher {
    pub(crate) fn is_match(&self, value: &str) -> bool {
        match self {
            Self::Exact(expected) => value == expected,
            Self::Contains(needle) => value.contains(needle),
        }
    }
}
