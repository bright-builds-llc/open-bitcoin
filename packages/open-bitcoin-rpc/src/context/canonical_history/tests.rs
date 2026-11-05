// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp
// - packages/bitcoin-knots/test/functional/mining_basic.py

use super::*;
use crate::{
    context::CanonicalObservation,
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use open_bitcoin_node::{
    SyncNetwork,
    core::{
        chainstate::CoinsView,
        codec::{encode_block, parse_block},
        consensus::{CanonicalChain, block_hash, block_merkle_root, check_block_header},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
        wallet::AddressNetwork,
    },
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

type Context = ManagedRpcContext<FjallChainstateStore, FjallCoinsView>;
static SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct Source(PathBuf);
impl Source {
    fn new() -> Self {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned fixture nonce");
        Self(
            std::env::temp_dir()
                .canonicalize()
                .expect("physical temp")
                .join(format!("phase167-rpc-{:x}", u128::from_le_bytes(nonce))),
        )
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).expect("owned source after all handles drop");
        }
    }
}
fn config(source: &Source, mode: BasicFilterIndexSetting) -> RuntimeConfig {
    let mut config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(source.0.clone()),
        block_filter_index: mode,
        ..Default::default()
    };
    config.sync.runtime.network = SyncNetwork::Regtest;
    config
}
fn open(source: &Source, mode: BasicFilterIndexSetting) -> (Context, DurableSyncRuntime) {
    Context::from_canonical_history_runtime_config(&config(source, mode))
        .expect("configuration-only earned producer")
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn frozen_regtest_block(row: &Value) -> Block {
    let encoded = row["block_hex"].as_str().expect("frozen block bytes");
    assert_eq!(encoded.len() % 2, 0, "whole hexadecimal bytes");
    let bytes = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16)
                .expect("valid frozen hex")
        })
        .collect::<Vec<_>>();
    parse_block(&bytes).expect("canonical frozen block")
}
fn submit(context: &mut Context, block: &Block) -> Result<Value, crate::RpcFailure> {
    let data = hex(&open_bitcoin_codec::encode_block(block).expect("exact serialized fixture"));
    dispatch(
        context,
        normalize_method_call(
            "submitblock",
            RequestParameters::Positional(vec![json!(data)]),
        )
        .expect("actual registry"),
    )
}
fn observe(context: &Context, queries: &[BlockHash]) -> CanonicalObservation {
    context
        .observe_canonical_state(queries)
        .expect("same-command complete actual observation")
}
fn stable(facts: &CanonicalObservation) -> impl std::fmt::Debug + PartialEq + use<> {
    (
        facts.coins.clone(),
        facts.maybe_progress.map(|p| {
            (
                p.generation(),
                p.accepted_target(),
                p.maybe_processed_endpoint(),
                p.maybe_safe_durable_endpoint(),
                p.protection(),
                p.initially_synchronized(),
                p.state(),
            )
        }),
        facts.maybe_stored_basic,
        facts.rows.clone(),
        facts.provenance.clone(),
    )
}
fn child(parent: &BlockHeader, height: u32) -> Block {
    let mut number = Vec::new();
    let mut value = height;
    while value > 0 {
        number.push(value as u8);
        value >>= 8;
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut encoded = vec![number.len() as u8];
        encoded.extend(number);
        encoded
    };
    script.push(0);
    let transaction = Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script).expect("height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000 >> (height / 150))
                .expect("independent pinned subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("fixture true"),
        }],
    };
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: Default::default(),
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    remine(&mut block);
    block
}
fn remine(block: &mut Block) {
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("fixture merkle")
        .0;
    block.header.nonce = (0..100_000)
        .find(|n| {
            block.header.nonce = *n;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded real regtest PoW");
}
fn assert_accepted(context: &mut Context, block: &Block) {
    assert_eq!(
        submit(context, block).expect("actual command outcome"),
        Value::Null
    );
}

#[test]
fn phase167_canonical_history_same_command_arc_complete_observation_and_refusals() {
    let _scenario = SCENARIO.lock().expect("inspection process isolation");
    for mode in [
        BasicFilterIndexSetting::Basic,
        BasicFilterIndexSetting::Disabled,
    ] {
        // Arrange
        let source = Source::new();
        let (mut context, runtime) = open(&source, mode);
        let genesis = CanonicalChain::Regtest.spec().genesis_block();
        let hash = block_hash(&genesis.header);
        let before = observe(&context, &[hash]);
        assert_eq!(before.rows.len(), 9);
        assert_eq!(
            before.coins.2,
            runtime
                .store()
                .coins_view()
                .complete_snapshot()
                .expect("independent durable Coins")
        );
        assert!(context.maybe_canonical_submission.is_some());
        // Act / Assert: real normalizer/decode refusals cannot modify any native row.
        for params in [
            RequestParameters::None,
            RequestParameters::Positional(vec![json!("00")]),
            RequestParameters::Positional(vec![Value::Null]),
        ] {
            let result = normalize_method_call("submitblock", params)
                .and_then(|call| dispatch(&mut context, call));
            assert!(result.is_err());
            assert_eq!(observe(&context, &[hash]), before);
        }
        let block = child(&genesis.header, 1);
        assert_accepted(&mut context, &block);
        let accepted = observe(&context, &[block_hash(&block.header)]);
        assert_eq!(
            accepted.coins.0.tip().expect("accepted tip").block_hash,
            block_hash(&block.header)
        );
        assert_eq!(
            accepted.coins.2,
            runtime
                .store()
                .coins_view()
                .complete_snapshot()
                .expect("actual same-store Coins")
        );
        assert_ne!(accepted.coins.0, before.coins.0);
        let mut witness_duplicate = block.clone();
        witness_duplicate.transactions[0].inputs[0].witness =
            ScriptWitness::new(vec![vec![0x44; 32]]);
        assert_eq!(
            submit(&mut context, &witness_duplicate).expect("intrinsic-before-context duplicate"),
            json!("duplicate")
        );
        assert_eq!(observe(&context, &[block_hash(&block.header)]), accepted);
        assert_eq!(
            submit(&mut context, &genesis).expect("genesis duplicate precedes parent lookup"),
            json!("duplicate")
        );
        let mut corrupt_base = block.clone();
        corrupt_base.transactions[0].outputs[0].value =
            Amount::from_sats(1).expect("valid amount data");
        assert_eq!(
            submit(&mut context, &corrupt_base).expect("genuine intrinsic rejection"),
            json!("bad-txnmrklroot")
        );
        assert_eq!(observe(&context, &[block_hash(&block.header)]), accepted);
        assert!(matches!(&context.wallet_state, WalletState::Unavailable));
        drop(context);
        drop(runtime);
    }
}

#[test]
fn phase167_canonical_history_contextual_failure_has_real_known_only_delta() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut overpay = child(&genesis.header, 1);
    overpay.transactions[0].outputs[0].value =
        Amount::from_sats(5_000_000_001).expect("range-valid excessive subsidy");
    remine(&mut overpay);
    let hash = block_hash(&overpay.header);
    let before = observe(&context, &[hash]);
    // Act
    assert_eq!(
        submit(&mut context, &overpay).expect("actual conclusive failure"),
        json!("bad-cb-amount")
    );
    let after = observe(&context, &[hash]);
    // Assert: accepted fields remain, genuinely admitted header rows need not.
    assert_eq!(after.coins, before.coins);
    assert_eq!(after.maybe_progress, before.maybe_progress);
    assert_eq!(after.maybe_stored_basic, before.maybe_stored_basic);
    assert_ne!(after.rows, before.rows);
    assert_ne!(
        after.provenance[&hash].0,
        open_bitcoin_node::network::BasicBlockValidationProvenance::ScriptsValid
    );
    assert_eq!(
        submit(&mut context, &overpay).expect("actual recorded permanent failure"),
        json!("duplicate-invalid")
    );
    assert_eq!(observe(&context, &[hash]).coins, before.coins);
    drop(context);
    drop(runtime);
}

#[test]
fn phase167_canonical_history_missing_input_has_public_knots_reason() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange: admit the same frozen canonical parent as Plan 15's negative.
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let corpus: Value = serde_json::from_slice(
        &fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../testdata/phase167/regtest-history.json"
        ))
        .expect("frozen ordinary corpus"),
    )
    .expect("canonical frozen JSON");
    let rows = corpus["rows"].as_array().expect("frozen rows");
    for row in rows.iter().skip(1).take(100) {
        assert_accepted(&mut context, &frozen_regtest_block(row));
    }
    let negative = corpus["negatives"]
        .as_array()
        .expect("frozen negatives")
        .iter()
        .find(|row| row["name"] == "missing-input")
        .expect("source-bound missing input");
    assert_eq!(negative["height"], 101);
    let bad = frozen_regtest_block(negative);
    assert_eq!(
        hex(&encode_block(&bad).expect("canonical re-encoding")),
        negative["block_hex"].as_str().expect("frozen block hex")
    );
    let hash = block_hash(&bad.header);
    let before = observe(&context, &[hash]);
    // Act: the public RPC dispatch maps typed MissingCoin to Knots' block reason.
    assert_eq!(
        submit(&mut context, &bad).expect("conclusive public rejection"),
        json!("bad-txns-inputs-missingorspent")
    );
    let after = observe(&context, &[hash]);
    // Assert: a known header may remain, but accepted state and BASIC do not move.
    assert_eq!(after.coins, before.coins);
    assert_eq!(after.maybe_progress, before.maybe_progress);
    assert_eq!(after.maybe_stored_basic, before.maybe_stored_basic);
    assert_eq!(after.provenance[&hash], before.provenance[&hash]);
    assert_ne!(
        after.provenance[&hash].0,
        open_bitcoin_node::network::BasicBlockValidationProvenance::ScriptsValid
    );
    drop(context);
    drop(runtime);
}

