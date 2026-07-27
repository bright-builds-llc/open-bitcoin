use super::*;

#[test]
fn pay_to_anchor_defaults_and_dust_relay_thresholds_are_pinned() {
    // Arrange / Act
    let config = PolicyConfig::default();
    let p2a_output = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: p2a_script(),
    };
    let witness_output = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&[
            0x00, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]),
    };
    let legacy_output = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: p2sh_script(),
    };

    // Assert
    assert_eq!(
        config.ephemeral_policy,
        EphemeralPolicy {
            anchor: true,
            send: false,
            dust: false,
        }
    );
    assert_eq!(
        config.dust_relay_fee_rate,
        DustRelayFeeRate::new(FeeRate::from_sats_per_kvb(3_000))
    );
    assert_eq!(
        dust_threshold_sats_at_rate(&witness_output, config.dust_relay_fee_rate),
        294
    );
    assert_eq!(
        dust_threshold_sats_at_rate(&legacy_output, config.dust_relay_fee_rate),
        540
    );
    assert_eq!(
        dust_threshold_sats_at_rate(&p2a_output, config.dust_relay_fee_rate),
        240
    );
    assert!(output_policy_result(p2a_output.script_pubkey, 0, config.ephemeral_policy).is_ok());
}

#[test]
fn dust_threshold_counts_compact_size_script_length_boundaries() {
    // Arrange
    let script_252 = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&vec![0x51; 252]),
    };
    let script_253 = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&vec![0x51; 253]),
    };

    // Act / Assert
    assert_eq!(
        dust_threshold_sats_at_rate(&script_252, DustRelayFeeRate::default()),
        1_227
    );
    assert_eq!(
        dust_threshold_sats_at_rate(&script_253, DustRelayFeeRate::default()),
        1_236
    );
}

#[test]
fn null_data_requires_push_only_suffix_and_may_carry_value() {
    // Arrange
    let permissions = EphemeralPolicy::default();

    // Act
    let empty = output_policy_result(script(&[]), 0, permissions);
    let non_push = output_policy_result(script(&[0x6a, 0xac]), 0, permissions);
    let truncated_push = output_policy_result(script(&[0x6a, 0x4c]), 0, permissions);
    let pushed_payload = output_policy_result(script(&[0x6a, 0x01, 0x01]), 0, permissions);
    let valued_payload = output_policy_result(script(&[0x6a, 0x01, 0x01]), 1, permissions);

    // Assert
    assert!(empty.is_err());
    assert!(non_push.is_err());
    assert!(truncated_push.is_err());
    assert!(pushed_payload.is_ok());
    assert!(valued_payload.is_ok());
}

#[test]
fn transaction_output_facts_enforce_knots_data_dust_and_bare_limits() {
    // Arrange
    let null_data = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&[0x6a, 0x01, 0x01]),
    };
    let monetary = open_bitcoin_primitives::TransactionOutput {
        value: Amount::from_sats(1_000).expect("valid monetary value"),
        script_pubkey: p2sh_script(),
    };
    let dust = open_bitcoin_primitives::TransactionOutput {
        value: Amount::from_sats(1).expect("valid dust value"),
        script_pubkey: p2sh_script(),
    };
    let permit_dust = PolicyConfig {
        ephemeral_policy: EphemeralPolicy {
            anchor: true,
            send: true,
            dust: true,
        },
        ..PolicyConfig::default()
    };

    // Act
    let two_data = transaction_output_policy_result(
        vec![null_data.clone(), null_data.clone(), monetary.clone()],
        PolicyConfig::default(),
    );
    let two_dust =
        transaction_output_policy_result(vec![dust.clone(), dust, monetary.clone()], permit_dust);
    let bare_data =
        transaction_output_policy_result(vec![null_data.clone()], PolicyConfig::default());
    let data_and_money = transaction_output_policy_result(
        vec![null_data.clone(), monetary],
        PolicyConfig::default(),
    );
    let permitted_bare_data = transaction_output_policy_result(
        vec![null_data],
        PolicyConfig {
            permit_bare_datacarrier: true,
            ..PolicyConfig::default()
        },
    );

    // Assert
    assert!(
        two_data
            .expect_err("multiple data outputs")
            .to_string()
            .contains("null-data")
    );
    assert!(
        two_dust
            .expect_err("multiple dust outputs")
            .to_string()
            .contains("dust")
    );
    assert!(
        bare_data
            .expect_err("bare data carrier")
            .to_string()
            .contains("bare data-carrier")
    );
    assert!(data_and_money.is_ok());
    assert!(permitted_bare_data.is_ok());
}

