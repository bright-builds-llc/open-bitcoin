use secp256k1::{Keypair, PublicKey, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};

use open_bitcoin_consensus::{crypto::hash160, taproot_tagged_hash};
use open_bitcoin_primitives::ScriptBuf;

use crate::WalletError;

const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32_ALPHABET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const BECH32M_CONST: u32 = 0x2bc8_30a3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressNetwork {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl AddressNetwork {
    pub const fn p2pkh_prefix(self) -> u8 {
        match self {
            Self::Mainnet => 0x00,
            Self::Testnet | Self::Signet | Self::Regtest => 0x6f,
        }
    }

    pub const fn p2sh_prefix(self) -> u8 {
        match self {
            Self::Mainnet => 0x05,
            Self::Testnet | Self::Signet | Self::Regtest => 0xc4,
        }
    }

    pub const fn wif_prefix(self) -> u8 {
        match self {
            Self::Mainnet => 0x80,
            Self::Testnet | Self::Signet | Self::Regtest => 0xef,
        }
    }

    pub const fn hrp(self) -> &'static str {
        match self {
            Self::Mainnet => "bc",
            Self::Testnet | Self::Signet => "tb",
            Self::Regtest => "bcrt",
        }
    }

    pub fn from_wif_prefix(prefix: u8) -> Result<Self, WalletError> {
        match prefix {
            0x80 => Ok(Self::Mainnet),
            0xef => Ok(Self::Testnet),
            _ => Err(WalletError::InvalidPrivateKey),
        }
    }

    pub const fn accepts_wif_network(self, wif_network: Self) -> bool {
        matches!(
            (self, wif_network),
            (Self::Mainnet, Self::Mainnet)
                | (Self::Testnet, Self::Testnet)
                | (Self::Signet, Self::Testnet)
                | (Self::Regtest, Self::Testnet)
        )
    }
}

impl core::fmt::Display for AddressNetwork {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Testnet => write!(f, "testnet"),
            Self::Signet => write!(f, "signet"),
            Self::Regtest => write!(f, "regtest"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    P2pkh,
    ShWpkh,
    Wpkh,
    Tr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub network: AddressNetwork,
    pub address_type: AddressType,
    text: String,
    pub script_pubkey: ScriptBuf,
}

impl Address {
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    network: AddressNetwork,
    secret_key: SecretKey,
    compressed: bool,
}

impl PrivateKey {
    pub fn from_wif(wif: &str) -> Result<Self, WalletError> {
        let decoded = base58_decode(wif)?;
        if decoded.len() != 37 && decoded.len() != 38 {
            return Err(WalletError::InvalidPrivateKey);
        }

        let (payload, checksum) = decoded.split_at(decoded.len() - 4);
        let expected = open_bitcoin_consensus::crypto::double_sha256(payload);
        if checksum != &expected[..4] {
            return Err(WalletError::InvalidChecksum);
        }

        let network = AddressNetwork::from_wif_prefix(payload[0])?;
        let (key_bytes, compressed) = match payload.len() {
            33 => (&payload[1..33], false),
            34 if payload[33] == 0x01 => (&payload[1..33], true),
            _ => return Err(WalletError::InvalidPrivateKey),
        };
        let secret_key = SecretKey::from_byte_array(
            <[u8; 32]>::try_from(key_bytes).map_err(|_| WalletError::InvalidPrivateKey)?,
        )
        .map_err(|_| WalletError::InvalidPrivateKey)?;

        Ok(Self {
            network,
            secret_key,
            compressed,
        })
    }

    pub fn network(&self) -> AddressNetwork {
        self.network
    }

    pub fn compressed(&self) -> bool {
        self.compressed
    }

    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    pub fn public_key(&self) -> PublicKey {
        let secp = Secp256k1::new();
        PublicKey::from_secret_key(&secp, &self.secret_key)
    }