#[test]
fn phase167_canonical_history_301_default_subsidies_full_drop_reopen_next() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut parent = genesis.header.clone();
    // Act: serialized test-local history, independent of future15 corpus.
    for height in 1..=301 {
        let block = child(&parent, height);
        assert_accepted(&mut context, &block);
        let expected = match height {
            149 => Some(5_000_000_000),
            150 | 299 => Some(2_500_000_000),
            300 | 301 => Some(1_250_000_000),
            _ => None,
        };
        if let Some(value) = expected {
            assert_eq!(block.transactions[0].outputs[0].value.to_sats(), value);
        }
        parent = block.header;
    }
    let hash = block_hash(&parent);
    let before = observe(&context, &[hash]);
    assert_eq!(before.coins.0.tip().expect("tip").height, 301);
    assert_eq!(
        before.coins.0.tip().expect("work").chain_work.to_hex(),
        format!("{:064x}", 604u64)
    );
    assert!(before.coins.0.utxos.keys().all(|outpoint| {
        outpoint.txid
            != open_bitcoin_node::core::consensus::transaction_txid(&genesis.transactions[0])
                .expect("genesis txid")
    }));
    let before_stable = stable(&before);
    assert_eq!(
        before
            .maybe_progress
            .expect("actual BASIC")
            .branch_identity(),
        block_hash(&genesis.header)
    );
    drop(before);
    drop(context);
    drop(runtime);
    // Assert: every Arc/context/closure/owner is dropped before safe original reproof.
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Unspecified);
    let reopened = observe(&context, &[hash]);
    assert_eq!(stable(&reopened), before_stable);
    assert_eq!(
        reopened
            .maybe_progress
            .expect("recovered branch")
            .branch_identity(),
        hash
    );
    assert_accepted(&mut context, &child(&parent, 302));
    assert_eq!(
        observe(&context, &[]).coins.0.tip().expect("next").height,
        302
    );
    assert!(!source.0.join("wallet.dat").exists());
    drop(context);
    drop(runtime);
}

