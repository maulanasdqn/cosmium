await new Promise((resolve) => {
  let tries = 0;
  const poll = () => {
    tries++;
    const el = document.querySelector('#fingerprint-data .visitor-info');
    const grade = el ? el.textContent.trim() : '';
    if (grade && grade.length > 0 && !grade.includes('Loading')) {
      resolve(grade);
    } else if (tries > 60) {
      const body = document.body ? document.body.innerText.substring(0, 500) : 'empty';
      resolve('TIMEOUT:' + body);
    } else {
      setTimeout(poll, 500);
    }
  };
  poll();
})