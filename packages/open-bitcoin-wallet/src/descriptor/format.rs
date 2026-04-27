use crate::WalletError;
use crate::address::{
    Address, AddressNetwork, PrivateKey, hex_encode, p2pkh_address, p2pkh_script, public_key_bytes,
    sh_wpkh_address, sh_wpkh_redeem_script, sh_wpkh_script, taproot_script, tr_address,
    wpkh_address,
};

use super::{
    CHECKSUM_SIZE, DEFAULT_RANGE_END, DEFAULT_RANGE_START, DerivationStep, DescriptorKind,
    DescriptorRange, DescriptorRole, ExtendedPrivateKeySource, ExtendedPublicKeySource,
    HARDENED_INDEX, KeyOrigin, KeySource, RANGE_METADATA_PREFIX, SingleKeyDescriptor, Wildcard,
    bip32::parse_extended_key,
    key_sources::{parse_key_source, parse_taproot_key_source},
};

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
            return parse_wrapped_key_source(inner, network, maybe_range, Self::ShWpkh);
        }
        if let Some(inner) = body
            .strip_prefix("pkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return parse_wrapped_key_source(inner, network, maybe_range, Self::Pkh);
        }
        if let Some(inner) = body
            .strip_prefix("wpkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return parse_wrapped_key_source(inner, network, maybe_range, Self::Wpkh);
        }
        if let Some(inner) = body
            .strip_prefix("tr(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return parse_wrapped_taproot_key_source(inner, network, maybe_range);
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
            let derived_script = self.script_pubkey_at(index)?;
            if derived_script == *script_pubkey {
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

    pub(super) fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.range(),
            Self::Tr(key) => key.range(),
        }
    }

    pub(super) fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.range_mut(),
            Self::Tr(key) => key.range_mut(),
        }
    }

    fn current_index(&self) -> u32 {
        self.range().map_or(0, |range| range.next_index)
    }
}

pub(super) fn split_descriptor_body_and_range(
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

pub(super) fn parse_extended_key_source(
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
        super::bip32::ParsedExtendedKey::Public(extended_key) => {
            Ok(KeySource::ExtendedPublic(ExtendedPublicKeySource {
                origin: maybe_origin,
                encoded_key,
                extended_key,
                path,
                maybe_wildcard,
                maybe_range,
            }))
        }
        super::bip32::ParsedExtendedKey::Private(extended_key) => {
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

pub(super) fn parse_origin(text: &str) -> Result<(Option<KeyOrigin>, &str), WalletError> {
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
    let fingerprint_bytes = crate::address::decode_hex(fingerprint_text)?;
    let fingerprint = [
        fingerprint_bytes[0],
        fingerprint_bytes[1],
        fingerprint_bytes[2],
        fingerprint_bytes[3],
    ];
    let mut path = Vec::new();
    for part in parts {
        path.push(parse_derivation_step(part)?);
    }

    Ok((Some(KeyOrigin { fingerprint, path }), remainder))
}

pub(super) fn parse_derivation_segments(
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

pub(super) fn parse_wildcard(text: &str) -> Result<Wildcard, WalletError> {
    match text {
        "*" => Ok(Wildcard::Unhardened),
        "*'" | "*h" => Ok(Wildcard::Hardened),
        _ => Err(WalletError::DescriptorSyntax(format!(
            "unsupported wildcard derivation: {text}"
        ))),
    }
}

pub(super) fn parse_derivation_step(text: &str) -> Result<DerivationStep, WalletError> {
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

pub(super) fn role_name(role: DescriptorRole) -> &'static str {
    match role {
        DescriptorRole::External => "external",
        DescriptorRole::Internal => "internal",
    }
}

pub(super) fn format_origin(maybe_origin: Option<&KeyOrigin>) -> String {
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

pub(super) fn format_derivation_path(path: &[DerivationStep]) -> String {
    let mut text = String::new();
    for step in path {
        text.push('/');
        text.push_str(&step.display());
    }
    text
}

pub(super) fn format_wildcard(maybe_wildcard: Option<Wildcard>) -> String {
    match maybe_wildcard {
        None => String::new(),
        Some(Wildcard::Unhardened) => "/*".to_string(),
        Some(Wildcard::Hardened) => "/*h".to_string(),
    }
}

pub(super) fn encode_wif(private_key: &PrivateKey) -> String {
    encode_wif_from_parts(
        private_key.network(),
        &private_key.secret_key().secret_bytes(),
        private_key.compressed(),
    )
}

pub(super) fn encode_wif_from_parts(
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

pub(super) fn base58_encode(bytes: &[u8]) -> String {
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

    let mut out = "1".repeat(bytes.iter().take_while(|byte| **byte == 0).count());
    for digit in digits.iter().rev() {
        out.push(BASE58_ALPHABET[*digit as usize] as char);
    }
    out
}

fn parse_wrapped_key_source(
    inner: &str,
    network: AddressNetwork,
    maybe_range: Option<DescriptorRange>,
    wrap: fn(KeySource) -> SingleKeyDescriptor,
) -> Result<SingleKeyDescriptor, WalletError> {
    parse_key_source(inner, network, maybe_range, false).map(wrap)
}

fn parse_wrapped_taproot_key_source(
    inner: &str,
    network: AddressNetwork,
    maybe_range: Option<DescriptorRange>,
) -> Result<SingleKeyDescriptor, WalletError> {
    parse_taproot_key_source(inner, network, maybe_range).map(SingleKeyDescriptor::Tr)
}

pub(super) fn base58_decode(input: &str) -> Result<Vec<u8>, WalletError> {
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
