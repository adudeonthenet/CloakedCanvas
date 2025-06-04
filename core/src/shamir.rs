/*
 * CloakedCanvas - MIT License
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sharks::{Sharks, Share};

use crate::KEY_SIZE;

/// Split a key into 3 shares requiring any 2 to recover.
pub fn split_key(key: &[u8; KEY_SIZE]) -> Vec<String> {
    let sharks = Sharks(2);
    sharks
        .dealer(key)
        .take(3)
        .map(|s| {
            let bytes: Vec<u8> = (&s).into();
            URL_SAFE_NO_PAD.encode(bytes)
        })
        .collect()
}

/// Reconstruct a key from at least 2 shares.
pub fn combine_key(shares: &[String]) -> Result<[u8; KEY_SIZE], String> {
    let sharks = Sharks(2);
    let share_objs: Vec<Share> = shares
        .iter()
        .map(|s| {
            let bytes = URL_SAFE_NO_PAD
                .decode(s.as_bytes())
                .map_err(|_| "base64")?;
            Share::try_from(bytes.as_slice()).map_err(|_| "share")
        })
        .collect::<Result<_, _>>()?;
    let secret = sharks
        .recover(share_objs.iter())
        .map_err(|_| "recover failed")?;
    let mut arr = [0u8; KEY_SIZE];
    arr.copy_from_slice(&secret);
    Ok(arr)
}
