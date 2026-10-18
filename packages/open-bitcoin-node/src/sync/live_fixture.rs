// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Explicit consuming fixture assembly; retained bytes never upgrade a raw owner.

use open_bitcoin_core::{
    chainstate::{
        Chainstate, ChainstateSnapshot, CoinsView, FlushPolicyTime, LiveWorkSourceBinding,
        coins::CompleteCoinsSnapshot,
    },
    consensus::{CanonicalChain, ConsensusParams, ScriptVerifyFlags, chain_proof::ProofWork},
    primitives::Block,
};
use open_bitcoin_network::{BlockRelayActivationPolicy, HeaderStore, RelayActivationConfig};

use super::{DurableSyncRuntime, SyncRuntimeConfig, SyncRuntimeError};
use crate::{
    FjallChainstateStore, FjallCoinsView, FjallNodeStore, ManagedChainstate, ManagedPeerNetwork,
    chainstate::{BasicFilterStartupMode, initialize_configured},
    storage::validation_history::{StoredValidationStatus, ValidationHistoryRecord},
};

type Source = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;
type Network = ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>;

pub(super) const CANONICAL_REFUSAL: &str = "canonical root is not a live fixture source";

pub(super) struct CapturedFixtureIdentity {
    binding: LiveWorkSourceBinding,
    params: ConsensusParams,
    expected: ChainstateSnapshot,
    coins: CompleteCoinsSnapshot,
    blocks: Vec<Block>,
    validation: Vec<ValidationHistoryRecord>,
}

fn invalid(message: impl std::fmt::Display) -> SyncRuntimeError {
    SyncRuntimeError::InvalidData {
        message: message.to_string(),
    }
}

fn canonical_root(hash: open_bitcoin_core::primitives::BlockHash) -> bool {
    [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ]
    .into_iter()
    .any(|chain| chain.spec().genesis_hash() == hash)
}

fn pure_source_gate(
    source: &Source,
    params: ConsensusParams,
) -> Result<LiveWorkSourceBinding, SyncRuntimeError> {
    let core = source.chainstate();
    core.validate_live_origin().map_err(invalid)?;
    let binding = core.bind_live_work_source(params).map_err(invalid)?;
    if source.has_pending_validation_history() {
        return Err(invalid("pending fixture validation publication"));
    }
    if core
        .active_chain()
        .first()
        .is_some_and(|root| canonical_root(root.block_hash))
    {
        return Err(invalid(CANONICAL_REFUSAL));
    }
    Ok(binding)
}

impl DurableSyncRuntime {
    /// Consumes the same genuine, flushed fixture core under explicit parameters.
    /// Initial source refusals precede startup. Later startup/concurrent I/O errors
    /// may follow effects and do not return the consumed source or promise rollback.
    pub fn from_live_fixture_source(
        source: Source,
        config: SyncRuntimeConfig,
        consensus_params: ConsensusParams,
        basic_filter_mode: BasicFilterStartupMode,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
    ) -> Result<Self, SyncRuntimeError> {
        let captured = capture_live_fixture_source(&source, consensus_params)?;
        let (sink, core) = source.into_parts();
        let store = sink.inner().clone();
        let now = FlushPolicyTime::from_unix_seconds(0);
        observe_stage(1);
        let (lifecycle, _view, _cache) =
            initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;
        let managed = ManagedChainstate::from_recovered_chainstate(sink, core, lifecycle)?;
        let mut network = Self::prepare_initialized_network(
            &store,
            managed,
            &config,
            basic_filter_mode,
            relay_activation,
            block_relay_activation,
            inbound_enabled,
        )?;
        initialize_live_fixture_source_graph(&mut network, &captured)?;
        check_captured_live_fixture_source(network.chainstate(), &captured)?;
        // Normal BASIC proof methods acquire publication themselves, outside content checks.
        Self::finish_initialized_runtime(store, network, config, consensus_params)
    }
}

pub(super) fn capture_live_fixture_source(
    source: &Source,
    params: ConsensusParams,
) -> Result<CapturedFixtureIdentity, SyncRuntimeError> {
    let binding = pure_source_gate(source, params)?;
    observe_stage(0);
    let store = source.store().inner();
    let _publication = store.filter_publication_guard()?;
    if !source.chainstate().coins().parent().belongs_to(store)
        || source.has_pending_validation_history()
    {
        return Err(invalid("foreign or pending fixture source"));
    }
    if store.load_header_store()?.is_some_and(|headers| {
        headers.entries().any(|entry| {
            entry.header.previous_block_hash == Default::default()
                && canonical_root(entry.block_hash)
        })
    }) {
        return Err(invalid(CANONICAL_REFUSAL));
    }
    let mut replay = Chainstate::new();
    let mut blocks = Vec::new();
    for position in source.chainstate().active_chain() {
        let block = store
            .load_block(position.block_hash)?
            .ok_or_else(|| invalid("missing retained fixture active body"))?;
        let accepted = replay
            .connect_block_with_current_time(
                &block,
                ProofWork::ZERO,
                i64::from(block.header.time),
                ScriptVerifyFlags::P2SH,
                params,
            )
            .map_err(invalid)?;
        if accepted != *position {
            return Err(invalid("independent fixture position mismatch"));
        }
        blocks.push(block);
    }
    let expected = replay.snapshot();
    let coins = replay.coins().complete_snapshot().map_err(invalid)?;
    let validation = read_validation(store, &expected)?;
    let captured = CapturedFixtureIdentity {
        binding,
        params,
        expected,
        coins,
        blocks,
        validation,
    };
    check_content(source, &captured)?;
    Ok(captured)
}

