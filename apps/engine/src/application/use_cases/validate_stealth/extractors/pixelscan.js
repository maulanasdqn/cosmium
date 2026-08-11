await new Promise((resolve) => {
  let tries = 0;
  const poll = () => {
    tries++;
    const el = document.querySelector('.consistency-status, [class*="status"], [class*="result"]');
    const text = el ? el.textContent.trim() : '';
    if (text && text.length > 0) {
      resolve(text);
    } else if (tries > 30) {
      const body = document.body ? document.body.innerText.substring(0, 500) : 'empty';
      resolve('TIMEOUT:' + body);
    } else {
      setTimeout(poll, 500);
    }
  };
  poll();
})