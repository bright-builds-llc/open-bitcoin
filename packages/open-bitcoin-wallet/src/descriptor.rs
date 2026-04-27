// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};

use crate::WalletError;
use crate::address::{
    Address, AddressNetwork, PrivateKey, decode_hex, hex_encode, p2pkh_address, p2pkh_script,
    public_key_bytes, sh_wpkh_address, sh_wpkh_redeem_script, sh_wpkh_script,
    taproot_output_key_from_private_key, taproot_output_key_from_xonly, taproot_script, tr_address,
    wpkh_address,
};

const CHECKSUM_SIZE: usize = 4;
const EXTENDED_KEY_PAYLOAD_LEN: usize = 78;
const RANGE_METADATA_PREFIX: &str = "ob:";
const DEFAULT_RANGE_START: u32 = 0;
const DEFAULT_RANGE_END: u32 = 1000;
const EXTENDED_PUBLIC_MAINNET: u32 = 0x0488_b21e;
const EXTENDED_PRIVATE_MAINNET: u32 = 0x0488_ade4;
const EXTENDED_PUBLIC_TESTNET: u32 = 0x0435_87cf;
const EXTENDED_PRIVATE_TESTNET: u32 = 0x0435_8394;
const HARDENED_INDEX: u32 = 1 << 31;
const SHA512_BLOCK_SIZE: usize = 128;
const SHA512_OUTPUT_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorKind {
    Pkh,
    ShWpkh,
    Wpkh,
    Tr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorRole {
    External,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DescriptorRange {
    start: u32,
    end: u32,
    next_index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivationStep {
    Unhardened(u32),
    Hardened(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wildcard {
    Unhardened,
    Hardened,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyOrigin {
    fingerprint: [u8; 4],
    path: Vec<DerivationStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedPublicKeySource {
    origin: Option<KeyOrigin>,
    encoded_key: String,
    extended_key: ExtendedPublicKey,
    path: Vec<DerivationStep>,
    maybe_wildcard: Option<Wildcard>,
    maybe_range: Option<DescriptorRange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedPrivateKeySource {
    origin: Option<KeyOrigin>,
    encoded_key: String,
    extended_key: ExtendedPrivateKey,
    path: Vec<DerivationStep>,
    maybe_wildcard: Option<Wildcard>,
    maybe_range: Option<DescriptorRange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedPublicKey {
    network: AddressNetwork,
    chain_code: [u8; 32],
    public_key: PublicKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedPrivateKey {
    network: AddressNetwork,
    chain_code: [u8; 32],
    private_key: PrivateKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    Public(PublicKey),
    Private(PrivateKey),
    ExtendedPublic(ExtendedPublicKeySource),
    ExtendedPrivate(ExtendedPrivateKeySource),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaprootKeySource {
    Public(XOnlyPublicKey),
    Private(PrivateKey),
    ExtendedPublic(ExtendedPublicKeySource),
    ExtendedPrivate(ExtendedPrivateKeySource),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleKeyDescriptor {
    Pkh(KeySource),
    ShWpkh(KeySource),
    Wpkh(KeySource),
    Tr(TaprootKeySource),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorRecord {
    pub id: u32,
    pub label: String,
    pub role: DescriptorRole,
    pub original_text: String,
    pub descriptor: SingleKeyDescriptor,
}

impl SingleKeyDescriptor {
    pub fn parse(text: &str, network: AddressNetwork) -> Result<Self, WalletError> {
        let (body, maybe_range) = split_descriptor_body_and_range(text)?;
        if body.contains('<') {
            return Err(WalletError::UnsupportedDescriptor(
                "multipath descriptors remain deferred".to_string(),
            ));
        }
        if body.contains("multi(") || body.contains("sortedmulti(") || body.contains("thresh(") {
            return Err(WalletError::UnsupportedDescriptor(
                "miniscript and multisig descriptors remain deferred".to_string(),
            ));
        }
        if body.starts_with("wsh(") {
            return Err(WalletError::UnsupportedDescriptor(
                "miniscript and multisig descriptors remain deferred".to_string(),
            ));
        }

        if let Some(inner) = body
            .strip_prefix("sh(wpkh(")
            .and_then(|value| value.strip_suffix("))"))
        {
            return Ok(Self::ShWpkh(parse_key_source(
                inner,
                network,
                maybe_range,
                false,
            )?));
        }
        if let Some(inner) = body
            .strip_prefix("pkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Pkh(parse_key_source(
                inner,
                network,
                maybe_range,
                false,
            )?));
        }
        if let Some(inner) = body
            .strip_prefix("wpkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Wpkh(parse_key_source(
                inner,
                network,
                maybe_range,
                false,
            )?));
        }
        if let Some(inner) = body
            .strip_prefix("tr(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Tr(parse_taproot_key_source(
                inner,
                network,
                maybe_range,
            )?));
        }

        Err(WalletError::DescriptorSyntax(format!(
            "expected pkh(...), sh(wpkh(...)), wpkh(...), or tr(...), got {body}"
        )))
    }

    pub fn kind(&self) -> DescriptorKind {
        match self {
            Self::Pkh(_) => DescriptorKind::Pkh,
            Self::ShWpkh(_) => DescriptorKind::ShWpkh,
            Self::Wpkh(_) => DescriptorKind::Wpkh,
            Self::Tr(_) => DescriptorKind::Tr,
        }
    }

    pub fn address(&self, network: AddressNetwork) -> Result<Address, WalletError> {
        self.address_at(network, self.current_index())
    }

    pub fn address_at(&self, network: AddressNetwork, index: u32) -> Result<Address, WalletError> {
        match self {
            Self::Pkh(key) => p2pkh_address(network, &key.public_key_at(index)?),
            Self::ShWpkh(key) => sh_wpkh_address(network, &key.public_key_at(index)?),
            Self::Wpkh(key) => wpkh_address(network, &key.public_key_at(index)?),
            Self::Tr(key) => tr_address(network, &key.output_key_at(index)?),
        }
    }

    pub fn script_pubkey(&self) -> Result<open_bitcoin_primitives::ScriptBuf, WalletError> {
        self.script_pubkey_at(self.current_index())
    }

    pub fn script_pubkey_at(
        &self,
        index: u32,
    ) -> Result<open_bitcoin_primitives::ScriptBuf, WalletError> {
        match self {
            Self::Pkh(key) => p2pkh_script(&key.public_key_at(index)?),
            Self::ShWpkh(key) => sh_wpkh_script(&key.public_key_at(index)?),
            Self::Wpkh(key) => crate::address::p2wpkh_script(&key.public_key_at(index)?),
            Self::Tr(key) => taproot_script(&key.output_key_at(index)?),
        }
    }

    pub fn estimated_input_vbytes(&self) -> usize {
        match self {
            Self::Pkh(key) => {
                let public_key_len = if key.is_compressed() { 33 } else { 65 };
                41 + 1 + 1 + 73 + 1 + public_key_len + 4
            }
            Self::ShWpkh(_) => 91,
            Self::Wpkh(_) => 68,
            Self::Tr(_) => 58,
        }
    }

    pub fn can_sign(&self) -> bool {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.can_sign(),
            Self::Tr(key) => key.can_sign(),
        }
    }

    pub fn display_text(&self) -> String {
        match self {
            Self::Pkh(key) => format!("pkh({})", key.display_key()),
            Self::ShWpkh(key) => format!("sh(wpkh({}))", key.display_key()),
            Self::Wpkh(key) => format!("wpkh({})", key.display_key()),
            Self::Tr(key) => format!("tr({})", key.display_key()),
        }
    }

    pub fn is_ranged(&self) -> bool {
        self.range().is_some()
    }

    pub fn range_start(&self) -> Option<u32> {
        self.range().map(|range| range.start)
    }

    pub fn range_end(&self) -> Option<u32> {
        self.range().map(|range| range.end)
    }

    pub fn next_index(&self) -> Option<u32> {
        self.range().map(|range| range.next_index)
    }

    pub fn storage_text(&self) -> String {
        let body = match self {
            Self::Pkh(key) => format!("pkh({})", key.storage_key_text()),
            Self::ShWpkh(key) => format!("sh(wpkh({}))", key.storage_key_text()),
            Self::Wpkh(key) => format!("wpkh({})", key.storage_key_text()),
            Self::Tr(key) => format!("tr({})", key.storage_key_text()),
        };
        let Some(range) = self.range() else {
            return body;
        };
        format!(
            "{body}#{}:{}:{}:{}",
            RANGE_METADATA_PREFIX, range.start, range.end, range.next_index
        )
    }

    pub fn signing_public_key_bytes(&self) -> Result<Option<Vec<u8>>, WalletError> {
        self.signing_public_key_bytes_at(self.current_index())
    }

    pub fn signing_public_key_bytes_at(&self, index: u32) -> Result<Option<Vec<u8>>, WalletError> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => Ok(Some(public_key_bytes(
                &key.public_key_at(index)?,
                key.is_compressed(),
            ))),
            Self::Tr(_) => Ok(None),
        }
    }

    pub fn redeem_script(&self) -> Result<Option<open_bitcoin_primitives::ScriptBuf>, WalletError> {
        self.redeem_script_at(self.current_index())
    }

    pub fn redeem_script_at(
        &self,
        index: u32,
    ) -> Result<Option<open_bitcoin_primitives::ScriptBuf>, WalletError> {
        match self {
            Self::ShWpkh(key) => Ok(Some(sh_wpkh_redeem_script(&key.public_key_at(index)?)?)),
            _ => Ok(None),
        }
    }

    pub fn private_key_at(&self, index: u32) -> Result<Option<PrivateKey>, WalletError> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.private_key_at(index),
            Self::Tr(key) => key.private_key_at(index),
        }
    }

    pub fn matching_index(
        &self,
        script_pubkey: &open_bitcoin_primitives::ScriptBuf,
    ) -> Result<Option<u32>, WalletError> {
        let Some(range) = self.range() else {
            return Ok((self.script_pubkey()? == *script_pubkey).then_some(0));
        };

        for index in range.start..=range.end {
            if self.script_pubkey_at(index)? == *script_pubkey {
                return Ok(Some(index));
            }
        }
        Ok(None)
    }

    pub fn advance_next_index(&mut self, role: DescriptorRole) -> Result<u32, WalletError> {
        let Some(range) = self.range_mut() else {
            return Err(WalletError::UnsupportedAddressRole(
                role_name(role).to_string(),
            ));
        };
        if range.start > range.end {
            return Err(WalletError::InvalidDescriptorRange {
                start: range.start,
                end: range.end,
            });
        }
        if range.next_index > range.end {
            return Err(WalletError::DescriptorCursorExhausted {
                role: role_name(role).to_string(),
                next_index: range.next_index,
                range_end: range.end,
            });
        }

        let index = range.next_index;
        range.next_index = range.next_index.saturating_add(1);
        Ok(index)
    }

    fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.range(),
            Self::Tr(key) => key.range(),
        }
    }

    fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.range_mut(),
            Self::Tr(key) => key.range_mut(),
        }
    }

    fn current_index(&self) -> u32 {
        self.range().map_or(0, |range| range.next_index)
    }
}

