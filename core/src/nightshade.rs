/*
 * CloakedCanvas - MIT License
 */

/// Simple placeholder Nightshade 'poison' algorithm.
/// XORs bytes with 0x13.
pub fn poison(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ 0x13).collect()
}

/// Reverse the Nightshade 'poison'.
pub fn unpoison(data: &[u8]) -> Vec<u8> {
    poison(data)
}
