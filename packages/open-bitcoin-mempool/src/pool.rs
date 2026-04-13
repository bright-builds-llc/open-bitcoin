use std::collections::{BTreeSet, HashMap, HashSet};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, TransactionValidationContext,
    transaction_txid, transaction_wtxid, validate_transaction_with_context,
};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid};

use crate::{
    AdmissionResult, LimitDirection, LimitKind, MEMPOOL_HEIGHT, MempoolEntry, MempoolError,
    PolicyConfig, RbfPolicy, signals_opt_in_rbf, transaction_sigops_cost,
    transaction_weight_and_virtual_size, validate_standard_transaction,
};

#[derive(Debug, Clone)]
struct MempoolState {
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    total_virtual_size: usize,
}

#[derive(Debug, Clone)]
pub struct Mempool {
    config: PolicyConfig,
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    total_virtual_size: usize,
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new(PolicyConfig::default())
    }
}

impl Mempool {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            config,
            entries: HashMap::new(),
            spent_outpoints: HashMap::new(),
            total_virtual_size: 0,
        }
    }

    pub fn config(&self) -> &PolicyConfig {
        &self.config
    }

    pub fn entries(&self) -> &HashMap<Txid, MempoolEntry> {
        &self.entries
    }

    pub fn entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    pub fn total_virtual_size(&self) -> usize {
        self.total_virtual_size
    }

    pub fn accept_transaction(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, MempoolError> {
        let txid = transaction_txid(&transaction).expect("typed transactions should derive txid");
        if self.entries.contains_key(&txid) {
            return Err(MempoolError::DuplicateTransaction { txid });
        }

        let input_contexts = derive_input_contexts(&transaction, chainstate, &self.entries)?;
        let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction);
        let sigops_cost = transaction_sigops_cost(&transaction, &input_contexts)?;
        validate_standard_transaction(
            &transaction,
            &input_contexts,
            &self.config,
            weight,
            sigops_cost,
        )?;

        let validation_context =
            build_validation_context(chainstate, input_contexts, verify_flags, consensus_params);
        let fee = validate_transaction_with_context(&transaction, &validation_context).map_err(
            |source| MempoolError::Validation {
                reason: source.to_string(),
            },
        )?;
        enforce_min_relay_fee(&self.config, fee.to_sats(), virtual_size)?;

        let direct_conflicts = self.direct_conflicts(&transaction);
        let replace_set = if direct_conflicts.is_empty() {
            BTreeSet::new()
        } else {
            self.validate_replacement(&transaction, &direct_conflicts, fee.to_sats(), virtual_size)?
        };

        let wtxid =
            transaction_wtxid(&transaction).expect("typed transactions should derive wtxid");
        let mut prospective_entries = self.entries.clone();
        for conflict_txid in &replace_set {
            prospective_entries.remove(conflict_txid);
        }
        prospective_entries.insert(
            txid,
            MempoolEntry::new(
                transaction,
                txid,
                wtxid,
                fee,
                virtual_size,
                weight,
                sigops_cost,
            ),
        );

        let prospective_state = recompute_state(prospective_entries);
        validate_limits(&prospective_state.entries, &self.config, txid)?;
        let (trimmed_state, evicted) = trim_to_size(prospective_state, &self.config);
        if !trimmed_state.entries.contains_key(&txid) {
            return Err(MempoolError::CandidateEvicted { txid });
        }

        self.entries = trimmed_state.entries;
        self.spent_outpoints = trimmed_state.spent_outpoints;
        self.total_virtual_size = trimmed_state.total_virtual_size;

        Ok(AdmissionResult {
            accepted: txid,
            replaced: replace_set.into_iter().collect(),
            evicted: evicted.into_iter().collect(),
        })
    }

    fn direct_conflicts(&self, transaction: &Transaction) -> BTreeSet<Txid> {
        let mut conflicts = BTreeSet::new();
        for input in &transaction.inputs {
            let Some(conflicting_txid) = self.spent_outpoints.get(&input.previous_output) else {
                continue;
            };
            conflicts.insert(*conflicting_txid);
        }

        conflicts
    }

    fn validate_replacement(
        &self,
        transaction: &Transaction,
        direct_conflicts: &BTreeSet<Txid>,
        candidate_fee_sats: i64,
        virtual_size: usize,
    ) -> Result<BTreeSet<Txid>, MempoolError> {
        match self.config.rbf_policy {
            RbfPolicy::Never => {
                return Err(MempoolError::ConflictNotAllowed {
                    conflicting: direct_conflicts.iter().copied().collect(),
                    policy: RbfPolicy::Never,
                });
            }
            RbfPolicy::OptIn => {
                let maybe_signaled = direct_conflicts
                    .iter()
                    .filter_map(|txid| self.entries.get(txid))
                    .any(|entry| signals_opt_in_rbf(&entry.transaction));
                if !maybe_signaled {
                    return Err(MempoolError::ConflictNotAllowed {
                        conflicting: direct_conflicts.iter().copied().collect(),
                        policy: RbfPolicy::OptIn,
                    });
                }
            }
            RbfPolicy::Always => {}
        }

        let replace_set = collect_conflicts_and_descendants(&self.entries, direct_conflicts);
        let conflicting_fee_sats = replace_set
            .iter()
            .filter_map(|txid| self.entries.get(txid))
            .map(MempoolEntry::fee_sats)
            .sum::<i64>();
        if candidate_fee_sats <= conflicting_fee_sats {
            return Err(MempoolError::ReplacementRejected {
                reason: format!(
                    "replacement fee {candidate_fee_sats} must exceed conflicting fee {conflicting_fee_sats}"
                ),
            });
        }

        let candidate_feerate =
            crate::types::FeeRate::from_fee_sats_and_vbytes(candidate_fee_sats, virtual_size);
        for conflicting_txid in direct_conflicts {
            let Some(entry) = self.entries.get(conflicting_txid) else {
                continue;
            };
            if candidate_feerate <= entry.fee_rate() {
                return Err(MempoolError::ReplacementRejected {
                    reason: format!(
                        "replacement feerate {} must exceed conflicting feerate {} for {:?}",
                        candidate_feerate,
                        entry.fee_rate(),
                        conflicting_txid
                    ),
                });
            }
        }

        let required_bump = self
            .config
            .incremental_relay_feerate
            .fee_for_virtual_size(virtual_size);
        let additional_fee = candidate_fee_sats - conflicting_fee_sats;
        if additional_fee < required_bump {
            return Err(MempoolError::ReplacementRejected {
                reason: format!(
                    "replacement fee bump {additional_fee} must be at least {required_bump}"
                ),
            });
        }

        let conflicting_inputs = direct_conflicts
            .iter()
            .filter_map(|txid| self.entries.get(txid))
            .flat_map(|entry| {
                entry
                    .transaction
                    .inputs
                    .iter()
                    .map(|input| input.previous_output.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();
        for input in &transaction.inputs {
            if self.entries.contains_key(&input.previous_output.txid)
                && !conflicting_inputs.contains(&input.previous_output)
            {
                return Err(MempoolError::ReplacementRejected {
                    reason: "replacement adds new unconfirmed inputs".to_string(),
                });
            }
        }

        Ok(replace_set)
    }
}

fn derive_input_contexts(
    transaction: &Transaction,
    chainstate: &ChainstateSnapshot,
    entries: &HashMap<Txid, MempoolEntry>,
) -> Result<Vec<TransactionInputContext>, MempoolError> {
    let maybe_tip = chainstate.tip();
    let mempool_median_time_past = maybe_tip.map_or(0, |tip| tip.median_time_past);
    let mut input_contexts = Vec::with_capacity(transaction.inputs.len());

    for input in &transaction.inputs {
        if let Some(coin) = chainstate.utxos.get(&input.previous_output) {
            input_contexts.push(TransactionInputContext {
                spent_output: coin.as_spent_output(),
                created_height: coin.created_height,
                created_median_time_past: coin.created_median_time_past,
            });
            continue;
        }

        let Some(parent_entry) = entries.get(&input.previous_output.txid) else {
            return Err(MempoolError::MissingInput {
                outpoint: input.previous_output.clone(),
            });
        };
        let output_index = input.previous_output.vout as usize;
        let Some(output) = parent_entry.transaction.outputs.get(output_index) else {
            return Err(MempoolError::MissingInput {
                outpoint: input.previous_output.clone(),
            });
        };

        input_contexts.push(TransactionInputContext {
            spent_output: open_bitcoin_consensus::SpentOutput {
                value: output.value,
                script_pubkey: output.script_pubkey.clone(),
                is_coinbase: false,
            },
            created_height: MEMPOOL_HEIGHT,
            created_median_time_past: mempool_median_time_past,
        });
    }

    Ok(input_contexts)
}

fn build_validation_context(
    chainstate: &ChainstateSnapshot,
    input_contexts: Vec<TransactionInputContext>,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> TransactionValidationContext {
    let maybe_tip = chainstate.tip();
    TransactionValidationContext {
        inputs: input_contexts,
        spend_height: maybe_tip.map_or(0, |tip| tip.height.saturating_add(1)),
        block_time: maybe_tip.map_or(0, |tip| i64::from(tip.header.time)),
        median_time_past: maybe_tip.map_or(0, |tip| tip.median_time_past),
        verify_flags,
        consensus_params,
    }
}

fn enforce_min_relay_fee(
    config: &PolicyConfig,
    fee_sats: i64,
    virtual_size: usize,
) -> Result<(), MempoolError> {
    let required_fee_sats = config.min_relay_feerate.fee_for_virtual_size(virtual_size);
    if fee_sats < required_fee_sats {
        let fee = open_bitcoin_primitives::Amount::from_sats(fee_sats)
            .expect("fee should be non-negative");
        return Err(MempoolError::RelayFeeTooLow {
            fee,
            required_fee_sats,
            virtual_size,
        });
    }

    Ok(())
}

fn validate_limits(
    entries: &HashMap<Txid, MempoolEntry>,
    config: &PolicyConfig,
    candidate_txid: Txid,
) -> Result<(), MempoolError> {
    let candidate_entry = entries
        .get(&candidate_txid)
        .expect("candidate should exist in prospective mempool state");
    if candidate_entry.ancestor_stats.count > config.max_ancestor_count {
        return Err(MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            txid: None,
            attempted: candidate_entry.ancestor_stats.count,
            max: config.max_ancestor_count,
        });
    }
    if candidate_entry.ancestor_stats.virtual_size > config.max_ancestor_virtual_size {
        return Err(MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::VirtualSize,
            txid: None,
            attempted: candidate_entry.ancestor_stats.virtual_size,
            max: config.max_ancestor_virtual_size,
        });
    }

    let mut candidate_ancestors = collect_ancestors(entries, candidate_txid);
    candidate_ancestors.insert(candidate_txid);
    for ancestor_txid in candidate_ancestors {
        let entry = entries
            .get(&ancestor_txid)
            .expect("ancestor should exist during descendant limit validation");
        if entry.descendant_stats.count > config.max_descendant_count {
            return Err(MempoolError::LimitExceeded {
                direction: LimitDirection::Descendant,
                kind: LimitKind::Count,
                txid: Some(ancestor_txid),
                attempted: entry.descendant_stats.count,
                max: config.max_descendant_count,
            });
        }
        if entry.descendant_stats.virtual_size > config.max_descendant_virtual_size {
            return Err(MempoolError::LimitExceeded {
                direction: LimitDirection::Descendant,
                kind: LimitKind::VirtualSize,
                txid: Some(ancestor_txid),
                attempted: entry.descendant_stats.virtual_size,
                max: config.max_descendant_virtual_size,
            });
        }
    }

    Ok(())
}

fn trim_to_size(mut state: MempoolState, config: &PolicyConfig) -> (MempoolState, BTreeSet<Txid>) {
    let mut evicted = BTreeSet::new();

    while state.total_virtual_size > config.max_mempool_virtual_size {
        let Some(victim_txid) = select_eviction_candidate(&state.entries) else {
            break;
        };
        let mut remove_set = collect_descendants(&state.entries, victim_txid);
        remove_set.insert(victim_txid);
        for txid in &remove_set {
            state.entries.remove(txid);
            evicted.insert(*txid);
        }
        state = recompute_state(state.entries);
    }

    (state, evicted)
}

fn select_eviction_candidate(entries: &HashMap<Txid, MempoolEntry>) -> Option<Txid> {
    let mut txids = entries.keys().copied().collect::<Vec<_>>();
    txids.sort_unstable();

    txids.into_iter().min_by(|left, right| {
        let left_entry = entries.get(left).expect("left entry should exist");
        let right_entry = entries.get(right).expect("right entry should exist");
        left_entry
            .descendant_score()
            .cmp(&right_entry.descendant_score())
            .then_with(|| left.cmp(right))
    })
}

fn recompute_state(mut entries: HashMap<Txid, MempoolEntry>) -> MempoolState {
    for entry in entries.values_mut() {
        entry.parents.clear();
        entry.children.clear();
        let stats = crate::AggregateStats::new(1, entry.virtual_size, entry.fee_sats());
        entry.ancestor_stats = stats;
        entry.descendant_stats = stats;
    }

    let txids = entries.keys().copied().collect::<Vec<_>>();
    let mut relations = Vec::new();
    for txid in &txids {
        let entry = entries
            .get(txid)
            .expect("txid should exist during relation scan");
        for input in &entry.transaction.inputs {
            let Some(parent_entry) = entries.get(&input.previous_output.txid) else {
                continue;
            };
            let output_index = input.previous_output.vout as usize;
            if output_index < parent_entry.transaction.outputs.len() {
                relations.push((input.previous_output.txid, *txid));
            }
        }
    }
    for (parent_txid, child_txid) in relations {
        if let Some(parent_entry) = entries.get_mut(&parent_txid) {
            parent_entry.children.insert(child_txid);
        }
        if let Some(child_entry) = entries.get_mut(&child_txid) {
            child_entry.parents.insert(parent_txid);
        }
    }

    let mut spent_outpoints = HashMap::new();
    for (txid, entry) in &entries {
        for input in &entry.transaction.inputs {
            spent_outpoints.insert(input.previous_output.clone(), *txid);
        }
    }

    let total_virtual_size = entries
        .values()
        .map(|entry| entry.virtual_size)
        .sum::<usize>();
    let txids = entries.keys().copied().collect::<Vec<_>>();
    for txid in &txids {
        let ancestors = collect_ancestors(&entries, *txid);
        let descendants = collect_descendants(&entries, *txid);
        let existing_entry = entries
            .get(txid)
            .expect("txid should exist during stats recompute");
        let ancestor_virtual_size = existing_entry.virtual_size
            + ancestors
                .iter()
                .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                .map(|ancestor| ancestor.virtual_size)
                .sum::<usize>();
        let ancestor_fee_sats = existing_entry.fee_sats()
            + ancestors
                .iter()
                .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                .map(MempoolEntry::fee_sats)
                .sum::<i64>();
        let descendant_virtual_size = existing_entry.virtual_size
            + descendants
                .iter()
                .filter_map(|descendant_txid| entries.get(descendant_txid))
                .map(|descendant| descendant.virtual_size)
                .sum::<usize>();
        let descendant_fee_sats = existing_entry.fee_sats()
            + descendants
                .iter()
                .filter_map(|descendant_txid| entries.get(descendant_txid))
                .map(MempoolEntry::fee_sats)
                .sum::<i64>();

        let entry = entries
            .get_mut(txid)
            .expect("txid should exist during stats update");
        entry.ancestor_stats = crate::AggregateStats::new(
            ancestors.len().saturating_add(1),
            ancestor_virtual_size,
            ancestor_fee_sats,
        );
        entry.descendant_stats = crate::AggregateStats::new(
            descendants.len().saturating_add(1),
            descendant_virtual_size,
            descendant_fee_sats,
        );
    }

    MempoolState {
        entries,
        spent_outpoints,
        total_virtual_size,
    }
}

fn collect_conflicts_and_descendants(
    entries: &HashMap<Txid, MempoolEntry>,
    direct_conflicts: &BTreeSet<Txid>,
) -> BTreeSet<Txid> {
    let mut txids = BTreeSet::new();
    for txid in direct_conflicts {
        txids.insert(*txid);
        txids.extend(collect_descendants(entries, *txid));
    }

    txids
}

fn collect_ancestors(entries: &HashMap<Txid, MempoolEntry>, txid: Txid) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.parents.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.parents.iter().copied());
        }
    }

    visited
}

