// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use crate::WalletError;
use crate::address::AddressNetwork;

use super::*;

fn encode_base58(bytes: &[u8]) -> String {
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

fn encode_base58check_payload(payload: &[u8]) -> String {
    let checksum = open_bitcoin_consensus::crypto::double_sha256(payload);
    let mut bytes = payload.to_vec();
    bytes.extend_from_slice(&checksum[..super::CHECKSUM_SIZE]);
    encode_base58(&bytes)
}

mod key_source_and_storage_helpers_cover_extended_branches;
mod parser_accepts_single_key_descriptors_with_optional_checksums;
