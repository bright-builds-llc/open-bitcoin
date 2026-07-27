// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use super::*;

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
