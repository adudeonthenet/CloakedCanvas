use cloakedcanvas_core::{nightshade_poison, nightshade_unpoison, split_key, combine_key, KEY_SIZE};

#[test]
fn nightshade_roundtrip() {
    let data = b"hello";
    let poisoned = nightshade_poison(data);
    let plain = nightshade_unpoison(&poisoned);
    assert_eq!(plain, data);
}

#[test]
fn shamir_split_combine() {
    let key = [7u8; KEY_SIZE];
    let shares = split_key(&key);
    assert_eq!(shares.len(), 3);
    let combined = combine_key(&shares[0..2].to_vec()).expect("combine");
    assert_eq!(combined, key);
}
