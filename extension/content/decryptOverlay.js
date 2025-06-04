function getFilename(url) {
  try {
    return new URL(url).pathname.split('/').pop() || 'vault.bin';
  } catch (_) {
    return 'vault.bin';
  }
}

function scanLinks() {
  document.querySelectorAll('a').forEach(a => {
    if (a.dataset.ccInit) return;
    const m = a.href.match(/\.ccvault#(.+)$/);
    if (m) {
      a.dataset.ccInit = '1';
      const btn = document.createElement('button');
      btn.textContent = '🔓';
      btn.style.marginLeft = '4px';
      btn.onclick = () => chrome.runtime.sendMessage({
        type: 'DECRYPT_VAULT',
        vaultUrl: a.href,
        filename: getFilename(a.href)
      });
      a.after(btn);
    }
  });
}

setInterval(scanLinks, 1000);