impl KeySource {
    pub fn public_key(&self) -> Result<PublicKey, WalletError> {
        self.public_key_at(self.current_index())
    }

    pub fn public_key_at(&self, index: u32) -> Result<PublicKey, WalletError> {
        match self {
            Self::Public(public_key) => Ok(*public_key),
            Self::Private(private_key) => Ok(private_key.public_key()),
            Self::ExtendedPublic(source) => source.public_key_at(index),
            Self::ExtendedPrivate(source) => source.public_key_at(index),
        }
    }

    pub fn private_key(&self) -> Option<&PrivateKey> {
        match self {
            Self::Public(_) => None,
            Self::Private(private_key) => Some(private_key),
            Self::ExtendedPublic(_) | Self::ExtendedPrivate(_) => None,
        }
    }

    pub fn private_key_at(&self, index: u32) -> Result<Option<PrivateKey>, WalletError> {
        match self {
            Self::Public(_) | Self::ExtendedPublic(_) => Ok(None),
            Self::Private(private_key) => Ok(Some(private_key.clone())),
            Self::ExtendedPrivate(source) => Ok(Some(source.private_key_at(index)?)),
        }
    }

    pub fn is_compressed(&self) -> bool {
        match self {
            Self::Public(public_key) => public_key.serialize().len() == 33,
            Self::Private(private_key) => private_key.compressed(),
            Self::ExtendedPublic(_) | Self::ExtendedPrivate(_) => true,
        }
    }

