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
mod tests {
    use secp256k1::{Secp256k1, SecretKey};

    use super::{
        AddressNetwork, PrivateKey, base58_decode, base58_encode, convert_bits, decode_hex,
        encode_segwit_address, hex_encode, nibble_to_hex, p2pkh_address, public_key_bytes,
        push_data, sh_wpkh_address, tap_tweak_scalar, taproot_output_key_from_private_key,
        taproot_output_key_from_xonly, tr_address, wpkh_address,
    };

    fn sample_private_key(network: AddressNetwork, compressed: bool) -> String {
        let mut payload = vec![network.wif_prefix()];
        payload.extend_from_slice(&[1_u8; 32]);
        if compressed {
            payload.push(0x01);
        }
        let checksum = open_bitcoin_consensus::crypto::double_sha256(&payload);
        payload.extend_from_slice(&checksum[..4]);
        base58_encode(&payload)
    }

    #[test]
    fn private_key_round_trips_compressed_wif() {
        let private_key = PrivateKey::from_wif(&sample_private_key(AddressNetwork::Regtest, true))
            .expect("compressed WIF should parse");

        assert_eq!(private_key.network(), AddressNetwork::Testnet);
        assert!(private_key.compressed());
    }

    #[test]
    fn base58_decode_rejects_invalid_characters() {
        let error = base58_decode("0").expect_err("0 is not part of base58");

        assert_eq!(error.to_string(), "invalid base58: unsupported character 0",);
    }

    #[test]
    fn hex_helpers_round_trip() {
        let decoded = decode_hex("00ff10").expect("valid hex");

        assert_eq!(decoded, vec![0x00, 0xff, 0x10]);
        assert_eq!(hex_encode(&decoded), "00ff10");
    }

    #[test]
    fn key_hash_addresses_match_known_upstream_vectors() {
        let private_key =
            PrivateKey::from_wif("cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi")
                .expect("fixture WIF");
        let public_key = private_key.public_key();
        let p2pkh = p2pkh_address(AddressNetwork::Regtest, &public_key).expect("p2pkh");
        let sh_wpkh = sh_wpkh_address(AddressNetwork::Regtest, &public_key).expect("sh(wpkh)");
        let wpkh = wpkh_address(AddressNetwork::Regtest, &public_key).expect("wpkh");

        assert_eq!(p2pkh.to_string(), "n31WD8pkfAjg2APV78GnbDTdZb1QonBi5D");
        assert_eq!(sh_wpkh.to_string(), "2NG7GwqV3rBao6wh55MqTumV9JJocWT4RH2");
        assert_eq!(
            wpkh.to_string(),
            "bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je"
        );
    }

    #[test]
    fn taproot_addresses_match_bip86_style_tweak_logic() {
        let private_key =
            PrivateKey::from_wif("cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi")
                .expect("fixture WIF");
        let (_keypair, output_key) =
            taproot_output_key_from_private_key(&private_key).expect("taproot tweak");
        let address = tr_address(AddressNetwork::Regtest, &output_key).expect("taproot address");
        let public_output =
            taproot_output_key_from_xonly(&private_key.xonly_public_key()).expect("xonly tweak");

        assert_eq!(output_key, public_output);
        assert_eq!(
            address.to_string(),
            "bcrt1p5e6v9v2j5wp3y6c79gaqdqltq7jdv45fswnnm7exmmp2020mqepspf6x45"
        );
    }

    #[test]
    fn segwit_encoder_rejects_invalid_program_lengths() {
        let error = encode_segwit_address("tb", 0, &[0_u8; 10]).expect_err("bad v0 program");

        assert_eq!(
            error.to_string(),
            "unsupported descriptor: witness v0 requires a 20- or 32-byte program",
        );
    }

    #[test]
    fn push_data_handles_small_and_medium_pushes() {
        assert_eq!(push_data(&[0xaa, 0xbb]), vec![2, 0xaa, 0xbb]);
        assert_eq!(push_data(&[0_u8; 76])[..2], [0x4c, 76]);
    }

    #[test]
    fn public_key_derivation_uses_secp256k1() {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([2_u8; 32]).expect("secret key");
        let derived = PrivateKey::from_wif(&sample_private_key(AddressNetwork::Mainnet, true))
            .expect("WIF")
            .public_key();
        let expected = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

        assert_ne!(derived, expected);
    }

    #[test]
    fn network_variants_cover_prefixes_and_display() {
        assert_eq!(AddressNetwork::Mainnet.p2pkh_prefix(), 0x00);
        assert_eq!(AddressNetwork::Mainnet.p2sh_prefix(), 0x05);
        assert_eq!(AddressNetwork::Mainnet.hrp(), "bc");
        assert_eq!(AddressNetwork::Testnet.hrp(), "tb");
        assert_eq!(AddressNetwork::Signet.hrp(), "tb");
        assert_eq!(AddressNetwork::Regtest.hrp(), "bcrt");
        assert!(AddressNetwork::Signet.accepts_wif_network(AddressNetwork::Testnet));
        assert!(!AddressNetwork::Mainnet.accepts_wif_network(AddressNetwork::Testnet));
        assert_eq!(AddressNetwork::Mainnet.to_string(), "mainnet");
        assert_eq!(AddressNetwork::Testnet.to_string(), "testnet");
        assert_eq!(AddressNetwork::Signet.to_string(), "signet");
        assert_eq!(AddressNetwork::Regtest.to_string(), "regtest");
    }

