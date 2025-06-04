// Placeholder for WASM decrypt logic
import init, { decrypt_data } from '../pkg/cloakedcanvas_core.js';

async function run() {
  await init();
  console.log('Decrypt WASM loaded');
  // TODO: hook up drag & drop handlers to feed encrypted files to decrypt_data
}

run();