fn collect_descendants(entries: &HashMap<Txid, MempoolEntry>, txid: Txid) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.children.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.children.iter().copied());
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use open_bitcoin_chainstate::{Chainstate, ChainstateSnapshot};
    use std::collections::{BTreeSet, HashMap};

    use open_bitcoin_consensus::crypto::hash160;
    use open_bitcoin_consensus::{
        ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
    };
    use open_bitcoin_primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    };

    use super::Mempool;
    use crate::{LimitDirection, LimitKind, MempoolEntry, MempoolError, PolicyConfig, RbfPolicy};

    const EASY_BITS: u32 = 0x207f_ffff;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn redeem_script() -> ScriptBuf {
        script(&[0x51])
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = hash160(redeem_script().as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    }

    fn serialized_script_num(value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![0x00];
        }

        let mut magnitude = value as u64;
        let mut encoded = Vec::new();
        while magnitude > 0 {
            encoded.push((magnitude & 0xff) as u8);
            magnitude >>= 8;
        }

        let mut script = Vec::with_capacity(encoded.len() + 2);
        script.push(encoded.len() as u8);
        script.extend(encoded);
        script.push(0x51);
        script
    }

    fn coinbase_transaction(height: u32, value: i64) -> Transaction {
        let mut script_sig = serialized_script_num(i64::from(height));
        script_sig.push(0x51);
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&script_sig),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(
        previous_txid: Txid,
        vout: u32,
        output_value: i64,
        sequence: u32,
    ) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(output_value).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn non_standard_spend(previous_txid: Txid) -> Transaction {
        let mut transaction = spend_transaction(
            previous_txid,
            0,
            499_000_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        transaction.outputs[0].script_pubkey = script(&[0x51]);
        transaction
    }

    fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
        let transactions = vec![coinbase_transaction(height, value)];
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash,
                merkle_root,
                time: 1_231_006_500 + height,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);
        block
    }

    fn mine_header(block: &mut Block) {
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("expected nonce at easy target");
    }

    fn sample_chainstate_snapshot(block_count: u32) -> (ChainstateSnapshot, Vec<Txid>) {
        let mut chainstate = Chainstate::new();
        let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
        let mut txids = Vec::new();

        for height in 0..block_count {
            let block = build_block(previous_hash, height, 500_000_000);
            let txid =
                open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid");
            txids.push(txid);
            chainstate
                .connect_block(
                    &block,
                    u128::from(height + 1),
                    ScriptVerifyFlags::P2SH
                        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                    ConsensusParams {
                        coinbase_maturity: 1,
                        ..ConsensusParams::default()
                    },
                )
                .expect("block should connect");
            previous_hash = open_bitcoin_consensus::block_hash(&block.header);
        }

        (chainstate.snapshot(), txids)
    }

    fn submit(
        mempool: &mut Mempool,
        snapshot: &ChainstateSnapshot,
        transaction: Transaction,
    ) -> Result<crate::AdmissionResult, MempoolError> {
        mempool.accept_transaction(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
    }

    #[test]
    fn accepts_standard_confirmed_spend() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let transaction = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::default();

        let result = submit(&mut mempool, &snapshot, transaction).expect("admission");
        let entry = mempool.entry(&result.accepted).expect("entry");

        assert!(result.replaced.is_empty());
        assert!(result.evicted.is_empty());
        assert_eq!(entry.ancestor_stats.count, 1);
    }

    #[test]
    fn getters_expose_config_entries_and_total_virtual_size() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let mut mempool = Mempool::default();
        let result = submit(
            &mut mempool,
            &snapshot,
            spend_transaction(
                coinbase_txids[0],
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
        )
        .expect("admission");

        assert_eq!(mempool.config().rbf_policy, RbfPolicy::Always);
        assert_eq!(mempool.entries().len(), 1);
        assert_eq!(
            mempool.total_virtual_size(),
            mempool.entry(&result.accepted).expect("entry").virtual_size
        );
    }

    #[test]
    fn duplicate_transactions_and_missing_inputs_are_rejected() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let transaction = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::default();
        submit(&mut mempool, &snapshot, transaction.clone()).expect("first admission");

        let duplicate = submit(&mut mempool, &snapshot, transaction).expect_err("duplicate");
        assert!(matches!(
            duplicate,
            MempoolError::DuplicateTransaction { .. }
        ));

        let missing = submit(
            &mut Mempool::default(),
            &snapshot,
            spend_transaction(
                Txid::from_byte_array([8_u8; 32]),
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
        )
        .expect_err("missing input");
        assert!(matches!(missing, MempoolError::MissingInput { .. }));
    }

    #[test]
    fn rejects_non_standard_output_scripts() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let error = submit(
            &mut Mempool::default(),
            &snapshot,
            non_standard_spend(coinbase_txids[0]),
        )
        .expect_err("non-standard output should fail");

        assert!(matches!(error, MempoolError::NonStandard { .. }));
    }

    #[test]
    fn rejects_entries_that_exceed_ancestor_limits() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let parent = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
        let child = spend_transaction(
            parent_txid,
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            max_ancestor_count: 1,
            ..PolicyConfig::default()
        });

        submit(&mut mempool, &snapshot, parent).expect("parent");
        let error = submit(&mut mempool, &snapshot, child).expect_err("limit should fail");

        assert!(matches!(
            error,
            MempoolError::LimitExceeded {
                direction: LimitDirection::Ancestor,
                kind: LimitKind::Count,
                ..
            }
        ));
    }

    #[test]
    fn tracks_parent_child_and_ancestor_descendant_metrics() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let parent = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
        let child = spend_transaction(
            parent_txid,
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::default();

        submit(&mut mempool, &snapshot, parent).expect("parent");
        let child_result = submit(&mut mempool, &snapshot, child).expect("child");
        let child_entry = mempool.entry(&child_result.accepted).expect("child entry");
        let parent_entry = mempool.entry(&parent_txid).expect("parent entry");

        assert_eq!(child_entry.parents, BTreeSet::from([parent_txid]));
        assert_eq!(child_entry.ancestor_stats.count, 2);
        assert_eq!(
            parent_entry.children,
            BTreeSet::from([child_result.accepted])
        );
        assert_eq!(parent_entry.descendant_stats.count, 2);
    }

    #[test]
    fn replacement_requires_a_fee_bump() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
        let original = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
        );
        let lower_fee_replacement = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_100,
            TransactionInput::SEQUENCE_FINAL,
        );
        let higher_fee_replacement = spend_transaction(
            coinbase_txids[0],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            rbf_policy: RbfPolicy::OptIn,
            ..PolicyConfig::default()
        });

        let original_result = submit(&mut mempool, &snapshot, original).expect("original");
        let lower_error = submit(&mut mempool, &snapshot, lower_fee_replacement)
            .expect_err("lower fee replacement should fail");
        let higher_result =
            submit(&mut mempool, &snapshot, higher_fee_replacement).expect("replacement");

        assert!(matches!(
            lower_error,
            MempoolError::ReplacementRejected { .. }
        ));
        assert_eq!(higher_result.replaced, vec![original_result.accepted]);
    }

    #[test]
    fn replacement_requires_opt_in_signal_when_policy_demands_it() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let original = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let replacement = spend_transaction(
            coinbase_txids[0],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            rbf_policy: RbfPolicy::OptIn,
            ..PolicyConfig::default()
        });

        submit(&mut mempool, &snapshot, original).expect("original");
        let error = submit(&mut mempool, &snapshot, replacement).expect_err("opt-in required");

        assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
    }

    #[test]
    fn replacement_rejects_new_unconfirmed_inputs() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
        let parent = spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
        let original = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
        );
        let original_txid = open_bitcoin_consensus::transaction_txid(&original).expect("txid");
        let mut replacement = spend_transaction(
            coinbase_txids[0],
            0,
            499_997_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        replacement.inputs.push(TransactionInput {
            previous_output: OutPoint {
                txid: parent_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
        let mut mempool = Mempool::new(PolicyConfig {
            rbf_policy: RbfPolicy::Always,
            ..PolicyConfig::default()
        });

        submit(&mut mempool, &snapshot, parent).expect("parent");
        submit(&mut mempool, &snapshot, original).expect("original");
        assert!(mempool.entry(&original_txid).is_some());

        let error = submit(&mut mempool, &snapshot, replacement)
            .expect_err("replacement with new unconfirmed input should fail");

        assert!(matches!(error, MempoolError::ReplacementRejected { .. }));
    }

    #[test]
    fn evicts_lowest_descendant_score_package_when_size_limit_is_exceeded() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
        let low_fee = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_200,
            TransactionInput::SEQUENCE_FINAL,
        );
        let high_fee = spend_transaction(
            coinbase_txids[1],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            max_mempool_virtual_size: 140,
            ..PolicyConfig::default()
        });

        let low_fee_result = submit(&mut mempool, &snapshot, low_fee).expect("low fee");
        let high_fee_result = submit(&mut mempool, &snapshot, high_fee).expect("high fee");

        assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
        assert!(mempool.entry(&low_fee_result.accepted).is_none());
        assert!(mempool.entry(&high_fee_result.accepted).is_some());
    }

    #[test]
    fn replacements_respect_disabled_policy() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let original = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let replacement = spend_transaction(
            coinbase_txids[0],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            rbf_policy: RbfPolicy::Never,
            ..PolicyConfig::default()
        });

        submit(&mut mempool, &snapshot, original).expect("original");
        let error = submit(&mut mempool, &snapshot, replacement).expect_err("rbf disabled");

        assert!(matches!(error, MempoolError::ConflictNotAllowed { .. }));
    }

    #[test]
    fn direct_helper_paths_cover_internal_edge_branches() {
        let empty_snapshot = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());
        let context = super::build_validation_context(
            &empty_snapshot,
            Vec::new(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        );
        assert_eq!(context.spend_height, 0);
        assert_eq!(context.block_time, 0);

        let relay_error = super::enforce_min_relay_fee(&PolicyConfig::default(), 0, 100)
            .expect_err("fee floor should fail");
        assert!(matches!(relay_error, MempoolError::RelayFeeTooLow { .. }));

        let candidate_txid = Txid::from_byte_array([7_u8; 32]);
        let candidate_transaction = spend_transaction(
            Txid::from_byte_array([6_u8; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let candidate_entry = MempoolEntry::new(
            candidate_transaction.clone(),
            candidate_txid,
            open_bitcoin_consensus::transaction_wtxid(&candidate_transaction).expect("wtxid"),
            Amount::from_sats(100).expect("amount"),
            100,
            400,
            0,
        );
        let missing_candidate = super::validate_limits(
            &HashMap::from([(candidate_txid, candidate_entry)]),
            &PolicyConfig {
                max_ancestor_count: 0,
                ..PolicyConfig::default()
            },
            candidate_txid,
        );
        assert!(missing_candidate.is_err());

        assert!(super::select_eviction_candidate(&HashMap::new()).is_none());
        let missing_ancestors =
            super::collect_ancestors(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
        let missing_descendants =
            super::collect_descendants(&HashMap::new(), Txid::from_byte_array([1_u8; 32]));
        assert!(missing_ancestors.is_empty());
        assert!(missing_descendants.is_empty());
    }

    #[test]
    fn admission_maps_validation_errors_and_replacement_policy_edges() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
        let invalid = spend_transaction(
            coinbase_txids[0],
            0,
            500_000_001,
            TransactionInput::SEQUENCE_FINAL,
        );
        let validation_error =
            submit(&mut Mempool::default(), &snapshot, invalid).expect_err("invalid spend");
        assert!(matches!(validation_error, MempoolError::Validation { .. }));

        let original = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
        );
        let mut mempool = Mempool::new(PolicyConfig {
            incremental_relay_feerate: crate::FeeRate::from_sats_per_kvb(10_000),
            ..PolicyConfig::default()
        });
        submit(&mut mempool, &snapshot, original).expect("original");
        let conflict_txid = *mempool.entries().keys().next().expect("conflict txid");

        let low_feerate_error = mempool
            .validate_replacement(
                &spend_transaction(
                    coinbase_txids[0],
                    0,
                    499_998_000,
                    TransactionInput::SEQUENCE_FINAL,
                ),
                &BTreeSet::from([conflict_txid]),
                2_000,
                2_000,
            )
            .expect_err("feerate should fail");
        assert!(matches!(
            low_feerate_error,
            MempoolError::ReplacementRejected { .. }
        ));

        let incremental_error = mempool
            .validate_replacement(
                &spend_transaction(
                    coinbase_txids[0],
                    0,
                    499_998_000,
                    TransactionInput::SEQUENCE_FINAL,
                ),
                &BTreeSet::from([conflict_txid]),
                1_001,
                10,
            )
            .expect_err("incremental relay bump should fail");
        assert!(matches!(
            incremental_error,
            MempoolError::ReplacementRejected { .. }
        ));

        let stale_conflict = mempool.validate_replacement(
            &spend_transaction(
                coinbase_txids[0],
                0,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &BTreeSet::from([Txid::from_byte_array([42_u8; 32])]),
            2_000,
            100,
        );
        assert!(stale_conflict.is_ok());
    }

    #[test]
    fn helper_functions_cover_missing_vout_and_limit_branches() {
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let parent = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let parent_txid = open_bitcoin_consensus::transaction_txid(&parent).expect("txid");
        let parent_wtxid = open_bitcoin_consensus::transaction_wtxid(&parent).expect("wtxid");
        let entries = HashMap::from([(
            parent_txid,
            MempoolEntry::new(
                parent,
                parent_txid,
                parent_wtxid,
                Amount::from_sats(1000).expect("amount"),
                100,
                400,
                0,
            ),
        )]);
        let missing_vout = super::derive_input_contexts(
            &spend_transaction(
                parent_txid,
                9,
                499_998_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &snapshot,
            &entries,
        )
        .expect_err("missing vout should fail");
        assert!(matches!(missing_vout, MempoolError::MissingInput { .. }));

        let candidate_txid = Txid::from_byte_array([11_u8; 32]);
        let candidate = MempoolEntry::new(
            spend_transaction(
                coinbase_txids[1],
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            candidate_txid,
            open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
                coinbase_txids[1],
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ))
            .expect("wtxid"),
            Amount::from_sats(1000).expect("amount"),
            100,
            400,
            0,
        );
        let mut descendant_parent = candidate.clone();
        descendant_parent.descendant_stats = crate::AggregateStats::new(2, 200, 2_000);
        let oversized_ancestor = super::validate_limits(
            &HashMap::from([(candidate_txid, candidate.clone())]),
            &PolicyConfig {
                max_ancestor_virtual_size: 50,
                ..PolicyConfig::default()
            },
            candidate_txid,
        )
        .expect_err("ancestor vsize should fail");
        assert!(matches!(
            oversized_ancestor,
            MempoolError::LimitExceeded { .. }
        ));

        let descendant_limit = super::validate_limits(
            &HashMap::from([(candidate_txid, descendant_parent)]),
            &PolicyConfig {
                max_descendant_count: 1,
                ..PolicyConfig::default()
            },
            candidate_txid,
        )
        .expect_err("descendant count should fail");
        assert!(matches!(
            descendant_limit,
            MempoolError::LimitExceeded { .. }
        ));

        let mut descendant_size_parent = candidate;
        descendant_size_parent.descendant_stats = crate::AggregateStats::new(1, 200, 1_000);
        let descendant_size = super::validate_limits(
            &HashMap::from([(candidate_txid, descendant_size_parent)]),
            &PolicyConfig {
                max_descendant_virtual_size: 50,
                ..PolicyConfig::default()
            },
            candidate_txid,
        )
        .expect_err("descendant size should fail");
        assert!(matches!(
            descendant_size,
            MempoolError::LimitExceeded { .. }
        ));
    }

    #[test]
    fn trim_and_graph_helpers_cover_remaining_internal_branches() {
        let empty_trimmed = super::trim_to_size(
            super::MempoolState {
                entries: HashMap::new(),
                spent_outpoints: HashMap::new(),
                total_virtual_size: 1,
            },
            &PolicyConfig {
                max_mempool_virtual_size: 0,
                ..PolicyConfig::default()
            },
        );
        assert!(empty_trimmed.1.is_empty());

        let base = spend_transaction(
            Txid::from_byte_array([1_u8; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let base_txid = open_bitcoin_consensus::transaction_txid(&base).expect("txid");
        let base_wtxid = open_bitcoin_consensus::transaction_wtxid(&base).expect("wtxid");
        let left = spend_transaction(base_txid, 0, 499_998_000, TransactionInput::SEQUENCE_FINAL);
        let left_txid = open_bitcoin_consensus::transaction_txid(&left).expect("txid");
        let left_wtxid = open_bitcoin_consensus::transaction_wtxid(&left).expect("wtxid");
        let right = spend_transaction(base_txid, 0, 499_997_000, TransactionInput::SEQUENCE_FINAL);
        let right_txid = open_bitcoin_consensus::transaction_txid(&right).expect("txid");
        let right_wtxid = open_bitcoin_consensus::transaction_wtxid(&right).expect("wtxid");
        let mut leaf =
            spend_transaction(left_txid, 0, 499_996_000, TransactionInput::SEQUENCE_FINAL);
        leaf.inputs.push(TransactionInput {
            previous_output: OutPoint {
                txid: right_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
        let leaf_txid = open_bitcoin_consensus::transaction_txid(&leaf).expect("txid");
        let leaf_wtxid = open_bitcoin_consensus::transaction_wtxid(&leaf).expect("wtxid");

        let entries = HashMap::from([
            (
                base_txid,
                MempoolEntry::new(
                    base,
                    base_txid,
                    base_wtxid,
                    Amount::from_sats(1000).expect("amount"),
                    100,
                    400,
                    0,
                ),
            ),
            (
                left_txid,
                MempoolEntry::new(
                    left,
                    left_txid,
                    left_wtxid,
                    Amount::from_sats(1000).expect("amount"),
                    100,
                    400,
                    0,
                ),
            ),
            (
                right_txid,
                MempoolEntry::new(
                    right,
                    right_txid,
                    right_wtxid,
                    Amount::from_sats(1000).expect("amount"),
                    100,
                    400,
                    0,
                ),
            ),
            (
                leaf_txid,
                MempoolEntry::new(
                    leaf,
                    leaf_txid,
                    leaf_wtxid,
                    Amount::from_sats(1000).expect("amount"),
                    100,
                    400,
                    0,
                ),
            ),
        ]);
        let recomputed = super::recompute_state(entries);
        let ancestors = super::collect_ancestors(&recomputed.entries, leaf_txid);
        let descendants = super::collect_descendants(&recomputed.entries, base_txid);
        assert!(ancestors.contains(&base_txid));
        assert!(descendants.contains(&leaf_txid));
    }

    #[test]
    fn recompute_state_skips_invalid_parent_links_and_candidate_eviction_is_reported() {
        let txid = Txid::from_byte_array([4_u8; 32]);
        let invalid_parent = MempoolEntry::new(
            spend_transaction(
                Txid::from_byte_array([1_u8; 32]),
                1,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            txid,
            open_bitcoin_consensus::transaction_wtxid(&spend_transaction(
                Txid::from_byte_array([1_u8; 32]),
                1,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ))
            .expect("wtxid"),
            Amount::from_sats(100).expect("amount"),
            100,
            400,
            0,
        );
        let state = super::recompute_state(HashMap::from([(txid, invalid_parent)]));
        assert!(state.entries.get(&txid).expect("entry").parents.is_empty());

        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let mut mempool = Mempool::new(PolicyConfig {
            max_mempool_virtual_size: 1,
            ..PolicyConfig::default()
        });
        let error = submit(
            &mut mempool,
            &snapshot,
            spend_transaction(
                coinbase_txids[0],
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
        )
        .expect_err("tiny mempool should evict candidate");

        assert!(matches!(error, MempoolError::CandidateEvicted { .. }));
    }
}
