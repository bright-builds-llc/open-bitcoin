// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/deploymentstatus.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chain_spec::CanonicalChain;

fn hash(hex: &str) -> BlockHash {
    let mut bytes = [0; 32];
    for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
        bytes[31 - index] =
            u8::from_str_radix(core::str::from_utf8(pair).expect("ASCII"), 16).expect("source hex");
    }
    BlockHash::from_byte_array(bytes)
}

// Private formula probes are source-derived scalar evidence, never a public history proof.
fn facts(chain: CanonicalChain, height: u32) -> RuleFacts {
    RuleFacts {
        network: chain.spec().rules(),
        height,
        candidate_hash: BlockHash::default(),
        parent_hash: BlockHash::from_byte_array([1; 32]),
        parent_mtp: 1_500_000_000,
        candidate_time: 1_500_000_600,
        origin: HistoryOrigin::IndependentlySeeded,
        bip34: Bip34Context::BeforeActivation,
    }
}

// Independent pinned literals, never read production activation getters for expectations.
const HEIGHTS: [(CanonicalChain, [u32; 5]); 4] = [
    (
        CanonicalChain::Mainnet,
        [227931, 363725, 388381, 419328, 481824],
    ),
    (
        CanonicalChain::Testnet3,
        [21111, 330776, 581885, 770112, 834624],
    ),
    (CanonicalChain::Signet, [1, 1, 1, 1, 1]),
    (CanonicalChain::Regtest, [1, 1, 1, 1, 0]),
];

#[test]
fn phase167_candidate_rules_bip34_before_at_after() {
    for (chain, [height, ..]) in HEIGHTS {
        for at in [height - 1, height, height + 1] {
            // Arrange / Act
            let rules = CandidateRules::from_facts(facts(chain, at));
            // Assert
            assert_eq!(rules.enforce_bip34_height(), at >= height);
            assert_eq!(
                rules.consensus_params().enforce_bip34_height_in_coinbase,
                at >= height
            );
        }
    }
}

#[test]
fn phase167_candidate_rules_version_floors_before_at_after() {
    for (chain, [bip34, dersig, cltv, ..]) in HEIGHTS {
        for boundary in [bip34, dersig, cltv] {
            for at in [boundary - 1, boundary, boundary + 1] {
                // Arrange
                let maybe_expected = if at >= cltv {
                    Some(4)
                } else if at >= dersig {
                    Some(3)
                } else if at >= bip34 {
                    Some(2)
                } else {
                    None
                };
                // Act
                let rules = CandidateRules::from_facts(facts(chain, at));
                // Assert
                assert_eq!(rules.maybe_minimum_block_version(), maybe_expected);
                for version in [-1, 0, 1, 2, 3, 4, 5] {
                    assert_eq!(
                        rules.accepts_block_version(version),
                        maybe_expected.is_none_or(|floor| version >= floor)
                    );
                }
            }
        }
    }
}

#[test]
fn phase167_candidate_rules_csv_finality_and_sequence_before_at_after() {
    for (chain, [_, _, _, boundary, _]) in HEIGHTS {
        for height in [boundary - 1, boundary, boundary + 1] {
            // Arrange / Act
            let rules = CandidateRules::from_facts(facts(chain, height));
            // Assert
            assert_eq!(rules.enforce_sequence_locks(), height >= boundary);
            assert_eq!(
                rules.consensus_params().enforce_bip113_median_time_past,
                height >= boundary
            );
            assert_eq!(
                rules.finality_cutoff(),
                if height >= boundary {
                    1_500_000_000
                } else {
                    1_500_000_600
                }
            );
        }
    }
}

#[test]
fn phase167_candidate_rules_segwit_commitment_independent_of_witness_flag() {
    for (chain, [_, _, _, _, boundary]) in HEIGHTS {
        for height in [boundary.saturating_sub(1), boundary, boundary + 1] {
            // Arrange / Act
            let rules = CandidateRules::from_facts(facts(chain, height));
            // Assert
            assert_eq!(rules.enforce_segwit_commitment(), height >= boundary);
            assert_eq!(rules.consensus_params().enforce_segwit, height >= boundary);
            assert!(rules.script_flags().contains(ScriptVerifyFlags::WITNESS));
        }
    }
}

