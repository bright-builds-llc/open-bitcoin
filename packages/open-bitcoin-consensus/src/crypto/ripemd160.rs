// Parity breadcrumbs:
// - packages/bitcoin-knots/src/hash.h
// - packages/bitcoin-knots/src/hash.cpp
// - packages/bitcoin-knots/src/crypto/ripemd160.cpp
// - packages/bitcoin-knots/src/crypto/sha256.cpp

const INITIAL_STATE: [u32; 5] = [
    0x6745_2301,
    0xefcd_ab89,
    0x98ba_dcfe,
    0x1032_5476,
    0xc3d2_e1f0,
];

pub struct Ripemd160;

impl Ripemd160 {
    pub fn digest(bytes: &[u8]) -> [u8; 20] {
        let mut state = INITIAL_STATE;
        let mut padded = bytes.to_vec();
        padded.push(0x80);

        while !(padded.len() + 8).is_multiple_of(64) {
            padded.push(0x00);
        }

        let bit_length = (bytes.len() as u64) * 8;
        padded.extend_from_slice(&bit_length.to_le_bytes());

        for chunk in padded.chunks_exact(64) {
            compress_chunk(&mut state, chunk);
        }

        let mut digest = [0_u8; 20];
        for (chunk, word) in digest.chunks_exact_mut(4).zip(state) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        digest
    }
}

