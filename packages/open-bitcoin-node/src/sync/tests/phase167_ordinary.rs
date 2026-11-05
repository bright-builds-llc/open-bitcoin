// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/pow.cpp

use open_bitcoin_core::consensus::{
    CandidateHistory, CanonicalChain, HistoryHeader, ScriptVerifyFlags, block_hash,
};
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};
use serde_json::Value;

use super::phase167_support as ordinary;
use crate::network::{ManagedNetworkAuthorityError, ManagedNetworkError};
use crate::storage::validation_history::ValidationProvenance;

fn expected_row(expected: &Value, height: usize) -> &Value {
    let row = &expected["rows"][height];
    assert_eq!(row["height"].as_u64(), Some(height as u64));
    row
}

fn source_bound_selected_flags(corpus: &Value, height: u32) -> ScriptVerifyFlags {
    let selected = CanonicalChain::Regtest.spec();
    let candidate = ordinary::block(ordinary::row(corpus, height as usize)).header;
    let parent = HistoryHeader::new(
        CanonicalChain::Regtest,
        height - 1,
        ordinary::block(ordinary::row(corpus, (height - 1) as usize)).header,
    );
    let ancestors: Vec<_> = (0..height - 1)
        .rev()
        .map(|ancestor| {
            HistoryHeader::new(
                CanonicalChain::Regtest,
                ancestor,
                ordinary::block(ordinary::row(corpus, ancestor as usize)).header,
            )
        })
        .collect();
    CandidateHistory::check(selected.rules(), candidate, parent, ancestors)
        .expect("complete selected default-Regtest context")
        .rules()
        .script_flags()
}

#[test]
fn phase167_ordinary_serialized_301_reopen_150_301_and_next_302() {
    // Arrange: frozen bytes and independent facts are data, not admission authority.
    let source = ordinary::Source::new();
    let corpus = ordinary::corpus();
    let expected = ordinary::expectations();
    assert_eq!(
        corpus["rows"].as_array().expect("serialized rows").len(),
        302
    );
    assert_eq!(
        expected["rows"].as_array().expect("expected rows").len(),
        302
    );
    assert_eq!(expected["genesis_utxo_count"].as_u64(), Some(0));
    assert_eq!(
        ordinary::block(ordinary::row(&corpus, 0)),
        CanonicalChain::Regtest.spec().genesis_block(),
    );
    let mut runtime = ordinary::open_genesis(&source);
    assert_eq!(ordinary::accepted_snapshot(&runtime).utxos.len(), 0);

    // Act: admit every header and connect every serialized body through real owners.
    for height in 1..=301_usize {
        let block = ordinary::block(ordinary::row(&corpus, height));
        if matches!(height, 1 | 100 | 101 | 149 | 150 | 201 | 299 | 300 | 301) {
            ordinary::assert_block_boundary(&block, expected_row(&expected, height));
        }
        if height == 201 {
            ordinary::assert_creating_ancestor_201(&block, &expected["creating_live_201"]);
            let flags = source_bound_selected_flags(&corpus, 201);
            assert!(
                flags.contains(ScriptVerifyFlags::CHECKSEQUENCEVERIFY | ScriptVerifyFlags::WITNESS)
            );
        }
        ordinary::admit_and_connect(&runtime, &block);
        if matches!(height, 101 | 149 | 150 | 201 | 299 | 300 | 301) {
            ordinary::assert_tip(&runtime, expected_row(&expected, height));
            ordinary::assert_scripts_valid(&runtime, &block);
        }
        if height == 150 || height == 301 {
            ordinary::checkpoint(&runtime, height as u32, expected_row(&expected, height));
            ordinary::assert_frozen_utxos(&runtime, expected_row(&expected, height));
            let before = ordinary::accepted_snapshot(&runtime);
            let basic_before = ordinary::basic_state(&runtime);
            drop(runtime);
            runtime = ordinary::reopen(&source);
            // Assert: all handles were released before copied/original durable reproof.
            assert_eq!(ordinary::accepted_snapshot(&runtime), before);
            assert_eq!(ordinary::basic_state(&runtime), basic_before);
            ordinary::assert_tip(&runtime, expected_row(&expected, height));
            ordinary::assert_scripts_valid(&runtime, &block);
        }
    }
    let final_state = ordinary::accepted_snapshot(&runtime);
    assert_eq!(
        final_state.utxos.len() as u64,
        expected_row(&expected, 301)["utxo_count"]
            .as_u64()
            .expect("independent UTXO count")
    );
    assert_eq!(
        runtime
            .network
            .header_entries()
            .expect("earned headers")
            .len(),
        302
    );
    let next = ordinary::block(&corpus["next_302"]);
    ordinary::admit_and_connect(&runtime, &next);
    let next_state = ordinary::accepted_snapshot(&runtime);
    let next_tip = next_state.tip().expect("next accepted tip");
    assert_eq!(next_tip.height, 302);
    assert_eq!(
        ordinary::hex(next_tip.block_hash.as_bytes()),
        expected["next_302"]["block_hash_raw"]
            .as_str()
            .expect("independent next hash")
    );
    assert_eq!(
        next_tip.chain_work.to_hex(),
        expected["next_302"]["chain_work_hex"]
            .as_str()
            .expect("independent next work")
    );
    for (outpoint, coin) in &final_state.utxos {
        assert_eq!(
            next_state.utxos.get(outpoint),
            Some(coin),
            "prior frozen outpoint persists"
        );
    }
    assert_eq!(next.transactions.len(), 1);
    assert_eq!(next.transactions[0].outputs.len(), 1);
    let next_outpoint = open_bitcoin_core::primitives::OutPoint {
        txid: open_bitcoin_core::consensus::transaction_txid(&next.transactions[0])
            .expect("serialized next coinbase identity"),
        vout: 0,
    };
    let new_coin = next_state
        .utxos
        .get(&next_outpoint)
        .expect("new next coinbase");
    assert_eq!(new_coin.output, next.transactions[0].outputs[0]);
    assert_eq!(new_coin.created_height, 302);
    assert_eq!(
        new_coin.created_median_time_past,
        expected_row(&expected, 301)["median_time_past"]
            .as_i64()
            .expect("creating parent MTP")
    );
    assert_eq!(
        new_coin.output.value.to_sats().to_string(),
        expected["next_302"]["coinbase_sats"]
            .as_str()
            .expect("independent next subsidy")
    );
    assert!(new_coin.is_coinbase);
    assert_eq!(next_state.utxos.len(), final_state.utxos.len() + 1);
    ordinary::checkpoint(&runtime, 302, &expected["next_302"]);
    ordinary::assert_frozen_utxos(&runtime, &expected["next_302"]);
    ordinary::assert_tip(&runtime, &expected["next_302"]);
    ordinary::assert_scripts_valid(&runtime, &next);
    drop(runtime);
}

