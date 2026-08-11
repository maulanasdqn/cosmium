(() => {
  const nts = Function.prototype.toString;
  const pf = new Set();
  Object.defineProperty(window, '__cosmiumPf', { value: pf, configurable: true, enumerable: false });
  const h = {
    apply(target, thisArg, args) {
      if (pf.has(thisArg)) return 'function ' + (thisArg.name || '') + '() { [native code] }';
      return nts.call(thisArg);
    }
  };
  Function.prototype.toString = new Proxy(nts, h);
  Function.prototype.toString.__cosmiumProxy = true;
  pf.add(Function.prototype.toString);

  const asFn = (fn) => { pf.add(fn); return fn; };

  const STACK_NOISE = ['cdc_', '__webdriver', 'chromiumoxide', 'puppeteer', '__playwright', 'selenium'];
  const origCapture = Error.captureStackTrace;
  if (origCapture) {
    Error.captureStackTrace = asFn(function(target, ctor) {
      origCapture.call(this, target, ctor);
      if (target.stack) target.stack = target.stack.split('\n').filter(l => !STACK_NOISE.some(n => l.includes(n))).join('\n');
    });
  }
  const origPrepare = Error.prepareStackTrace;
  Error.prepareStackTrace = function(err, frames) {
    const cleaned = frames.filter(f => {
      const name = (f.getFunctionName && f.getFunctionName()) || '';
      const file = (f.getFileName && f.getFileName()) || '';
      return !STACK_NOISE.some(n => name.includes(n) || file.includes(n));
    });
    return origPrepare ? origPrepare(err, cleaned) : err + '\n' + cleaned.map(f => '    at ' + f).join('\n');
  };

  const AUTOMATION_VARS = [
    'callPhantom', '_phantom', '__phantomas', 'domAutomation', 'domAutomationController',
    '__selenium_evaluate', '__fxdriver_evaluate', '__driver_evaluate',
    '__webdriver_evaluate', '__selenium_unwrapped', '__fxdriver_unwrapped',
    '__driver_unwrapped', '__webdriver_unwrapped', '__puppeteer_evaluation_script__',
  ];
  for (const v of AUTOMATION_VARS) { try { if (v in window) delete window[v]; } catch (_) {} }
  try {
    for (const key of Object.getOwnPropertyNames(window)) {
      if (key.startsWith('cdc_') || key.startsWith('$cdc_') || key.startsWith('$chrome_')) {
        try { delete window[key]; } catch (_) {}
      }
    }
  } catch (_) {}

  const NavProto = Object.getPrototypeOf(navigator);
  try { if ('webdriver' in NavProto) delete NavProto.webdriver; } catch (_) {}
  Object.defineProperty(NavProto, 'webdriver', {
    get: asFn(function() { return false; }), enumerable: true, configurable: true,
  });

  if (!window.chrome) window.chrome = {};
  if (!window.chrome.runtime) {
    window.chrome.runtime = {
      connect: asFn(function(){}), sendMessage: asFn(function(){}), id: undefined,
      PlatformOs: { MAC:'mac', WIN:'win', ANDROID:'android', CROS:'cros', LINUX:'linux', OPENBSD:'openbsd' },
      PlatformArch: { ARM:'arm', X86_32:'x86-32', X86_64:'x86-64', MIPS:'mips', MIPS64:'mips64' },
      PlatformNaclArch: { ARM:'arm', X86_32:'x86-32', X86_64:'x86-64', MIPS:'mips', MIPS64:'mips64' },
      RequestUpdateCheckStatus: { THROTTLED:'throttled', NO_UPDATE:'no_update', UPDATE_AVAILABLE:'update_available' },
      OnInstalledReason: { INSTALL:'install', UPDATE:'update', CHROME_UPDATE:'chrome_update', SHARED_MODULE_UPDATE:'shared_module_update' },
      OnRestartRequiredReason: { APP_UPDATE:'app_update', OS_UPDATE:'os_update', PERIODIC:'periodic' },
    };
  }
  if (!window.chrome.app) {
    window.chrome.app = {
      isInstalled: false,
      InstallState: { DISABLED:'disabled', INSTALLED:'installed', NOT_INSTALLED:'not_installed' },
      RunningState: { CANNOT_RUN:'cannot_run', READY_TO_RUN:'ready_to_run', RUNNING:'running' },
      getDetails: asFn(function(){ return null; }),
      getIsInstalled: asFn(function(){ return false; }),
      installState: asFn(function(cb){ if(cb) cb('not_installed'); }),
      runningState: asFn(function(){ return 'cannot_run'; }),
    };
  }
  if (!window.chrome.csi) {
    window.chrome.csi = asFn(function() {
      return { onloadT: 0, startE: Date.now(), pageT: performance.now(), tran: 15 };
    });
  }
  if (!window.chrome.loadTimes) {
    window.chrome.loadTimes = asFn(function() {
      const now = Date.now() / 1000;
      return {
        commitLoadTime: now, connectionInfo: 'h2', finishDocumentLoadTime: 0,
        finishLoadTime: 0, firstPaintAfterLoadTime: 0, firstPaintTime: 0,
        navigationType: 'Other', npnNegotiatedProtocol: 'h2',
        requestTime: now - 0.16, startLoadTime: now - 0.16,
        wasAlternateProtocolAvailable: false, wasFetchedViaSpdy: true, wasNpnNegotiated: true,
      };
    });
  }

  if (navigator.plugins.length === 0) {
    const fp = {
      0: { type:'application/pdf', suffixes:'pdf', description:'Portable Document Format' },
      name:'PDF Viewer', description:'Portable Document Format', filename:'internal-pdf-viewer', length:1,
    };
    Object.setPrototypeOf(fp, Plugin.prototype);
    Object.setPrototypeOf(fp[0], MimeType.prototype);
    const fps = { 0:fp, length:1, item:asFn(function(i){return this[i]||null}), namedItem:asFn(function(n){return this[0]&&this[0].name===n?this[0]:null}), refresh:asFn(function(){}) };
    Object.setPrototypeOf(fps, PluginArray.prototype);
    Object.defineProperty(navigator, 'plugins', { get:()=>fps, configurable:true });
    const fmt = { 0:fp[0], length:1, item:asFn(function(i){return this[i]||null}), namedItem:asFn(function(n){return this[0]&&this[0].type===n?this[0]:null}) };
    Object.setPrototypeOf(fmt, MimeTypeArray.prototype);
    Object.defineProperty(navigator, 'mimeTypes', { get:()=>fmt, configurable:true });
  }

  const origQuery = window.Permissions && Permissions.prototype.query;
  if (origQuery) {
    Permissions.prototype.query = asFn(function(p) {
      return p.name === 'notifications'
        ? Promise.resolve({ state: Notification.permission || 'denied', onchange: null })
        : origQuery.call(this, p);
    });
  }

  if (window.outerWidth === 0) Object.defineProperty(window, 'outerWidth', { get:()=>window.innerWidth, configurable:true });
  if (window.outerHeight === 0) Object.defineProperty(window, 'outerHeight', { get:()=>window.innerHeight+85, configurable:true });
  if (navigator.connection && navigator.connection.rtt === 0)
    Object.defineProperty(navigator.connection, 'rtt', { get:()=>50, configurable:true });

  Document.prototype.hasFocus = asFn(function(){ return true; });
  Object.defineProperty(document, 'visibilityState', { get:()=>'visible', configurable:true });
  Object.defineProperty(document, 'hidden', { get:()=>false, configurable:true });
  if (navigator.pdfViewerEnabled === undefined)
    Object.defineProperty(navigator, 'pdfViewerEnabled', { get:()=>true, configurable:true });
  try {
    if (Notification.permission === 'denied')
      Object.defineProperty(Notification, 'permission', { get:()=>'default', configurable:true });
  } catch {}

  try {
    const origSrcDesc = Object.getOwnPropertyDescriptor(HTMLIFrameElement.prototype, 'contentWindow');
    if (origSrcDesc && origSrcDesc.get) {
      const origGet = origSrcDesc.get;
      Object.defineProperty(HTMLIFrameElement.prototype, 'contentWindow', {
        get: asFn(function() {
          const w = origGet.call(this);
          if (w) {
            try {
              if (!w.chrome) w.chrome = window.chrome;
              const nv = Object.getPrototypeOf(w.navigator);
              if (nv && nv.webdriver !== false) {
                try { delete nv.webdriver; } catch(_) {}
                Object.defineProperty(nv, 'webdriver', { get:()=>false, configurable:true, enumerable:true });
              }
            } catch (_) {}
          }
          return w;
        }),
        configurable: true, enumerable: true,
      });
    }
  } catch (_) {}
})();
