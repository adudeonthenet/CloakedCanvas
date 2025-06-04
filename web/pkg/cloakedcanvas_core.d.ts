export const KEY_SIZE: number;
export const NONCE_SIZE: number;
export function encrypt_data(key: Uint8Array, data: Uint8Array): Promise<{ciphertext: Uint8Array, nonce: Uint8Array}>;
export function decrypt_data(key: Uint8Array, data: Uint8Array, nonce: Uint8Array): Promise<Uint8Array>;
export default function init(): Promise<void>;
