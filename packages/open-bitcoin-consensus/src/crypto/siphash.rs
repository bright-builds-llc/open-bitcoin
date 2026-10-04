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

/// SipHash-2-4 over arbitrary raw bytes, matching Knots `CSipHasher`.
pub fn siphash_bytes(k0: u64, k1: u64, bytes: &[u8]) -> u64 {
    let mut v0 = 0x736f_6d65_7073_6575_u64 ^ k0;
    let mut v1 = 0x646f_7261_6e64_6f6d_u64 ^ k1;
    let mut v2 = 0x6c79_6765_6e65_7261_u64 ^ k0;
    let mut v3 = 0x7465_6462_7974_6573_u64 ^ k1;
    let mut words = bytes.chunks_exact(8);
    for word in &mut words {
        let mut array = [0_u8; 8];
        array.copy_from_slice(word);
        let value = u64::from_le_bytes(array);
        v3 ^= value;
        (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
        (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
        v0 ^= value;
    }
    let mut tail = ((bytes.len() & 0xff) as u64) << 56;
    for (index, byte) in words.remainder().iter().enumerate() {
        tail |= u64::from(*byte) << (8 * index);
    }
    v3 ^= tail;
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    v0 ^= tail;
    v2 ^= 0xff;
    for _ in 0..4 {
        (v0, v1, v2, v3) = sipround(v0, v1, v2, v3);
    }
    v0 ^ v1 ^ v2 ^ v3
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
    use super::{siphash_bytes, siphash_uint256};
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

    #[test]
    fn siphash_bytes_matches_all_pinned_length_vectors() {
        // Arrange: src/test/hash_tests.cpp::siphash_4_2_testvec.
        let expected = [
            0x726fdb47dd0e0e31,
            0x74f839c593dc67fd,
            0x0d6c8009d9a94f5a,
            0x85676696d7fb7e2d,
            0xcf2794e0277187b7,
            0x18765564cd99a68d,
            0xcbc9466e58fee3ce,
            0xab0200f58b01d137,
            0x93f5f5799a932462,
            0x9e0082df0ba9e4b0,
            0x7a5dbbc594ddb9f3,
            0xf4b32f46226bada7,
            0x751e8fbc860ee5fb,
            0x14ea5627c0843d90,
            0xf723ca908e7af2ee,
            0xa129ca6149be45e5,
            0x3f2acc7f57c29bdb,
            0x699ae9f52cbe4794,
            0x4bc1b3f0968dd39c,
            0xbb6dc91da77961bd,
            0xbed65cf21aa2ee98,
            0xd0f2cbb02e3b67c7,
            0x93536795e3a33e88,
            0xa80c038ccd5ccec8,
            0xb8ad50c6f649af94,
            0xbce192de8a85b8ea,
            0x17d835b85bbb15f3,
            0x2f2e6163076bcfad,
            0xde4daaaca71dc9a5,
            0xa6a2506687956571,
            0xad87a3535c49ef28,
            0x32d892fad841c342,
            0x7127512f72f27cce,
            0xa7f32346f95978e3,
            0x12e0b01abb051238,
            0x15e034d40fa197ae,
            0x314dffbe0815a3b4,
            0x027990f029623981,
            0xcadcd4e59ef40c4d,
            0x9abfd8766a33735c,
            0x0e3ea96b5304a7d0,
            0xad0c42d6fc585992,
            0x187306c89bc215a9,
            0xd4a60abcf3792b95,
            0xf935451de4f21df2,
            0xa9538f0419755787,
            0xdb9acddff56ca510,
            0xd06c98cd5c0975eb,
            0xe612a3cb9ecba951,
            0xc766e62cfcadaf96,
            0xee64435a9752fe72,
            0xa192d576b245165a,
            0x0a8787bf8ecb74b2,
            0x81b3e73d20b49b6f,
            0x7fa8220ba3b2ecea,
            0x245731c13ca42499,
            0xb78dbfaf3a8d83bd,
            0xea1ad565322a1a0b,
            0x60e61c23a3795013,
            0x6606d7e446282b93,
            0x6ca4ecb15c5f91e1,
            0x9f626da15c9625f3,
            0xe51b38608ef25f57,
            0x958a324ceb064572,
        ];
        let bytes: Vec<u8> = (0..64).collect();
        for (length, digest) in expected.into_iter().enumerate() {
            // Act
            let actual = siphash_bytes(0x0706050403020100, 0x0f0e0d0c0b0a0908, &bytes[..length]);
            // Assert
            assert_eq!(actual, digest, "length {length}");
        }
    }

    #[test]
    fn siphash_bytes_matches_independent_wrapped_length_vectors() {
        // Arrange: pinned test_framework/crypto/siphash.py, ascending bytes mod 256.
        let cases = [
            (64, 0xacd2c40b8502cad8),
            (255, 0xa9c169fec74db21a),
            (256, 0x999d0526d2a7bfd7),
            (257, 0x8a817b8d55b29748),
        ];
        for (length, expected) in cases {
            let bytes: Vec<u8> = (0..length).map(|index| index as u8).collect();
            // Act
            let actual = siphash_bytes(0x0706050403020100, 0x0f0e0d0c0b0a0908, &bytes);
            // Assert
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn siphash_bytes_preserves_fixed_width_result() {
        // Arrange
        let value = Wtxid::from_byte_array(core::array::from_fn(|index| index as u8));
        // Act
        let actual = siphash_bytes(0x0706050403020100, 0x0f0e0d0c0b0a0908, value.as_bytes());
        // Assert
        assert_eq!(
            actual,
            siphash_uint256(0x0706050403020100, 0x0f0e0d0c0b0a0908, &value)
        );
    }
}