#[test]
fn phase167_candidate_rules_script_additions_before_at_after_no_policy() {
    for (chain, [_, dersig, cltv, csv, segwit]) in HEIGHTS {
        for boundary in [dersig, cltv, csv, segwit] {
            for height in [boundary.saturating_sub(1), boundary, boundary + 1] {
                // Arrange
                let mut expected = ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::WITNESS
                    | ScriptVerifyFlags::TAPROOT;
                for (activation, flag) in [
                    (dersig, ScriptVerifyFlags::DERSIG),
                    (cltv, ScriptVerifyFlags::CHECKLOCKTIMEVERIFY),
                    (csv, ScriptVerifyFlags::CHECKSEQUENCEVERIFY),
                    (segwit, ScriptVerifyFlags::NULLDUMMY),
                ] {
                    if height >= activation {
                        expected |= flag;
                    }
                }
                // Act
                let rules = CandidateRules::from_facts(facts(chain, height));
                // Assert
                assert_eq!(rules.script_flags(), expected);
                for policy in [
                    ScriptVerifyFlags::STRICTENC,
                    ScriptVerifyFlags::LOW_S,
                    ScriptVerifyFlags::MINIMALDATA,
                    ScriptVerifyFlags::CLEANSTACK,
                ] {
                    assert!(!rules.script_flags().contains(policy));
                }
            }
        }
    }
}

#[test]
fn phase167_candidate_rules_exception_replacement_precedes_additions() {
    for (chain, height, hex, expected) in [
        (
            CanonicalChain::Mainnet,
            170060,
            "00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22",
            ScriptVerifyFlags::NONE,
        ),
        (
            CanonicalChain::Mainnet,
            692261,
            "0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad",
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::DERSIG
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
                | ScriptVerifyFlags::NULLDUMMY,
        ),
        (
            CanonicalChain::Testnet3,
            834625, // Deliberate scalar probe after all additions, not an actual-header claim.
            "00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105",
            ScriptVerifyFlags::DERSIG
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
                | ScriptVerifyFlags::NULLDUMMY,
        ),
        (
            CanonicalChain::Testnet3,
            1, // Deliberate scalar probe before testnet3's buried additions.
            "00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105",
            ScriptVerifyFlags::NONE,
        ),
    ] {
        // Arrange
        let mut input = facts(chain, height);
        input.candidate_hash = hash(hex);
        // Act / Assert
        assert_eq!(CandidateRules::from_facts(input).script_flags(), expected);
        input.network = CanonicalChain::Regtest.spec().rules();
        assert!(
            CandidateRules::from_facts(input)
                .script_flags()
                .contains(ScriptVerifyFlags::TAPROOT)
        );
        input.network = chain.spec().rules();
        input.candidate_hash = BlockHash::default();
        assert!(
            CandidateRules::from_facts(input)
                .script_flags()
                .contains(ScriptVerifyFlags::TAPROOT)
        );
    }
}

#[test]
fn phase167_candidate_rules_bip30_repeat_requires_exact_network_height_hash() {
    for (height, hex) in [
        (
            91842,
            "00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec",
        ),
        (
            91880,
            "00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721",
        ),
    ] {
        // Arrange
        let mut input = facts(CanonicalChain::Mainnet, height);
        input.candidate_hash = hash(hex);
        // Act / Assert
        assert_eq!(
            CandidateRules::from_facts(input).bip30_disposition(),
            Bip30Disposition::HistoricalRepeat
        );
        input.height += 1;
        assert!(CandidateRules::from_facts(input).requires_bip30_scan());
        input.height = height;
        input.network = CanonicalChain::Testnet3.spec().rules();
        assert!(CandidateRules::from_facts(input).requires_bip30_scan());
        input.network = CanonicalChain::Mainnet.spec().rules();
        input.candidate_hash = BlockHash::default();
        assert!(CandidateRules::from_facts(input).requires_bip30_scan());
    }
}

