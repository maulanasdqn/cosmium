use super::stealth::StealthConfig;

impl StealthConfig {
    pub fn navigator_overrides_script(&self) -> String {
        format!(
            r#"(() => {{
  const langs = {langs};
  const hc = {hc};
  const dm = {dm};
  const d = (o, k, v) => Object.defineProperty(o, k, {{ get: () => v, configurable: true, enumerable: true }});
  d(navigator, 'languages', Object.freeze(langs));
  d(navigator, 'language', langs[0]);
  d(navigator, 'hardwareConcurrency', hc);
  d(navigator, 'deviceMemory', dm);
}})();"#,
            langs = self.languages_json,
            hc = self.hardware_concurrency,
            dm = self.device_memory,
        )
    }

    pub fn screen_script(&self) -> String {
        format!(
            r#"(() => {{
  const S = screen; const W = window;
  const d = (o, k, v) => Object.defineProperty(o, k, {{ get: () => v, configurable: true }});
  d(S, 'width', {w}); d(S, 'height', {h});
  d(S, 'availWidth', {aw}); d(S, 'availHeight', {ah});
  d(S, 'availLeft', {al}); d(S, 'availTop', {at});
  d(S, 'colorDepth', {cd}); d(S, 'pixelDepth', {cd});
  d(W, 'devicePixelRatio', {dpr});
  const origMM = W.matchMedia.bind(W);
  W.matchMedia = function(q) {{
    const r = origMM(q);
    const dw = /\(device-width:\s*(\d+)px\)/.exec(q);
    const dh = /\(device-height:\s*(\d+)px\)/.exec(q);
    if (dw && parseInt(dw[1]) !== {w}) return {{ ...r, matches: false }};
    if (dh && parseInt(dh[1]) !== {h}) return {{ ...r, matches: false }};
    if (dw && parseInt(dw[1]) === {w}) return {{ ...r, matches: true }};
    if (dh && parseInt(dh[1]) === {h}) return {{ ...r, matches: true }};
    return r;
  }};
}})();"#,
            w = self.screen_width,
            h = self.screen_height,
            aw = self.avail_width,
            ah = self.avail_height,
            al = self.avail_left,
            at = self.avail_top,
            cd = self.color_depth,
            dpr = self.device_pixel_ratio,
        )
    }

    pub fn client_rects_script(&self) -> String {
        format!(
            r#"(() => {{
  const seed = {seed};
  function hash(a, b, c, d) {{
    let h = seed ^ (a * 374761393 + b * 668265263 + c * 1274126177 + d * 1150885431) | 0;
    h = Math.imul(h ^ (h >>> 13), 1274126177);
    h = h ^ (h >>> 16);
    return ((h & 0xfff) - 2048) / 204800;
  }}
  const origBCR = Element.prototype.getBoundingClientRect;
  Element.prototype.getBoundingClientRect = function() {{
    const r = origBCR.call(this);
    const n = hash(r.x|0, r.y|0, r.width|0, r.height|0);
    return new DOMRect(r.x + n, r.y + n, r.width + n, r.height + n);
  }};
  const origGCR = Element.prototype.getClientRects;
  Element.prototype.getClientRects = function() {{
    const rects = origGCR.call(this);
    const out = [];
    for (let i = 0; i < rects.length; i++) {{
      const r = rects[i];
      const n = hash(r.x|0, r.y|0, r.width|0, i);
      out.push(new DOMRect(r.x + n, r.y + n, r.width + n, r.height + n));
    }}
    return out;
  }};
}})();"#,
            seed = self.noise_seed,
        )
    }

    pub fn intl_script(&self) -> String {
        let q = |s: &str| serde_json::to_string(s).unwrap_or_default();
        format!(
            r#"(() => {{
  const loc = {locale}; const tz = {tz};
  const wrap = (Ctor) => {{
    const Orig = Ctor;
    return function(...args) {{
      if (args.length === 0 || args[0] === undefined) args[0] = loc;
      return new Orig(...args);
    }};
  }};
  Intl.DateTimeFormat = wrap(Intl.DateTimeFormat);
  Intl.NumberFormat = wrap(Intl.NumberFormat);
  if (Intl.ListFormat) Intl.ListFormat = wrap(Intl.ListFormat);
  if (Intl.RelativeTimeFormat) Intl.RelativeTimeFormat = wrap(Intl.RelativeTimeFormat);
  if (Intl.DisplayNames) Intl.DisplayNames = wrap(Intl.DisplayNames);
  if (Intl.PluralRules) Intl.PluralRules = wrap(Intl.PluralRules);
}})();"#,
            locale = q(&self.locale),
            tz = q(&self.timezone),
        )
    }

    pub fn browser_state_script(&self) -> String {
        format!(
            r#"(() => {{
  const HL = {hl};
  const origDesc = Object.getOwnPropertyDescriptor(History.prototype, 'length');
  Object.defineProperty(History.prototype, 'length', {{
    get() {{ const real = origDesc.get.call(this); return real < HL ? HL : real; }},
    configurable: true,
  }});
  const exts = {exts};
  const dlCount = {dl};
  if (window.chrome) {{
    if (!chrome.downloads) chrome.downloads = {{}};
    const fakeItems = Array.from({{ length: dlCount }}, (_, i) => ({{
      id: i + 1, url: 'https://example.com/file' + (i + 1) + '.pdf',
      filename: 'file' + (i + 1) + '.pdf', state: 'complete',
      totalBytes: 102400 + i * 51200, bytesReceived: 102400 + i * 51200,
      exists: true, canResume: false, paused: false, danger: 'safe',
      mime: 'application/pdf', startTime: new Date(Date.now() - (i + 1) * 86400000).toISOString(),
    }}));
    chrome.downloads.search = function(q, cb) {{
      if (typeof cb === 'function') cb(fakeItems);
      return Promise.resolve(fakeItems);
    }};
    chrome.downloads.getFileIcon = function(id, opts, cb) {{
      const fn = cb || opts;
      if (typeof fn === 'function') fn('data:image/png;base64,iVBOR');
      return Promise.resolve('data:image/png;base64,iVBOR');
    }};
    if (!chrome.management) chrome.management = {{}};
    chrome.management.getAll = function(cb) {{
      if (typeof cb === 'function') cb(exts);
      return Promise.resolve(exts);
    }};
    chrome.management.getSelf = function(cb) {{
      const self = {{ id: 'cosmium', name: 'Chrome', type: 'extension',
        enabled: true, version: '1.0', mayDisable: false }};
      if (typeof cb === 'function') cb(self);
      return Promise.resolve(self);
    }};
  }}
  const _pf = window.__cosmiumPf;
  if (_pf) {{
    if (chrome.downloads) {{
      _pf.add(chrome.downloads.search);
      _pf.add(chrome.downloads.getFileIcon);
    }}
    if (chrome.management) {{
      _pf.add(chrome.management.getAll);
      _pf.add(chrome.management.getSelf);
    }}
  }}
}})();"#,
            hl = self.history_length,
            dl = self.download_count,
            exts = self.extensions_json,
        )
    }
}
