// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/consensus/consensus.h
// - packages/bitcoin-knots/src/validation.cpp

use super::{CanonicalChain, ChainSpec};
use crate::ScriptVerifyFlags;
use open_bitcoin_primitives::BlockHash;

/// Immutable selected source facts. These values confer no ancestry or support evidence.
/// Only a selected ChainSpec can construct this value.
/// ```compile_fail
/// use open_bitcoin_consensus::chain_spec::{CanonicalChain, NetworkRules};
/// let _ = NetworkRules { chain: CanonicalChain::Regtest };
/// ```
/// ```compile_fail
/// use open_bitcoin_consensus::chain_spec::CanonicalChain;
/// let mut rules = CanonicalChain::Regtest.spec().rules();
/// rules.chain = CanonicalChain::Mainnet;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkRules {
    chain: CanonicalChain,
}

impl ChainSpec {
    /// Pinned consensus facts, independent of fixture settings and validation qualification.
    pub const fn rules(&self) -> NetworkRules {
        NetworkRules {
            chain: self.chain(),
        }
    }
}

impl NetworkRules {
    /// Selected canonical identity.
    pub const fn chain(&self) -> CanonicalChain {
        self.chain
    }
    /// Blocks before an immature coinbase may be spent.
    pub const fn coinbase_maturity(&self) -> u32 {
        100
    }
    /// Number of blocks per subsidy halving.
    pub const fn subsidy_halving_interval(&self) -> u32 {
        match self.chain {
            CanonicalChain::Regtest => 150,
            _ => 210_000,
        }
    }
    /// Compact representation of the selected proof-of-work limit.
    pub const fn pow_limit_bits(&self) -> u32 {
        match self.chain {
            CanonicalChain::Mainnet | CanonicalChain::Testnet3 => 0x1d00_ffff,
            CanonicalChain::Signet => 0x1e03_77ae,
            CanonicalChain::Regtest => 0x207f_ffff,
        }
    }
    /// Target time between blocks.
    pub const fn pow_target_spacing_seconds(&self) -> i64 {
        600
    }
    /// Target elapsed time per difficulty period.
    pub const fn pow_target_timespan_seconds(&self) -> i64 {
        match self.chain {
            CanonicalChain::Regtest => 86_400,
            _ => 1_209_600,
        }
    }
    /// Number of blocks in the selected difficulty period.
    pub const fn difficulty_adjustment_interval(&self) -> u32 {
        (self.pow_target_timespan_seconds() / self.pow_target_spacing_seconds()) as u32
    }
    /// Whether a sufficiently late block can use minimum difficulty.
    pub const fn allow_min_difficulty_blocks(&self) -> bool {
        matches!(
            self.chain,
            CanonicalChain::Testnet3 | CanonicalChain::Regtest
        )
    }
    /// Whether period boundaries preserve the previous target.
    pub const fn no_pow_retargeting(&self) -> bool {
        matches!(self.chain, CanonicalChain::Regtest)
    }
    /// Default selectors do not opt in to BIP94.
    pub const fn enforce_bip94(&self) -> bool {
        false
    }
    /// BIP34 coinbase-height activation height.
    pub const fn bip34_height(&self) -> u32 {
        self.activation_heights()[0]
    }
    /// BIP66 strict-DER activation height.
    pub const fn bip66_height(&self) -> u32 {
        self.activation_heights()[1]
    }
    /// BIP65 CHECKLOCKTIMEVERIFY activation height.
    pub const fn bip65_height(&self) -> u32 {
        self.activation_heights()[2]
    }
    /// CSV, sequence-lock and BIP113 activation height.
    pub const fn csv_height(&self) -> u32 {
        self.activation_heights()[3]
    }
    /// Segwit commitment and NULLDUMMY activation height.
    pub const fn segwit_height(&self) -> u32 {
        self.activation_heights()[4]
    }
    const fn activation_heights(&self) -> [u32; 5] {
        match self.chain {
            CanonicalChain::Mainnet => [227_931, 363_725, 388_381, 419_328, 481_824],
            CanonicalChain::Testnet3 => [21_111, 330_776, 581_885, 770_112, 834_624],
            CanonicalChain::Signet => [1, 1, 1, 1, 1],
            CanonicalChain::Regtest => [1, 1, 1, 1, 0],
        }
    }
    /// Pinned BIP34 ancestor commitment; null source hashes are represented as absence.
    pub const fn maybe_bip34_hash(&self) -> Option<BlockHash> {
        match self.chain {
            CanonicalChain::Mainnet => Some(BlockHash::from_byte_array([
                0xb8, 0x08, 0x08, 0x9c, 0x75, 0x6a, 0xdd, 0x15, 0x91, 0xb1, 0xd1, 0x7b, 0xab, 0x44,
                0xbb, 0xa3, 0xfe, 0xd9, 0xe0, 0x2f, 0x94, 0x2a, 0xb4, 0x89, 0x4b, 0x02, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ])),
            CanonicalChain::Testnet3 => Some(BlockHash::from_byte_array([
                0xf8, 0x8e, 0xcd, 0x99, 0x12, 0xd0, 0x0d, 0x3f, 0x5c, 0x2a, 0x8e, 0x0f, 0x50, 0x41,
                0x7d, 0x3e, 0x41, 0x5c, 0x75, 0xb3, 0xab, 0xe5, 0x84, 0x34, 0x6d, 0xa9, 0xb3, 0x23,
                0x00, 0x00, 0x00, 0x00,
            ])),
            CanonicalChain::Signet | CanonicalChain::Regtest => None,
        }
    }
    /// Exact hash-specific replacements before deployment additions.
    pub const fn script_flag_exceptions(&self) -> &'static [ScriptFlagException] {
        match self.chain {
            CanonicalChain::Mainnet => &MAINNET_SCRIPT_EXCEPTIONS,
            CanonicalChain::Testnet3 => &TESTNET_SCRIPT_EXCEPTIONS,
            CanonicalChain::Signet | CanonicalChain::Regtest => &[],
        }
    }
    /// Complete hash-specific replacement before deployment additions.
    pub fn maybe_script_flag_exception(&self, hash: BlockHash) -> Option<ScriptVerifyFlags> {
        self.script_flag_exceptions()
            .iter()
            .find(|exception| exception.block_hash() == hash)
            .map(ScriptFlagException::replacement_flags)
    }
    /// Mainnet's exact historical repeat pairs; these never authorize another network.
    pub const fn bip30_repeat_pairs(&self) -> &'static [(u32, BlockHash)] {
        match self.chain {
            CanonicalChain::Mainnet => &BIP30_REPEATS,
            _ => &[],
        }
    }
    /// BIP30 checking resumes even after a matching BIP34 ancestor at this height.
    pub const fn bip30_recheck_height(&self) -> u32 {
        1_983_702
    }
    /// Timestamp locktime threshold from the transaction consensus contract.
    pub const fn locktime_threshold(&self) -> u32 {
        500_000_000
    }
    /// Base-two units for time-based sequence locks.
    pub const fn sequence_locktime_granularity(&self) -> u32 {
        9
    }
}