#[test]
fn transaction_output_facts_enforce_bare_anchor_toggle_and_companions() {
    // Arrange
    let anchor = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: p2a_script(),
    };
    let dust = open_bitcoin_primitives::TransactionOutput {
        value: Amount::from_sats(1).expect("valid dust value"),
        script_pubkey: p2sh_script(),
    };
    let null_data = open_bitcoin_primitives::TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(&[0x6a, 0x01, 0x01]),
    };
    let monetary = open_bitcoin_primitives::TransactionOutput {
        value: Amount::from_sats(1_000).expect("valid monetary value"),
        script_pubkey: p2sh_script(),
    };
    let permit_dust = PolicyConfig {
        ephemeral_policy: EphemeralPolicy {
            anchor: true,
            send: true,
            dust: true,
        },
        ..PolicyConfig::default()
    };
    let deny_bare_anchor = PolicyConfig {
        permit_bare_anchor: false,
        ..permit_dust.clone()
    };
    let permit_bare_anchor = PolicyConfig {
        permit_bare_anchor: true,
        ..permit_dust.clone()
    };

    // Act
    let default_anchor =
        transaction_output_policy_result(vec![anchor.clone()], PolicyConfig::default());
    let default_dust = transaction_output_policy_result(vec![dust.clone()], permit_dust.clone());
    let denied_anchor =
        transaction_output_policy_result(vec![anchor.clone()], deny_bare_anchor.clone());
    let denied_dust =
        transaction_output_policy_result(vec![dust.clone()], deny_bare_anchor.clone());
    let permitted_anchor =
        transaction_output_policy_result(vec![anchor.clone()], permit_bare_anchor.clone());
    let permitted_dust =
        transaction_output_policy_result(vec![dust.clone()], permit_bare_anchor.clone());
    let anchor_with_monetary = transaction_output_policy_result(
        vec![anchor.clone(), monetary.clone()],
        PolicyConfig {
            permit_bare_anchor: false,
            ..PolicyConfig::default()
        },
    );
    let dust_with_monetary =
        transaction_output_policy_result(vec![dust, monetary], deny_bare_anchor);
    let data_with_anchor = transaction_output_policy_result(
        vec![null_data, anchor],
        PolicyConfig {
            permit_bare_datacarrier: true,
            permit_bare_anchor: false,
            ..PolicyConfig::default()
        },
    );

    // Assert
    assert!(PolicyConfig::default().permit_bare_anchor);
    assert!(default_anchor.is_ok());
    assert!(default_dust.is_ok());
    assert!(
        denied_anchor
            .expect_err("disabled bare anchor")
            .to_string()
            .contains("bare-anchor")
    );
    assert!(
        denied_dust
            .expect_err("disabled bare dust")
            .to_string()
            .contains("bare-anchor")
    );
    assert!(permitted_anchor.is_ok());
    assert!(permitted_dust.is_ok());
    assert!(anchor_with_monetary.is_ok());
    assert!(dust_with_monetary.is_ok());
    assert!(data_with_anchor.is_ok());
}

#[test]
fn pay_to_anchor_send_and_nonzero_dust_permissions_are_independent() {
    // Arrange
    let allow_all = EphemeralPolicy {
        anchor: true,
        send: true,
        dust: true,
    };

    // Act / Assert
    assert!(output_policy_result(p2a_script(), 0, allow_all).is_ok());
    assert!(output_policy_result(p2sh_script(), 0, allow_all).is_ok());
    assert!(output_policy_result(p2sh_script(), 1, allow_all).is_ok());

    assert!(
        output_policy_result(
            p2a_script(),
            0,
            EphemeralPolicy {
                anchor: false,
                send: true,
                dust: true,
            },
        )
        .expect_err("anchor permission gates P2A")
        .to_string()
        .contains("anchor")
    );
    assert!(
        output_policy_result(
            p2sh_script(),
            0,
            EphemeralPolicy {
                anchor: true,
                send: false,
                dust: true,
            },
        )
        .expect_err("send permission gates dusty non-anchor output")
        .to_string()
        .contains("non-anchor")
    );
    assert!(
        output_policy_result(
            p2sh_script(),
            1,
            EphemeralPolicy {
                anchor: true,
                send: true,
                dust: false,
            },
        )
        .expect_err("dust permission gates nonzero dust")
        .to_string()
        .contains("nonzero")
    );
}

