// Parity breadcrumbs:
// - packages/bitcoin-knots/src/hash.h
// - packages/bitcoin-knots/src/hash.cpp
// - packages/bitcoin-knots/src/crypto/ripemd160.cpp
// - packages/bitcoin-knots/src/crypto/sha256.cpp
// - packages/bitcoin-knots/src/crypto/siphash.h
// - packages/bitcoin-knots/src/crypto/siphash.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp

use open_bitcoin_primitives::Wtxid;

fn sipround(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> (u64, u64, u64, u64) {
    v0 = v0.wrapping_add(v1);
    v1 = v1.rotate_left(13);
    v1 ^= v0;
    v0 = v0.rotate_left(32);
    v2 = v2.wrapping_add(v3);
    v3 = v3.rotate_left(16);
    v3 ^= v2;
    v0 = v0.wrapping_add(v3);
    v3 = v3.rotate_left(21);
    v3 ^= v0;
    v2 = v2.wrapping_add(v1);
    v1 = v1.rotate_left(17);
    v1 ^= v2;
    v2 = v2.rotate_left(32);
    (v0, v1, v2, v3)
}

fn read_u64_le(bytes: &[u8; 32], word_index: usize) -> u64 {
    let start = word_index * 8;
    let chunk: [u8; 8] = [
        bytes[start],
        bytes[start + 1],
        bytes[start + 2],
        bytes[start + 3],
        bytes[start + 4],
        bytes[start + 5],
        bytes[start + 6],
        bytes[start + 7],
    ];
    u64::from_le_bytes(chunk)
}

/// SipHash-2-4 over a 256-bit value, matching Knots `SipHashUint256`.
pub fn siphash_uint256(k0: u64, k1: u64, value: &Wtxid) -> u64 {
    let bytes = value.to_byte_array();
    let mut d = read_u64_le(&bytes, 0);

    let mut v0 = 0x736f_6d65_7073_6575_u64 ^ k0;
    let mut v1 = 0x646f_7261_6e64_6f6d_u64 ^ k1;
    let mut v2 = 0x6c79_6765_6e65_7261_u64 ^ k0;
    let mut v3 = 0x7465_6462_7974_6573_u64 ^ k1 ^ d;

    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= d;

    d = read_u64_le(&bytes, 1);
    v3 ^= d;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= d;

    d = read_u64_le(&bytes, 2);
    v3 ^= d;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= d;

    d = read_u64_le(&bytes, 3);
    v3 ^= d;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= d;

    v3 ^= 4_u64 << 59;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= 4_u64 << 59;
    v2 ^= 0xff;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);

    v0 ^ v1 ^ v2 ^ v3
}

#[cfg(test)]
mod tests {
    use super::siphash_uint256;
    use open_bitcoin_primitives::Wtxid;

    #[test]
    fn siphash_uint256_matches_knots_vector() {
        let wtxid = Wtxid::from_byte_array([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
            0x17, 0x18, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x31, 0x32, 0x33, 0x34,
            0x35, 0x36, 0x37, 0x38,
        ]);

        let digest = siphash_uint256(0x0102_0304_0506_0708, 0x1112_1314_1516_1718, &wtxid);

        assert_eq!(digest, 5_278_054_393_720_050_254);
    }

    #[test]
    fn siphash_uint256_is_deterministic_for_zero_keys() {
        let wtxid = Wtxid::from_byte_array([0xab; 32]);
        let first = siphash_uint256(0, 0, &wtxid);
        let second = siphash_uint256(0, 0, &wtxid);

        assert_eq!(first, second);
    }
}