    pub fn can_sign(&self) -> bool {
        matches!(self, Self::Private(_) | Self::ExtendedPrivate(_))
    }

    fn display_key(&self) -> String {
        match self {
            Self::Public(public_key) => hex_encode(&public_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.public_key().serialize()),
            Self::ExtendedPublic(source) => source.display_text(),
            Self::ExtendedPrivate(source) => source.display_text(),
        }
    }

    fn storage_key_text(&self) -> String {
        match self {
            Self::Public(public_key) => hex_encode(&public_key.serialize()),
            Self::Private(private_key) => encode_wif(private_key),
            Self::ExtendedPublic(source) => source.storage_text(),
            Self::ExtendedPrivate(source) => source.storage_text(),
        }
    }

    fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range,
            Self::ExtendedPrivate(source) => source.maybe_range,
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range.as_mut(),
            Self::ExtendedPrivate(source) => source.maybe_range.as_mut(),
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    fn current_index(&self) -> u32 {
        self.range().map_or(0, |range| range.next_index)
    }
}

impl TaprootKeySource {
    pub fn output_key(&self) -> Result<XOnlyPublicKey, WalletError> {
        self.output_key_at(self.current_index())
    }

    pub fn output_key_at(&self, index: u32) -> Result<XOnlyPublicKey, WalletError> {
        match self {
            Self::Public(internal_key) => taproot_output_key_from_xonly(internal_key),
            Self::Private(private_key) => taproot_output_key_from_private_key(private_key)
                .map(|(_keypair, output_key)| output_key),
            Self::ExtendedPublic(source) => {
                let public_key = source.public_key_at(index)?;
                taproot_output_key_from_xonly(&public_key.x_only_public_key().0)
            }
            Self::ExtendedPrivate(source) => {
                let private_key = source.private_key_at(index)?;
                taproot_output_key_from_private_key(&private_key)
                    .map(|(_keypair, output_key)| output_key)
            }
        }
    }