#[test]
fn anchor_send_dust_permission_matrix_covers_forms_values_and_all_boolean_combinations() {
    for anchor in [false, true] {
        for send in [false, true] {
            for dust in [false, true] {
                // Arrange
                let permissions = EphemeralPolicy { anchor, send, dust };

                // Act
                let anchor_zero = output_policy_result(p2a_script(), 0, permissions);
                let anchor_nonzero = output_policy_result(p2a_script(), 1, permissions);
                let ordinary_zero = output_policy_result(p2sh_script(), 0, permissions);
                let ordinary_nonzero = output_policy_result(p2sh_script(), 1, permissions);

                // Assert
                assert_eq!(anchor_zero.is_ok(), anchor);
                assert_eq!(anchor_nonzero.is_ok(), anchor && dust);
                assert_eq!(ordinary_zero.is_ok(), send);
                assert_eq!(ordinary_nonzero.is_ok(), send && dust);
            }
        }
    }
}

#[test]
fn pay_to_anchor_spend_rejects_nonempty_witness_stuffing() {
    // Arrange
    let transaction = open_bitcoin_primitives::Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([0x32; 32]),
                vout: 0,
            },
            script_sig: script(&[]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01]]),
        }],
        outputs: vec![open_bitcoin_primitives::TransactionOutput {
            value: Amount::from_sats(1_000).expect("valid output value"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    };
    let input_context = open_bitcoin_consensus::TransactionInputContext {
        spent_output: open_bitcoin_consensus::SpentOutput {
            value: Amount::from_sats(1_000).expect("valid spent value"),
            script_pubkey: p2a_script(),
            is_coinbase: false,
        },
        created_height: 1,
        created_median_time_past: 1,
    };

    // Act
    let error = validate_standard_transaction(
        &transaction,
        &[input_context],
        &PolicyConfig::default(),
        100,
        0,
    )
    .expect_err("witness stuffing is non-standard");

    // Assert
    assert!(
        error
            .to_string()
            .contains("pay-to-anchor witness must be empty")
    );
}

#[test]
fn ordinary_below_static_floor_cannot_be_sponsored_by_high_fee_sibling() {
    // Arrange
    let members = [member(1, 2, 99, 99, 100), member(3, 2, 2_000, 2_000, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    assert!(matches!(
        result,
        Err(PackageFeeError::StaticFloorNotMet { member, .. }) if member == identity(1)
    ));
}

#[test]
fn ordinary_members_and_aggregate_rolling_floor_form_one_ordered_group() {
    // Arrange
    let members = [member(5, 2, 100, 125, 100), member(7, 2, 100, 875, 100)];

    // Act
    let assessment = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    )
    .expect("fee group");
    let group = assessment
        .try_effective_fee_group(EffectiveFeeGroupId::from_u64(1))
        .expect("checked group");

    // Assert
    assert_eq!(
        group.ordered_wtxids(),
        &[identity(5).wtxid, identity(7).wtxid]
    );
    assert_eq!(group.base_fee_sats().to_sats(), 200);
    assert_eq!(group.modified_fee_sats().to_sats(), 1_000);
    assert_eq!(assessment.base_fee_sats().to_sats(), 200);
    assert_eq!(assessment.modified_fee_sats().to_sats(), 1_000);
    assert_eq!(assessment.virtual_size(), TransactionVirtualSize::new(200));
    assert_eq!(
        assessment.effective_fee_rate(),
        FeeRate::from_sats_per_kvb(5_000)
    );
}

#[test]
fn aggregate_exactly_at_rolling_floor_passes() {
    // Arrange
    let members = [member(9, 2, 500, 500, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    assert!(result.is_ok());
}

#[test]
fn aggregate_one_satoshi_below_rolling_floor_is_reconsiderable() {
    // Arrange
    let members = [member(11, 2, 499, 499, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    let error = result.expect_err("rolling floor should reject");
    assert!(matches!(
        error,
        PackageFeeError::RollingFloorNotMet {
            required_fee_sats: 500,
            ..
        }
    ));
    assert!(error.to_string().contains("499"));
}