    pub fn xonly_public_key(&self) -> XOnlyPublicKey {
        self.public_key().x_only_public_key().0
    }
}

pub(crate) fn decode_hex(input: &str) -> Result<Vec<u8>, WalletError> {
    let trimmed = input.trim();
    if !trimmed.len().is_multiple_of(2) {
        return Err(WalletError::InvalidHex(
            "hex strings must have even length".to_string(),
        ));
    }

    let mut out = Vec::with_capacity(trimmed.len() / 2);
    let bytes = trimmed.as_bytes();
    for pair in bytes.chunks_exact(2) {
        let high = decode_nibble(pair[0])?;
        let low = decode_nibble(pair[1])?;
        out.push((high << 4) | low);
    }

    Ok(out)
}

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(nibble_to_hex(byte >> 4));
        output.push(nibble_to_hex(byte & 0x0f));
    }
    output
}

pub(crate) fn p2pkh_script(pubkey: &PublicKey) -> Result<ScriptBuf, WalletError> {
    let pubkey_hash = hash160(&public_key_bytes(pubkey, true));
    script(&[0x76, 0xa9, 20], Some(&pubkey_hash), &[0x88, 0xac])
}

pub(crate) fn p2wpkh_program(pubkey: &PublicKey) -> Result<[u8; 20], WalletError> {
    Ok(hash160(&public_key_bytes(pubkey, true)))
}

pub(crate) fn p2wpkh_script(pubkey: &PublicKey) -> Result<ScriptBuf, WalletError> {
    let program = p2wpkh_program(pubkey)?;
    script(&[0x00, 20], Some(&program), &[])
}

pub(crate) fn sh_wpkh_redeem_script(pubkey: &PublicKey) -> Result<ScriptBuf, WalletError> {
    p2wpkh_script(pubkey)
}

pub(crate) fn sh_wpkh_script(pubkey: &PublicKey) -> Result<ScriptBuf, WalletError> {
    let redeem_script = sh_wpkh_redeem_script(pubkey)?;
    let redeem_hash = hash160(redeem_script.as_bytes());
    script(&[0xa9, 20], Some(&redeem_hash), &[0x87])
}

pub(crate) fn taproot_output_key_from_private_key(
    private_key: &PrivateKey,
) -> Result<(Keypair, XOnlyPublicKey), WalletError> {
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, private_key.secret_key());
    let (internal_key, _) = XOnlyPublicKey::from_keypair(&keypair);
    let tweak = tap_tweak_scalar(&internal_key, None)?;
    let tweaked_keypair = keypair
        .add_xonly_tweak(&secp, &tweak)
        .map_err(|_| WalletError::TaprootTweak)?;
    let (output_key, _) = XOnlyPublicKey::from_keypair(&tweaked_keypair);

    Ok((tweaked_keypair, output_key))
}

pub(crate) fn taproot_output_key_from_xonly(
    internal_key: &XOnlyPublicKey,
) -> Result<XOnlyPublicKey, WalletError> {
    let secp = Secp256k1::verification_only();
    let tweak = tap_tweak_scalar(internal_key, None)?;
    let (output_key, _) = internal_key
        .add_tweak(&secp, &tweak)
        .map_err(|_| WalletError::TaprootTweak)?;

    Ok(output_key)
}

pub(crate) fn taproot_script(output_key: &XOnlyPublicKey) -> Result<ScriptBuf, WalletError> {
    script(&[0x51, 32], Some(&output_key.serialize()), &[])
}

pub fn p2pkh_address(network: AddressNetwork, pubkey: &PublicKey) -> Result<Address, WalletError> {
    let pubkey_hash = hash160(&public_key_bytes(pubkey, true));
    let text = base58check_encode(network.p2pkh_prefix(), &pubkey_hash);
    let script_pubkey = p2pkh_script(pubkey)?;

    Ok(Address {
        network,
        address_type: AddressType::P2pkh,
        text,
        script_pubkey,
    })
}

pub fn sh_wpkh_address(
    network: AddressNetwork,
    pubkey: &PublicKey,
) -> Result<Address, WalletError> {
    let redeem_script = sh_wpkh_redeem_script(pubkey)?;
    let redeem_hash = hash160(redeem_script.as_bytes());
    let text = base58check_encode(network.p2sh_prefix(), &redeem_hash);
    let script_pubkey = sh_wpkh_script(pubkey)?;

    Ok(Address {
        network,
        address_type: AddressType::ShWpkh,
        text,
        script_pubkey,
    })
}

