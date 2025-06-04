declare module '../pkg/cloakedcanvas_core.js' {
  export const KEY_SIZE: number;
  export const NONCE_SIZE: number;
  export function encrypt_data(key: Uint8Array, data: Uint8Array): {ciphertext: Uint8Array, nonce: Uint8Array};
  export function decrypt_data(key: Uint8Array, data: Uint8Array, nonce: Uint8Array): Uint8Array;
  export function nightshade_poison(data: Uint8Array): Uint8Array;
  export function nightshade_unpoison(data: Uint8Array): Uint8Array;
  export default function init(): Promise<void>;
}
