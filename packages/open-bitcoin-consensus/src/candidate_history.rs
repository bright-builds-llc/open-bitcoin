// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Checked structural branch facts. No value in this module proves canonical acceptance.

use crate::block_hash;
use crate::chain_spec::{CanonicalChain, NetworkRules};
use open_bitcoin_primitives::{BlockHash, BlockHeader};
use std::collections::BTreeSet;

pub mod header_rules;
mod rules;
pub use rules::{Bip30Disposition, CandidateRules};

/// Untrusted branch row; even its claimed identity is checked against the header bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryHeader {
    pub chain: CanonicalChain,
    pub height: u32,
    pub block_hash: BlockHash,
    pub header: BlockHeader,
}

impl HistoryHeader {
    /// Record the identity computed from a header; height/network remain unproved inputs.
    pub fn new(chain: CanonicalChain, height: u32, header: BlockHeader) -> Self {
        Self {
            chain,
            height,
            block_hash: block_hash(&header),
            header,
        }
    }
}

/// Structural coverage classification, never earned lineage or product qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryOrigin {
    /// Supplied links terminate at the selected genesis. Headers may still be invalid.
    GenesisLinked,
    /// Checked contiguous suffix above genesis; an owner must prove its earned origin.
    CheckedSuffix {
        oldest_height: u32,
        oldest_hash: BlockHash,
    },
    /// Independent historical anchor supplied without proof of its branch membership.
    IndependentlySeeded,
}

/// BIP34 branch knowledge. Unknown ancestry is never encoded as absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bip34Context {
    BeforeActivation,
    NoSelectedCommitment,
    UnneededAtRecheck,
    Linked(BlockHash),
    /// Conditional historical fact; it cannot establish canonical branch membership.
    IndependentlySeeded(BlockHash),
}

/// Missing or inconsistent supplied ancestry; no shorter-window fallback is permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryError {
    HeightOverflow,
    WrongNetwork { height: u32 },
    HashMismatch { height: u32 },
    ParentMismatch { height: u32 },
    HeightMismatch { expected: u32, actual: u32 },
    RepeatedHash { height: u32 },
    WrongGenesis,
    MissingAncestor { height: u32 },
    UnexpectedBip34Seed,
}

impl core::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "candidate history: {self:?}")
    }
}
impl std::error::Error for HistoryError {}

/// Immutable structural context bound to exact candidate bytes and selected rules.
/// Owners must independently establish the parent in their sealed accepted lineage.
/// ```compile_fail
/// use open_bitcoin_consensus::CandidateHistory;
/// fn forge(history: &mut CandidateHistory) { history.height = 1; }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateHistory {
    network_rules: NetworkRules,
    candidate: BlockHeader,
    candidate_hash: BlockHash,
    height: u32,
    branch: Vec<HistoryHeader>,
    origin: HistoryOrigin,
    parent_mtp: i64,
    expiry_mtp: i64,
    maybe_retarget_index: Option<usize>,
    maybe_recovery_index: Option<usize>,
    bip34_context: Bip34Context,
}

impl CandidateHistory {
    /// Check a parent and its contiguous ancestors in descending height order.
    /// No PoW, contextual validity, coins ownership or origin authority is established.
    pub fn check(
        network_rules: NetworkRules,
        candidate: BlockHeader,
        parent: HistoryHeader,
        ancestors: Vec<HistoryHeader>,
    ) -> Result<Self, HistoryError> {
        let mut history = Self::check_windows(network_rules, candidate, parent, ancestors)?;
        history.bip34_context = history.linked_bip34_context()?;
        Ok(history)
    }

    /// Check bounded branch windows with an independent historical BIP34 seed.
    /// The seed's bytes/network/height are checked, but branch membership is unproved.
    /// This constructor ALWAYS returns IndependentlySeeded, even for genesis-linked rows.
    /// Canonical network/coins owners must reject this classification for product authority.
    pub fn check_seeded(
        network_rules: NetworkRules,
        candidate: BlockHeader,
        parent: HistoryHeader,
        ancestors: Vec<HistoryHeader>,
        maybe_bip34_seed: Option<HistoryHeader>,
    ) -> Result<Self, HistoryError> {
        let mut history = Self::check_windows(network_rules, candidate, parent, ancestors)?;
        history.origin = HistoryOrigin::IndependentlySeeded;
        history.bip34_context = history.seeded_bip34_context(maybe_bip34_seed)?;
        Ok(history)
    }