pub fn wpkh_address(network: AddressNetwork, pubkey: &PublicKey) -> Result<Address, WalletError> {
    let program = p2wpkh_program(pubkey)?;
    let text = encode_segwit_address(network.hrp(), 0, &program)?;
    let script_pubkey = p2wpkh_script(pubkey)?;

    Ok(Address {
        network,
        address_type: AddressType::Wpkh,
        text,
        script_pubkey,
    })
}

pub fn tr_address(
    network: AddressNetwork,
    output_key: &XOnlyPublicKey,
) -> Result<Address, WalletError> {
    let text = encode_segwit_address(network.hrp(), 1, &output_key.serialize())?;
    let script_pubkey = taproot_script(output_key)?;

    Ok(Address {
        network,
        address_type: AddressType::Tr,
        text,
        script_pubkey,
    })
}

pub(crate) fn push_data(data: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(data.len() + 3);
    match data.len() {
        0..=75 => bytes.push(data.len() as u8),
        76..=255 => {
            bytes.push(0x4c);
            bytes.push(data.len() as u8);
        }
        256..=520 => {
            bytes.push(0x4d);
            bytes.extend_from_slice(&(data.len() as u16).to_le_bytes());
        }
        _ => panic!("push length out of supported range"),
    }
    bytes.extend_from_slice(data);
    bytes
}

pub(crate) fn public_key_bytes(pubkey: &PublicKey, compressed: bool) -> Vec<u8> {
    if compressed {
        pubkey.serialize().to_vec()
    } else {
        pubkey.serialize_uncompressed().to_vec()
    }
}

fn base58check_encode(prefix: u8, payload: &[u8]) -> String {
    let mut data = Vec::with_capacity(payload.len() + 5);
    data.push(prefix);
    data.extend_from_slice(payload);
    let checksum = open_bitcoin_consensus::crypto::double_sha256(&data);
    data.extend_from_slice(&checksum[..4]);
    base58_encode(&data)
}