fn read_validation(
    store: &FjallNodeStore,
    expected: &ChainstateSnapshot,
) -> Result<Vec<ValidationHistoryRecord>, SyncRuntimeError> {
    if !store.recovered_validation_history()?.is_complete()? {
        return Err(invalid("incomplete fixture validation coverage"));
    }
    expected
        .active_chain
        .iter()
        .map(|position| {
            let record = store
                .maybe_validation_history_record(position.block_hash)?
                .ok_or_else(|| invalid("missing fixture ScriptsValid record"))?;
            let identity = record.identity();
            if record.status() != StoredValidationStatus::ScriptsValid
                || identity.hash() != position.block_hash
                || identity.parent_hash() != position.previous_block_hash()
                || identity.height() != position.height
            {
                return Err(invalid("different fixture ScriptsValid identity"));
            }
            Ok(record)
        })
        .collect()
}

fn check_content(
    source: &Source,
    captured: &CapturedFixtureIdentity,
) -> Result<(), SyncRuntimeError> {
    let core = source.chainstate();
    core.check_live_work_source(&captured.binding)
        .map_err(invalid)?;
    core.bind_live_work_source(captured.params)
        .map_err(invalid)?;
    let store = source.store().inner();
    if !core.coins().parent().belongs_to(store) || source.has_pending_validation_history() {
        return Err(invalid("foreign or pending fixture source"));
    }
    let actual = core.overlay_snapshot();
    let (positions, maybe_counts) = store.load_chain_meta_for_open()?;
    let physical = store.coins_view().complete_snapshot().map_err(invalid)?;
    if actual.active_chain != captured.expected.active_chain
        || actual.undo_by_block != captured.expected.undo_by_block
        || actual.maybe_confirmed_txid_counts != captured.expected.maybe_confirmed_txid_counts
        || positions != captured.expected.active_chain
        || store.load_all_undo_records()? != captured.expected.undo_by_block
        || maybe_counts.as_ref().is_some_and(|counts| {
            Some(counts) != captured.expected.maybe_confirmed_txid_counts.as_ref()
        })
        || core.coins().complete_snapshot().map_err(invalid)? != captured.coins
        || physical != captured.coins
        || !physical.head_blocks.is_empty()
        || physical.maybe_best_block != core.tip().map(|tip| tip.block_hash)
    {
        return Err(invalid("independent fixture physical identity mismatch"));
    }
    for (position, expected_block) in captured.expected.active_chain.iter().zip(&captured.blocks) {
        if store.load_block(position.block_hash)?.as_ref() != Some(expected_block) {
            return Err(invalid("captured fixture body changed"));
        }
    }
    if read_validation(store, &captured.expected)? != captured.validation {
        return Err(invalid("captured fixture validation changed"));
    }
    Ok(())
}

pub(super) fn check_captured_live_fixture_source(
    source: &Source,
    captured: &CapturedFixtureIdentity,
) -> Result<(), SyncRuntimeError> {
    pure_source_gate(source, captured.params)?;
    let store = source.store().inner();
    let _publication = store.filter_publication_guard()?;
    check_content(source, captured)
}

pub(super) fn initialize_live_fixture_source_graph(
    network: &mut Network,
    captured: &CapturedFixtureIdentity,
) -> Result<(), SyncRuntimeError> {
    let Some(root) = captured.blocks.first() else {
        return Ok(());
    };
    let mut headers = HeaderStore::default();
    let current_time = captured
        .blocks
        .iter()
        .map(|block| i64::from(block.header.time))
        .max()
        .ok_or_else(|| invalid("missing fixture graph time"))?;
    for block in &captured.blocks {
        headers.insert_header_with_params(block.header.clone(), &captured.params)?;
    }
    network.try_admit_fixture_headers(headers, current_time, captured.params)?;
    network.retain_checked_candidate(root, captured.params, current_time)?;
    for block in captured.blocks.iter().skip(1) {
        network.retain_cached_candidate(block, captured.params, current_time)?;
    }
    Ok(())
}

#[cfg(test)]
thread_local! { static STAGES: std::cell::Cell<[usize; 2]> = const { std::cell::Cell::new([0; 2]) }; }

fn observe_stage(_index: usize) {
    #[cfg(test)]
    STAGES.with(|stages| {
        let mut counts = stages.get();
        counts[_index] += 1;
        stages.set(counts);
    });
}

#[cfg(test)]
pub(super) fn take_stage_observations() -> [usize; 2] {
    STAGES.with(|stages| stages.replace([0; 2]))
}
