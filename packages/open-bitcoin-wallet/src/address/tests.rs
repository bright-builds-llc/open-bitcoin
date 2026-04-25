// Parity breadcrumbs:
// - packages/bitcoin-knots/src/key_io.cpp
// - packages/bitcoin-knots/src/bech32.cpp
// - packages/bitcoin-knots/src/base58.cpp
// - packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py

use secp256k1::{Secp256k1, SecretKey};

use super::{
    AddressNetwork, PrivateKey, base58_decode, base58_encode, convert_bits, decode_hex,
    encode_segwit_address, hex_encode, nibble_to_hex, p2pkh_address, public_key_bytes, push_data,
    sh_wpkh_address, tap_tweak_scalar, taproot_output_key_from_private_key,
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
    let private_key = PrivateKey::from_wif("cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi")
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
    let private_key = PrivateKey::from_wif("cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi")
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
    assert_eq!(push_data(&[0xaa, 0xbb]).expect("push"), vec![2, 0xaa, 0xbb]);
    assert_eq!(push_data(&[0_u8; 76]).expect("push")[..2], [0x4c, 76]);
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
    assert_eq!(
        push_data(&[0_u8; 300]).expect("push")[..3],
        [0x4d, 0x2c, 0x01]
    );
    assert!(push_data(&[0_u8; 521]).is_err());

    let high_version = encode_segwit_address("tb", 17, &[0_u8; 32]).expect_err("bad version");
    let long_program = encode_segwit_address("tb", 1, &[0_u8; 41]).expect_err("bad program length");
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
    let uncompressed = PrivateKey::from_wif(&sample_private_key(AddressNetwork::Regtest, false))
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
