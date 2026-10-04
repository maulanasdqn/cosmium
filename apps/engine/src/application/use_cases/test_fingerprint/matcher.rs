pub(crate) enum Matcher {
    Exact(String),
    Contains(&'static str),
    OneOf(Vec<String>),
}

impl Matcher {
    pub(crate) fn is_match(&self, value: &str) -> bool {
        match self {
            Self::Exact(expected) => value == expected,
            Self::Contains(needle) => value.contains(needle),
            Self::OneOf(options) => options.iter().any(|option| option == value),
        }
    }
}

pub(crate) fn intl_locale_probe(tag: &str) -> super::probe::ProbeDef {
    let base = tag.split('-').next().unwrap_or(tag);
    let options = if base == tag {
        vec![base.to_owned()]
    } else {
        vec![tag.to_owned(), base.to_owned()]
    };
    super::probe::ProbeDef {
        id: "intl_locale",
        expression: "new Intl.NumberFormat().resolvedOptions().locale".to_owned(),
        expected_display: options.join(" or "),
        expected: Matcher::OneOf(options),
        negate: false,
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;

    use super::intl_locale_probe;

    #[test]
    fn intl_probe_accepts_base_language_for_regional_tag() {
        let probe = intl_locale_probe("de-DE");
        assert!(probe.expected.is_match("de"));
        assert!(probe.expected.is_match("de-DE"));
        assert!(!probe.expected.is_match("en-US"));
    }

    #[test]
    fn intl_probe_for_bare_language_accepts_only_itself() {
        let probe = intl_locale_probe("ja");
        assert!(probe.expected.is_match("ja"));
        assert!(!probe.expected.is_match("ja-JP"));
    }

    #[test]
    fn one_of_accepts_any_listed_value() {
        let matcher = Matcher::OneOf(vec!["de-DE".into(), "de".into()]);
        assert!(matcher.is_match("de"));
        assert!(matcher.is_match("de-DE"));
        assert!(!matcher.is_match("en-US"));
    }
}