fn assert_header_refusal(runtime: &super::DurableSyncRuntime, negative: &Value) {
    let bad = ordinary::block(negative);
    let before = ordinary::accepted_snapshot(runtime);
    let basic_before = ordinary::basic_state(runtime);
    let headers_before = runtime.network.header_entries().expect("known headers");
    let provenance_before = runtime
        .store
        .validation_provenance(block_hash(&bad.header))
        .expect("pre-header provenance");
    let refused = runtime.network.receive_message(
        1,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![bad.header.clone()],
        }),
        i64::MIN,
        runtime.verify_flags,
        runtime.consensus_params,
    );
    let refusal = refused.expect_err("header must refuse before earning work");
    assert!(
        refusal.to_string().contains(
            negative["expected_reason"]
                .as_str()
                .expect("independent header reason")
        ),
        "observed header precedence: {refusal:?}"
    );
    assert_eq!(
        runtime.network.header_entries().expect("unchanged headers"),
        headers_before
    );
    assert_eq!(ordinary::accepted_snapshot(runtime), before);
    assert_eq!(ordinary::basic_state(runtime), basic_before);
    assert_eq!(
        runtime
            .store
            .validation_provenance(block_hash(&bad.header))
            .expect("post-header provenance"),
        provenance_before
    );
}

