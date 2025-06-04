import init, {
  encrypt_file,
  generate_preview,
  put_vault,
  decrypt_vault
} from './wasm/cloakedcanvas_wasm.js';

await init();

chrome.runtime.onMessage.addListener((msg, sender, sendResp) => {
  (async () => {
    if (msg.type === 'PROTECT_FILE') {
      const { arrayBuffer, name, mime } = msg.file;
      const { previewBuf, vaultUrl } = await protect(arrayBuffer, name, mime);
      sendResp({ previewBuf, vaultUrl });
    } else if (msg.type === 'DECRYPT_VAULT') {
      const clearBuf = await decrypt_vault(msg.vaultUrl);
      const blob = new Blob([clearBuf], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      chrome.downloads.download({ url, filename: msg.filename });
    }
  })();
  return true;
});

async function protect(buf, fname, mime) {
  const preview = await generate_preview(buf, mime);
  const { url: signedUrl, aes_key } = await put_vault(buf, fname);
  return { previewBuf: preview, vaultUrl: `${signedUrl}#${aes_key}` };
}
