use super::stealth::StealthConfig;

const WEBGL_TEMPLATE: &str = r#"(() => {
  const V = __VENDOR__;
  const R = __RENDERER__;
  const pf = window.__cosmiumPf;
  const asFn = (fn) => { if (pf) pf.add(fn); return fn; };
  const patch = (proto) => {
    if (!proto || !proto.getParameter) return;
    const orig = proto.getParameter;
    proto.getParameter = asFn(function getParameter(p) {
      if (p === 37445) return V;
      if (p === 37446) return R;
      return orig.call(this, p);
    });
  };
  patch(window.WebGLRenderingContext && WebGLRenderingContext.prototype);
  patch(window.WebGL2RenderingContext && WebGL2RenderingContext.prototype);
})();"#;

const CANVAS_TEMPLATE: &str = r#"(() => {
  const seed = __SEED__;
  const pf = window.__cosmiumPf;
  const asFn = (fn) => { if (pf) pf.add(fn); return fn; };
  const rnd = (n) => {
    let x = (seed ^ Math.imul(n | 0, 2654435761)) >>> 0;
    x ^= x << 13; x >>>= 0; x ^= x >>> 17; x ^= x << 5; x >>>= 0;
    return x;
  };
  const perturb = (data) => {
    for (let i = 0; i < data.length; i += 4) {
      const r = rnd(i);
      if ((r & 0xff) < 6) {
        const ch = i + (r % 3);
        const v = data[ch] + ((r & 0x100) ? 1 : -1);
        data[ch] = v < 0 ? 0 : v > 255 ? 255 : v;
      }
    }
  };
  const C2D = window.CanvasRenderingContext2D;
  if (!C2D) return;
  const origGID = C2D.prototype.getImageData;
  const origPID = C2D.prototype.putImageData;
  C2D.prototype.getImageData = asFn(function getImageData() {
    const img = origGID.apply(this, arguments);
    perturb(img.data);
    return img;
  });
  const noiseCanvas = (canvas) => {
    try {
      const ctx = canvas.getContext && canvas.getContext('2d');
      if (!ctx) return;
      const w = canvas.width, h = canvas.height;
      if (!w || !h) return;
      const img = origGID.call(ctx, 0, 0, w, h);
      perturb(img.data);
      origPID.call(ctx, img, 0, 0);
    } catch (_) {}
  };
  const HC = window.HTMLCanvasElement;
  if (!HC) return;
  const origTDU = HC.prototype.toDataURL;
  HC.prototype.toDataURL = asFn(function toDataURL() {
    noiseCanvas(this);
    return origTDU.apply(this, arguments);
  });
  const origTB = HC.prototype.toBlob;
  if (origTB) {
    HC.prototype.toBlob = asFn(function toBlob() {
      noiseCanvas(this);
      return origTB.apply(this, arguments);
    });
  }
})();"#;

const AUDIO_TEMPLATE: &str = r#"(() => {
  const seed = __SEED__;
  const BL = __BL__;
  const pf = window.__cosmiumPf;
  const asFn = (fn) => { if (pf) pf.add(fn); return fn; };
  const noise = (n) => {
    let x = (seed ^ Math.imul(n | 0, 2246822519)) >>> 0;
    x ^= x << 13; x >>>= 0; x ^= x >>> 17; x ^= x << 5; x >>>= 0;
    return (x / 4294967295 - 0.5) * 2e-7;
  };
  const seen = new WeakSet();
  const AB = window.AudioBuffer;
  if (AB) {
    const origGCD = AB.prototype.getChannelData;
    AB.prototype.getChannelData = asFn(function getChannelData(ch) {
      const data = origGCD.call(this, ch);
      if (!seen.has(data)) {
        for (let i = 0; i < data.length; i += 97) data[i] += noise(i + ch * 131);
        seen.add(data);
      }
      return data;
    });
    const origCFC = AB.prototype.copyFromChannel;
    if (origCFC) {
      AB.prototype.copyFromChannel = asFn(function copyFromChannel(dest, ch, start) {
        origCFC.call(this, dest, ch, start);
        for (let i = 0; i < dest.length; i += 97) dest[i] += noise(i + ch * 131);
      });
    }
  }
  const AN = window.AnalyserNode;
  if (AN) {
    const origFFD = AN.prototype.getFloatFrequencyData;
    AN.prototype.getFloatFrequencyData = asFn(function getFloatFrequencyData(arr) {
      origFFD.call(this, arr);
      for (let i = 0; i < arr.length; i += 13) arr[i] += noise(i) * 1e3;
    });
  }
  const proto = window.AudioContext && AudioContext.prototype;
  if (proto && BL > 0) {
    try {
      Object.defineProperty(proto, 'baseLatency', {
        get: asFn(function baseLatency() { return BL; }), configurable: true,
      });
    } catch (_) {}
  }
})();"#;

impl StealthConfig {
    pub fn webgl_script(&self) -> String {
        let v = serde_json::to_string(&self.webgl_vendor).unwrap_or_else(|_| "\"\"".into());
        let r = serde_json::to_string(&self.webgl_renderer).unwrap_or_else(|_| "\"\"".into());
        WEBGL_TEMPLATE
            .replace("__VENDOR__", &v)
            .replace("__RENDERER__", &r)
    }

    pub fn canvas_script(&self) -> String {
        CANVAS_TEMPLATE.replace("__SEED__", &self.noise_seed.to_string())
    }

    pub fn audio_script(&self) -> String {
        AUDIO_TEMPLATE
            .replace("__SEED__", &self.noise_seed.to_string())
            .replace("__BL__", &format!("{}", self.audio_base_latency))
    }
}

#[cfg(test)]
#[path = "stealth_scripts_fp_tests.rs"]
mod tests;
