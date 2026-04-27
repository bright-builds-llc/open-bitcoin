// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use secp256k1::{PublicKey, XOnlyPublicKey};

use crate::WalletError;
use crate::address::{
    Address, AddressNetwork, PrivateKey, decode_hex, hex_encode, p2pkh_address, p2pkh_script,
    public_key_bytes, sh_wpkh_address, sh_wpkh_redeem_script, sh_wpkh_script,
    taproot_output_key_from_private_key, taproot_output_key_from_xonly, taproot_script, tr_address,
    wpkh_address,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    Public(PublicKey),
    Private(PrivateKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaprootKeySource {
    Public(XOnlyPublicKey),
    Private(PrivateKey),
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
        let body = text
            .trim()
            .split_once('#')
            .map_or(text.trim(), |(descriptor, _)| descriptor.trim());
        if body.contains('*') || body.contains('<') {
            return Err(WalletError::UnsupportedDescriptor(
                "ranged descriptors are deferred to later wallet phases".to_string(),
            ));
        }

        if let Some(inner) = body
            .strip_prefix("sh(wpkh(")
            .and_then(|value| value.strip_suffix("))"))
        {
            return Ok(Self::ShWpkh(parse_key_source(inner, network)?));
        }
        if let Some(inner) = body
            .strip_prefix("pkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Pkh(parse_key_source(inner, network)?));
        }
        if let Some(inner) = body
            .strip_prefix("wpkh(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Wpkh(parse_key_source(inner, network)?));
        }
        if let Some(inner) = body
            .strip_prefix("tr(")
            .and_then(|value| value.strip_suffix(')'))
        {
            return Ok(Self::Tr(parse_taproot_key_source(inner, network)?));
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
        match self {
            Self::Pkh(key) => p2pkh_address(network, &key.public_key()),
            Self::ShWpkh(key) => sh_wpkh_address(network, &key.public_key()),
            Self::Wpkh(key) => wpkh_address(network, &key.public_key()),
            Self::Tr(key) => tr_address(network, &key.output_key()?),
        }
    }

    pub fn script_pubkey(&self) -> Result<open_bitcoin_primitives::ScriptBuf, WalletError> {
        match self {
            Self::Pkh(key) => p2pkh_script(&key.public_key()),
            Self::ShWpkh(key) => sh_wpkh_script(&key.public_key()),
            Self::Wpkh(key) => crate::address::p2wpkh_script(&key.public_key()),
            Self::Tr(key) => taproot_script(&key.output_key()?),
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
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => key.private_key().is_some(),
            Self::Tr(key) => key.private_key().is_some(),
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
        false
    }

    pub fn range_start(&self) -> Option<u32> {
        None
    }

    pub fn range_end(&self) -> Option<u32> {
        None
    }

    pub fn next_index(&self) -> Option<u32> {
        None
    }

    pub fn storage_text(&self) -> String {
        self.display_text()
    }

    pub fn signing_public_key_bytes(&self) -> Result<Option<Vec<u8>>, WalletError> {
        match self {
            Self::Pkh(key) | Self::ShWpkh(key) | Self::Wpkh(key) => Ok(Some(public_key_bytes(
                &key.public_key(),
                key.is_compressed(),
            ))),
            Self::Tr(_) => Ok(None),
        }
    }

    pub fn redeem_script(&self) -> Result<Option<open_bitcoin_primitives::ScriptBuf>, WalletError> {
        match self {
            Self::ShWpkh(key) => Ok(Some(sh_wpkh_redeem_script(&key.public_key())?)),
            _ => Ok(None),
        }
    }
}

impl KeySource {
    pub fn public_key(&self) -> PublicKey {
        match self {
            Self::Public(public_key) => *public_key,
            Self::Private(private_key) => private_key.public_key(),
        }
    }

    pub fn private_key(&self) -> Option<&PrivateKey> {
        match self {
            Self::Public(_) => None,
            Self::Private(private_key) => Some(private_key),
        }
    }

    pub fn is_compressed(&self) -> bool {
        match self {
            Self::Public(public_key) => public_key.serialize().len() == 33,
            Self::Private(private_key) => private_key.compressed(),
        }
    }

    fn display_key(&self) -> String {
        match self {
            Self::Public(public_key) => hex_encode(&public_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.public_key().serialize()),
        }
    }
}

impl TaprootKeySource {
    pub fn output_key(&self) -> Result<XOnlyPublicKey, WalletError> {
        match self {
            Self::Public(internal_key) => taproot_output_key_from_xonly(internal_key),
            Self::Private(private_key) => taproot_output_key_from_private_key(private_key)
                .map(|(_keypair, output_key)| output_key),
        }
    }

    pub fn private_key(&self) -> Option<&PrivateKey> {
        match self {
            Self::Public(_) => None,
            Self::Private(private_key) => Some(private_key),
        }
    }

    fn display_key(&self) -> String {
        match self {
            Self::Public(internal_key) => hex_encode(&internal_key.serialize()),
            Self::Private(private_key) => hex_encode(&private_key.xonly_public_key().serialize()),
        }
    }
}

fn parse_key_source(text: &str, network: AddressNetwork) -> Result<KeySource, WalletError> {
    if let Ok(private_key) = PrivateKey::from_wif(text) {
        if !network.accepts_wif_network(private_key.network()) {
            return Err(WalletError::NetworkMismatch {
                expected: network.to_string(),
                actual: private_key.network().to_string(),
            });
        }
        return Ok(KeySource::Private(private_key));
    }

    let bytes = decode_hex(text)?;
    let public_key = PublicKey::from_slice(&bytes).map_err(|_| WalletError::InvalidPublicKey)?;
    Ok(KeySource::Public(public_key))
}

fn parse_taproot_key_source(
    text: &str,
    network: AddressNetwork,
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
    fn parser_rejects_ranged_descriptors() {
        let error = SingleKeyDescriptor::parse(
            "wpkh(cTe1f5rdT8A8DFgVWTjyPwACsDPJM9ff4QngFxUixCSvvbg1x6sh/*)",
            AddressNetwork::Regtest,
        )
        .expect_err("ranged descriptors are deferred");

        assert_eq!(
            error.to_string(),
            "unsupported descriptor: ranged descriptors are deferred to later wallet phases",
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
