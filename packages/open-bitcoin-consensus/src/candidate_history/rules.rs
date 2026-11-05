// Parity breadcrumbs:
// - packages/bitcoin-knots/src/deploymentstatus.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::{Bip34Context, CandidateHistory, HistoryOrigin};
use crate::chain_spec::NetworkRules;
use crate::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use crate::{BlockValidationContext, ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::BlockHash;

/// Exact duplicate-outpoint admission decision; it never authorizes arbitrary overwrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bip30Disposition {
    ScanRequired,
    HistoricalRepeat,
    Bip34Commitment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuleFacts {
    network: NetworkRules,
    height: u32,
    candidate_hash: BlockHash,
    parent_hash: BlockHash,
    parent_mtp: i64,
    candidate_time: i64,
    origin: HistoryOrigin,
    bip34: Bip34Context,
}

/// One immutable rule result bound to structural candidate/parent identity.
/// This preserves the history evidence class; it grants no canonical owner authority.
/// ```compile_fail
/// use open_bitcoin_consensus::{CandidateRules, ScriptVerifyFlags};
/// fn forge(rules: &mut CandidateRules) { rules.flags = ScriptVerifyFlags::NONE; }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateRules {
    facts: RuleFacts,
    flags: ScriptVerifyFlags,
    maybe_minimum_version: Option<i32>,
    bip30: Bip30Disposition,
}

impl CandidateRules {
    /// Derive only from a checked structural history, retaining its origin classification.
    pub fn derive(history: &CandidateHistory) -> Self {
        Self::from_facts(RuleFacts {
            network: history.network_rules(),
            height: history.height(),
            candidate_hash: history.candidate_hash(),
            parent_hash: history.parent().block_hash,
            parent_mtp: history.parent_median_time_past(),
            candidate_time: i64::from(history.candidate().time),
            origin: history.origin(),
            bip34: history.bip34_context(),
        })
    }

    fn from_facts(facts: RuleFacts) -> Self {
        let network = facts.network;
        let height = facts.height;
        let mut flags = network
            .maybe_script_flag_exception(facts.candidate_hash)
            .unwrap_or(
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            );
        for (activation, addition) in [
            (network.bip66_height(), ScriptVerifyFlags::DERSIG),
            (
                network.bip65_height(),
                ScriptVerifyFlags::CHECKLOCKTIMEVERIFY,
            ),
            (network.csv_height(), ScriptVerifyFlags::CHECKSEQUENCEVERIFY),
            (network.segwit_height(), ScriptVerifyFlags::NULLDUMMY),
        ] {
            if height >= activation {
                flags |= addition;
            }
        }
        let maybe_minimum_version = if height >= network.bip65_height() {
            Some(4)
        } else if height >= network.bip66_height() {
            Some(3)
        } else if height >= network.bip34_height() {
            Some(2)
        } else {
            None
        };
        Self {
            facts,
            flags,
            maybe_minimum_version,
            bip30: bip30_disposition(facts),
        }
    }

