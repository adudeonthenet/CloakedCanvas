export const KEY_SIZE = 32;
export const NONCE_SIZE = 12;

export default async function init() {
  return;
}

export async function encrypt_data(key, data) {
  const iv = crypto.getRandomValues(new Uint8Array(NONCE_SIZE));
  const cryptoKey = await crypto.subtle.importKey('raw', key, 'AES-GCM', false, ['encrypt']);
  const ct = new Uint8Array(await crypto.subtle.encrypt({name: 'AES-GCM', iv}, cryptoKey, data));
  return {ciphertext: ct, nonce: iv};
}

export async function decrypt_data(key, data, nonce) {
  const cryptoKey = await crypto.subtle.importKey('raw', key, 'AES-GCM', false, ['decrypt']);
  const pt = new Uint8Array(await crypto.subtle.decrypt({name: 'AES-GCM', iv: nonce}, cryptoKey, data));
  return pt;
}