fn assert_body_refusal(runtime: &super::DurableSyncRuntime, negative: &Value) {
    let bad = ordinary::block(negative);
    let before = ordinary::accepted_snapshot(runtime);
    let basic_before = ordinary::basic_state(runtime);
    let provenance_before = runtime
        .store
        .validation_provenance(block_hash(&bad.header))
        .expect("pre-body provenance");
    runtime
        .network
        .receive_message(
            1,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![bad.header.clone()],
            }),
            i64::MIN,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("valid header may become known");
    runtime
        .store
        .save_block(&bad, crate::PersistMode::Sync)
        .expect("retained bad body");
    let refused =
        runtime
            .network
            .connect_local_block(&bad, runtime.verify_flags, runtime.consensus_params);
    let refusal = refused.expect_err("body must refuse before accepted state");
    if negative["name"] == "missing-input" {
        // Direct managed admission exposes a typed missing coin; public submission
        // maps it to the frozen Knots block reason at a separate boundary.
        assert_eq!(
            negative["expected_reason"].as_str(),
            Some("bad-txns-inputs-missingorspent")
        );
        assert!(matches!(
            &refusal,
            ManagedNetworkAuthorityError::Operation(ManagedNetworkError::Chainstate(
                open_bitcoin_core::chainstate::ChainstateError::MissingCoin { outpoint }
            )) if outpoint == &bad.transactions[1].inputs[0].previous_output
        ));
    } else {
        assert!(
            refusal.to_string().contains(
                negative["expected_reason"]
                    .as_str()
                    .expect("independent body reason")
            ),
            "observed body precedence for {}: {refusal:?}",
            negative["name"].as_str().expect("named negative")
        );
    }
    assert_eq!(ordinary::accepted_snapshot(runtime), before);
    assert_eq!(ordinary::basic_state(runtime), basic_before);
    assert_eq!(
        runtime
            .store
            .validation_provenance(block_hash(&bad.header))
            .expect("durable refusal provenance"),
        provenance_before
    );
    assert_ne!(provenance_before, ValidationProvenance::ScriptsValid);
}

#[test]
fn phase167_ordinary_serialized_refusals_preserve_accepted_state() {
    // Arrange: an actual default-Regtest canonical genesis and real prefix.
    let source = ordinary::Source::new();
    let corpus = ordinary::corpus();
    let runtime = ordinary::open_genesis(&source);
    for height in 1..=150_usize {
        if height == 100 {
            assert_body_refusal(&runtime, ordinary::named_negative(&corpus, "depth99-at100"));
        }
        if height == 101 {
            let flags = source_bound_selected_flags(&corpus, 101);
            assert!(flags.contains(
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::DERSIG
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
                    | ScriptVerifyFlags::NULLDUMMY
                    | ScriptVerifyFlags::WITNESS
            ));
            assert!(runtime.verify_flags.contains(ScriptVerifyFlags::P2SH));
            for name in [
                "absolute-nonfinal",
                "bip68-height-nonfinal",
                "witness-without-commitment",
                "missing-input",
                "aggregate-sigops-80004",
            ] {
                assert_body_refusal(&runtime, ordinary::named_negative(&corpus, name));
            }
        }
        if height == 149 {
            for name in [
                "wrong-version",
                "wrong-difficulty",
                "time-equals-parent-mtp",
                "invalid-pow",
            ] {
                assert_header_refusal(&runtime, ordinary::named_negative(&corpus, name));
            }
            assert_body_refusal(
                &runtime,
                ordinary::named_negative(&corpus, "overpay-one-satoshi-149"),
            );
        }
        ordinary::admit_and_connect(&runtime, &ordinary::block(ordinary::row(&corpus, height)));
    }
    // Assert: all invalid candidates are known only or refused before header work.
    assert_eq!(
        ordinary::accepted_snapshot(&runtime)
            .tip()
            .expect("accepted")
            .height,
        150
    );
    ordinary::checkpoint(&runtime, 150, &ordinary::expectations()["rows"][150]);
    drop(runtime);
}

#[test]
fn phase167_ordinary_remaining_eight_negatives_at_isolated_heights() {
    // Arrange: every case owns a fresh canonical source and one extra identity.
    let corpus = ordinary::corpus();
    for (name, height) in [
        ("overpay-one-satoshi-150", 150),
        ("overpay-one-satoshi-299", 299),
        ("overpay-one-satoshi-300", 300),
        ("wrong-bip34-height", 150),
        ("bip68-time-nonfinal", 201),
        ("witness-reserve-31", 151),
        ("witness-reserve-33", 151),
        ("witness-commitment-mismatch", 151),
    ] {
        let source = ordinary::Source::new();
        let runtime = ordinary::open_genesis(&source);
        let negative = ordinary::named_negative(&corpus, name);
        assert_eq!(negative["height"].as_u64(), Some(height));
        assert_eq!(negative["parent_height"].as_u64(), Some(height - 1));
        for prefix in 1..height {
            ordinary::admit_and_connect(
                &runtime,
                &ordinary::block(ordinary::row(&corpus, prefix as usize)),
            );
        }
        // Act and Assert: one candidate per source stays below 306 known identities.
        assert_body_refusal(&runtime, negative);
        assert_eq!(
            ordinary::accepted_snapshot(&runtime)
                .tip()
                .expect("unchanged parent")
                .height,
            height as u32 - 1
        );
        assert!(
            runtime
                .network
                .header_entries()
                .expect("bounded identities")
                .len()
                <= 306
        );
        drop(runtime);
    }
}

