// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use crate::{cases, error::BenchError};

pub const REQUIRED_GROUP_IDS: [&str; 11] = [
    "consensus-script",
    "block-transaction-codec",
    "chainstate",
    "mempool-policy",
    "network-wire-sync",
    "sync-runtime",
    "storage-recovery",
    "operator-runtime",
    "wallet",
    "wallet-rescan",
    "rpc-cli",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchGroupId {
    ConsensusScript,
    BlockTransactionCodec,
    Chainstate,
    MempoolPolicy,
    NetworkWireSync,
    SyncRuntime,
    StorageRecovery,
    OperatorRuntime,
    Wallet,
    WalletRescan,
    RpcCli,
}

impl BenchGroupId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsensusScript => "consensus-script",
            Self::BlockTransactionCodec => "block-transaction-codec",
            Self::Chainstate => "chainstate",
            Self::MempoolPolicy => "mempool-policy",
            Self::NetworkWireSync => "network-wire-sync",
            Self::SyncRuntime => "sync-runtime",
            Self::StorageRecovery => "storage-recovery",
            Self::OperatorRuntime => "operator-runtime",
            Self::Wallet => "wallet",
            Self::WalletRescan => "wallet-rescan",
            Self::RpcCli => "rpc-cli",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchDurability {
    Pure,
    Ephemeral,
    Durable,
}

impl BenchDurability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pure => "pure",
            Self::Ephemeral => "ephemeral",
            Self::Durable => "durable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchMeasurement {
    pub focus: &'static str,
    pub fixture: &'static str,
    pub durability: BenchDurability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KnotsMapping {
    pub benchmark_names: &'static [&'static str],
    pub source_files: &'static [&'static str],
    pub notes: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct BenchCase {
    pub id: &'static str,
    pub group: BenchGroupId,
    pub description: &'static str,
    pub measurement: BenchMeasurement,
    pub knots_mapping: &'static KnotsMapping,
    pub run_once: fn() -> Result<(), BenchError>,
}

#[derive(Debug, Clone, Copy)]
pub struct BenchGroup {
    pub id: BenchGroupId,
    pub description: &'static str,
    pub cases: &'static [BenchCase],
}

pub(crate) const CONSENSUS_SCRIPT_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["VerifyScriptBench"],
    source_files: &["packages/bitcoin-knots/src/bench/verify_script.cpp"],
    notes: "Maps script validation coverage to the Knots P2WPKH script verification benchmark.",
};

pub(crate) const BLOCK_TRANSACTION_CODEC_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["DeserializeBlockTest", "SaveBlockBench", "ReadBlockBench"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/checkblock.cpp",
        "packages/bitcoin-knots/src/bench/readwriteblock.cpp",
    ],
    notes: "Maps block and transaction codec coverage to Knots block deserialization and read/write benchmarks.",
};

pub(crate) const CHAINSTATE_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["DeserializeAndCheckBlockTest", "ReadRawBlockBench"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/checkblock.cpp",
        "packages/bitcoin-knots/src/bench/readwriteblock.cpp",
    ],
    notes: "Maps chainstate-adjacent validation and storage smoke coverage to Knots block check and raw-read benchmarks.",
};

pub(crate) const MEMPOOL_POLICY_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["ComplexMemPool"],
    source_files: &["packages/bitcoin-knots/src/bench/mempool_stress.cpp"],
    notes: "Maps mempool policy smoke coverage to the Knots complex mempool stress benchmark.",
};

pub(crate) const NETWORK_WIRE_SYNC_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["AddrMan", "EvictionProtection"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/addrman.cpp",
        "packages/bitcoin-knots/src/bench/peer_eviction.cpp",
    ],
    notes: "Maps network wire and sync planning coverage to Knots address-manager and peer eviction benchmarks.",
};

pub(crate) const SYNC_RUNTIME_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &[],
    source_files: &[
        "packages/bitcoin-knots/src/net_processing.cpp",
        "packages/bitcoin-knots/src/headerssync.cpp",
        "packages/bitcoin-knots/src/sync.cpp",
        "packages/bitcoin-knots/src/node/blockstorage.cpp",
    ],
    notes: "No direct Knots benchmark covers the durable sync runtime. This group anchors deterministic Phase 22 sync evidence to the corresponding Knots sync and block-storage sources.",
};

pub(crate) const STORAGE_RECOVERY_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["ReadRawBlockBench", "SaveBlockBench"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/readwriteblock.cpp",
        "packages/bitcoin-knots/src/node/blockstorage.cpp",
    ],
    notes: "Maps durable storage write, read, and reopen evidence to Knots raw block read/write and block-storage ownership.",
};

pub(crate) const OPERATOR_RUNTIME_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &[],
    source_files: &[
        "packages/bitcoin-knots/src/bitcoin-cli.cpp",
        "packages/bitcoin-knots/src/init.cpp",
        "packages/bitcoin-knots/src/interfaces/node.h",
    ],
    notes: "No direct Knots benchmark covers operator-facing status or dashboard collection. This group tracks the runtime-collected status and dashboard rendering overhead that Open Bitcoin adds around the baseline-compatible surfaces.",
};

