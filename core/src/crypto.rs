use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;

pub const KEY_SIZE: usize = 32; // 256-bit
pub const NONCE_SIZE: usize = 12; // GCM standard

/// Encrypt bytes with AES-256-GCM.
/// Returns ciphertext and nonce.
pub fn encrypt_data(key: &[u8; KEY_SIZE], plaintext: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_SIZE]), String> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.encrypt(nonce, plaintext) {
        Ok(ct) => Ok((ct, nonce_bytes)),
        Err(_) => Err("encryption failed".into()),
    }
}

/// Decrypt bytes with AES-256-GCM.
pub fn decrypt_data(key: &[u8; KEY_SIZE], ciphertext: &[u8], nonce: &[u8; NONCE_SIZE]) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    match cipher.decrypt(nonce, ciphertext) {
        Ok(pt) => Ok(pt),
        Err(_) => Err("decryption failed".into()),
    }
}
