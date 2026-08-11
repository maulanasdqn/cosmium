(() => {
  const url = location.href;
  const title = document.title;
  const bodySnip = (document.body ? document.body.innerText : '').substring(0, 1000);
  return JSON.stringify({ url, title, body: bodySnip });
})()