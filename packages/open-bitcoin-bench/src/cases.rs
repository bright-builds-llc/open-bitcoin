pub mod chainstate;
pub mod codec;
pub mod consensus;
pub mod mempool;
pub mod network;

use crate::registry::BenchCase;

pub fn registered_cases() -> Vec<BenchCase> {
    let mut cases = Vec::new();
    cases.extend_from_slice(&consensus::CASES);
    cases.extend_from_slice(&codec::CASES);
    cases.extend_from_slice(&chainstate::CASES);
    cases.extend_from_slice(&mempool::CASES);
    cases.extend_from_slice(&network::CASES);
    cases
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::registry::BenchGroupId;

    use super::registered_cases;

    #[test]
    fn registered_cases_include_core_node_groups() {
        // Arrange
        let required = HashSet::from([
            BenchGroupId::ConsensusScript,
            BenchGroupId::BlockTransactionCodec,
            BenchGroupId::Chainstate,
            BenchGroupId::MempoolPolicy,
            BenchGroupId::NetworkWireSync,
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
    fn stateful_core_cases_are_repeatable() {
        // Arrange
        let cases = registered_cases();

        // Act / Assert
        for group in [BenchGroupId::Chainstate, BenchGroupId::MempoolPolicy] {
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
    fn codec_and_network_cases_run_from_fixtures() {
        // Arrange
        let cases = registered_cases();

        // Act / Assert
        for group in [
            BenchGroupId::BlockTransactionCodec,
            BenchGroupId::NetworkWireSync,
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
