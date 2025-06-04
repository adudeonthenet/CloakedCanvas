//! CloakedCanvas core crypto library.

mod crypto;

pub use crypto::{encrypt_data, decrypt_data, KEY_SIZE, NONCE_SIZE};

pub fn hello() -> &'static str {
    "CloakedCanvas core ready!"
}
