//! CloakedCanvas core crypto library.

mod crypto;

pub use crypto::{encrypt_data, decrypt_data, KEY_SIZE, NONCE_SIZE};

pub fn hello() -> &'static str {
    "CloakedCanvas core ready!"
}

/// XOR every byte in `data` with the given `key`.
///
/// This is a simple reversible operation useful as a placeholder
/// until real encryption is implemented.
pub fn xor_mask(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_expected_message() {
        assert_eq!(hello(), "CloakedCanvas core ready!");
    }

    #[test]
    fn xor_mask_roundtrip() {
        let original = b"hello";
        let key = 0xAA;
        let encrypted = xor_mask(original, key);
        let decrypted = xor_mask(&encrypted, key);
        assert_eq!(decrypted, original);
    }
}
