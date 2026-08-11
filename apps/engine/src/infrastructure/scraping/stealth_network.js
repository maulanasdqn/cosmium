(() => {
  const BLOCKED_HOSTS = /^(127\.\d+\.\d+\.\d+|localhost|0\.0\.0\.0|\[::1\])(:\d+)?$/;

  function isBlockedUrl(url) {
    try {
      const u = new URL(url, location.href);
      return BLOCKED_HOSTS.test(u.host);
    } catch { return false; }
  }

  const origFetch = window.fetch;
  window.fetch = function(input, init) {
    const url = typeof input === 'string' ? input : (input && input.url) || '';
    if (isBlockedUrl(url)) return Promise.reject(new TypeError('Failed to fetch'));
    return origFetch.apply(this, arguments);
  };

  const OrigXHR = window.XMLHttpRequest;
  const origOpen = OrigXHR.prototype.open;
  OrigXHR.prototype.open = function(method, url) {
    if (isBlockedUrl(url)) {
      this.__blocked = true;
      return origOpen.call(this, method, 'data:text/plain,');
    }
    return origOpen.apply(this, arguments);
  };
  const origSend = OrigXHR.prototype.send;
  OrigXHR.prototype.send = function() {
    if (this.__blocked) {
      Object.defineProperty(this, 'status', { get: () => 0 });
      Object.defineProperty(this, 'readyState', { get: () => 4 });
      setTimeout(() => {
        if (this.onerror) this.onerror(new Event('error'));
        this.dispatchEvent(new Event('error'));
      }, 0);
      return;
    }
    return origSend.apply(this, arguments);
  };

  const OrigWS = window.WebSocket;
  window.WebSocket = function(url, protocols) {
    if (isBlockedUrl(url)) throw new DOMException('WebSocket connection failed', 'SecurityError');
    return new OrigWS(url, protocols);
  };
  window.WebSocket.prototype = OrigWS.prototype;
  window.WebSocket.CONNECTING = OrigWS.CONNECTING;
  window.WebSocket.OPEN = OrigWS.OPEN;
  window.WebSocket.CLOSING = OrigWS.CLOSING;
  window.WebSocket.CLOSED = OrigWS.CLOSED;

  const origSrc = Object.getOwnPropertyDescriptor(HTMLImageElement.prototype, 'src');
  if (origSrc && origSrc.set) {
    Object.defineProperty(HTMLImageElement.prototype, 'src', {
      set(v) { origSrc.set.call(this, isBlockedUrl(v) ? 'data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==' : v); },
      get() { return origSrc.get.call(this); },
      configurable: true,
    });
  }

  const origEvSrc = window.EventSource;
  if (origEvSrc) {
    window.EventSource = function(url, opts) {
      if (isBlockedUrl(url)) throw new DOMException('EventSource failed', 'SecurityError');
      return new origEvSrc(url, opts);
    };
    window.EventSource.prototype = origEvSrc.prototype;
  }

  const _pf = window.__cosmiumPf;
  if (_pf) {
    _pf.add(window.fetch);
    _pf.add(OrigXHR.prototype.open);
    _pf.add(OrigXHR.prototype.send);
    _pf.add(window.WebSocket);
  }
})();
