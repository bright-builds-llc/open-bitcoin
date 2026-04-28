// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

pub mod chainstate;
pub mod codec;
pub mod consensus;
pub mod mempool;
pub mod network;
pub mod operator_runtime;
pub mod rpc_cli;
pub mod storage_recovery;
pub mod sync_runtime;
pub mod wallet;
pub mod wallet_rescan;

use crate::registry::BenchCase;

pub fn registered_cases() -> Vec<BenchCase> {
    let mut cases = Vec::new();
    cases.extend_from_slice(&consensus::CASES);
    cases.extend_from_slice(&codec::CASES);
    cases.extend_from_slice(&chainstate::CASES);
    cases.extend_from_slice(&mempool::CASES);
    cases.extend_from_slice(&network::CASES);
    cases.extend_from_slice(&sync_runtime::CASES);
    cases.extend_from_slice(&storage_recovery::CASES);
    cases.extend_from_slice(&operator_runtime::CASES);
    cases.extend_from_slice(&wallet::CASES);
    cases.extend_from_slice(&wallet_rescan::CASES);
    cases.extend_from_slice(&rpc_cli::CASES);
    cases
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::registry::BenchGroupId;

    use super::registered_cases;

    #[test]
    fn registered_cases_include_all_required_groups() {
        // Arrange
        let required = HashSet::from([
            BenchGroupId::ConsensusScript,
            BenchGroupId::BlockTransactionCodec,
            BenchGroupId::Chainstate,
            BenchGroupId::MempoolPolicy,
            BenchGroupId::NetworkWireSync,
            BenchGroupId::SyncRuntime,
            BenchGroupId::StorageRecovery,
            BenchGroupId::OperatorRuntime,
            BenchGroupId::Wallet,
            BenchGroupId::WalletRescan,
            BenchGroupId::RpcCli,
        ]);

        // Act
        let groups = registered_cases()
            .into_iter()
            .map(|case| case.group)
            .collect::<HashSet<_>>();

        // Assert
        for required_group in required {
            assert!(
                groups.contains(&required_group),
                "missing benchmark group {:?}",
                required_group
            );
        }
    }

    #[test]
    fn wallet_and_rpc_cli_cases_are_executable() {
        // Arrange
        let cases = registered_cases();

        // Act / Assert
        for group in [BenchGroupId::Wallet, BenchGroupId::RpcCli] {
            let Some(case) = cases.iter().find(|case| case.group == group) else {
                panic!("missing executable benchmark case for {:?}", group);
            };
            assert!(
                (case.run_once)().is_ok(),
                "benchmark case should succeed for {:?}",
                group
            );
        }
    }

    #[test]
    fn stateful_core_cases_are_repeatable() {
        // Arrange
        let cases = registered_cases();

        // Act / Assert
        for group in [
            BenchGroupId::Chainstate,
            BenchGroupId::MempoolPolicy,
            BenchGroupId::SyncRuntime,
            BenchGroupId::StorageRecovery,
            BenchGroupId::WalletRescan,
        ] {
            let Some(case) = cases.iter().find(|case| case.group == group) else {
                panic!("missing stateful benchmark case for {:?}", group);
            };
            assert!(
                (case.run_once)().is_ok(),
                "first run should succeed for {:?}",
                group
            );
            assert!(
                (case.run_once)().is_ok(),
                "second run should succeed for {:?}",
                group
            );
        }
    }

    #[test]
    fn fixture_backed_cases_run_from_shared_inputs() {
        // Arrange
        let cases = registered_cases();

        // Act / Assert
        for group in [
            BenchGroupId::BlockTransactionCodec,
            BenchGroupId::NetworkWireSync,
            BenchGroupId::OperatorRuntime,
        ] {
            let Some(case) = cases.iter().find(|case| case.group == group) else {
                panic!("missing fixture-backed benchmark case for {:?}", group);
            };
            assert!(
                (case.run_once)().is_ok(),
                "fixture-backed case should succeed for {:?}",
                group
            );
        }
    }
}