    pub fn private_key(&self) -> Option<&PrivateKey> {
        match self {
            Self::Public(_) => None,
            Self::Private(private_key) => Some(private_key),
            Self::ExtendedPublic(_) | Self::ExtendedPrivate(_) => None,
        }
    }

    pub fn private_key_at(&self, index: u32) -> Result<Option<PrivateKey>, WalletError> {
        match self {
            Self::Public(_) | Self::ExtendedPublic(_) => Ok(None),
            Self::Private(private_key) => Ok(Some(private_key.clone())),
            Self::ExtendedPrivate(source) => Ok(Some(source.private_key_at(index)?)),
        }
    }

    pub fn can_sign(&self) -> bool {
        matches!(self, Self::Private(_) | Self::ExtendedPrivate(_))
    }

    fn display_key(&self) -> String {
        match self {
            Self::Public(internal_key) => hex_encode(&internal_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.xonly_public_key().serialize()),
            Self::ExtendedPublic(source) => source.display_text(),
            Self::ExtendedPrivate(source) => source.display_text(),
        }
    }

    fn storage_key_text(&self) -> String {
        match self {
            Self::Public(internal_key) => hex_encode(&internal_key.serialize()),
            Self::Private(private_key) => encode_wif(private_key),
            Self::ExtendedPublic(source) => source.storage_text(),
            Self::ExtendedPrivate(source) => source.storage_text(),
        }
    }

    fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range,
            Self::ExtendedPrivate(source) => source.maybe_range,
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range.as_mut(),
            Self::ExtendedPrivate(source) => source.maybe_range.as_mut(),
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    fn current_index(&self) -> u32 {
        self.range().map_or(0, |range| range.next_index)
    }
}

fn parse_key_source(
    text: &str,
    network: AddressNetwork,
    maybe_range: Option<DescriptorRange>,
    allow_xonly: bool,
) -> Result<KeySource, WalletError> {
    if let Ok(private_key) = PrivateKey::from_wif(text) {
        if !network.accepts_wif_network(private_key.network()) {
            return Err(WalletError::NetworkMismatch {
                expected: network.to_string(),
                actual: private_key.network().to_string(),
            });
        }
        return Ok(KeySource::Private(private_key));
    }

    if let Ok(source) = parse_extended_key_source(text, network, maybe_range) {
        return Ok(source);
    }

    let bytes = decode_hex(text)?;
    if allow_xonly && bytes.len() == 32 {
        return Err(WalletError::InvalidPublicKey);
    }
    let public_key = PublicKey::from_slice(&bytes).map_err(|_| WalletError::InvalidPublicKey)?;
    Ok(KeySource::Public(public_key))
}

fn parse_taproot_key_source(
    text: &str,
    network: AddressNetwork,
    maybe_range: Option<DescriptorRange>,
) -> Result<TaprootKeySource, WalletError> {
    if let Ok(private_key) = PrivateKey::from_wif(text) {
        if !network.accepts_wif_network(private_key.network()) {
            return Err(WalletError::NetworkMismatch {
                expected: network.to_string(),
                actual: private_key.network().to_string(),
            });
        }
        return Ok(TaprootKeySource::Private(private_key));
    }

    if let Ok(source) = parse_extended_key_source(text, network, maybe_range) {
        return match source {
            KeySource::ExtendedPublic(source) => Ok(TaprootKeySource::ExtendedPublic(source)),
            KeySource::ExtendedPrivate(source) => Ok(TaprootKeySource::ExtendedPrivate(source)),
            _ => Err(WalletError::DescriptorSyntax(
                "extended key parser returned unexpected direct key source".to_string(),
            )),
        };
    }

    let bytes = decode_hex(text)?;
    match bytes.len() {
        32 => {
            let array = <[u8; 32]>::try_from(bytes.as_slice())
                .map_err(|_| WalletError::InvalidXOnlyPublicKey)?;
            let public_key = XOnlyPublicKey::from_byte_array(array)
                .map_err(|_| WalletError::InvalidXOnlyPublicKey)?;
            Ok(TaprootKeySource::Public(public_key))
        }
        33 | 65 => {
            let public_key =
                PublicKey::from_slice(&bytes).map_err(|_| WalletError::InvalidPublicKey)?;
            Ok(TaprootKeySource::Public(public_key.x_only_public_key().0))
        }
        _ => Err(WalletError::InvalidXOnlyPublicKey),
    }
}

