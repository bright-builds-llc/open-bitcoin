use crate::error::BenchError;

pub const REQUIRED_GROUP_IDS: [&str; 7] = [
    "consensus-script",
    "block-transaction-codec",
    "chainstate",
    "mempool-policy",
    "network-wire-sync",
    "wallet",
    "rpc-cli",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchGroupId {
    ConsensusScript,
    BlockTransactionCodec,
    Chainstate,
    MempoolPolicy,
    NetworkWireSync,
    Wallet,
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
            Self::Wallet => "wallet",
            Self::RpcCli => "rpc-cli",
        }
    }
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
    pub knots_mapping: &'static KnotsMapping,
    pub run_once: fn() -> Result<(), BenchError>,
}

#[derive(Debug, Clone, Copy)]
pub struct BenchGroup {
    pub id: BenchGroupId,
    pub description: &'static str,
    pub cases: &'static [BenchCase],
}

const CONSENSUS_SCRIPT_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["VerifyScriptBench"],
    source_files: &["packages/bitcoin-knots/src/bench/verify_script.cpp"],
    notes: "Maps script validation coverage to the Knots P2WPKH script verification benchmark.",
};

const BLOCK_TRANSACTION_CODEC_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["DeserializeBlockTest", "SaveBlockBench", "ReadBlockBench"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/checkblock.cpp",
        "packages/bitcoin-knots/src/bench/readwriteblock.cpp",
    ],
    notes: "Maps block and transaction codec coverage to Knots block deserialization and read/write benchmarks.",
};

const CHAINSTATE_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["DeserializeAndCheckBlockTest", "ReadRawBlockBench"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/checkblock.cpp",
        "packages/bitcoin-knots/src/bench/readwriteblock.cpp",
    ],
    notes: "Maps chainstate-adjacent validation and storage smoke coverage to Knots block check and raw-read benchmarks.",
};

const MEMPOOL_POLICY_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["ComplexMemPool"],
    source_files: &["packages/bitcoin-knots/src/bench/mempool_stress.cpp"],
    notes: "Maps mempool policy smoke coverage to the Knots complex mempool stress benchmark.",
};

const NETWORK_WIRE_SYNC_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["AddrMan", "EvictionProtection"],
    source_files: &[
        "packages/bitcoin-knots/src/bench/addrman.cpp",
        "packages/bitcoin-knots/src/bench/peer_eviction.cpp",
    ],
    notes: "Maps network wire and sync planning coverage to Knots address-manager and peer eviction benchmarks.",
};

const WALLET_MAPPING: KnotsMapping = KnotsMapping {
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

const RPC_CLI_MAPPING: KnotsMapping = KnotsMapping {
    benchmark_names: &["RpcMempool"],
    source_files: &["packages/bitcoin-knots/src/bench/rpc_mempool.cpp"],
    notes: "Maps RPC and CLI dispatch smoke coverage to the Knots mempool RPC benchmark.",
};

const CONSENSUS_SCRIPT_CASES: [BenchCase; 1] = [BenchCase {
    id: "consensus-script.registry",
    group: BenchGroupId::ConsensusScript,
    description: "Registry contract for consensus script validation benchmarks.",
    knots_mapping: &CONSENSUS_SCRIPT_MAPPING,
    run_once: metadata_case,
}];

const BLOCK_TRANSACTION_CODEC_CASES: [BenchCase; 1] = [BenchCase {
    id: "block-transaction-codec.registry",
    group: BenchGroupId::BlockTransactionCodec,
    description: "Registry contract for block and transaction codec benchmarks.",
    knots_mapping: &BLOCK_TRANSACTION_CODEC_MAPPING,
    run_once: metadata_case,
}];

const CHAINSTATE_CASES: [BenchCase; 1] = [BenchCase {
    id: "chainstate.registry",
    group: BenchGroupId::Chainstate,
    description: "Registry contract for chainstate connect, disconnect, and storage-adjacent benchmarks.",
    knots_mapping: &CHAINSTATE_MAPPING,
    run_once: metadata_case,
}];

const MEMPOOL_POLICY_CASES: [BenchCase; 1] = [BenchCase {
    id: "mempool-policy.registry",
    group: BenchGroupId::MempoolPolicy,
    description: "Registry contract for mempool policy benchmarks.",
    knots_mapping: &MEMPOOL_POLICY_MAPPING,
    run_once: metadata_case,
}];

const NETWORK_WIRE_SYNC_CASES: [BenchCase; 1] = [BenchCase {
    id: "network-wire-sync.registry",
    group: BenchGroupId::NetworkWireSync,
    description: "Registry contract for network wire and sync planning benchmarks.",
    knots_mapping: &NETWORK_WIRE_SYNC_MAPPING,
    run_once: metadata_case,
}];

const WALLET_CASES: [BenchCase; 1] = [BenchCase {
    id: "wallet.registry",
    group: BenchGroupId::Wallet,
    description: "Registry contract for wallet balance, selection, signing, and transaction-building benchmarks.",
    knots_mapping: &WALLET_MAPPING,
    run_once: metadata_case,
}];

const RPC_CLI_CASES: [BenchCase; 1] = [BenchCase {
    id: "rpc-cli.registry",
    group: BenchGroupId::RpcCli,
    description: "Registry contract for RPC and CLI dispatch benchmarks.",
    knots_mapping: &RPC_CLI_MAPPING,
    run_once: metadata_case,
}];

static BENCH_GROUPS: [BenchGroup; 7] = [
    BenchGroup {
        id: BenchGroupId::ConsensusScript,
        description: "Consensus script validation",
        cases: &CONSENSUS_SCRIPT_CASES,
    },
    BenchGroup {
        id: BenchGroupId::BlockTransactionCodec,
        description: "Block and transaction parsing or serialization",
        cases: &BLOCK_TRANSACTION_CODEC_CASES,
    },
    BenchGroup {
        id: BenchGroupId::Chainstate,
        description: "Chainstate connect, disconnect, reorg, and storage-adjacent operations",
        cases: &CHAINSTATE_CASES,
    },
    BenchGroup {
        id: BenchGroupId::MempoolPolicy,
        description: "Mempool admission, replacement, and policy accounting",
        cases: &MEMPOOL_POLICY_CASES,
    },
    BenchGroup {
        id: BenchGroupId::NetworkWireSync,
        description: "Network wire encoding, address management, peer policy, and sync planning",
        cases: &NETWORK_WIRE_SYNC_CASES,
    },
    BenchGroup {
        id: BenchGroupId::Wallet,
        description: "Wallet balance, coin selection, signing, and transaction creation",
        cases: &WALLET_CASES,
    },
    BenchGroup {
        id: BenchGroupId::RpcCli,
        description: "RPC and CLI request dispatch",
        cases: &RPC_CLI_CASES,
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

fn metadata_case() -> Result<(), BenchError> {
    Ok(())
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
        ] {
            assert!(
                mappings.iter().any(|name| name.contains(marker)),
                "missing Knots mapping marker {marker}"
            );
        }
    }
}