#[test]
fn phase167_ordinary_creating_history_live_extension() {
    // Arrange: canonical source starts at proved genesis and owns all 200 ancestors.
    let source = ordinary::Source::new();
    let corpus = ordinary::corpus();
    let expected = ordinary::expectations();
    let runtime = ordinary::open_genesis(&source);
    for height in 1..=200 {
        ordinary::admit_and_connect(&runtime, &ordinary::block(ordinary::row(&corpus, height)));
    }
    let candidate = ordinary::block(ordinary::row(&corpus, 201));
    ordinary::assert_creating_ancestor_201(&candidate, &expected["creating_live_201"]);
    ordinary::assert_tip(&runtime, expected_row(&expected, 200));
    // Act: real managed apply must traverse the distant height-zero creating parent.
    ordinary::admit_and_connect(&runtime, &candidate);
    // Assert: exact accepted tip and durable script provenance identify this case.
    ordinary::assert_tip(&runtime, expected_row(&expected, 201));
    ordinary::assert_scripts_valid(&runtime, &candidate);
}

#[test]
fn phase167_ordinary_missing_envelope_context_is_unseeded_structure_only() {
    use open_bitcoin_core::consensus::{
        CandidateHistory, HistoryError, HistoryHeader, HistoryOrigin,
    };

    // Arrange: all supplied headers are from one source-bound ordinary branch.
    let corpus = ordinary::corpus();
    let selected = CanonicalChain::Regtest.spec();
    let candidate = ordinary::block(ordinary::row(&corpus, 201)).header;
    let parent = HistoryHeader::new(
        CanonicalChain::Regtest,
        200,
        ordinary::block(ordinary::row(&corpus, 200)).header,
    );
    let suffix = |oldest: u32| -> Vec<HistoryHeader> {
        (oldest..200)
            .rev()
            .map(|height| {
                HistoryHeader::new(
                    CanonicalChain::Regtest,
                    height,
                    ordinary::block(ordinary::row(&corpus, height as usize)).header,
                )
            })
            .collect()
    };

    // Act: MTP truncation refuses before any later context decision.
    let missing_mtp = CandidateHistory::check(
        selected.rules(),
        candidate.clone(),
        parent.clone(),
        suffix(191),
    );
    let checked = CandidateHistory::check(selected.rules(), candidate, parent, suffix(0))
        .expect("complete source-bound structural context");

    // Assert: precedence is the actual constructor result, never a stage result.
    assert!(matches!(
        missing_mtp,
        Err(HistoryError::MissingAncestor { height: 190 })
    ));
    assert_eq!(checked.height(), 201);
    assert!(matches!(checked.origin(), HistoryOrigin::GenesisLinked));
    assert_eq!(selected.rules().difficulty_adjustment_interval(), 144);
    assert!(selected.rules().no_pow_retargeting());
    // Both source-bound period boundaries preserve prior bits under selected Regtest.
    for height in [144_u32, 288] {
        let boundary = ordinary::block(ordinary::row(&corpus, height as usize));
        let prior = ordinary::block(ordinary::row(&corpus, height as usize - 1));
        assert_eq!(boundary.header.bits, prior.header.bits);
        let boundary_parent = HistoryHeader::new(CanonicalChain::Regtest, height - 1, prior.header);
        let ancestors = (0..height - 1)
            .rev()
            .map(|ancestor| {
                HistoryHeader::new(
                    CanonicalChain::Regtest,
                    ancestor,
                    ordinary::block(ordinary::row(&corpus, ancestor as usize)).header,
                )
            })
            .collect();
        let boundary_history = CandidateHistory::check(
            selected.rules(),
            boundary.header,
            boundary_parent,
            ancestors,
        )
        .expect("complete unseeded period-boundary structure");
        assert_eq!(
            boundary_history
                .maybe_retarget_anchor()
                .expect("structural boundary anchor")
                .height,
            height - 144
        );
    }
    // Difficulty validation skips retarget-anchor use because no_pow_retargeting is true.
}