fn split_descriptor_body_and_range(
    text: &str,
) -> Result<(&str, Option<DescriptorRange>), WalletError> {
    let trimmed = text.trim();
    let mut parts = trimmed.split('#');
    let body = parts.next().unwrap_or(trimmed).trim();
    let mut maybe_range = None;

    for part in parts {
        let suffix = part.trim();
        let Some(encoded) = suffix.strip_prefix(RANGE_METADATA_PREFIX) else {
            continue;
        };
        let values = encoded.split(':').collect::<Vec<_>>();
        if values.len() != 3 {
            return Err(WalletError::DescriptorSyntax(format!(
                "invalid descriptor range metadata: {suffix}"
            )));
        }
        let start = values[0].parse::<u32>().map_err(|_| {
            WalletError::DescriptorSyntax(format!("invalid range start: {}", values[0]))
        })?;
        let end = values[1].parse::<u32>().map_err(|_| {
            WalletError::DescriptorSyntax(format!("invalid range end: {}", values[1]))
        })?;
        let next_index = values[2].parse::<u32>().map_err(|_| {
            WalletError::DescriptorSyntax(format!("invalid next index: {}", values[2]))
        })?;
        if start > end {
            return Err(WalletError::InvalidDescriptorRange { start, end });
        }
        maybe_range = Some(DescriptorRange {
            start,
            end,
            next_index,
        });
    }

    Ok((body, maybe_range))
}

fn parse_extended_key_source(
    text: &str,
    network: AddressNetwork,
    maybe_range: Option<DescriptorRange>,
) -> Result<KeySource, WalletError> {
    let (maybe_origin, remainder) = parse_origin(text)?;
    let mut segments = remainder.split('/').collect::<Vec<_>>();
    let encoded_key = segments
        .first()
        .ok_or_else(|| WalletError::DescriptorSyntax("missing extended key".to_string()))?
        .trim()
        .to_string();
    let _ = segments.remove(0);
    let (path, maybe_wildcard) = parse_derivation_segments(&segments)?;
    let maybe_range = if maybe_wildcard.is_some() {
        Some(maybe_range.unwrap_or(DescriptorRange {
            start: DEFAULT_RANGE_START,
            end: DEFAULT_RANGE_END,
            next_index: DEFAULT_RANGE_START,
        }))
    } else {
        maybe_range
    };

    match parse_extended_key(&encoded_key, network)? {
        ParsedExtendedKey::Public(extended_key) => {
            Ok(KeySource::ExtendedPublic(ExtendedPublicKeySource {
                origin: maybe_origin,
                encoded_key,
                extended_key,
                path,
                maybe_wildcard,
                maybe_range,
            }))
        }
        ParsedExtendedKey::Private(extended_key) => {
            Ok(KeySource::ExtendedPrivate(ExtendedPrivateKeySource {
                origin: maybe_origin,
                encoded_key,
                extended_key,
                path,
                maybe_wildcard,
                maybe_range,
            }))
        }
    }
}

fn parse_origin(text: &str) -> Result<(Option<KeyOrigin>, &str), WalletError> {
    let Some(stripped) = text.strip_prefix('[') else {
        return Ok((None, text));
    };
    let Some((origin, remainder)) = stripped.split_once(']') else {
        return Err(WalletError::DescriptorSyntax(
            "missing closing key origin bracket".to_string(),
        ));
    };
    let mut parts = origin.split('/');
    let fingerprint_text = parts.next().unwrap_or_default();
    if fingerprint_text.len() != 8 {
        return Err(WalletError::DescriptorSyntax(format!(
            "invalid key fingerprint: {fingerprint_text}"
        )));
    }
    let fingerprint_bytes = decode_hex(fingerprint_text)?;
    let fingerprint = <[u8; 4]>::try_from(fingerprint_bytes.as_slice()).map_err(|_| {
        WalletError::DescriptorSyntax(format!("invalid key fingerprint: {fingerprint_text}"))
    })?;
    let mut path = Vec::new();
    for part in parts {
        path.push(parse_derivation_step(part)?);
    }

    Ok((Some(KeyOrigin { fingerprint, path }), remainder))
}