fn base58_encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut digits = vec![0_u8];
    for &byte in bytes {
        let mut carry = u32::from(byte);
        for digit in &mut digits {
            let value = u32::from(*digit) * 256 + carry;
            *digit = (value % 58) as u8;
            carry = value / 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    let mut out = String::new();
    for byte in bytes {
        if *byte == 0 {
            out.push('1');
        } else {
            break;
        }
    }
    for digit in digits.iter().rev() {
        out.push(BASE58_ALPHABET[*digit as usize] as char);
    }
    out
}

fn base58_decode(input: &str) -> Result<Vec<u8>, WalletError> {
    if input.is_empty() {
        return Err(WalletError::InvalidBase58("empty string".to_string()));
    }

    let mut bytes = vec![0_u8];
    for character in input.bytes() {
        let Some(value) = BASE58_ALPHABET.iter().position(|byte| *byte == character) else {
            return Err(WalletError::InvalidBase58(format!(
                "unsupported character {}",
                character as char
            )));
        };

        let mut carry = value as u32;
        for byte in &mut bytes {
            let next = u32::from(*byte) * 58 + carry;
            *byte = (next & 0xff) as u8;
            carry = next >> 8;
        }
        while carry > 0 {
            bytes.push((carry & 0xff) as u8);
            carry >>= 8;
        }
    }

    for character in input.bytes() {
        if character == b'1' {
            bytes.push(0);
        } else {
            break;
        }
    }
    bytes.reverse();
    Ok(bytes)
}

fn encode_segwit_address(hrp: &str, version: u8, program: &[u8]) -> Result<String, WalletError> {
    if version > 16 {
        return Err(WalletError::UnsupportedDescriptor(
            "witness versions above 16 are unsupported".to_string(),
        ));
    }
    if version == 0 && !matches!(program.len(), 20 | 32) {
        return Err(WalletError::UnsupportedDescriptor(
            "witness v0 requires a 20- or 32-byte program".to_string(),
        ));
    }
    if !(2..=40).contains(&program.len()) {
        return Err(WalletError::UnsupportedDescriptor(
            "witness programs must be between 2 and 40 bytes".to_string(),
        ));
    }

    let mut data = Vec::with_capacity(1 + program.len());
    data.push(version);
    data.extend(convert_bits(program, 8, 5, true)?);
    let bech32m = version != 0;
    let checksum = create_bech32_checksum(hrp, &data, bech32m);
    let mut text = String::with_capacity(hrp.len() + 1 + data.len() + checksum.len());
    text.push_str(hrp);
    text.push('1');
    for value in data.into_iter().chain(checksum) {
        text.push(BECH32_ALPHABET[value as usize] as char);
    }

    Ok(text)
}

fn create_bech32_checksum(hrp: &str, data: &[u8], bech32m: bool) -> [u8; 6] {
    let mut values = hrp_expand(hrp);
    values.extend_from_slice(data);
    values.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    let polymod = bech32_polymod(&values) ^ if bech32m { BECH32M_CONST } else { 1 };
    let mut checksum = [0_u8; 6];
    for (index, item) in checksum.iter_mut().enumerate() {
        *item = ((polymod >> (5 * (5 - index))) & 0x1f) as u8;
    }
    checksum
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(hrp.len() * 2 + 1);
    for byte in hrp.bytes() {
        out.push(byte >> 5);
    }
    out.push(0);
    for byte in hrp.bytes() {
        out.push(byte & 0x1f);
    }
    out
}

fn bech32_polymod(values: &[u8]) -> u32 {
    const GENERATORS: [u32; 5] = [
        0x3b6a_57b2,
        0x2650_8e6d,
        0x1ea1_19fa,
        0x3d42_33dd,
        0x2a14_62b3,
    ];

    let mut checksum = 1_u32;
    for value in values {
        let top = checksum >> 25;
        checksum = ((checksum & 0x01ff_ffff) << 5) ^ u32::from(*value);
        for (index, generator) in GENERATORS.iter().enumerate() {
            if ((top >> index) & 1) != 0 {
                checksum ^= generator;
            }
        }
    }
    checksum
}

fn convert_bits(
    data: &[u8],
    from_bits: u32,
    to_bits: u32,
    pad: bool,
) -> Result<Vec<u8>, WalletError> {
    let mut acc = 0_u32;
    let mut bits = 0_u32;
    let maxv = (1_u32 << to_bits) - 1;
    let max_acc = (1_u32 << (from_bits + to_bits - 1)) - 1;
    let mut out = Vec::new();

    for value in data {
        if (u32::from(*value) >> from_bits) != 0 {
            return Err(WalletError::DescriptorSyntax(
                "bit-conversion input is out of range".to_string(),
            ));
        }
        acc = ((acc << from_bits) | u32::from(*value)) & max_acc;
        bits += from_bits;
        while bits >= to_bits {
            bits -= to_bits;
            out.push(((acc >> bits) & maxv) as u8);
        }
    }

    if pad {
        if bits > 0 {
            out.push(((acc << (to_bits - bits)) & maxv) as u8);
        }
    } else if bits >= from_bits || ((acc << (to_bits - bits)) & maxv) != 0 {
        return Err(WalletError::DescriptorSyntax(
            "non-zero padding during bit conversion".to_string(),
        ));
    }

    Ok(out)
}

fn script(
    prefix: &[u8],
    maybe_payload: Option<&[u8]>,
    suffix: &[u8],
) -> Result<ScriptBuf, WalletError> {
    let mut bytes = Vec::with_capacity(
        prefix.len() + maybe_payload.map_or(0, |payload| payload.len()) + suffix.len(),
    );
    bytes.extend_from_slice(prefix);
    if let Some(payload) = maybe_payload {
        bytes.extend_from_slice(payload);
    }
    bytes.extend_from_slice(suffix);
    Ok(ScriptBuf::from_bytes(bytes)?)
}

fn tap_tweak_scalar(
    internal_key: &XOnlyPublicKey,
    maybe_merkle_root: Option<[u8; 32]>,
) -> Result<Scalar, WalletError> {
    let mut preimage = Vec::with_capacity(64);
    preimage.extend_from_slice(&internal_key.serialize());
    if let Some(merkle_root) = maybe_merkle_root {
        preimage.extend_from_slice(&merkle_root);
    }

    Scalar::from_be_bytes(taproot_tagged_hash("TapTweak", &preimage).to_byte_array())
        .map_err(|_| WalletError::TaprootTweak)
}

fn decode_nibble(byte: u8) -> Result<u8, WalletError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(WalletError::InvalidHex(format!(
            "unsupported hex character {}",
            byte as char
        ))),
    }
}

const fn nibble_to_hex(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '?',
    }
}

#[cfg(test)]
mod tests;
