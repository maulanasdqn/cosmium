pub const STEALTH_SCRIPT: &str = r#"
(() => {
  // ── 1. chrome.runtime ─────────────────────────────────────────────
  // Headless Chrome doesn't create window.chrome.runtime, but every
  // real Chrome install has it.  DataDome checks its existence.
  if (!window.chrome) { window.chrome = {}; }
  if (!window.chrome.runtime) {
    window.chrome.runtime = {
      connect: function() {},
      sendMessage: function() {},
      id: undefined,
    };
  }

  // ── 2. navigator.plugins ──────────────────────────────────────────
  // Headless has an empty PluginArray.  Real Chrome on Mac has ≥ 2.
  // We define a fake PluginArray with the standard PDF plugin.
  if (navigator.plugins.length === 0) {
    const fakePlugin = {
      0: { type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format' },
      name: 'PDF Viewer',
      description: 'Portable Document Format',
      filename: 'internal-pdf-viewer',
      length: 1,
    };
    Object.setPrototypeOf(fakePlugin, Plugin.prototype);
    Object.setPrototypeOf(fakePlugin[0], MimeType.prototype);

    const fakePlugins = {
      0: fakePlugin,
      length: 1,
      item: function(i) { return this[i] || null; },
      namedItem: function(n) { return this[0] && this[0].name === n ? this[0] : null; },
      refresh: function() {},
    };
    Object.setPrototypeOf(fakePlugins, PluginArray.prototype);

    Object.defineProperty(navigator, 'plugins', {
      get: () => fakePlugins,
      configurable: true,
    });

    // Also fix navigator.mimeTypes
    const fakeMimeTypes = {
      0: fakePlugin[0],
      length: 1,
      item: function(i) { return this[i] || null; },
      namedItem: function(n) {
        return this[0] && this[0].type === n ? this[0] : null;
      },
    };
    Object.setPrototypeOf(fakeMimeTypes, MimeTypeArray.prototype);
    Object.defineProperty(navigator, 'mimeTypes', {
      get: () => fakeMimeTypes,
      configurable: true,
    });
  }

  // ── 3. Permissions API ────────────────────────────────────────────
  // In headless, Permissions.query for 'notifications' returns
  // "prompt" instead of the more common "denied" seen in real browsers.
  const origQuery = window.Permissions && Permissions.prototype.query;
  if (origQuery) {
    Permissions.prototype.query = function(parameters) {
      return parameters.name === 'notifications'
        ? Promise.resolve({ state: Notification.permission || 'denied', onchange: null })
        : origQuery.call(this, parameters);
    };
  }

  // ── 4. iframe contentWindow ───────────────────────────────────────
  // Headless Chrome sometimes exposes a null contentWindow on
  // newly-created cross-origin iframes before they load.  Some
  // detectors probe this.
  try {
    const origCreateElement = document.createElement.bind(document);
    // Only patch if it's still the native function
    if (document.createElement.toString().includes('[native code]')) {
      // No-op: the iframe contentWindow leak is only present in older
      // headless; --headless=new in Chromium 112+ behaves like headed.
    }
  } catch {}

  // ── 5. Outer/inner dimensions ─────────────────────────────────────
  // Headless often has window.outerWidth/outerHeight === 0.
  if (window.outerWidth === 0) {
    Object.defineProperty(window, 'outerWidth', {
      get: () => window.innerWidth,
      configurable: true,
    });
  }
  if (window.outerHeight === 0) {
    Object.defineProperty(window, 'outerHeight', {
      get: () => window.innerHeight + 85, // chrome UI bar
      configurable: true,
    });
  }

  // ── 6. connection.rtt ─────────────────────────────────────────────
  // Headless often reports rtt=0 on navigator.connection — real
  // browsers have a non-zero value.
  if (navigator.connection && navigator.connection.rtt === 0) {
    Object.defineProperty(navigator.connection, 'rtt', {
      get: () => 50,
      configurable: true,
    });
  }

  // ── 7. Prevent toString detection ─────────────────────────────────
  // Some detectors check that overridden functions still look native.
  const nativeToString = Function.prototype.toString;
  const patchedFns = new Set();
  const handler = {
    apply: function(target, thisArg, args) {
      if (patchedFns.has(thisArg)) {
        return 'function ' + (thisArg.name || '') + '() { [native code] }';
      }
      return nativeToString.call(thisArg);
    }
  };
  Function.prototype.toString = new Proxy(nativeToString, handler);
  patchedFns.add(Function.prototype.toString);
  if (origQuery) patchedFns.add(Permissions.prototype.query);
})();
"#;
