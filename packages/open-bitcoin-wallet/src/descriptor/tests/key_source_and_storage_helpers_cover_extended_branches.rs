// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/descriptors.md
// - packages/bitcoin-knots/src/script/descriptor.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py

use super::*;

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
