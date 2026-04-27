use secp256k1::{PublicKey, XOnlyPublicKey};

use crate::WalletError;
use crate::address::{
    PrivateKey, decode_hex, hex_encode, taproot_output_key_from_private_key,
    taproot_output_key_from_xonly,
};

use super::{
    DescriptorRange, ExtendedPrivateKeySource, ExtendedPublicKeySource, KeySource,
    TaprootKeySource,
    format::{encode_wif, parse_extended_key_source},
};

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

    pub(super) fn display_key(&self) -> String {
        match self {
            Self::Public(public_key) => hex_encode(&public_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.public_key().serialize()),
            Self::ExtendedPublic(source) => source.display_text(),
            Self::ExtendedPrivate(source) => source.display_text(),
        }
    }

    pub(super) fn storage_key_text(&self) -> String {
        match self {
            Self::Public(public_key) => hex_encode(&public_key.serialize()),
            Self::Private(private_key) => encode_wif(private_key),
            Self::ExtendedPublic(source) => source.storage_text(),
            Self::ExtendedPrivate(source) => source.storage_text(),
        }
    }

    pub(super) fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range,
            Self::ExtendedPrivate(source) => source.maybe_range,
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    pub(super) fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        if let Self::ExtendedPublic(source) = self {
            return source.maybe_range.as_mut();
        }
        if let Self::ExtendedPrivate(source) = self {
            return source.maybe_range.as_mut();
        }
        None
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

    pub(super) fn display_key(&self) -> String {
        match self {
            Self::Public(internal_key) => hex_encode(&internal_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.xonly_public_key().serialize()),
            Self::ExtendedPublic(source) => source.display_text(),
            Self::ExtendedPrivate(source) => source.display_text(),
        }
    }

    pub(super) fn storage_key_text(&self) -> String {
        match self {
            Self::Public(internal_key) => hex_encode(&internal_key.serialize()),
            Self::Private(private_key) => encode_wif(private_key),
            Self::ExtendedPublic(source) => source.storage_text(),
            Self::ExtendedPrivate(source) => source.storage_text(),
        }
    }

    pub(super) fn range(&self) -> Option<DescriptorRange> {
        match self {
            Self::ExtendedPublic(source) => source.maybe_range,
            Self::ExtendedPrivate(source) => source.maybe_range,
            Self::Public(_) | Self::Private(_) => None,
        }
    }

    pub(super) fn range_mut(&mut self) -> Option<&mut DescriptorRange> {
        if let Self::ExtendedPublic(source) = self {
            return source.maybe_range.as_mut();
        }
        if let Self::ExtendedPrivate(source) = self {
            return source.maybe_range.as_mut();
        }
        None
    }

    fn current_index(&self) -> u32 {
        self.range().map_or(0, |range| range.next_index)
    }
}

pub(super) fn parse_key_source(
    text: &str,
    network: crate::address::AddressNetwork,
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

pub(super) fn parse_taproot_key_source(
    text: &str,
    network: crate::address::AddressNetwork,
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

    if let Ok(KeySource::ExtendedPublic(source)) =
        parse_extended_key_source(text, network, maybe_range)
    {
        return Ok(TaprootKeySource::ExtendedPublic(source));
    }
    if let Ok(KeySource::ExtendedPrivate(source)) =
        parse_extended_key_source(text, network, maybe_range)
    {
        return Ok(TaprootKeySource::ExtendedPrivate(source));
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

impl ExtendedPublicKeySource {
    pub(super) fn public_key_at(&self, index: u32) -> Result<PublicKey, WalletError> {
        let mut key = self.extended_key.clone();
        for step in self
            .path
            .iter()
            .copied()
            .chain(self.maybe_wildcard.map(|wildcard| wildcard.step(index)))
        {
            key = key.derive_child(step)?;
        }
        Ok(key.public_key)
    }

    pub(super) fn display_text(&self) -> String {
        self.storage_text()
    }

    pub(super) fn storage_text(&self) -> String {
        format!(
            "{}{}{}{}",
            super::format::format_origin(self.origin.as_ref()),
            self.encoded_key,
            super::format::format_derivation_path(&self.path),
            super::format::format_wildcard(self.maybe_wildcard),
        )
    }
}

impl ExtendedPrivateKeySource {
    pub(super) fn public_key_at(&self, index: u32) -> Result<PublicKey, WalletError> {
        Ok(self.private_key_at(index)?.public_key())
    }

    pub(super) fn private_key_at(&self, index: u32) -> Result<PrivateKey, WalletError> {
        let mut key = self.extended_key.clone();
        for step in self
            .path
            .iter()
            .copied()
            .chain(self.maybe_wildcard.map(|wildcard| wildcard.step(index)))
        {
            key = key.derive_child(step)?;
        }
        Ok(key.private_key)
    }

    pub(super) fn display_text(&self) -> String {
        self.storage_text()
    }

    pub(super) fn storage_text(&self) -> String {
        format!(
            "{}{}{}{}",
            super::format::format_origin(self.origin.as_ref()),
            self.encoded_key,
            super::format::format_derivation_path(&self.path),
            super::format::format_wildcard(self.maybe_wildcard),
        )
    }
}