#[test]
fn phase167_ordinary_branch_mtp_disagreement_refuses_same_spend() {
    // Arrange: isolate the real alternate branch below the 306-known bound.
    let source = ordinary::Source::new();
    let corpus = ordinary::corpus();
    let scenario = &corpus["branch_mtp_scenario"];
    assert_eq!(scenario["isolated_prefix_last_height"].as_u64(), Some(193));
    assert_eq!(scenario["independent_known_identities"].as_u64(), Some(202));
    let runtime = ordinary::open_genesis(&source);
    for height in 1..=193 {
        ordinary::admit_and_connect(&runtime, &ordinary::block(ordinary::row(&corpus, height)));
    }
    let branch = scenario["rows"]
        .as_array()
        .expect("independent branch bytes");
    assert_eq!(branch.len(), 8);
    for row in &branch[..7] {
        ordinary::admit_and_connect(&runtime, &ordinary::block(row));
    }
    let before = ordinary::accepted_snapshot(&runtime);
    let basic_before = ordinary::basic_state(&runtime);
    let candidate = ordinary::block(&branch[7]);
    let provenance_before = runtime
        .store
        .validation_provenance(block_hash(&candidate.header))
        .expect("branch pre-refusal provenance");
    assert_eq!(before.tip().expect("branch tip").height, 200);
    assert_eq!(
        before.tip().expect("branch parent MTP").median_time_past,
        scenario["parent_mtp"]
            .as_i64()
            .expect("independent branch MTP")
    );
    assert_ne!(
        scenario["parent_mtp"],
        ordinary::expectations()["rows"][200]["median_time_past"]
    );

    // Act: the same serialized time-lock spend is nonfinal on this genuine branch.
    assert_eq!(
        candidate.transactions[1],
        ordinary::block(ordinary::row(&corpus, 201)).transactions[1]
    );
    runtime
        .network
        .receive_message(
            1,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![candidate.header.clone()],
            }),
            i64::MIN,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("actual alternate header context");
    runtime
        .store
        .save_block(&candidate, crate::PersistMode::Sync)
        .expect("retained alternate body");
    let refusal = runtime
        .network
        .connect_local_block(&candidate, runtime.verify_flags, runtime.consensus_params)
        .expect_err("branch-time spend must remain nonfinal");

    // Assert: only the candidate's header becomes known; accepted state stays exact.
    assert!(
        refusal.to_string().contains(
            scenario["expected_reason"]
                .as_str()
                .expect("independent refusal")
        ),
        "observed branch precedence: {refusal:?}"
    );
    assert_eq!(ordinary::accepted_snapshot(&runtime), before);
    assert_eq!(ordinary::basic_state(&runtime), basic_before);
    assert_eq!(
        runtime
            .network
            .header_entries()
            .expect("known branch")
            .len(),
        202
    );
    assert_ne!(provenance_before, ValidationProvenance::ScriptsValid);
    assert_eq!(
        runtime
            .store
            .validation_provenance(block_hash(&candidate.header))
            .expect("branch post-refusal provenance"),
        provenance_before
    );
    drop(runtime);
}

#[test]
fn phase167_ordinary_corrupt_durable_history_refuses_without_source_mutation() {
    // Arrange: first earn a real default-Regtest history, then close every owner.
    let source = ordinary::Source::new();
    let corpus = ordinary::corpus();
    let runtime = ordinary::open_genesis(&source);
    for height in 1..=150 {
        ordinary::admit_and_connect(&runtime, &ordinary::block(ordinary::row(&corpus, height)));
    }
    ordinary::checkpoint(&runtime, 150, &ordinary::expectations()["rows"][150]);
    drop(runtime);
    ordinary::corrupt_untrusted_work_after_closed_runtime(&source);
    let hostile_before = ordinary::physical_manifest(source.path());
    assert!(!hostile_before.is_empty());

    // Act: product history reopen must reject the inconsistent durable source.
    let refusal = super::DurableSyncRuntime::reopen_canonical_history_configured(
        source.path(),
        ordinary::selected_config(),
        crate::chainstate::BasicFilterStartupMode::PreserveSaved,
    )
    .err()
    .expect("corrupt stored work/coverage must not mint canonical authority");

    // Assert: no source file is rewritten, removed or added by the failed reproof.
    assert_eq!(ordinary::physical_manifest(source.path()), hostile_before);
    eprintln!("ORDINARY_CORRUPT_SOURCE_REFUSAL {refusal}");
}
