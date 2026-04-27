// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use crate::WalletError;
use crate::address::AddressNetwork;

use super::{DescriptorKind, DescriptorRole, SingleKeyDescriptor};

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
    if let SingleKeyDescriptor::Tr(key) = &taproot_private {
        assert!(key.private_key().is_some());
        assert!(key.private_key_at(0).expect("private key").is_some());
    }
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

#[test]
fn internal_parsers_cover_remaining_descriptor_error_branches() {
    let wsh_error = SingleKeyDescriptor::parse("wsh(multi(1,02aa))", AddressNetwork::Regtest)
        .expect_err("wsh miniscript remains deferred");
    let wildcard_error = super::format::parse_derivation_segments(&["1", "*", "2"])
        .expect_err("wildcard must be final");
    let empty_step_error = super::format::parse_derivation_segments(&[""]).expect_err("empty step");
    let unsupported_wildcard =
        super::format::parse_wildcard("*x").expect_err("unsupported wildcard");
    let invalid_step =
        super::format::parse_derivation_step("not-a-step").expect_err("invalid step");
    let out_of_range =
        super::format::parse_derivation_step("2147483648").expect_err("out of range");
    let missing_bracket = super::format::parse_origin("[deadbeef/0").expect_err("missing bracket");
    let invalid_fingerprint =
        super::format::parse_origin("[abcd/0]xpub").expect_err("short fingerprint");
    let bad_range_metadata = SingleKeyDescriptor::parse(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:1:2",
        AddressNetwork::Regtest,
    )
    .expect_err("bad range metadata");
    let inverted_range = SingleKeyDescriptor::parse(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:3:1:0",
        AddressNetwork::Regtest,
    )
    .expect_err("inverted range");

    assert!(wsh_error.to_string().contains("miniscript and multisig"));
    assert!(
        wildcard_error
            .to_string()
            .contains("wildcard must be the final")
    );
    assert!(
        empty_step_error
            .to_string()
            .contains("empty derivation step")
    );
    assert!(
        unsupported_wildcard
            .to_string()
            .contains("unsupported wildcard")
    );
    assert!(invalid_step.to_string().contains("invalid derivation step"));
    assert!(out_of_range.to_string().contains("out of range"));
    assert!(missing_bracket.to_string().contains("missing closing"));
    assert!(
        invalid_fingerprint
            .to_string()
            .contains("invalid key fingerprint")
    );
    assert!(
        bad_range_metadata
            .to_string()
            .contains("invalid descriptor range metadata")
    );
    assert!(
        inverted_range
            .to_string()
            .contains("invalid descriptor range")
    );

    let wsh_only = SingleKeyDescriptor::parse("wsh(pk(02aa))", AddressNetwork::Regtest)
        .expect_err("plain wsh descriptors remain deferred");
    assert!(wsh_only.to_string().contains("miniscript and multisig"));
}

