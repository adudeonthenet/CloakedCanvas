function toTransferObj(file) {
  return file.arrayBuffer().then(ab => ({ arrayBuffer: ab, name: file.name, mime: file.type }));
}

function replaceFileInForm(original, replacement) {
  const dt = new DataTransfer();
  dt.items.add(replacement);
  const input = original instanceof File ? document.querySelector(`input[type=file][data-cc-src='${original.name}']`) : null;
  if (input) {
    input.files = dt.files;
  }
}

function injectLink(url) {
  const ta = document.querySelector('textarea, [contenteditable="true"]');
  if (ta) ta.value += `\n${url}`;
  if (ta && ta.dispatchEvent) ta.dispatchEvent(new Event('input', { bubbles: true }));
}

async function interceptUpload(file) {
  chrome.runtime.sendMessage(
    { type: 'PROTECT_FILE', file: await toTransferObj(file) },
    ({ previewBuf, vaultUrl }) => {
      const previewFile = new File([previewBuf], file.name, { type: file.type });
      replaceFileInForm(file, previewFile);
      injectLink(vaultUrl);
    }
  );
}

document.addEventListener('change', e => {
  if (e.target instanceof HTMLInputElement && e.target.type === 'file') {
    [...e.target.files].forEach(interceptUpload);
  }
});

window.addEventListener('drop', e => {
  if (e.dataTransfer?.files?.length) {
    e.preventDefault();
    [...e.dataTransfer.files].forEach(interceptUpload);
  }
});