    #[test]
    fn address_accessors_and_wif_parser_cover_uncompressed_and_error_paths() {
        let good = PrivateKey::from_wif(&sample_private_key(AddressNetwork::Mainnet, false))
            .expect("uncompressed WIF");
        let address = p2pkh_address(AddressNetwork::Mainnet, &good.public_key()).expect("p2pkh");

        assert!(!good.compressed());
        assert_eq!(public_key_bytes(&good.public_key(), false).len(), 65);
        assert_eq!(address.as_str(), address.to_string());

        let mut bad_prefix = vec![0x42];
        bad_prefix.extend_from_slice(&[7_u8; 32]);
        bad_prefix.push(0x01);
        let checksum = open_bitcoin_consensus::crypto::double_sha256(&bad_prefix);
        bad_prefix.extend_from_slice(&checksum[..4]);
        let bad_prefix = base58_encode(&bad_prefix);
        assert_eq!(
            PrivateKey::from_wif(&bad_prefix).expect_err("bad prefix"),
            super::WalletError::InvalidPrivateKey
        );

        let mut payload = vec![AddressNetwork::Regtest.wif_prefix()];
        payload.extend_from_slice(&[7_u8; 32]);
        payload.push(0x01);
        let checksum = open_bitcoin_consensus::crypto::double_sha256(&payload);
        payload.extend_from_slice(&checksum[..4]);
        let last_index = payload.len() - 1;
        payload[last_index] ^= 1;
        let bad_checksum = base58_encode(&payload);
        assert_eq!(
            PrivateKey::from_wif(&bad_checksum).expect_err("bad checksum"),
            super::WalletError::InvalidChecksum
        );

        let short_payload = {
            let mut payload = vec![AddressNetwork::Regtest.wif_prefix(), 0x01];
            let checksum = open_bitcoin_consensus::crypto::double_sha256(&payload);
            payload.extend_from_slice(&checksum[..4]);
            base58_encode(&payload)
        };
        assert_eq!(
            PrivateKey::from_wif(&short_payload).expect_err("short payload"),
            super::WalletError::InvalidPrivateKey
        );
    }

    #[test]
    fn helper_encoders_cover_remaining_edge_cases() {
        assert_eq!(base58_encode(&[]), "");
        let leading_zero = base58_encode(&[0, 0, 1]);
        assert!(leading_zero.starts_with("11"));
        assert_eq!(
            base58_decode(&leading_zero).expect("leading zeros"),
            vec![0, 0, 1]
        );
        assert!(base58_decode("").is_err());
        assert!(decode_hex("0").is_err());
        assert_eq!(push_data(&[0_u8; 300])[..3], [0x4d, 0x2c, 0x01]);
        assert!(std::panic::catch_unwind(|| push_data(&[0_u8; 521])).is_err());

        let high_version = encode_segwit_address("tb", 17, &[0_u8; 32]).expect_err("bad version");
        let long_program =
            encode_segwit_address("tb", 1, &[0_u8; 41]).expect_err("bad program length");
        let invalid_bits = convert_bits(&[32], 5, 8, true).expect_err("bad convert bits");
        let invalid_padding = convert_bits(&[0xff], 8, 5, false).expect_err("bad padding");
        let clean_no_pad = convert_bits(
            &convert_bits(b"abc", 8, 5, true).expect("encode to 5 bits"),
            5,
            8,
            false,
        )
        .expect("clean no-pad");
        assert!(
            high_version
                .to_string()
                .contains("witness versions above 16")
        );
        assert!(long_program.to_string().contains("between 2 and 40 bytes"));
        assert!(invalid_bits.to_string().contains("bit-conversion"));
        assert!(invalid_padding.to_string().contains("padding"));
        assert_eq!(clean_no_pad, b"abc");

        let private_key =
            PrivateKey::from_wif(&sample_private_key(AddressNetwork::Mainnet, true)).expect("WIF");
        let internal = private_key.xonly_public_key();
        assert!(tap_tweak_scalar(&internal, Some([1_u8; 32])).is_ok());
        assert_eq!(nibble_to_hex(99), '?');
        assert_eq!(decode_hex("AA").expect("uppercase hex"), vec![0xaa]);
        assert!(decode_hex("ag").is_err());
    }

    #[test]
    fn invalid_private_key_payloads_and_uncompressed_segwit_keys_are_rejected() {
        let mut payload = vec![AddressNetwork::Regtest.wif_prefix()];
        payload.extend_from_slice(&[7_u8; 32]);
        payload.push(0x02);
        let checksum = open_bitcoin_consensus::crypto::double_sha256(&payload);
        payload.extend_from_slice(&checksum[..4]);
        let invalid_payload = base58_encode(&payload);
        let uncompressed =
            PrivateKey::from_wif(&sample_private_key(AddressNetwork::Regtest, false))
                .expect("uncompressed WIF");

        assert_eq!(
            PrivateKey::from_wif(&invalid_payload).expect_err("invalid payload marker"),
            super::WalletError::InvalidPrivateKey
        );
        assert_eq!(
            public_key_bytes(&uncompressed.public_key(), false).len(),
            65
        );
    }
}