#[test]
fn internal_extended_key_helpers_cover_error_and_branch_paths() {
    let short_payload =
        super::format::encode_wif_from_parts(AddressNetwork::Regtest, &[7_u8; 32], true);
    let short_error = super::bip32::parse_extended_key(&short_payload, AddressNetwork::Regtest)
        .expect_err("wif is not an extended key");

    let mismatched_tpub = super::bip32::parse_extended_key(
        "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B",
        AddressNetwork::Mainnet,
    )
    .expect_err("testnet tpub cannot load in mainnet wallet");
    let mismatched_tprv = super::bip32::parse_extended_key(
        "tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK",
        AddressNetwork::Mainnet,
    )
    .expect_err("testnet tprv cannot load in mainnet wallet");

    let mut invalid_private_payload = vec![0_u8; super::EXTENDED_KEY_PAYLOAD_LEN];
    invalid_private_payload[..4].copy_from_slice(&super::EXTENDED_PRIVATE_TESTNET.to_be_bytes());
    invalid_private_payload[45] = 1;
    let invalid_private_error = super::bip32::parse_extended_key(
        &encode_base58check_payload(&invalid_private_payload),
        AddressNetwork::Regtest,
    )
    .expect_err("extended private payload must start with zero marker");

    let mut unsupported_version_payload = vec![0_u8; super::EXTENDED_KEY_PAYLOAD_LEN];
    unsupported_version_payload[..4].copy_from_slice(&0x0102_0304_u32.to_be_bytes());
    let unsupported_version_error = super::bip32::parse_extended_key(
        &encode_base58check_payload(&unsupported_version_payload),
        AddressNetwork::Regtest,
    )
    .expect_err("unsupported version");

    let short_checksum_error = super::bip32::parse_extended_key("1", AddressNetwork::Regtest)
        .expect_err("short base58check payload");
    let mut bad_checksum_payload = vec![0_u8; super::EXTENDED_KEY_PAYLOAD_LEN];
    bad_checksum_payload[..4].copy_from_slice(&super::EXTENDED_PUBLIC_TESTNET.to_be_bytes());
    let mut bad_checksum = encode_base58check_payload(&bad_checksum_payload).into_bytes();
    let last = bad_checksum
        .last_mut()
        .expect("base58check string should have at least one byte");
    *last = if *last == b'1' { b'2' } else { b'1' };
    let invalid_checksum_error = super::bip32::parse_extended_key(
        std::str::from_utf8(&bad_checksum).expect("valid utf8"),
        AddressNetwork::Regtest,
    )
    .expect_err("checksum mismatch");

    let extended_public = match super::bip32::parse_extended_key(
        "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B",
        AddressNetwork::Regtest,
    )
    .expect("extended public")
    {
        super::bip32::ParsedExtendedKey::Public(key) => key,
        super::bip32::ParsedExtendedKey::Private(_) => panic!("expected public"),
    };
    let hardened_public_error = extended_public
        .derive_child(super::DerivationStep::Hardened(0))
        .expect_err("public extended keys cannot derive hardened children");

    let extended_private = match super::bip32::parse_extended_key(
        "tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK",
        AddressNetwork::Regtest,
    )
    .expect("extended private")
    {
        super::bip32::ParsedExtendedKey::Private(key) => key,
        super::bip32::ParsedExtendedKey::Public(_) => panic!("expected private"),
    };
    assert!(
        extended_private
            .derive_child(super::DerivationStep::Hardened(0))
            .is_ok()
    );
    assert_eq!(
        super::bip32::network_for_extended_version(0xffff_ffff),
        AddressNetwork::Mainnet,
    );
    assert_eq!(
        super::bip32::network_for_extended_version(super::EXTENDED_PUBLIC_MAINNET),
        AddressNetwork::Mainnet,
    );
    assert!(!super::bip32::accepts_extended_network(
        AddressNetwork::Mainnet,
        AddressNetwork::Testnet
    ));
    assert_eq!(
        super::DerivationStep::Hardened(5).index(),
        super::HARDENED_INDEX + 5,
    );
    assert_eq!(
        super::Wildcard::Hardened.step(7),
        super::DerivationStep::Hardened(7),
    );

    assert!(short_error.to_string().contains("extended key payload"));
    assert!(mismatched_tpub.to_string().contains("network mismatch"));
    assert!(mismatched_tprv.to_string().contains("network mismatch"));
    assert!(
        invalid_private_error
            .to_string()
            .contains("invalid private key")
    );
    assert!(
        unsupported_version_error
            .to_string()
            .contains("unsupported extended key version")
    );
    assert!(
        short_checksum_error
            .to_string()
            .contains("shorter than checksum")
    );
    assert!(invalid_checksum_error.to_string().contains("checksum"));
    assert!(hardened_public_error.to_string().contains("hardened child"));
    assert_eq!(
        super::bip32::network_for_extended_version(super::EXTENDED_PUBLIC_TESTNET),
        AddressNetwork::Testnet,
    );
}

