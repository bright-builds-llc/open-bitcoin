// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/consensus/consensus.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::ConsensusParams;

fn hash(hex: &str) -> BlockHash {
    let mut bytes = [0; 32];
    for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
        let text = core::str::from_utf8(pair).expect("source ASCII");
        bytes[31 - index] = u8::from_str_radix(text, 16).expect("source hex");
    }
    BlockHash::from_byte_array(bytes)
}

#[expect(
    clippy::too_many_arguments,
    reason = "independent source table columns"
)]
fn assert_rules(
    chain: CanonicalChain,
    halving: u32,
    timespan: i64,
    bits: u32,
    min_difficulty: bool,
    no_retarget: bool,
    heights: [u32; 5],
    maybe_hash: Option<&str>,
) {
    // Arrange
    let spec = chain.spec();
    // Act
    let rules = spec.rules();
    // Assert
    assert_eq!(rules.chain(), chain);
    assert_eq!(rules.coinbase_maturity(), 100);
    assert_eq!(rules.subsidy_halving_interval(), halving);
    assert_eq!(rules.pow_target_spacing_seconds(), 600);
    assert_eq!(rules.pow_target_timespan_seconds(), timespan);
    assert_eq!(
        rules.difficulty_adjustment_interval(),
        (timespan / 600) as u32
    );
    assert_eq!(rules.pow_limit_bits(), bits);
    assert_eq!(rules.allow_min_difficulty_blocks(), min_difficulty);
    assert_eq!(rules.no_pow_retargeting(), no_retarget);
    assert!(!rules.enforce_bip94());
    assert_eq!(
        [
            rules.bip34_height(),
            rules.bip66_height(),
            rules.bip65_height(),
            rules.csv_height(),
            rules.segwit_height()
        ],
        heights
    );
    assert_eq!(rules.maybe_bip34_hash(), maybe_hash.map(hash));
    assert_eq!(rules.bip30_recheck_height(), 1_983_702);
    assert_eq!(rules.locktime_threshold(), 500_000_000);
    assert_eq!(rules.sequence_locktime_granularity(), 9);
}

// Independent literals transcribed from pinned chainparams.cpp, not production getters.
#[test]
fn phase167_network_rules_mainnet_exact_source_table() {
    assert_rules(
        CanonicalChain::Mainnet,
        210_000,
        1_209_600,
        0x1d00ffff,
        false,
        false,
        [227931, 363725, 388381, 419328, 481824],
        Some("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8"),
    );
}
#[test]
fn phase167_network_rules_testnet3_exact_source_table() {
    assert_rules(
        CanonicalChain::Testnet3,
        210_000,
        1_209_600,
        0x1d00ffff,
        true,
        false,
        [21111, 330776, 581885, 770112, 834624],
        Some("0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8"),
    );
}
#[test]
fn phase167_network_rules_signet_exact_source_table() {
    assert_rules(
        CanonicalChain::Signet,
        210_000,
        1_209_600,
        0x1e0377ae,
        false,
        false,
        [1, 1, 1, 1, 1],
        None,
    );
}
#[test]
fn phase167_network_rules_regtest_exact_source_table() {
    assert_rules(
        CanonicalChain::Regtest,
        150,
        86400,
        0x207fffff,
        true,
        true,
        [1, 1, 1, 1, 0],
        None,
    );
}

#[test]
fn phase167_network_rules_exact_script_replacements_and_cross_network_refusals() {
    // Arrange
    let entries = [
        (
            CanonicalChain::Mainnet,
            "00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22",
            ScriptVerifyFlags::NONE,
        ),
        (
            CanonicalChain::Mainnet,
            "0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad",
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        ),
        (
            CanonicalChain::Testnet3,
            "00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105",
            ScriptVerifyFlags::NONE,
        ),
    ];
    let chains = [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ];
    // Act / Assert
    for chain in chains {
        let rules = chain.spec().rules();
        let expected_count = if chain == CanonicalChain::Mainnet {
            2
        } else if chain == CanonicalChain::Testnet3 {
            1
        } else {
            0
        };
        assert_eq!(rules.script_flag_exceptions().len(), expected_count);
        for (owner, display_hash, flags) in entries {
            let expected = if owner == chain { Some(flags) } else { None };
            assert_eq!(
                rules.maybe_script_flag_exception(hash(display_hash)),
                expected
            );
        }
        assert_eq!(
            rules.maybe_script_flag_exception(BlockHash::default()),
            None
        );
    }
}

#[test]
fn phase167_network_rules_exact_bip30_repeats_are_network_scoped() {
    // Arrange
    let expected = [
        (
            91842,
            hash("00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec"),
        ),
        (
            91880,
            hash("00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721"),
        ),
    ];
    // Act / Assert
    assert_eq!(
        CanonicalChain::Mainnet.spec().rules().bip30_repeat_pairs(),
        &expected
    );
    for chain in [
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        assert!(chain.spec().rules().bip30_repeat_pairs().is_empty());
    }
}

#[test]
fn phase167_network_rules_fixture_changes_cannot_mutate_selected_facts_or_support() {
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        // Arrange
        let spec = chain.spec();
        let before = spec.rules();
        let scope = spec.validation_scope();
        // Act
        let fixture = ConsensusParams {
            coinbase_maturity: 1,
            subsidy_halving_interval: 7,
            pow_limit_bits: 0,
            ..ConsensusParams::default()
        };
        // Assert
        assert_eq!(spec.rules(), before);
        assert_ne!(fixture.coinbase_maturity, before.coinbase_maturity());
        assert_ne!(
            fixture.subsidy_halving_interval,
            before.subsidy_halving_interval()
        );
        assert_ne!(fixture.pow_limit_bits, before.pow_limit_bits());
        assert_eq!(
            scope.evidence_class(),
            super::super::ValidationEvidenceClass::IdentityOnly
        );
        assert!(!scope.ordinary_history());
        assert!(spec.require_ordinary_history().is_err());
    }
    assert!(
        CanonicalChain::Signet
            .spec()
            .validation_scope()
            .missing_prerequisites()
            .contains(&super::super::MissingValidationPrerequisite::SignetChallengeValidation)
    );
}

#[test]
fn phase167_network_rules_all_selector_pairs_are_distinct() {
    // Arrange
    let chains = [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ];
    // Act / Assert
    for (index, chain) in chains.iter().enumerate() {
        for other in &chains[index + 1..] {
            assert_ne!(chain.spec().rules(), other.spec().rules());
        }
    }
}