    fn check_windows(
        network_rules: NetworkRules,
        candidate: BlockHeader,
        parent: HistoryHeader,
        ancestors: Vec<HistoryHeader>,
    ) -> Result<Self, HistoryError> {
        let height = parent
            .height
            .checked_add(1)
            .ok_or(HistoryError::HeightOverflow)?;
        let mut branch = vec![parent];
        branch.extend(ancestors);
        let candidate_hash = block_hash(&candidate);
        let origin = check_branch(network_rules, &candidate, candidate_hash, &branch)?;
        let mut history = Self {
            network_rules,
            candidate_hash,
            candidate,
            height,
            branch,
            origin,
            parent_mtp: 0,
            expiry_mtp: 0,
            maybe_retarget_index: None,
            maybe_recovery_index: None,
            bip34_context: Bip34Context::BeforeActivation,
        };
        history.check_required_windows()?;
        Ok(history)
    }

    /// Selected immutable source facts.
    pub const fn network_rules(&self) -> NetworkRules {
        self.network_rules
    }
    /// Exact candidate header.
    pub const fn candidate(&self) -> &BlockHeader {
        &self.candidate
    }
    /// Candidate identity computed from the header, never supplied separately.
    pub const fn candidate_hash(&self) -> BlockHash {
        self.candidate_hash
    }
    /// Candidate height derived by checked parent increment.
    pub const fn height(&self) -> u32 {
        self.height
    }
    /// Checked parent followed by its linked ancestors, newest first.
    pub fn branch(&self) -> &[HistoryHeader] {
        &self.branch
    }
    /// The exact parent to which external lineage owners must bind.
    pub fn parent(&self) -> &HistoryHeader {
        &self.branch[0]
    }
    /// Structural coverage, not accepted history evidence.
    pub const fn origin(&self) -> HistoryOrigin {
        self.origin
    }
    /// Upper-median timestamp with the complete min(11,height+1) window.
    pub fn median_time_past_at(&self, height: u32) -> Result<i64, HistoryError> {
        let index = self.index_at(height)?;
        let count = (height.min(10) + 1) as usize;
        let rows = self
            .branch
            .get(index..index + count)
            .ok_or(HistoryError::MissingAncestor {
                height: height.saturating_sub(10),
            })?;
        let mut times: Vec<_> = rows.iter().map(|row| i64::from(row.header.time)).collect();
        times.sort_unstable();
        Ok(times[count / 2])
    }
    /// Parent MTP used by contextual header and BIP113 checks.
    pub const fn parent_median_time_past(&self) -> i64 {
        self.parent_mtp
    }
    /// MTP of parent minus min(144,parent.height), used by software expiry.
    pub const fn expiry_median_time_past(&self) -> i64 {
        self.expiry_mtp
    }
    /// Required boundary anchor, if this candidate ends a difficulty interval.
    pub fn maybe_retarget_anchor(&self) -> Option<&HistoryHeader> {
        self.maybe_retarget_index
            .and_then(|index| self.branch.get(index))
    }
    /// Last nonminimum-difficulty header or period boundary on this branch.
    pub fn maybe_min_difficulty_recovery(&self) -> Option<&HistoryHeader> {
        self.maybe_recovery_index
            .and_then(|index| self.branch.get(index))
    }
    /// Linked BIP34 ancestor only. Use bip34_context() to distinguish other states.
    pub fn maybe_bip34_ancestor(&self) -> Option<&HistoryHeader> {
        if !matches!(self.bip34_context, Bip34Context::Linked(_)) {
            return None;
        }
        self.branch
            .iter()
            .find(|row| row.height == self.network_rules.bip34_height())
    }
    /// Explicit linked/conditional/unneeded/absent state; no unknown-to-absent fallback.
    pub const fn bip34_context(&self) -> Bip34Context {
        self.bip34_context
    }

    fn index_at(&self, height: u32) -> Result<usize, HistoryError> {
        let offset = self
            .parent()
            .height
            .checked_sub(height)
            .ok_or(HistoryError::MissingAncestor { height })? as usize;
        if self.branch.get(offset).is_none() {
            return Err(HistoryError::MissingAncestor { height });
        }
        Ok(offset)
    }