pub(crate) const WALLET_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &[
        "WalletBalance",
        "CoinSelection",
        "WalletCreateTx",
        "WalletAvailableCoins",
    ],
    source_files: &[
        "packages/bitcoin-knots/src/bench/wallet_balance.cpp",
        "packages/bitcoin-knots/src/bench/coin_selection.cpp",
        "packages/bitcoin-knots/src/bench/wallet_create_tx.cpp",
    ],
    notes: "Maps wallet balance, coin selection, and transaction creation coverage to the Knots wallet benchmark set.",
};

pub(crate) const WALLET_RESCAN_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &[],
    source_files: &[
        "packages/bitcoin-knots/src/wallet/wallet.cpp",
        "packages/bitcoin-knots/doc/managing-wallets.md",
    ],
    notes: "No direct Knots benchmark covers the managed rescan runtime. This group tracks durable wallet-rescan cost against the baseline wallet ownership and operator guidance surfaces.",
};

pub(crate) const RPC_CLI_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["RpcMempool"],
    source_files: &["packages/bitcoin-knots/src/bench/rpc_mempool.cpp"],
    notes: "Maps RPC and CLI dispatch smoke coverage to the Knots mempool RPC benchmark.",
};

static BENCH_GROUPS: [BenchGroup; 11] = [
    BenchGroup {
        id: BenchGroupId::ConsensusScript,
        description: "Consensus script validation",
        cases: &cases::consensus::CASES,
    },
    BenchGroup {
        id: BenchGroupId::BlockTransactionCodec,
        description: "Block and transaction parsing or serialization",
        cases: &cases::codec::CASES,
    },
    BenchGroup {
        id: BenchGroupId::Chainstate,
        description: "Chainstate connect, disconnect, reorg, and storage-adjacent operations",
        cases: &cases::chainstate::CASES,
    },
    BenchGroup {
        id: BenchGroupId::MempoolPolicy,
        description: "Mempool admission, replacement, and policy accounting",
        cases: &cases::mempool::CASES,
    },
    BenchGroup {
        id: BenchGroupId::NetworkWireSync,
        description: "Network wire encoding, address management, peer policy, and sync planning",
        cases: &cases::network::CASES,
    },
    BenchGroup {
        id: BenchGroupId::SyncRuntime,
        description: "Headers sync plus block download or connect through the durable runtime",
        cases: &cases::sync_runtime::CASES,
    },
    BenchGroup {
        id: BenchGroupId::StorageRecovery,
        description: "Durable storage write or read and restart recovery",
        cases: &cases::storage_recovery::CASES,
    },
    BenchGroup {
        id: BenchGroupId::OperatorRuntime,
        description: "Status rendering and dashboard projection from runtime-collected operator snapshots",
        cases: &cases::operator_runtime::CASES,
    },
    BenchGroup {
        id: BenchGroupId::Wallet,
        description: "Wallet balance, coin selection, signing, and transaction creation",
        cases: &cases::wallet::CASES,
    },
    BenchGroup {
        id: BenchGroupId::WalletRescan,
        description: "Durable wallet rescan runtime and managed wallet freshness updates",
        cases: &cases::wallet_rescan::CASES,
    },
    BenchGroup {
        id: BenchGroupId::RpcCli,
        description: "RPC and CLI request dispatch",
        cases: &cases::rpc_cli::CASES,
    },
];

pub fn benchmark_groups() -> &'static [BenchGroup] {
    &BENCH_GROUPS
}

pub fn group_ids() -> Vec<&'static str> {
    benchmark_groups()
        .iter()
        .map(|group| group.id.as_str())
        .collect()
}

pub fn list_output() -> String {
    group_ids().join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::{REQUIRED_GROUP_IDS, benchmark_groups, group_ids, list_output};

    #[test]
    fn list_output_prints_all_required_group_ids() {
        // Arrange
        let output = list_output();

        // Act
        let ids = output.lines().collect::<Vec<_>>();

        // Assert
        for required_id in REQUIRED_GROUP_IDS {
            assert!(
                ids.contains(&required_id),
                "missing benchmark group id {required_id}"
            );
        }
    }

    #[test]
    fn registry_contains_exact_required_group_ids_in_order() {
        // Arrange / Act
        let ids = group_ids();

        // Assert
        assert_eq!(ids, REQUIRED_GROUP_IDS);
    }

    #[test]
    fn registry_contains_required_knots_mapping_markers() {
        // Arrange
        let mappings = benchmark_groups()
            .iter()
            .flat_map(|group| group.cases)
            .flat_map(|case| case.knots_mapping.benchmark_names)
            .copied()
            .collect::<Vec<_>>();

        // Act / Assert
        for marker in [
            "VerifyScriptBench",
            "DeserializeBlockTest",
            "ComplexMemPool",
            "RpcMempool",
            "WalletBalance",
            "CoinSelection",
            "WalletCreateTx",
            "AddrMan",
            "EvictionProtection",
            "ReadRawBlockBench",
            "SaveBlockBench",
        ] {
            assert!(
                mappings.iter().any(|name| name.contains(marker)),
                "missing Knots mapping marker {marker}"
            );
        }
    }
}
