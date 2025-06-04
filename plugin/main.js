/* CloakedCanvas UXP panel */
import init, { encrypt_data, KEY_SIZE } from '../web/pkg/cloakedcanvas_core.js';

function nightshadePoison(data) {
  return data.map(b => b ^ 0x13);
}

console.log('CloakedCanvas plugin loaded');
const wasmReady = init();

document.addEventListener('DOMContentLoaded', () => {
  const btn = document.getElementById('exportBtn');
  const storeSel = document.getElementById('storage');
  if (btn && storeSel) {
    btn.addEventListener('click', async () => {
      await wasmReady;
      console.log('Export button clicked');
      const level = document.querySelector('input[name="level"]:checked').value;
      const storage = storeSel.value;
      const key = new Uint8Array(KEY_SIZE).fill(42);
      let data = new TextEncoder().encode('sample');
      if (level === '3') {
        console.log('L3 mode active');
        data = nightshadePoison(data);
      }
      const result = await encrypt_data(key, data);
      console.log('Encrypted bytes', result.ciphertext, 'store:', storage);
    });
  }
});
