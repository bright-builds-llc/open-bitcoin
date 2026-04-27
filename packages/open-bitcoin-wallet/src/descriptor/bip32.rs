use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};

use crate::WalletError;
use crate::address::{AddressNetwork, PrivateKey, public_key_bytes};

use super::{
    CHECKSUM_SIZE, DerivationStep, EXTENDED_KEY_PAYLOAD_LEN, EXTENDED_PRIVATE_MAINNET,
    EXTENDED_PRIVATE_TESTNET, EXTENDED_PUBLIC_MAINNET, EXTENDED_PUBLIC_TESTNET, ExtendedPrivateKey,
    ExtendedPublicKey, HARDENED_INDEX, SHA512_BLOCK_SIZE, SHA512_OUTPUT_SIZE, Wildcard,
    format::encode_wif_from_parts,
};

#[derive(Debug)]
pub(super) enum ParsedExtendedKey {
    Public(ExtendedPublicKey),
    Private(ExtendedPrivateKey),
}

pub(super) fn parse_extended_key(
    encoded: &str,
    network: AddressNetwork,
) -> Result<ParsedExtendedKey, WalletError> {
    let decoded = decode_base58check(encoded)?;
    if decoded.len() != EXTENDED_KEY_PAYLOAD_LEN {
        return Err(WalletError::InvalidBase58(
            "extended key payload must be 78 bytes".to_string(),
        ));
    }
    let version = u32::from_be_bytes(array_from_slice_4(&decoded[0..4], "extended key version")?);
    let chain_code = array_from_slice_32(&decoded[13..45], "extended key chain code")?;
    let key_data = &decoded[45..78];

    match version {
        EXTENDED_PUBLIC_MAINNET | EXTENDED_PUBLIC_TESTNET => {
            let key_network = network_for_extended_version(version);
            if !accepts_extended_network(network, key_network) {
                return Err(WalletError::NetworkMismatch {
                    expected: network.to_string(),
                    actual: key_network.to_string(),
                });
            }
            let public_key =
                PublicKey::from_slice(key_data).map_err(|_| WalletError::InvalidPublicKey)?;
            Ok(ParsedExtendedKey::Public(ExtendedPublicKey {
                network: key_network,
                chain_code,
                public_key,
            }))
        }
        EXTENDED_PRIVATE_MAINNET | EXTENDED_PRIVATE_TESTNET => {
            let key_network = network_for_extended_version(version);
            if !accepts_extended_network(network, key_network) {
                return Err(WalletError::NetworkMismatch {
                    expected: network.to_string(),
                    actual: key_network.to_string(),
                });
            }
            if key_data.first().copied() != Some(0) {
                return Err(WalletError::InvalidPrivateKey);
            }
            let private_key = decode_extended_private_key(key_network, &key_data[1..])?;
            Ok(ParsedExtendedKey::Private(ExtendedPrivateKey {
                network: key_network,
                chain_code,
                private_key,
            }))
        }
        _ => Err(WalletError::InvalidBase58(
            "unsupported extended key version".to_string(),
        )),
    }
}

pub(super) fn network_for_extended_version(version: u32) -> AddressNetwork {
    match version {
        EXTENDED_PUBLIC_MAINNET | EXTENDED_PRIVATE_MAINNET => AddressNetwork::Mainnet,
        EXTENDED_PUBLIC_TESTNET | EXTENDED_PRIVATE_TESTNET => AddressNetwork::Testnet,
        _ => AddressNetwork::Mainnet,
    }
}

pub(super) fn accepts_extended_network(
    wallet_network: AddressNetwork,
    key_network: AddressNetwork,
) -> bool {
    matches!(
        (wallet_network, key_network),
        (AddressNetwork::Mainnet, AddressNetwork::Mainnet)
            | (AddressNetwork::Testnet, AddressNetwork::Testnet)
            | (AddressNetwork::Signet, AddressNetwork::Testnet)
            | (AddressNetwork::Regtest, AddressNetwork::Testnet)
    )
}

pub(super) fn decode_extended_private_key(
    network: AddressNetwork,
    secret_bytes: &[u8],
) -> Result<PrivateKey, WalletError> {
    let secret_key = SecretKey::from_byte_array(
        <[u8; 32]>::try_from(secret_bytes).map_err(|_| WalletError::InvalidPrivateKey)?,
    )
    .map_err(|_| WalletError::InvalidPrivateKey)?;
    let wif = encode_wif_from_parts(network, &secret_key.secret_bytes(), true);
    PrivateKey::from_wif(&wif)
}

fn decode_base58check(input: &str) -> Result<Vec<u8>, WalletError> {
    let decoded = super::format::base58_decode(input)?;
    if decoded.len() < CHECKSUM_SIZE {
        return Err(WalletError::InvalidBase58(
            "base58check payload shorter than checksum".to_string(),
        ));
    }
    let (payload, checksum) = decoded.split_at(decoded.len() - CHECKSUM_SIZE);
    let expected = open_bitcoin_consensus::crypto::double_sha256(payload);
    if checksum != &expected[..CHECKSUM_SIZE] {
        return Err(WalletError::InvalidChecksum);
    }
    Ok(payload.to_vec())
}