fn compress_chunk(state: &mut [u32; 5], chunk: &[u8]) {
    let mut words = [0_u32; 16];
    for (index, word) in words.iter_mut().enumerate() {
        let offset = index * 4;
        *word = u32::from_le_bytes([
            chunk[offset],
            chunk[offset + 1],
            chunk[offset + 2],
            chunk[offset + 3],
        ]);
    }

    let mut a1 = state[0];
    let mut b1 = state[1];
    let mut c1 = state[2];
    let mut d1 = state[3];
    let mut e1 = state[4];
    let mut a2 = a1;
    let mut b2 = b1;
    let mut c2 = c1;
    let mut d2 = d1;
    let mut e2 = e1;

    round_left_1(&mut a1, b1, &mut c1, d1, e1, words[0], 11);
    round_right_1(&mut a2, b2, &mut c2, d2, e2, words[5], 8);
    round_left_1(&mut e1, a1, &mut b1, c1, d1, words[1], 14);
    round_right_1(&mut e2, a2, &mut b2, c2, d2, words[14], 9);
    round_left_1(&mut d1, e1, &mut a1, b1, c1, words[2], 15);
    round_right_1(&mut d2, e2, &mut a2, b2, c2, words[7], 9);
    round_left_1(&mut c1, d1, &mut e1, a1, b1, words[3], 12);
    round_right_1(&mut c2, d2, &mut e2, a2, b2, words[0], 11);
    round_left_1(&mut b1, c1, &mut d1, e1, a1, words[4], 5);
    round_right_1(&mut b2, c2, &mut d2, e2, a2, words[9], 13);
    round_left_1(&mut a1, b1, &mut c1, d1, e1, words[5], 8);
    round_right_1(&mut a2, b2, &mut c2, d2, e2, words[2], 15);
    round_left_1(&mut e1, a1, &mut b1, c1, d1, words[6], 7);
    round_right_1(&mut e2, a2, &mut b2, c2, d2, words[11], 15);
    round_left_1(&mut d1, e1, &mut a1, b1, c1, words[7], 9);
    round_right_1(&mut d2, e2, &mut a2, b2, c2, words[4], 5);
    round_left_1(&mut c1, d1, &mut e1, a1, b1, words[8], 11);
    round_right_1(&mut c2, d2, &mut e2, a2, b2, words[13], 7);
    round_left_1(&mut b1, c1, &mut d1, e1, a1, words[9], 13);
    round_right_1(&mut b2, c2, &mut d2, e2, a2, words[6], 7);
    round_left_1(&mut a1, b1, &mut c1, d1, e1, words[10], 14);
    round_right_1(&mut a2, b2, &mut c2, d2, e2, words[15], 8);
    round_left_1(&mut e1, a1, &mut b1, c1, d1, words[11], 15);
    round_right_1(&mut e2, a2, &mut b2, c2, d2, words[8], 11);
    round_left_1(&mut d1, e1, &mut a1, b1, c1, words[12], 6);
    round_right_1(&mut d2, e2, &mut a2, b2, c2, words[1], 14);
    round_left_1(&mut c1, d1, &mut e1, a1, b1, words[13], 7);
    round_right_1(&mut c2, d2, &mut e2, a2, b2, words[10], 14);
    round_left_1(&mut b1, c1, &mut d1, e1, a1, words[14], 9);
    round_right_1(&mut b2, c2, &mut d2, e2, a2, words[3], 12);
    round_left_1(&mut a1, b1, &mut c1, d1, e1, words[15], 8);
    round_right_1(&mut a2, b2, &mut c2, d2, e2, words[12], 6);

    round_left_2(&mut e1, a1, &mut b1, c1, d1, words[7], 7);
    round_right_2(&mut e2, a2, &mut b2, c2, d2, words[6], 9);
    round_left_2(&mut d1, e1, &mut a1, b1, c1, words[4], 6);
    round_right_2(&mut d2, e2, &mut a2, b2, c2, words[11], 13);
    round_left_2(&mut c1, d1, &mut e1, a1, b1, words[13], 8);
    round_right_2(&mut c2, d2, &mut e2, a2, b2, words[3], 15);
    round_left_2(&mut b1, c1, &mut d1, e1, a1, words[1], 13);
    round_right_2(&mut b2, c2, &mut d2, e2, a2, words[7], 7);
    round_left_2(&mut a1, b1, &mut c1, d1, e1, words[10], 11);
    round_right_2(&mut a2, b2, &mut c2, d2, e2, words[0], 12);
    round_left_2(&mut e1, a1, &mut b1, c1, d1, words[6], 9);
    round_right_2(&mut e2, a2, &mut b2, c2, d2, words[13], 8);
    round_left_2(&mut d1, e1, &mut a1, b1, c1, words[15], 7);
    round_right_2(&mut d2, e2, &mut a2, b2, c2, words[5], 9);
    round_left_2(&mut c1, d1, &mut e1, a1, b1, words[3], 15);
    round_right_2(&mut c2, d2, &mut e2, a2, b2, words[10], 11);
    round_left_2(&mut b1, c1, &mut d1, e1, a1, words[12], 7);
    round_right_2(&mut b2, c2, &mut d2, e2, a2, words[14], 7);
    round_left_2(&mut a1, b1, &mut c1, d1, e1, words[0], 12);
    round_right_2(&mut a2, b2, &mut c2, d2, e2, words[15], 7);
    round_left_2(&mut e1, a1, &mut b1, c1, d1, words[9], 15);
    round_right_2(&mut e2, a2, &mut b2, c2, d2, words[8], 12);
    round_left_2(&mut d1, e1, &mut a1, b1, c1, words[5], 9);
    round_right_2(&mut d2, e2, &mut a2, b2, c2, words[12], 7);
    round_left_2(&mut c1, d1, &mut e1, a1, b1, words[2], 11);
    round_right_2(&mut c2, d2, &mut e2, a2, b2, words[4], 6);
    round_left_2(&mut b1, c1, &mut d1, e1, a1, words[14], 7);
    round_right_2(&mut b2, c2, &mut d2, e2, a2, words[9], 15);
    round_left_2(&mut a1, b1, &mut c1, d1, e1, words[11], 13);
    round_right_2(&mut a2, b2, &mut c2, d2, e2, words[1], 13);
    round_left_2(&mut e1, a1, &mut b1, c1, d1, words[8], 12);
    round_right_2(&mut e2, a2, &mut b2, c2, d2, words[2], 11);

    round_left_3(&mut d1, e1, &mut a1, b1, c1, words[3], 11);
    round_right_3(&mut d2, e2, &mut a2, b2, c2, words[15], 9);
    round_left_3(&mut c1, d1, &mut e1, a1, b1, words[10], 13);
    round_right_3(&mut c2, d2, &mut e2, a2, b2, words[5], 7);
    round_left_3(&mut b1, c1, &mut d1, e1, a1, words[14], 6);
    round_right_3(&mut b2, c2, &mut d2, e2, a2, words[1], 15);
    round_left_3(&mut a1, b1, &mut c1, d1, e1, words[4], 7);
    round_right_3(&mut a2, b2, &mut c2, d2, e2, words[3], 11);
    round_left_3(&mut e1, a1, &mut b1, c1, d1, words[9], 14);
    round_right_3(&mut e2, a2, &mut b2, c2, d2, words[7], 8);
    round_left_3(&mut d1, e1, &mut a1, b1, c1, words[15], 9);
    round_right_3(&mut d2, e2, &mut a2, b2, c2, words[14], 6);
    round_left_3(&mut c1, d1, &mut e1, a1, b1, words[8], 13);
    round_right_3(&mut c2, d2, &mut e2, a2, b2, words[6], 6);
    round_left_3(&mut b1, c1, &mut d1, e1, a1, words[1], 15);
    round_right_3(&mut b2, c2, &mut d2, e2, a2, words[9], 14);
    round_left_3(&mut a1, b1, &mut c1, d1, e1, words[2], 14);
    round_right_3(&mut a2, b2, &mut c2, d2, e2, words[11], 12);
    round_left_3(&mut e1, a1, &mut b1, c1, d1, words[7], 8);
    round_right_3(&mut e2, a2, &mut b2, c2, d2, words[8], 13);
    round_left_3(&mut d1, e1, &mut a1, b1, c1, words[0], 13);
    round_right_3(&mut d2, e2, &mut a2, b2, c2, words[12], 5);
    round_left_3(&mut c1, d1, &mut e1, a1, b1, words[6], 6);
    round_right_3(&mut c2, d2, &mut e2, a2, b2, words[2], 14);
    round_left_3(&mut b1, c1, &mut d1, e1, a1, words[13], 5);
    round_right_3(&mut b2, c2, &mut d2, e2, a2, words[10], 13);
    round_left_3(&mut a1, b1, &mut c1, d1, e1, words[11], 12);
    round_right_3(&mut a2, b2, &mut c2, d2, e2, words[0], 13);
    round_left_3(&mut e1, a1, &mut b1, c1, d1, words[5], 7);
    round_right_3(&mut e2, a2, &mut b2, c2, d2, words[4], 7);
    round_left_3(&mut d1, e1, &mut a1, b1, c1, words[12], 5);
    round_right_3(&mut d2, e2, &mut a2, b2, c2, words[13], 5);

    round_left_4(&mut c1, d1, &mut e1, a1, b1, words[1], 11);
    round_right_4(&mut c2, d2, &mut e2, a2, b2, words[8], 15);
    round_left_4(&mut b1, c1, &mut d1, e1, a1, words[9], 12);
    round_right_4(&mut b2, c2, &mut d2, e2, a2, words[6], 5);
    round_left_4(&mut a1, b1, &mut c1, d1, e1, words[11], 14);
    round_right_4(&mut a2, b2, &mut c2, d2, e2, words[4], 8);
    round_left_4(&mut e1, a1, &mut b1, c1, d1, words[10], 15);
    round_right_4(&mut e2, a2, &mut b2, c2, d2, words[1], 11);
    round_left_4(&mut d1, e1, &mut a1, b1, c1, words[0], 14);
    round_right_4(&mut d2, e2, &mut a2, b2, c2, words[3], 14);
    round_left_4(&mut c1, d1, &mut e1, a1, b1, words[8], 15);
    round_right_4(&mut c2, d2, &mut e2, a2, b2, words[11], 14);
    round_left_4(&mut b1, c1, &mut d1, e1, a1, words[12], 9);
    round_right_4(&mut b2, c2, &mut d2, e2, a2, words[15], 6);
    round_left_4(&mut a1, b1, &mut c1, d1, e1, words[4], 8);
    round_right_4(&mut a2, b2, &mut c2, d2, e2, words[0], 14);
    round_left_4(&mut e1, a1, &mut b1, c1, d1, words[13], 9);
    round_right_4(&mut e2, a2, &mut b2, c2, d2, words[5], 6);
    round_left_4(&mut d1, e1, &mut a1, b1, c1, words[3], 14);
    round_right_4(&mut d2, e2, &mut a2, b2, c2, words[12], 9);
    round_left_4(&mut c1, d1, &mut e1, a1, b1, words[7], 5);
    round_right_4(&mut c2, d2, &mut e2, a2, b2, words[2], 12);
    round_left_4(&mut b1, c1, &mut d1, e1, a1, words[15], 6);
    round_right_4(&mut b2, c2, &mut d2, e2, a2, words[13], 9);
    round_left_4(&mut a1, b1, &mut c1, d1, e1, words[14], 8);
    round_right_4(&mut a2, b2, &mut c2, d2, e2, words[9], 12);
    round_left_4(&mut e1, a1, &mut b1, c1, d1, words[5], 6);
    round_right_4(&mut e2, a2, &mut b2, c2, d2, words[7], 5);
    round_left_4(&mut d1, e1, &mut a1, b1, c1, words[6], 5);
    round_right_4(&mut d2, e2, &mut a2, b2, c2, words[10], 15);
    round_left_4(&mut c1, d1, &mut e1, a1, b1, words[2], 12);
    round_right_4(&mut c2, d2, &mut e2, a2, b2, words[14], 8);

    round_left_5(&mut b1, c1, &mut d1, e1, a1, words[4], 9);
    round_right_5(&mut b2, c2, &mut d2, e2, a2, words[12], 8);
    round_left_5(&mut a1, b1, &mut c1, d1, e1, words[0], 15);
    round_right_5(&mut a2, b2, &mut c2, d2, e2, words[15], 5);
    round_left_5(&mut e1, a1, &mut b1, c1, d1, words[5], 5);
    round_right_5(&mut e2, a2, &mut b2, c2, d2, words[10], 12);
    round_left_5(&mut d1, e1, &mut a1, b1, c1, words[9], 11);
    round_right_5(&mut d2, e2, &mut a2, b2, c2, words[4], 9);
    round_left_5(&mut c1, d1, &mut e1, a1, b1, words[7], 6);
    round_right_5(&mut c2, d2, &mut e2, a2, b2, words[1], 12);
    round_left_5(&mut b1, c1, &mut d1, e1, a1, words[12], 8);
    round_right_5(&mut b2, c2, &mut d2, e2, a2, words[5], 5);
    round_left_5(&mut a1, b1, &mut c1, d1, e1, words[2], 13);
    round_right_5(&mut a2, b2, &mut c2, d2, e2, words[8], 14);
    round_left_5(&mut e1, a1, &mut b1, c1, d1, words[10], 12);
    round_right_5(&mut e2, a2, &mut b2, c2, d2, words[7], 6);
    round_left_5(&mut d1, e1, &mut a1, b1, c1, words[14], 5);
    round_right_5(&mut d2, e2, &mut a2, b2, c2, words[6], 8);
    round_left_5(&mut c1, d1, &mut e1, a1, b1, words[1], 12);
    round_right_5(&mut c2, d2, &mut e2, a2, b2, words[2], 13);
    round_left_5(&mut b1, c1, &mut d1, e1, a1, words[3], 13);
    round_right_5(&mut b2, c2, &mut d2, e2, a2, words[13], 6);
    round_left_5(&mut a1, b1, &mut c1, d1, e1, words[8], 14);
    round_right_5(&mut a2, b2, &mut c2, d2, e2, words[14], 5);
    round_left_5(&mut e1, a1, &mut b1, c1, d1, words[11], 11);
    round_right_5(&mut e2, a2, &mut b2, c2, d2, words[0], 15);
    round_left_5(&mut d1, e1, &mut a1, b1, c1, words[6], 8);
    round_right_5(&mut d2, e2, &mut a2, b2, c2, words[3], 13);
    round_left_5(&mut c1, d1, &mut e1, a1, b1, words[15], 5);
    round_right_5(&mut c2, d2, &mut e2, a2, b2, words[9], 11);
    round_left_5(&mut b1, c1, &mut d1, e1, a1, words[13], 6);
    round_right_5(&mut b2, c2, &mut d2, e2, a2, words[11], 11);

    let saved = state[0];
    state[0] = state[1].wrapping_add(c1).wrapping_add(d2);
    state[1] = state[2].wrapping_add(d1).wrapping_add(e2);
    state[2] = state[3].wrapping_add(e1).wrapping_add(a2);
    state[3] = state[4].wrapping_add(a1).wrapping_add(b2);
    state[4] = saved.wrapping_add(b1).wrapping_add(c2);
}