fn parse_derivation_segments(
    segments: &[&str],
) -> Result<(Vec<DerivationStep>, Option<Wildcard>), WalletError> {
    let mut path = Vec::new();
    let mut maybe_wildcard = None;
    for (index, segment) in segments.iter().enumerate() {
        if segment.is_empty() {
            return Err(WalletError::DescriptorSyntax(
                "empty derivation step".to_string(),
            ));
        }
        if segment.starts_with('*') {
            if index + 1 != segments.len() {
                return Err(WalletError::DescriptorSyntax(
                    "wildcard must be the final derivation step".to_string(),
                ));
            }
            maybe_wildcard = Some(parse_wildcard(segment)?);
            continue;
        }
        path.push(parse_derivation_step(segment)?);
    }
    Ok((path, maybe_wildcard))
}

fn parse_wildcard(text: &str) -> Result<Wildcard, WalletError> {
    match text {
        "*" => Ok(Wildcard::Unhardened),
        "*'" | "*h" => Ok(Wildcard::Hardened),
        _ => Err(WalletError::DescriptorSyntax(format!(
            "unsupported wildcard derivation: {text}"
        ))),
    }
}

fn parse_derivation_step(text: &str) -> Result<DerivationStep, WalletError> {
    let hardened = text.ends_with('\'') || text.ends_with('h');
    let digits = if hardened {
        &text[..text.len() - 1]
    } else {
        text
    };
    let value = digits
        .parse::<u32>()
        .map_err(|_| WalletError::DescriptorSyntax(format!("invalid derivation step: {text}")))?;
    if value >= HARDENED_INDEX {
        return Err(WalletError::DescriptorSyntax(format!(
            "derivation step out of range: {text}"
        )));
    }
    Ok(if hardened {
        DerivationStep::Hardened(value)
    } else {
        DerivationStep::Unhardened(value)
    })
}

enum ParsedExtendedKey {
    Public(ExtendedPublicKey),
    Private(ExtendedPrivateKey),
}

