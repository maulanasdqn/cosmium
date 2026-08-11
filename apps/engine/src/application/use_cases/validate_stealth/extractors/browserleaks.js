(() => {
  const rows = document.querySelectorAll('table tr');
  const data = {};
  for (const row of rows) {
    const cells = row.querySelectorAll('td');
    if (cells.length >= 2) {
      const key = cells[0].textContent.trim();
      const val = cells[1].textContent.trim();
      if (key) data[key] = val;
    }
  }
  return JSON.stringify(data);
})()