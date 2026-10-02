await new Promise((resolve) => {
  let tries = 0;
  const pct = (text, label) => {
    const m = text.match(new RegExp('(\\d+)% ' + label + ':'));
    return m ? Number(m[1]) : null;
  };
  const poll = () => {
    tries++;
    const text = document.body ? document.body.innerText : '';
    const headless = pct(text, 'headless');
    const stealth = pct(text, 'stealth');
    const like = pct(text, 'like headless');
    if (headless !== null && stealth !== null && like !== null) {
      resolve(JSON.stringify({ headless, stealth, like_headless: like }));
    } else if (tries > 60) {
      resolve('TIMEOUT:' + text.substring(0, 500));
    } else {
      setTimeout(poll, 500);
    }
  };
  poll();
})
