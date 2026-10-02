await new Promise((resolve) => {
  let tries = 0;
  const after = (lines, label) => {
    const i = lines.indexOf(label);
    return i > 0 ? lines[i - 1] : null;
  };
  const poll = () => {
    tries++;
    const text = document.body ? document.body.innerText : '';
    const m = text.match(/Your Browser Fingerprint is (\w+)/);
    const lines = text.split('\n').map((l) => l.trim()).filter(Boolean);
    const fingerprint = after(lines, 'Fingerprint');
    const settled = fingerprint && !fingerprint.startsWith('Collecting');
    if (m && (settled || tries > 40)) {
      resolve(JSON.stringify({
        verdict: m[1].toLowerCase(),
        location: after(lines, 'Location'),
        proxy: after(lines, 'Proxy'),
        fingerprint,
        bot: after(lines, 'Bot check'),
      }));
    } else if (tries > 50) {
      resolve('TIMEOUT:' + text.substring(0, 500));
    } else {
      setTimeout(poll, 500);
    }
  };
  poll();
})
