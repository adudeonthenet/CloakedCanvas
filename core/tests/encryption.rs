use cloakedcanvas_core::{encrypt_data, decrypt_data, encrypt_file, KEY_SIZE};
use std::fs;
use std::path::Path;

#[test]
fn round_trip() {
    let key = [42u8; KEY_SIZE];
    let msg = b"hello world";
    let (ct, nonce) = encrypt_data(&key, msg).expect("encryption");
    let pt = decrypt_data(&key, &ct, &nonce).expect("decryption");
    assert_eq!(pt, msg);
}

#[test]
fn encrypt_file_creates_vault() {
    let path = Path::new("../assets/branding/preview_watermark.svg");
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::copy(path, tmp.path()).unwrap();
    let key = [1u8; KEY_SIZE];
    let out = encrypt_file(tmp.path(), &key).expect("encrypt");
    assert!(out.exists());
}