fn round_left_1(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f1(b, *c, d), x, 0, r);
}

fn round_left_2(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f2(b, *c, d), x, 0x5a82_7999, r);
}

fn round_left_3(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f3(b, *c, d), x, 0x6ed9_eba1, r);
}

fn round_left_4(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f4(b, *c, d), x, 0x8f1b_bcdc, r);
}

fn round_left_5(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f5(b, *c, d), x, 0xa953_fd4e, r);
}

fn round_right_1(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f5(b, *c, d), x, 0x50a2_8be6, r);
}

fn round_right_2(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f4(b, *c, d), x, 0x5c4d_d124, r);
}

fn round_right_3(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f3(b, *c, d), x, 0x6d70_3ef3, r);
}

fn round_right_4(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f2(b, *c, d), x, 0x7a6d_76e9, r);
}

fn round_right_5(a: &mut u32, b: u32, c: &mut u32, d: u32, e: u32, x: u32, r: u32) {
    round(a, b, c, d, e, f1(b, *c, d), x, 0, r);
}

// Keep the helper shape close to the RIPEMD-160 reference round schedule.
#[allow(clippy::too_many_arguments)]
fn round(a: &mut u32, _b: u32, c: &mut u32, _d: u32, e: u32, f: u32, x: u32, k: u32, r: u32) {
    *a = rol((*a).wrapping_add(f).wrapping_add(x).wrapping_add(k), r).wrapping_add(e);
    *c = rol(*c, 10);
}

fn rol(value: u32, shift: u32) -> u32 {
    value.rotate_left(shift)
}

fn f1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn f2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn f3(x: u32, y: u32, z: u32) -> u32 {
    (x | !y) ^ z
}

fn f4(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn f5(x: u32, y: u32, z: u32) -> u32 {
    x ^ (y | !z)
}