/// A pinned exact-block script flag replacement, with read-only source facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptFlagException {
    block_hash: BlockHash,
    retain_p2sh_witness: bool,
}
impl ScriptFlagException {
    /// Exact candidate identity to which the replacement applies.
    pub const fn block_hash(&self) -> BlockHash {
        self.block_hash
    }
    /// Entire replacement flag set, before buried deployment additions.
    pub fn replacement_flags(&self) -> ScriptVerifyFlags {
        if self.retain_p2sh_witness {
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS
        } else {
            ScriptVerifyFlags::NONE
        }
    }
}

const MAINNET_SCRIPT_EXCEPTIONS: [ScriptFlagException; 2] = [
    ScriptFlagException {
        block_hash: BlockHash::from_byte_array([
            0x22, 0x9c, 0x4f, 0xac, 0x88, 0xba, 0xb1, 0x94, 0xeb, 0x08, 0xf1, 0xa5, 0x28, 0xcc,
            0x30, 0x8d, 0xed, 0x23, 0x97, 0xf4, 0xf4, 0xeb, 0x6e, 0x75, 0xdc, 0x02, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        retain_p2sh_witness: false,
    },
    ScriptFlagException {
        block_hash: BlockHash::from_byte_array([
            0xad, 0x95, 0xe3, 0xa1, 0x5e, 0xe5, 0xff, 0xd5, 0x85, 0xc5, 0xe8, 0x1d, 0x44, 0xb5,
            0x6a, 0x98, 0x1e, 0x84, 0x2d, 0x5b, 0xc3, 0x14, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        retain_p2sh_witness: true,
    },
];
const TESTNET_SCRIPT_EXCEPTIONS: [ScriptFlagException; 1] = [ScriptFlagException {
    block_hash: BlockHash::from_byte_array([
        0x05, 0xb1, 0x32, 0xa4, 0xf7, 0x4a, 0x87, 0x99, 0xa5, 0x7a, 0x42, 0x02, 0xd0, 0xee, 0xb0,
        0x96, 0x12, 0xcc, 0x08, 0xd2, 0x95, 0x40, 0x1f, 0x00, 0x7c, 0x45, 0x30, 0xdd, 0x00, 0x00,
        0x00, 0x00,
    ]),
    retain_p2sh_witness: false,
}];
const BIP30_REPEATS: [(u32, BlockHash); 2] = [
    (
        91_842,
        BlockHash::from_byte_array([
            0xec, 0xca, 0xe0, 0x00, 0xe3, 0xc8, 0xe4, 0xe0, 0x93, 0x93, 0x63, 0x60, 0x43, 0x1f,
            0x3b, 0x76, 0x03, 0xc5, 0x63, 0xc1, 0xff, 0x61, 0x81, 0x39, 0x0a, 0x4d, 0x0a, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
    ),
    (
        91_880,
        BlockHash::from_byte_array([
            0x21, 0xd7, 0x7c, 0xcb, 0x4c, 0x08, 0x38, 0x6a, 0x04, 0xac, 0x01, 0x96, 0xae, 0x10,
            0xf6, 0xa1, 0xd2, 0xc2, 0xa3, 0x77, 0x55, 0x8c, 0xa1, 0x90, 0xf1, 0x43, 0x07, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
    ),
];

#[cfg(test)]
mod tests;