fn parse_extended_key(
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

fn network_for_extended_version(version: u32) -> AddressNetwork {
    match version {
        EXTENDED_PUBLIC_MAINNET | EXTENDED_PRIVATE_MAINNET => AddressNetwork::Mainnet,
        EXTENDED_PUBLIC_TESTNET | EXTENDED_PRIVATE_TESTNET => AddressNetwork::Testnet,
        _ => AddressNetwork::Mainnet,
    }
}

fn accepts_extended_network(wallet_network: AddressNetwork, key_network: AddressNetwork) -> bool {
    matches!(
        (wallet_network, key_network),
        (AddressNetwork::Mainnet, AddressNetwork::Mainnet)
            | (AddressNetwork::Testnet, AddressNetwork::Testnet)
            | (AddressNetwork::Signet, AddressNetwork::Testnet)
            | (AddressNetwork::Regtest, AddressNetwork::Testnet)
    )
}

fn decode_extended_private_key(
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
    let decoded = base58_decode(input)?;
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

fn role_name(role: DescriptorRole) -> &'static str {
    match role {
        DescriptorRole::External => "external",
        DescriptorRole::Internal => "internal",
    }
}

impl ExtendedPublicKeySource {
    fn public_key_at(&self, index: u32) -> Result<PublicKey, WalletError> {
        let mut key = self.extended_key.clone();
        for step in &self.path {
            key = key.derive_child(*step)?;
        }
        if let Some(wildcard) = self.maybe_wildcard {
            key = key.derive_child(wildcard.step(index))?;
        }
        Ok(key.public_key)
    }

    fn display_text(&self) -> String {
        self.storage_text()
    }

    fn storage_text(&self) -> String {
        format!(
            "{}{}{}{}",
            format_origin(self.origin.as_ref()),
            self.encoded_key,
            format_derivation_path(&self.path),
            format_wildcard(self.maybe_wildcard),
        )
    }
}

impl ExtendedPrivateKeySource {
    fn public_key_at(&self, index: u32) -> Result<PublicKey, WalletError> {
        Ok(self.private_key_at(index)?.public_key())
    }

    fn private_key_at(&self, index: u32) -> Result<PrivateKey, WalletError> {
        let mut key = self.extended_key.clone();
        for step in &self.path {
            key = key.derive_child(*step)?;
        }
        if let Some(wildcard) = self.maybe_wildcard {
            key = key.derive_child(wildcard.step(index))?;
        }
        Ok(key.private_key)
    }

    fn display_text(&self) -> String {
        self.storage_text()
    }

    fn storage_text(&self) -> String {
        format!(
            "{}{}{}{}",
            format_origin(self.origin.as_ref()),
            self.encoded_key,
            format_derivation_path(&self.path),
            format_wildcard(self.maybe_wildcard),
        )
    }
}

impl ExtendedPublicKey {
    fn derive_child(&self, step: DerivationStep) -> Result<Self, WalletError> {
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
    fn derive_child(&self, step: DerivationStep) -> Result<Self, WalletError> {
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
    fn is_hardened(self) -> bool {
        matches!(self, Self::Hardened(_))
    }

    fn index(self) -> u32 {
        match self {
            Self::Unhardened(value) => value,
            Self::Hardened(value) => value + HARDENED_INDEX,
        }
    }

    fn display(self) -> String {
        match self {
            Self::Unhardened(value) => value.to_string(),
            Self::Hardened(value) => format!("{value}h"),
        }
    }
}

impl Wildcard {
    fn step(self, index: u32) -> DerivationStep {
        match self {
            Self::Unhardened => DerivationStep::Unhardened(index),
            Self::Hardened => DerivationStep::Hardened(index),
        }
    }
}

fn format_origin(maybe_origin: Option<&KeyOrigin>) -> String {
    let Some(origin) = maybe_origin else {
        return String::new();
    };
    let mut text = String::from("[");
    text.push_str(&hex_encode(&origin.fingerprint));
    for step in &origin.path {
        text.push('/');
        text.push_str(&step.display());
    }
    text.push(']');
    text
}

fn format_derivation_path(path: &[DerivationStep]) -> String {
    let mut text = String::new();
    for step in path {
        text.push('/');
        text.push_str(&step.display());
    }
    text
}

fn format_wildcard(maybe_wildcard: Option<Wildcard>) -> String {
    match maybe_wildcard {
        None => String::new(),
        Some(Wildcard::Unhardened) => "/*".to_string(),
        Some(Wildcard::Hardened) => "/*h".to_string(),
    }
}

fn encode_wif(private_key: &PrivateKey) -> String {
    encode_wif_from_parts(
        private_key.network(),
        &private_key.secret_key().secret_bytes(),
        private_key.compressed(),
    )
}

fn encode_wif_from_parts(
    network: AddressNetwork,
    secret_bytes: &[u8; 32],
    compressed: bool,
) -> String {
    let mut payload = Vec::with_capacity(34);
    payload.push(network.wif_prefix());
    payload.extend_from_slice(secret_bytes);
    if compressed {
        payload.push(1);
    }
    let checksum = open_bitcoin_consensus::crypto::double_sha256(&payload);
    payload.extend_from_slice(&checksum[..CHECKSUM_SIZE]);
    base58_encode(&payload)
}

fn base58_encode(bytes: &[u8]) -> String {
    const BASE58_ALPHABET: &[u8; 58] =
        b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
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
    const BASE58_ALPHABET: &[u8; 58] =
        b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
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

#[cfg(test)]
mod tests {
    use crate::address::AddressNetwork;

    use super::{DescriptorKind, DescriptorRole, SingleKeyDescriptor};

    #[test]
    fn parser_accepts_single_key_descriptors_with_optional_checksums() {
        let legacy = SingleKeyDescriptor::parse(
            "pkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ignored",
            AddressNetwork::Regtest,
        )
        .expect("pkh");
        let nested = SingleKeyDescriptor::parse(
            "sh(wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404))",
            AddressNetwork::Regtest,
        )
        .expect("sh(wpkh)");
        let bech32 = SingleKeyDescriptor::parse(
            "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)",
            AddressNetwork::Regtest,
        )
        .expect("wpkh");
        let taproot = SingleKeyDescriptor::parse(
            "tr(8d3a0c2f945bd9b7c1eec86a1c44d7cb61f4705ce2352d2d76f03af7b14747e3)",
            AddressNetwork::Regtest,
        )
        .expect("tr");

        assert_eq!(legacy.kind(), DescriptorKind::Pkh);
        assert_eq!(nested.kind(), DescriptorKind::ShWpkh);
        assert_eq!(bech32.kind(), DescriptorKind::Wpkh);
        assert_eq!(taproot.kind(), DescriptorKind::Tr);
    }

    #[test]
    fn parser_accepts_ranged_single_key_descriptors_and_rejects_multipath() {
        let ranged = SingleKeyDescriptor::parse(
            "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
            AddressNetwork::Regtest,
        )
        .expect("ranged descriptors are supported");
        let error = SingleKeyDescriptor::parse(
            "wpkh(tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/<0;1>/*)",
            AddressNetwork::Regtest,
        )
        .expect_err("multipath remains deferred");

        assert!(ranged.is_ranged());
        assert_eq!(ranged.range_start(), Some(0));
        assert_eq!(ranged.range_end(), Some(1000));
        assert_eq!(
            error.to_string(),
            "unsupported descriptor: multipath descriptors remain deferred",
        );
    }

    #[test]
    fn record_preserves_original_text_and_role() {
        let descriptor = SingleKeyDescriptor::parse(
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            AddressNetwork::Regtest,
        )
        .expect("descriptor");
        let record = super::DescriptorRecord {
            id: 7,
            label: "receive".to_string(),
            role: DescriptorRole::External,
            original_text: "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)".to_string(),
            descriptor,
        };

        assert_eq!(record.id, 7);
        assert_eq!(record.role, DescriptorRole::External);
        assert_eq!(
            record.descriptor.display_text(),
            "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
        );
    }

    #[test]
    fn descriptor_methods_cover_private_public_and_taproot_paths() {
        let legacy = SingleKeyDescriptor::parse(
            "pkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            AddressNetwork::Regtest,
        )
        .expect("legacy");
        let nested = SingleKeyDescriptor::parse(
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            AddressNetwork::Regtest,
        )
        .expect("nested");
        let watch_only = SingleKeyDescriptor::parse(
            "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("watch-only");
        let taproot_private = SingleKeyDescriptor::parse(
            "tr(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            AddressNetwork::Regtest,
        )
        .expect("taproot private");
        let taproot_public = SingleKeyDescriptor::parse(
            "tr(4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("taproot public");

        assert!(legacy.can_sign());
        assert_eq!(legacy.estimated_input_vbytes(), 154);
        assert!(legacy.display_text().starts_with("pkh("));
        assert!(legacy.signing_public_key_bytes().expect("pubkey").is_some());
        assert!(legacy.redeem_script().expect("redeem").is_none());
        assert!(legacy.address(AddressNetwork::Regtest).is_ok());
        assert!(legacy.script_pubkey().is_ok());

        assert!(nested.can_sign());
        assert_eq!(nested.estimated_input_vbytes(), 91);
        assert!(nested.display_text().starts_with("sh(wpkh("));
        assert!(nested.redeem_script().expect("redeem").is_some());

        assert!(!watch_only.can_sign());
        assert_eq!(watch_only.estimated_input_vbytes(), 68);
        assert!(watch_only.display_text().starts_with("wpkh("));
        assert!(
            watch_only
                .signing_public_key_bytes()
                .expect("pubkey")
                .is_some()
        );

        assert!(taproot_private.can_sign());
        assert_eq!(taproot_private.estimated_input_vbytes(), 58);
        assert!(taproot_private.display_text().starts_with("tr("));
        assert!(taproot_private.address(AddressNetwork::Regtest).is_ok());
        assert!(taproot_private.script_pubkey().is_ok());
        assert!(
            taproot_private
                .signing_public_key_bytes()
                .expect("taproot")
                .is_none()
        );

        assert!(!taproot_public.can_sign());
        assert!(taproot_public.display_text().starts_with("tr("));
        assert!(taproot_public.address(AddressNetwork::Regtest).is_ok());
        assert!(taproot_public.script_pubkey().is_ok());
    }

    #[test]
    fn parser_reports_network_and_key_errors() {
        let network_error = SingleKeyDescriptor::parse(
            "wpkh(KwFfNUhSDaASSAwtG7ssQM1uVX8RgX5GHWnnLfhfiQDigjioWXHH)",
            AddressNetwork::Regtest,
        )
        .expect_err("mainnet WIF should not load into regtest wallet");
        let invalid_taproot = SingleKeyDescriptor::parse(
            "tr(00112233445566778899aabbccddeeff001122)",
            AddressNetwork::Regtest,
        )
        .expect_err("bad xonly");
        let unsupported =
            SingleKeyDescriptor::parse("combo(02aa)", AddressNetwork::Regtest).expect_err("combo");
        let taproot_network_error = SingleKeyDescriptor::parse(
            "tr(KwFfNUhSDaASSAwtG7ssQM1uVX8RgX5GHWnnLfhfiQDigjioWXHH)",
            AddressNetwork::Regtest,
        )
        .expect_err("taproot mainnet WIF");
        let compressed_taproot = SingleKeyDescriptor::parse(
            "tr(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("taproot compressed public");
        let uncompressed_taproot = SingleKeyDescriptor::parse(
            "tr(044d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d07662a3eada2d0fe208b6d257ceb0f064284662e857f57b66b54c198bd310ded36d0)",
            AddressNetwork::Regtest,
        )
        .expect("taproot uncompressed public");

        assert!(network_error.to_string().contains("network mismatch"));
        assert!(invalid_taproot.to_string().contains("invalid x-only"));
        assert!(unsupported.to_string().contains("expected pkh"));
        assert!(
            taproot_network_error
                .to_string()
                .contains("network mismatch")
        );
        assert!(compressed_taproot.address(AddressNetwork::Regtest).is_ok());
        assert!(
            uncompressed_taproot
                .address(AddressNetwork::Regtest)
                .is_ok()
        );
    }
}