impl ExtendedPublicKey {
    pub(super) fn derive_child(&self, step: DerivationStep) -> Result<Self, WalletError> {
        let child_index = step.index();
        if step.is_hardened() {
            return Err(WalletError::UnsupportedDescriptor(
                "cannot derive hardened child without private key material".to_string(),
            ));
        }
        let mut data = public_key_bytes(&self.public_key, true);
        data.extend_from_slice(&child_index.to_be_bytes());
        let mac = hmac_sha512(&self.chain_code, &data);
        let tweak =
            Scalar::from_be_bytes(array_from_slice_32(&mac[..32], "extended public tweak")?)
                .map_err(|_| WalletError::InvalidPrivateKey)?;
        let secp = Secp256k1::verification_only();
        let public_key = self
            .public_key
            .add_exp_tweak(&secp, &tweak)
            .map_err(|_| WalletError::InvalidPublicKey)?;
        let chain_code = array_from_slice_32(&mac[32..], "extended public child chain code")?;

        Ok(Self {
            network: self.network,
            chain_code,
            public_key,
        })
    }
}

impl ExtendedPrivateKey {
    pub(super) fn derive_child(&self, step: DerivationStep) -> Result<Self, WalletError> {
        let child_index = step.index();
        let mut data = Vec::with_capacity(37);
        if step.is_hardened() {
            data.push(0);
            data.extend_from_slice(self.private_key.secret_key().secret_bytes().as_slice());
        } else {
            data.extend_from_slice(
                public_key_bytes(&self.private_key.public_key(), true).as_slice(),
            );
        }
        data.extend_from_slice(&child_index.to_be_bytes());
        let mac = hmac_sha512(&self.chain_code, &data);
        let tweak =
            Scalar::from_be_bytes(array_from_slice_32(&mac[..32], "extended private tweak")?)
                .map_err(|_| WalletError::InvalidPrivateKey)?;
        let secret_key = (*self.private_key.secret_key())
            .add_tweak(&tweak)
            .map_err(|_| WalletError::InvalidPrivateKey)?;
        let chain_code = array_from_slice_32(&mac[32..], "extended private child chain code")?;
        let private_key = decode_extended_private_key(self.network, &secret_key.secret_bytes())?;

        Ok(Self {
            network: self.network,
            chain_code,
            private_key,
        })
    }
}

impl DerivationStep {
    pub(super) fn is_hardened(self) -> bool {
        matches!(self, Self::Hardened(_))
    }

    pub(super) fn index(self) -> u32 {
        match self {
            Self::Unhardened(value) => value,
            Self::Hardened(value) => value + HARDENED_INDEX,
        }
    }

    pub(super) fn display(self) -> String {
        match self {
            Self::Unhardened(value) => value.to_string(),
            Self::Hardened(value) => format!("{value}h"),
        }
    }
}

impl Wildcard {
    pub(super) fn step(self, index: u32) -> DerivationStep {
        match self {
            Self::Unhardened => DerivationStep::Unhardened(index),
            Self::Hardened => DerivationStep::Hardened(index),
        }
    }
}

fn array_from_slice_4(slice: &[u8], label: &str) -> Result<[u8; 4], WalletError> {
    <[u8; 4]>::try_from(slice)
        .map_err(|_| WalletError::DescriptorSyntax(format!("{label} must be 4 bytes")))
}

fn array_from_slice_32(slice: &[u8], label: &str) -> Result<[u8; 32], WalletError> {
    <[u8; 32]>::try_from(slice)
        .map_err(|_| WalletError::DescriptorSyntax(format!("{label} must be 32 bytes")))
}

fn copy_array_8(slice: &[u8]) -> [u8; 8] {
    let mut output = [0_u8; 8];
    output.copy_from_slice(slice);
    output
}

fn hmac_sha512(key: &[u8; 32], data: &[u8]) -> [u8; SHA512_OUTPUT_SIZE] {
    let mut block = [0_u8; SHA512_BLOCK_SIZE];
    block[..key.len()].copy_from_slice(key);

    let mut inner = [0x36_u8; SHA512_BLOCK_SIZE];
    let mut outer = [0x5c_u8; SHA512_BLOCK_SIZE];
    for (index, byte) in block.iter().enumerate() {
        inner[index] ^= byte;
        outer[index] ^= byte;
    }

    let mut inner_data = Vec::with_capacity(SHA512_BLOCK_SIZE + data.len());
    inner_data.extend_from_slice(&inner);
    inner_data.extend_from_slice(data);
    let inner_hash = sha512(&inner_data);

    let mut outer_data = Vec::with_capacity(SHA512_BLOCK_SIZE + SHA512_OUTPUT_SIZE);
    outer_data.extend_from_slice(&outer);
    outer_data.extend_from_slice(&inner_hash);
    sha512(&outer_data)
}

fn sha512(data: &[u8]) -> [u8; SHA512_OUTPUT_SIZE] {
    const INITIAL: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];
    const K: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];

    let bit_len = (data.len() as u128) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while !(padded.len() + 16).is_multiple_of(128) {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    let mut state = INITIAL;
    for chunk in padded.chunks_exact(128) {
        let mut w = [0_u64; 80];
        for (index, word) in w.iter_mut().take(16).enumerate() {
            let start = index * 8;
            *word = u64::from_be_bytes(copy_array_8(&chunk[start..start + 8]));
        }
        for index in 16..80 {
            let s0 = w[index - 15].rotate_right(1)
                ^ w[index - 15].rotate_right(8)
                ^ (w[index - 15] >> 7);
            let s1 =
                w[index - 2].rotate_right(19) ^ w[index - 2].rotate_right(61) ^ (w[index - 2] >> 6);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for index in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut output = [0_u8; SHA512_OUTPUT_SIZE];
    for (index, value) in state.iter().enumerate() {
        output[index * 8..(index + 1) * 8].copy_from_slice(&value.to_be_bytes());
    }
    output
}
