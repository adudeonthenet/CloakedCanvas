use cloakedcanvas_core::{encrypt_data, decrypt_data, KEY_SIZE};

#[test]
fn round_trip() {
    let key = [42u8; KEY_SIZE];
    let msg = b"hello world";
    let (ct, nonce) = encrypt_data(&key, msg).expect("encryption");
    let pt = decrypt_data(&key, &ct, &nonce).expect("decryption");
    assert_eq!(pt, msg);
}
