use crate::domain::profile::Profile;

pub fn accept_lang_switch_value(accept_language: &str) -> String {
    accept_language
        .split(',')
        .filter_map(|part| {
            let lang = part.split(';').next().unwrap_or("").trim();
            (!lang.is_empty()).then(|| lang.to_owned())
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub fn user_agent_for(p: &Profile) -> String {
    p.chrome_version.as_ref().map_or_else(
        || p.identity.user_agent.clone(),
        |ver| rewrite_ua_version(&p.identity.user_agent, ver),
    )
}

pub(super) fn primary_lang(langs: &[String]) -> &str {
    langs.first().map_or("en-US", String::as_str)
}

fn rewrite_ua_version(ua: &str, full_version: &str) -> String {
    let major = full_version.split('.').next().unwrap_or(full_version);
    let Ok(chrome_re) = regex::Regex::new(r"Chrome/\d+\.\d+\.\d+\.\d+") else {
        return ua.to_owned();
    };
    let out = chrome_re.replace(ua, format!("Chrome/{major}.0.0.0"));
    out.into_owned()
}