    fn check_required_windows(&mut self) -> Result<(), HistoryError> {
        let parent = self.parent();
        let parent_height = parent.height;
        let timely = i64::from(self.candidate.time)
            <= i64::from(parent.header.time) + self.network_rules.pow_target_spacing_seconds() * 2;
        self.parent_mtp = self.median_time_past_at(parent_height)?;
        self.expiry_mtp = self.median_time_past_at(parent_height.saturating_sub(144))?;
        let interval = self.network_rules.difficulty_adjustment_interval();
        if self.height.is_multiple_of(interval) {
            self.maybe_retarget_index = Some(self.index_at(self.height - interval)?);
        } else if self.network_rules.allow_min_difficulty_blocks() && timely {
            self.maybe_recovery_index = Some(
                self.branch
                    .iter()
                    .position(|row| {
                        row.height.is_multiple_of(interval)
                            || row.header.bits != self.network_rules.pow_limit_bits()
                    })
                    .ok_or(HistoryError::MissingAncestor {
                        height: parent_height.saturating_sub(self.branch.len() as u32),
                    })?,
            );
        }
        Ok(())
    }

    fn maybe_unneeded_bip34(&self) -> Option<Bip34Context> {
        if self.parent().height < self.network_rules.bip34_height() {
            return Some(Bip34Context::BeforeActivation);
        }
        if self.network_rules.maybe_bip34_hash().is_none() {
            return Some(Bip34Context::NoSelectedCommitment);
        }
        if self.height >= self.network_rules.bip30_recheck_height() {
            return Some(Bip34Context::UnneededAtRecheck);
        }
        None
    }

    fn linked_bip34_context(&self) -> Result<Bip34Context, HistoryError> {
        if let Some(context) = self.maybe_unneeded_bip34() {
            return Ok(context);
        }
        let index = self.index_at(self.network_rules.bip34_height())?;
        Ok(Bip34Context::Linked(self.branch[index].block_hash))
    }

    fn seeded_bip34_context(
        &self,
        maybe_seed: Option<HistoryHeader>,
    ) -> Result<Bip34Context, HistoryError> {
        if let Some(context) = self.maybe_unneeded_bip34() {
            if maybe_seed.is_some() {
                return Err(HistoryError::UnexpectedBip34Seed);
            }
            return Ok(context);
        }
        let height = self.network_rules.bip34_height();
        let seed = maybe_seed.ok_or(HistoryError::MissingAncestor { height })?;
        check_row(self.network_rules, &seed, height)?;
        // A seed that overlaps checked links must agree with those links.
        if let Ok(index) = self.index_at(height)
            && self.branch[index].block_hash != seed.block_hash
        {
            return Err(HistoryError::HashMismatch { height });
        }
        Ok(Bip34Context::IndependentlySeeded(seed.block_hash))
    }
}

fn check_branch(
    rules: NetworkRules,
    candidate: &BlockHeader,
    candidate_hash: BlockHash,
    branch: &[HistoryHeader],
) -> Result<HistoryOrigin, HistoryError> {
    let mut expected_hash = candidate.previous_block_hash;
    let mut expected_height = branch[0].height;
    let mut seen = BTreeSet::from([candidate_hash]);
    for row in branch {
        check_row(rules, row, expected_height)?;
        if !seen.insert(row.block_hash) {
            return Err(HistoryError::RepeatedHash { height: row.height });
        }
        if row.block_hash != expected_hash {
            return Err(HistoryError::ParentMismatch { height: row.height });
        }
        if row.height == 0 && row.block_hash != rules.chain().spec().genesis_hash() {
            return Err(HistoryError::WrongGenesis);
        }
        expected_hash = row.header.previous_block_hash;
        expected_height = row.height.saturating_sub(1);
    }
    let oldest = branch.last().ok_or(HistoryError::MissingAncestor {
        height: expected_height,
    })?;
    if oldest.height == 0 {
        return Ok(HistoryOrigin::GenesisLinked);
    }
    Ok(HistoryOrigin::CheckedSuffix {
        oldest_height: oldest.height,
        oldest_hash: oldest.block_hash,
    })
}

fn check_row(
    rules: NetworkRules,
    row: &HistoryHeader,
    expected_height: u32,
) -> Result<(), HistoryError> {
    if row.chain != rules.chain() {
        return Err(HistoryError::WrongNetwork { height: row.height });
    }
    if row.height != expected_height {
        return Err(HistoryError::HeightMismatch {
            expected: expected_height,
            actual: row.height,
        });
    }
    if row.block_hash != block_hash(&row.header) {
        return Err(HistoryError::HashMismatch { height: row.height });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
