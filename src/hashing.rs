use blake2_rfc::blake2b::{Blake2b, blake2b};

// Copied from: https://github.com/paritytech/substrate/blob/master/core/primitives/src/hashing.rs

/// Do a Blake2 256-bit hash and place result in `dest`.
pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
    dest.copy_from_slice(blake2b(32, &[], data).as_bytes());
}

pub fn blake2_256_pair_into(pair1: &[u8], pair2: &[u8], dest: &mut [u8; 32]) {
    let mut hasher = Blake2b::new(32);
    hasher.update(pair1);
    hasher.update(pair2);
    dest.copy_from_slice(hasher.finalize().as_bytes());
}

/// Do a Blake2 256-bit hash and return result.
pub fn blake2_256(data: &[u8]) -> [u8; 32] {
    let mut r = [0; 32];
    blake2_256_into(data, &mut r);
    r
}

pub fn blake2b_256_pair(pair1: &[u8], pair2: &[u8]) -> [u8; 32] {
    let mut r = [0; 32];
    blake2_256_pair_into(pair1, pair2, &mut r);
    r
}
