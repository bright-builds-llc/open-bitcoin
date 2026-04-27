// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use secp256k1::{PublicKey, XOnlyPublicKey};

use crate::address::{AddressNetwork, PrivateKey};

mod bip32;
mod format;
mod key_sources;
#[cfg(test)]
mod tests;

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