#[test]
fn phase167_canonical_history_maturity_99_refuses_and_100_accepts_real_spend() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut parent = genesis.header.clone();
    let first = child(&parent, 1);
    let outpoint = OutPoint {
        txid: open_bitcoin_node::core::consensus::transaction_txid(&first.transactions[0])
            .expect("first txid"),
        vout: 0,
    };
    for height in 1..=99 {
        let block = child(&parent, height);
        assert_accepted(&mut context, &block);
        parent = block.header;
    }
    let spend = Transaction {
        version: 2,
        lock_time: 0,
        inputs: vec![TransactionInput {
            previous_output: outpoint.clone(),
            script_sig: ScriptBuf::from_bytes(vec![]).expect("empty script"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(4_900_000_000).expect("spend less fee"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
    };
    let mut immature = child(&parent, 100);
    immature.transactions.push(spend.clone());
    remine(&mut immature);
    let before = observe(&context, &[]);
    // Act / Assert
    assert_eq!(
        submit(&mut context, &immature).expect("actual maturity refusal"),
        json!("bad-txns-premature-spend-of-coinbase")
    );
    assert_eq!(observe(&context, &[]).coins, before.coins);
    let hundred = child(&parent, 100);
    assert_accepted(&mut context, &hundred);
    let mut mature = child(&hundred.header, 101);
    mature.transactions[0].outputs[0].value =
        Amount::from_sats(5_100_000_000).expect("reward plus actual100000000 fee");
    mature.transactions.push(spend);
    remine(&mut mature);
    assert_accepted(&mut context, &mature);
    assert!(!observe(&context, &[]).coins.0.utxos.contains_key(&outpoint));
    drop(context);
    drop(runtime);
}

#[test]
fn phase167_canonical_history_generic_fixture_foreign_and_activation_are_closed() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange
    let source = Source::new();
    let selected = config(&source, BasicFilterIndexSetting::Basic);
    let mut fixture_config = selected.clone();
    fixture_config.maybe_data_dir = None;
    let fixture = ManagedRpcContext::from_fixture_runtime_config(&fixture_config);
    assert!(fixture.maybe_canonical_submission.is_none());
    assert!(fixture.observe_canonical_state(&[]).is_err());
    // Act / Assert: all ordinary generic aliases preserve capability diagnostics,
    // then independently refuse unproved ownership if implementation support grows.
    for result in [
        ManagedRpcContext::try_from_runtime_config(&selected),
        ManagedRpcContext::from_runtime_config(&selected),
        ManagedRpcContext::try_from_runtime_config_with_store(&selected, None),
    ] {
        assert!(matches!(
            result,
            Err(CanonicalContextError::UnsupportedValidationScope(_))
                | Err(CanonicalContextError::UnprovedOwner)
        ));
    }
    assert!(!source.0.exists());
    let mut foreign = selected.clone();
    foreign.sync.runtime.network = SyncNetwork::Mainnet;
    assert!(matches!(
        Context::from_canonical_history_runtime_config(&foreign),
        Err(CanonicalContextError::ChainMismatch)
    ));
    assert!(!source.0.exists());
    let mut activated = selected;
    activated.inbound.enabled = true;
    assert!(Context::from_canonical_history_runtime_config(&activated).is_err());
    assert!(!source.0.exists());
    drop(fixture);
}

#[test]
fn phase167_canonical_history_persisted_peer_status_cannot_mask_new_earned_tip() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange: genuine owner-generated peer-sync status at the actual genesis tip.
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("shell clock")
        .as_secs();
    let state = runtime
        .durable_sync_state(
            open_bitcoin_node::SyncLifecycleState::Stopped,
            None,
            i64::try_from(now).expect("current shell time"),
        )
        .expect("actual owner status");
    runtime
        .persist_durable_sync_state(state)
        .expect("real supported status write");
    let query = || {
        normalize_method_call("getblockchaininfo", RequestParameters::None)
            .expect("normal registry")
    };
    assert_eq!(
        dispatch(&mut context, query()).expect("genuine root status")["blocks"],
        0
    );
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    // Act
    assert_accepted(&mut context, &block);
    let actual = observe(&context, &[block_hash(&block.header)]);
    let public = dispatch(&mut context, query()).expect("ordinary actual RPC projection");
    // Assert: this proposed UNRUN regression exposes the dispatch/node.rs seam;
    // a persisted earlier peer snapshot cannot replace current earned chain facts.
    assert_eq!(
        actual
            .coins
            .0
            .tip()
            .expect("genuine observed accepted tip")
            .height,
        1
    );
    assert_eq!(public["blocks"], 1);
    assert_eq!(public["headers"], 1);
    let mut display = block_hash(&block.header).to_byte_array();
    display.reverse();
    assert_eq!(public["bestblockhash"], hex(&display));

    drop(context);
    drop(runtime);
}

#[test]
fn phase167_canonical_history_header_ahead_is_not_collapsed_to_accepted_height() {
    let _scenario = SCENARIO.lock().expect("scenario");
    // Arrange: genuine command accepts one block, then admits a nonpermanent
    // witness-mutated child's header through its real existing processing path.
    let source = Source::new();
    let (mut context, runtime) = open(&source, BasicFilterIndexSetting::Basic);
    let first = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    assert_accepted(&mut context, &first);
    let mut ahead = child(&first.header, 2);
    ahead.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::from_serialized_bytes(
            [vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed], vec![0; 32]].concat(),
        ),
    });
    ahead.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![7]]);
    remine(&mut ahead);
    // Act: wrong reserve has genuine Mutated category, not permanent failure.
    assert_eq!(
        submit(&mut context, &ahead).expect("actual contextual witness outcome"),
        json!("bad-witness-nonce-size")
    );
    let headers = context
        .network
        .best_chain_entries()
        .expect("actual best-work header chain");
    let accepted = observe(&context, &[block_hash(&ahead.header)]);
    let public = dispatch(
        &mut context,
        normalize_method_call("getblockchaininfo", RequestParameters::None).expect("registry"),
    )
    .expect("ordinary projection");
    // Assert: this UNRUN control requires real blocks1/headers2/hash1.
    assert_eq!(headers.last().expect("real admitted header").height, 2);
    assert_eq!(accepted.coins.0.tip().expect("accepted tip").height, 1);
    assert_eq!(public["blocks"], 1);
    assert_eq!(public["headers"], 2);
    let mut display = block_hash(&first.header).to_byte_array();
    display.reverse();
    assert_eq!(public["bestblockhash"], hex(&display));
    drop(context);
    drop(runtime);
}
