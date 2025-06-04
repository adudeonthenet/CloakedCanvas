// Placeholder for WASM decrypt logic
import init, { decrypt_data, KEY_SIZE, NONCE_SIZE } from '../pkg/cloakedcanvas_core.js';

function nightshadeUnpoison(data: Uint8Array): Uint8Array {
  return data.map(b => b ^ 0x13);
}

async function run() {
  await init();
  console.log('Decrypt WASM loaded');

  const key = new Uint8Array(KEY_SIZE).fill(42);

  async function handleFile(file: File) {
    const buf = await file.arrayBuffer();
    const bytes = new Uint8Array(buf);
    const nonce = bytes.slice(0, NONCE_SIZE);
    const ct = bytes.slice(NONCE_SIZE);
    let pt = await decrypt_data(key, ct, nonce);
    pt = nightshadeUnpoison(pt);
    const blob = new Blob([pt]);
    const img = document.createElement('img');
    img.src = URL.createObjectURL(blob);
    document.body.appendChild(img);
  }

  document.addEventListener('dragover', e => {
    e.preventDefault();
  });
  document.addEventListener('drop', e => {
    e.preventDefault();
    const file = e.dataTransfer?.files[0];
    if (file) handleFile(file);
  });
}

run();