    /// Selected source facts, without independent scalar configuration.
    pub const fn network_rules(&self) -> NetworkRules {
        self.facts.network
    }
    /// Exact computed candidate identity.
    pub const fn candidate_hash(&self) -> BlockHash {
        self.facts.candidate_hash
    }
    /// Exact parent to which external owner/generation binding must agree.
    pub const fn parent_hash(&self) -> BlockHash {
        self.facts.parent_hash
    }
    /// Checked parent+1 height.
    pub const fn height(&self) -> u32 {
        self.facts.height
    }
    /// Structural or conditional evidence class, never canonical acceptance evidence.
    pub const fn origin(&self) -> HistoryOrigin {
        self.facts.origin
    }
    /// Explicit BIP34 ancestry state, including independent seed taint.
    pub const fn bip34_context(&self) -> Bip34Context {
        self.facts.bip34
    }
    /// Consensus flags after exception replacement and deployment additions.
    pub const fn script_flags(&self) -> ScriptVerifyFlags {
        self.flags
    }
    /// None means no buried-deployment version floor, including no implicit version1 floor.
    pub const fn maybe_minimum_block_version(&self) -> Option<i32> {
        self.maybe_minimum_version
    }
    /// Whether the candidate version meets every active buried-deployment floor.
    pub fn accepts_block_version(&self, version: i32) -> bool {
        self.maybe_minimum_version
            .is_none_or(|minimum| version >= minimum)
    }
    /// BIP34 coinbase-height prefix activation.
    pub fn enforce_bip34_height(&self) -> bool {
        self.facts.height >= self.facts.network.bip34_height()
    }
    /// CSV activation gates both BIP68 sequence locks and BIP113 finality.
    pub fn enforce_sequence_locks(&self) -> bool {
        self.facts.height >= self.facts.network.csv_height()
    }
    /// Commitment activation is independent of the WITNESS script flag.
    pub fn enforce_segwit_commitment(&self) -> bool {
        self.facts.height >= self.facts.network.segwit_height()
    }
    /// Parent MTP after CSV, otherwise this exact candidate's timestamp.
    pub fn finality_cutoff(&self) -> i64 {
        if self.enforce_sequence_locks() {
            self.facts.parent_mtp
        } else {
            self.facts.candidate_time
        }
    }
    /// Pinned BIP30 scan/exception disposition for this candidate and ancestry.
    pub const fn bip30_disposition(&self) -> Bip30Disposition {
        self.bip30
    }
    /// Whether every transaction output must be checked for a live prior coin.
    pub fn requires_bip30_scan(&self) -> bool {
        self.bip30 == Bip30Disposition::ScanRequired
    }
    /// Complete existing validation carrier; a copied carrier is not owner authority.
    pub fn consensus_params(&self) -> ConsensusParams {
        let network = self.facts.network;
        ConsensusParams {
            coinbase_maturity: network.coinbase_maturity(),
            subsidy_halving_interval: network.subsidy_halving_interval(),
            locktime_threshold: network.locktime_threshold(),
            sequence_locktime_granularity: network.sequence_locktime_granularity(),
            pow_limit_bits: network.pow_limit_bits(),
            pow_target_spacing_seconds: network.pow_target_spacing_seconds(),
            pow_target_timespan_seconds: network.pow_target_timespan_seconds(),
            allow_min_difficulty_blocks: network.allow_min_difficulty_blocks(),
            no_pow_retargeting: network.no_pow_retargeting(),
            enforce_bip34_height_in_coinbase: self.enforce_bip34_height(),
            enforce_bip113_median_time_past: self.enforce_sequence_locks(),
            enforce_segwit: self.enforce_segwit_commitment(),
        }
    }
}

fn bip30_disposition(facts: RuleFacts) -> Bip30Disposition {
    if facts.height >= facts.network.bip30_recheck_height() {
        return Bip30Disposition::ScanRequired;
    }
    if facts
        .network
        .bip30_repeat_pairs()
        .contains(&(facts.height, facts.candidate_hash))
    {
        return Bip30Disposition::HistoricalRepeat;
    }
    let maybe_ancestor_hash = match facts.bip34 {
        Bip34Context::Linked(hash) | Bip34Context::IndependentlySeeded(hash) => Some(hash),
        _ => None,
    };
    if let Some(expected_hash) = facts.network.maybe_bip34_hash()
        && maybe_ancestor_hash == Some(expected_hash)
    {
        return Bip30Disposition::Bip34Commitment;
    }
    Bip30Disposition::ScanRequired
}

impl CandidateHistory {
    /// Shared rule result for this exact structural context.
    pub fn rules(&self) -> CandidateRules {
        CandidateRules::derive(self)
    }
    /// Existing contextual carrier with exact linked difficulty and MTP facts.
    /// Shell-observed current_time is supplied explicitly and confers no admission seal.
    pub fn block_validation_context(&self, current_time: i64) -> BlockValidationContext {
        BlockValidationContext {
            height: self.height(),
            previous_header: self.parent().header.clone(),
            maybe_retarget_anchor: self.maybe_retarget_anchor().map(|row| RetargetAnchor {
                first_block_time: i64::from(row.header.time),
            }),
            maybe_min_difficulty_recovery_target: self.maybe_min_difficulty_recovery().map(|row| {
                MinDifficultyRecoveryTarget {
                    bits: row.header.bits,
                }
            }),
            previous_median_time_past: self.parent_median_time_past(),
            current_time,
            consensus_params: self.rules().consensus_params(),
        }
    }
}

#[cfg(test)]
mod tests;