#[test]
fn key_source_and_storage_helpers_cover_extended_branches() {
    let maybe_origin = super::format::parse_origin("[deadbeef/1h/2]xpub").expect("origin parsing");
    assert!(maybe_origin.0.is_some());
    assert_eq!(
        super::format::format_origin(maybe_origin.0.as_ref()),
        "[deadbeef/1h/2]"
    );
    assert_eq!(
        super::format::format_derivation_path(&[
            super::DerivationStep::Unhardened(3),
            super::DerivationStep::Hardened(4),
        ]),
        "/3/4h",
    );
    assert_eq!(super::format::format_wildcard(None), "");
    assert_eq!(
        super::format::format_wildcard(Some(super::Wildcard::Hardened)),
        "/*h",
    );
    assert_eq!(
        super::format::role_name(DescriptorRole::Internal),
        "internal",
    );
    assert_eq!(super::format::base58_encode(&[0_u8]), "11");
    assert!(super::format::base58_decode("").is_err());

    let ranged_public = SingleKeyDescriptor::parse(
        "wpkh([deadbeef/1]tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/0/*h)",
        AddressNetwork::Regtest,
    )
    .expect("ranged public");
    let ranged_private = SingleKeyDescriptor::parse(
        "tr([deadbeef/1]tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/0/*)",
        AddressNetwork::Regtest,
    )
    .expect("ranged private");

    match &ranged_public {
        SingleKeyDescriptor::Wpkh(super::KeySource::ExtendedPublic(source)) => {
            assert!(source.display_text().contains("tpub"));
            assert!(source.storage_text().contains("/*h"));
            let key_source = super::KeySource::ExtendedPublic(source.clone());
            assert!(key_source.display_key().contains("tpub"));
            assert!(key_source.storage_key_text().contains("tpub"));
        }
        _ => panic!("expected ranged extended public wpkh"),
    }
    match &ranged_private {
        SingleKeyDescriptor::Tr(super::TaprootKeySource::ExtendedPrivate(source)) => {
            assert!(source.display_text().contains("tprv"));
            assert!(source.storage_text().contains("/*"));
        }
        _ => panic!("expected ranged extended private tr"),
    }

    let mut direct_public = super::KeySource::Public(
        secp256k1::PublicKey::from_slice(
            &crate::address::decode_hex(
                "024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766",
            )
            .expect("pubkey hex"),
        )
        .expect("public key"),
    );
    let mut direct_private = super::KeySource::Private(
        crate::address::PrivateKey::from_wif(
            "cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi",
        )
        .expect("wif"),
    );
    assert!(direct_public.private_key().is_none());
    assert!(direct_private.private_key().is_some());
    assert!(!direct_public.can_sign());
    assert!(direct_private.can_sign());
    assert!(direct_private.storage_key_text().starts_with('c'));
    assert!(direct_public.range().is_none());
    assert!(direct_private.range().is_none());
    assert!(direct_public.range_mut().is_none());
    assert!(direct_private.range_mut().is_none());

    let taproot_watch = super::TaprootKeySource::Public(
        secp256k1::XOnlyPublicKey::from_byte_array(
            <[u8; 32]>::try_from(
                crate::address::decode_hex(
                    "4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766",
                )
                .expect("xonly hex")
                .as_slice(),
            )
            .expect("xonly len"),
        )
        .expect("xonly key"),
    );
    assert!(taproot_watch.private_key().is_none());
    assert!(!taproot_watch.can_sign());
    assert!(taproot_watch.storage_key_text().len() > 10);
    let mut taproot_watch_mut = taproot_watch.clone();
    assert!(taproot_watch_mut.range().is_none());
    assert!(taproot_watch_mut.range_mut().is_none());

    match SingleKeyDescriptor::parse(
        "wpkh(tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("extended public")
    {
        SingleKeyDescriptor::Wpkh(super::KeySource::ExtendedPublic(mut source)) => {
            assert!(source.display_text().contains("tpub"));
            assert!(source.storage_text().contains("tpub"));
            source
                .maybe_range
                .as_mut()
                .expect("range")
                .next_index = 4;
            assert_eq!(source.maybe_range.expect("range").next_index, 4);
            assert!(source.public_key_at(1).is_ok());
        }
        _ => panic!("expected extended public source"),
    }

    match SingleKeyDescriptor::parse(
        "tr(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("extended private")
    {
        SingleKeyDescriptor::Tr(super::TaprootKeySource::ExtendedPrivate(mut source)) => {
            assert!(source.display_text().contains("tprv"));
            assert!(source.storage_text().contains("tprv"));
            source
                .maybe_range
                .as_mut()
                .expect("range")
                .next_index = 5;
            assert_eq!(source.maybe_range.expect("range").next_index, 5);
            assert!(source.private_key_at(1).is_ok());
            let mut taproot_source = super::TaprootKeySource::ExtendedPrivate(source.clone());
            assert!(taproot_source.private_key().is_none());
            assert!(taproot_source.private_key_at(1).expect("private key").is_some());
            assert!(taproot_source.output_key_at(1).is_ok());
            assert!(taproot_source.display_key().contains("tprv"));
            assert!(taproot_source.storage_key_text().contains("tprv"));
            assert!(taproot_source.range().is_some());
            assert!(taproot_source.range_mut().is_some());
        }
        _ => panic!("expected extended private source"),
    }

    match SingleKeyDescriptor::parse(
        "tr(tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("extended public taproot")
    {
        SingleKeyDescriptor::Tr(super::TaprootKeySource::ExtendedPublic(mut source)) => {
            assert!(source.display_text().contains("tpub"));
            assert!(source.storage_text().contains("tpub"));
            assert!(source.public_key_at(1).is_ok());
            assert!(source.maybe_range.is_some());
            assert!(source.maybe_range.as_mut().is_some());
            let mut taproot_source = super::TaprootKeySource::ExtendedPublic(source.clone());
            assert!(taproot_source.display_key().contains("tpub"));
            assert!(taproot_source.storage_key_text().contains("tpub"));
            assert!(taproot_source.range().is_some());
            assert!(taproot_source.range_mut().is_some());
        }
        _ => panic!("expected extended public taproot"),
    }

    match SingleKeyDescriptor::parse(
        "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("extended private keysource")
    {
        SingleKeyDescriptor::Wpkh(super::KeySource::ExtendedPrivate(source)) => {
            assert!(source.display_text().contains("tprv"));
            assert!(source.storage_text().contains("tprv"));
            let mut key_source = super::KeySource::ExtendedPrivate(source.clone());
            assert!(key_source.display_key().contains("tprv"));
            assert!(key_source.storage_key_text().contains("tprv"));
            assert!(key_source.range().is_some());
            assert!(key_source.range_mut().is_none() || key_source.range().is_some());
        }
        _ => panic!("expected extended private key source"),
    }
}

#[test]
fn internal_format_and_range_helpers_cover_success_paths() {
    let (body, maybe_range) = super::format::split_descriptor_body_and_range(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:1:4:2",
    )
    .expect("valid descriptor metadata");
    let (maybe_origin, remainder) =
        super::format::parse_origin("[deadbeef/1/2]tpub").expect("origin");
    let (path, maybe_wildcard) =
        super::format::parse_derivation_segments(&["1", "2h", "*h"]).expect("segments");
    let wildcard = super::format::parse_wildcard("*").expect("wildcard");
    let step = super::format::parse_derivation_step("9h").expect("step");

    assert_eq!(
        body,
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)"
    );
    let range = maybe_range.expect("range metadata");
    assert_eq!(range.start, 1);
    assert_eq!(range.end, 4);
    assert_eq!(range.next_index, 2);
    assert_eq!(remainder, "tpub");
    assert!(maybe_origin.is_some());
    assert_eq!(path.len(), 2);
    assert_eq!(maybe_wildcard, Some(super::Wildcard::Hardened));
    assert_eq!(wildcard, super::Wildcard::Unhardened);
    assert_eq!(step.display(), "9h");
    assert_eq!(
        super::format::role_name(DescriptorRole::External),
        "external"
    );
    assert_eq!(
        super::format::format_wildcard(Some(super::Wildcard::Unhardened)),
        "/*"
    );
    assert_eq!(super::format::format_derivation_path(&path), "/1/2h");
    assert_eq!(super::format::base58_encode(&[]), "");
    assert!(super::format::base58_decode("0").is_err());
}

#[test]
fn internal_range_and_matching_helpers_cover_remaining_error_paths() {
    let script = crate::address::p2pkh_script(
        &secp256k1::PublicKey::from_slice(
            &crate::address::decode_hex(
                "024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766",
            )
            .expect("pubkey hex"),
        )
        .expect("pubkey"),
    )
    .expect("script");

    let plain = SingleKeyDescriptor::parse(
        "pkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
        AddressNetwork::Regtest,
    )
    .expect("plain descriptor");
    assert_eq!(plain.matching_index(&script).expect("matching"), Some(0));
    assert_eq!(
        plain
            .matching_index(
                &crate::address::p2wpkh_script(
                    &secp256k1::PublicKey::from_slice(
                        &crate::address::decode_hex(
                            "024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766",
                        )
                        .expect("pubkey hex"),
                    )
                    .expect("pubkey"),
                )
                .expect("wpkh script")
            )
            .expect("non-match"),
        None
    );

    let mut ranged = SingleKeyDescriptor::parse(
        "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)#ob:0:0:1",
        AddressNetwork::Regtest,
    )
    .expect("ranged descriptor");
    let exhausted = ranged
        .advance_next_index(DescriptorRole::External)
        .expect_err("cursor already exhausted");

    let mut non_ranged = SingleKeyDescriptor::parse(
        "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
        AddressNetwork::Regtest,
    )
    .expect("non-ranged");
    let unsupported = non_ranged
        .advance_next_index(DescriptorRole::Internal)
        .expect_err("non-ranged descriptors cannot advance");

    let ranged_no_match = SingleKeyDescriptor::parse(
        "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)#ob:0:1:0",
        AddressNetwork::Regtest,
    )
    .expect("ranged no match");
    let ranged_match_script = ranged_no_match
        .script_pubkey_at(1)
        .expect("matching script");
    let ranged_match = ranged_no_match
        .matching_index(&ranged_match_script)
        .expect("matching index");
    let no_match = ranged_no_match
        .matching_index(
            &crate::address::p2pkh_script(
                &secp256k1::PublicKey::from_slice(
                    &crate::address::decode_hex(
                        "024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766",
                    )
                    .expect("pubkey hex"),
                )
                .expect("pubkey"),
            )
            .expect("other script"),
        )
        .expect("missing match");

    let invalid_start = SingleKeyDescriptor::parse(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:a:2:0",
        AddressNetwork::Regtest,
    )
    .expect_err("invalid start");
    let invalid_end = SingleKeyDescriptor::parse(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:1:b:0",
        AddressNetwork::Regtest,
    )
    .expect_err("invalid end");
    let invalid_next = SingleKeyDescriptor::parse(
        "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)#ob:1:2:c",
        AddressNetwork::Regtest,
    )
    .expect_err("invalid next index");
    let mut invalid_range_descriptor = SingleKeyDescriptor::parse(
        "tr(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("taproot ranged");
    if let SingleKeyDescriptor::Tr(key) = &mut invalid_range_descriptor {
        assert!(key.range_mut().is_some());
        let range = key.range_mut().expect("range");
        range.start = 3;
        range.end = 1;
    }
    let invalid_range_error = invalid_range_descriptor
        .advance_next_index(DescriptorRole::Internal)
        .expect_err("mutated invalid range should be rejected");

    assert!(
        exhausted
            .to_string()
            .contains("descriptor cursor exhausted")
    );
    assert!(unsupported.to_string().contains("unsupported address role"));
    assert_eq!(ranged_match, Some(1));
    assert_eq!(no_match, None);
    assert!(invalid_start.to_string().contains("invalid range start"));
    assert!(invalid_end.to_string().contains("invalid range end"));
    assert!(invalid_next.to_string().contains("invalid next index"));
    assert!(
        invalid_range_error
            .to_string()
            .contains("invalid descriptor range")
    );
}

#[test]
fn key_source_wrappers_cover_public_private_and_extended_accessors() {
    let ranged = SingleKeyDescriptor::parse(
        "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("ranged private");
    let taproot_ranged = SingleKeyDescriptor::parse(
        "tr(tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/1/1/*)",
        AddressNetwork::Regtest,
    )
    .expect("taproot ranged public");

    match ranged {
        SingleKeyDescriptor::Wpkh(key) => {
            assert!(key.public_key().is_ok());
            assert!(key.private_key().is_none());
            assert!(key.private_key_at(0).expect("private key").is_some());
            assert!(key.is_compressed());
            assert!(key.display_key().contains("tprv"));
            assert!(key.storage_key_text().contains("tprv"));
            assert!(key.range().is_some());
        }
        _ => panic!("expected ranged wpkh"),
    }

    match taproot_ranged {
        SingleKeyDescriptor::Tr(mut key) => {
            assert!(key.output_key().is_ok());
            assert!(key.private_key().is_none());
            assert!(key.private_key_at(0).expect("private key").is_none());
            assert!(key.display_key().contains("tpub"));
            assert!(key.storage_key_text().contains("tpub"));
            assert!(key.range().is_some());
            assert!(key.range_mut().is_some());
        }
        _ => panic!("expected ranged tr"),
    }

    let allow_xonly_error = super::key_sources::parse_key_source(
        "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff",
        AddressNetwork::Regtest,
        None,
        true,
    )
    .expect_err("32-byte x-only input is invalid for legacy key source parsing");
    assert_eq!(allow_xonly_error, WalletError::InvalidPublicKey);
}