#[test]
fn phase167_candidate_rules_bip30_bip34_commitment_and_recheck_equality() {
    for (chain, hex) in [
        (
            CanonicalChain::Mainnet,
            "000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8",
        ),
        (
            CanonicalChain::Testnet3,
            "0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8",
        ),
    ] {
        for height in [1_983_701, 1_983_702, 1_983_703] {
            for context in [
                Bip34Context::Linked(hash(hex)),
                Bip34Context::IndependentlySeeded(hash(hex)),
            ] {
                // Arrange
                let mut input = facts(chain, height);
                input.bip34 = context;
                // Act / Assert
                assert_eq!(
                    CandidateRules::from_facts(input).requires_bip30_scan(),
                    height >= 1_983_702
                );
                input.bip34 = Bip34Context::Linked(BlockHash::default());
                assert!(CandidateRules::from_facts(input).requires_bip30_scan());
                input.bip34 = Bip34Context::BeforeActivation;
                assert!(CandidateRules::from_facts(input).requires_bip30_scan());
            }
        }
    }
}

#[test]
fn phase167_candidate_rules_regtest_null_hash_never_skips_scan() {
    // Arrange
    let mut input = facts(CanonicalChain::Regtest, 2);
    input.bip34 = Bip34Context::Linked(BlockHash::default());
    // Act / Assert
    assert!(CandidateRules::from_facts(input).requires_bip30_scan());
}

#[test]
fn phase167_candidate_rules_public_path_binds_header_and_selected_params() {
    // Arrange
    let history = super::super::tests::check_rows(
        CanonicalChain::Regtest,
        super::super::tests::branch(CanonicalChain::Regtest, 0, 10),
    )
    .expect("linked structural context");
    // Act
    let rules = history.rules();
    let context = history.block_validation_context(1_800_000_000);
    // Assert
    assert_eq!(rules.height(), 11);
    assert_eq!(
        rules.candidate_hash(),
        crate::block_hash(history.candidate())
    );
    assert_eq!(rules.parent_hash(), history.parent().block_hash);
    assert_eq!(rules.origin(), history.origin());
    assert_eq!(context.consensus_params, rules.consensus_params());
    assert_eq!(
        context.previous_median_time_past,
        history.parent_median_time_past()
    );
    assert_eq!(context.consensus_params.subsidy_halving_interval, 150);
    assert_eq!(context.consensus_params.coinbase_maturity, 100);
    assert!(rules.enforce_bip34_height());
    assert!(rules.enforce_sequence_locks());
    assert!(rules.enforce_segwit_commitment());
    assert_eq!(rules.maybe_minimum_block_version(), Some(4));
}

#[test]
fn bip34_rule_context_distinguishes_linked_and_independently_seeded_history() {
    // Arrange
    let linked = super::super::tests::check_rows(
        CanonicalChain::Mainnet,
        super::super::tests::branch(CanonicalChain::Mainnet, 227_787, 227_941),
    )
    .expect("linked structural BIP34 ancestry");
    let linked_seed = linked
        .branch()
        .iter()
        .find(|row| row.height == 227_931)
        .expect("linked BIP34 ancestor")
        .clone();
    let mut suffix = super::super::tests::branch(CanonicalChain::Mainnet, 227_940, 228_094);
    let parent = suffix.remove(0);
    let candidate = open_bitcoin_primitives::BlockHeader {
        version: 4,
        previous_block_hash: parent.block_hash,
        time: parent.header.time + 1,
        bits: parent.header.bits,
        ..Default::default()
    };
    let independent_seed =
        super::super::tests::branch(CanonicalChain::Mainnet, 227_931, 227_931).remove(0);
    let seeded = CandidateHistory::check_seeded(
        linked.network_rules(),
        candidate,
        parent,
        suffix,
        Some(independent_seed.clone()),
    )
    .expect("checked seed bytes without lineage authority");

    // Act
    let linked_context = linked.rules().bip34_context();
    let seeded_context = seeded.rules().bip34_context();

    // Assert
    assert_eq!(linked_context, Bip34Context::Linked(linked_seed.block_hash));
    assert_eq!(
        seeded_context,
        Bip34Context::IndependentlySeeded(independent_seed.block_hash)
    );
    assert_eq!(seeded.origin(), HistoryOrigin::IndependentlySeeded);
}
