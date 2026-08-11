use chromiumoxide::cdp::browser_protocol::emulation::{
    SetUserAgentOverrideParams, UserAgentBrandVersion, UserAgentMetadata,
};

pub const STEALTH_SCRIPT: &str = include_str!("stealth.js");
pub const STEALTH_NETWORK_SCRIPT: &str = include_str!("stealth_network.js");

#[derive(Debug, Clone)]
pub struct StealthConfig {
    pub brands_json: String,
    pub full_version_list_json: String,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub bitness: String,
    pub model: String,
    pub mobile: bool,
    pub wow64: bool,
    pub full_version: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub avail_left: u32,
    pub avail_top: u32,
    pub color_depth: u32,
    pub device_pixel_ratio: f32,
    pub locale: String,
    pub timezone: String,
    pub noise_seed: u32,
    pub user_agent: String,
    pub accept_language: String,
    pub history_length: u32,
    pub download_count: u32,
    pub extensions_json: String,
    pub languages_json: String,
    pub hardware_concurrency: u32,
    pub device_memory: f32,
}

impl StealthConfig {
    pub fn ua_data_script(&self) -> String {
        let q = |s: &str| serde_json::to_string(s).unwrap_or_default();
        format!(
            r#"(() => {{
  const brands = {b}; const fvl = {fvl};
  const mobile = {m}; const platform = {p};
  const he = {{
    architecture: {ar}, bitness: {bi}, brands, fullVersionList: fvl, mobile,
    model: {mo}, platform, platformVersion: {pv}, uaFullVersion: {fv}, wow64: {w},
  }};
  const ud = Object.create(NavigatorUAData.prototype);
  Object.defineProperties(ud, {{
    brands:   {{ get: () => brands,   enumerable: true }},
    mobile:   {{ get: () => mobile,   enumerable: true }},
    platform: {{ get: () => platform, enumerable: true }},
  }});
  ud.getHighEntropyValues = function(hints) {{
    const r = {{ brands, mobile, platform }};
    for (const h of hints) {{ if (h in he) r[h] = he[h]; }}
    return Promise.resolve(r);
  }};
  ud.toJSON = function() {{ return {{ brands, mobile, platform }}; }};
  Object.defineProperty(navigator, 'userAgentData', {{ get: () => ud, configurable: true, enumerable: true }});
  const _pf = new Set([ud.getHighEntropyValues, ud.toJSON]);
  if (Function.prototype.toString.__cosmiumProxy) return;
  const _nts = Function.prototype.toString;
  Function.prototype.toString = new Proxy(_nts, {{
    apply(t, self, a) {{
      if (_pf.has(self)) return 'function ' + (self.name || '') + '() {{ [native code] }}';
      return _nts.call(self);
    }}
  }});
  Function.prototype.toString.__cosmiumProxy = true;
}})();"#,
            b = self.brands_json,
            fvl = self.full_version_list_json,
            m = if self.mobile { "true" } else { "false" },
            p = q(&self.platform),
            ar = q(&self.architecture),
            bi = q(&self.bitness),
            mo = q(&self.model),
            pv = q(&self.platform_version),
            fv = q(&self.full_version),
            w = if self.wow64 { "true" } else { "false" },
        )
    }

    pub fn cdp_ua_override(&self) -> SetUserAgentOverrideParams {
        let brands: Option<Vec<UserAgentBrandVersion>> =
            serde_json::from_str(&self.brands_json).ok();
        let fvl: Option<Vec<UserAgentBrandVersion>> =
            serde_json::from_str(&self.full_version_list_json).ok();
        let meta = UserAgentMetadata {
            brands,
            full_version_list: fvl,
            platform: self.platform.clone(),
            platform_version: self.platform_version.clone(),
            architecture: self.architecture.clone(),
            model: self.model.clone(),
            mobile: self.mobile,
            bitness: Some(self.bitness.clone()),
            wow64: Some(self.wow64),
            form_factors: None,
        };
        SetUserAgentOverrideParams {
            user_agent: self.user_agent.clone(),
            accept_language: Some(self.accept_language.clone()),
            platform: Some(self.platform.clone()),
            user_agent_metadata: Some(meta),
        }
    }
}
