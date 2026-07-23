// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{OutPoint, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{AdmissionContext, Mempool, MempoolOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolSnapshotRecord {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub transaction: Transaction,
    pub fee_sats: i64,
    pub virtual_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MempoolSnapshot {
    pub records: Vec<MempoolSnapshotRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRecoveryStatus {
    Recovered,
    DroppedConfirmed,
    DroppedDuplicate,
    DroppedMissingParent,
    DroppedPolicyIncompatible,
    DroppedEvicted,
}

impl MempoolRecoveryStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recovered => "recovered",
            Self::DroppedConfirmed => "dropped_confirmed",
            Self::DroppedDuplicate => "dropped_duplicate",
            Self::DroppedMissingParent => "dropped_missing_parent",
            Self::DroppedPolicyIncompatible => "dropped_policy_incompatible",
            Self::DroppedEvicted => "dropped_evicted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolRecoveryRecord {
    pub txid: Txid,
    pub status: MempoolRecoveryStatus,
}

impl MempoolSnapshot {
    pub fn from_mempool(mempool: &Mempool) -> Self {
        let mut records = mempool
            .entries()
            .values()
            .map(|entry| MempoolSnapshotRecord {
                txid: entry.txid,
                wtxid: entry.wtxid,
                transaction: entry.transaction.clone(),
                fee_sats: entry.fee_sats(),
                virtual_size: entry.virtual_size.as_usize(),
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|record| record.txid);

        Self { records }
    }

    #[allow(deprecated)] // Plan 130-08 migrates recovery; Plan 130-11 removes the projection.
    pub fn replay_into_mempool(
        &self,
        mempool: &mut Mempool,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Vec<MempoolRecoveryRecord> {
        self.records
            .iter()
            .map(|record| {
                let status = if transaction_is_confirmed(record, chainstate) {
                    MempoolRecoveryStatus::DroppedConfirmed
                } else {
                    recovery_status_from_outcome(mempool.accept_transaction_outcome_with_context(
                        record.transaction.clone(),
                        chainstate,
                        verify_flags,
                        consensus_params,
                        AdmissionContext::legacy_unknown(),
                    ))
                };
                MempoolRecoveryRecord {
                    txid: record.txid,
                    status,
                }
            })
            .collect()
    }
}

fn transaction_is_confirmed(
    record: &MempoolSnapshotRecord,
    chainstate: &ChainstateSnapshot,
) -> bool {
    (0..record.transaction.outputs.len()).any(|index| {
        let Ok(vout) = u32::try_from(index) else {
            return false;
        };
        chainstate.utxos.contains_key(&OutPoint {
            txid: record.txid,
            vout,
        })
    })
}

fn recovery_status_from_outcome(
    outcome: Result<MempoolOutcome, open_bitcoin_mempool::MempoolError>,
) -> MempoolRecoveryStatus {
    match outcome {
        Ok(MempoolOutcome::Accepted { .. }) | Ok(MempoolOutcome::Replaced { .. }) => {
            MempoolRecoveryStatus::Recovered
        }
        Ok(MempoolOutcome::Duplicate { .. }) => MempoolRecoveryStatus::DroppedDuplicate,
        Ok(MempoolOutcome::Orphaned { .. }) => MempoolRecoveryStatus::DroppedMissingParent,
        Ok(MempoolOutcome::Rejected { .. }) | Err(_) => {
            MempoolRecoveryStatus::DroppedPolicyIncompatible
        }
        Ok(MempoolOutcome::Evicted { .. }) | Ok(MempoolOutcome::Expired { .. }) => {
            MempoolRecoveryStatus::DroppedEvicted
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use open_bitcoin_core::{
        chainstate::{ChainstateSnapshot, Coin},
        consensus::{
            ConsensusParams, ScriptVerifyFlags, crypto::hash160, transaction_txid,
            transaction_wtxid,
        },
        primitives::{
            Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
            TransactionOutput, Txid,
        },
    };
    use open_bitcoin_mempool::PolicyConfig;

    use super::{
        MempoolRecoveryStatus, MempoolSnapshot, MempoolSnapshotRecord, recovery_status_from_outcome,
    };

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = hash160(script(&[0x51]).as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    }

    fn chainstate_with_utxo(outpoint: OutPoint, value_sats: i64) -> ChainstateSnapshot {
        let mut utxos = HashMap::new();
        utxos.insert(
            outpoint,
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(value_sats).expect("valid amount"),
                    script_pubkey: p2sh_script(),
                },
                is_coinbase: false,
                created_height: 0,
                created_median_time_past: 0,
            },
        );

        ChainstateSnapshot::new(Vec::new(), utxos, HashMap::new())
    }

    fn spend_transaction(previous_output: OutPoint, output_value_sats: i64) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output,
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(output_value_sats).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn snapshot_record(transaction: Transaction) -> MempoolSnapshotRecord {
        MempoolSnapshotRecord {
            txid: transaction_txid(&transaction).expect("txid"),
            wtxid: transaction_wtxid(&transaction).expect("wtxid"),
            transaction,
            fee_sats: 1_000,
            virtual_size: 100,
        }
    }

    #[test]
    fn mempool_snapshot_replay_recovers_accepted_records() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let record = snapshot_record(spend_transaction(previous_output, 499_000));
        let snapshot = MempoolSnapshot {
            records: vec![record.clone()],
        };
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

        // Act
        let recovery = snapshot.replay_into_mempool(
            &mut mempool,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );

        // Assert
        assert_eq!(recovery[0].status, MempoolRecoveryStatus::Recovered);
        assert_eq!(recovery[0].status.as_str(), "recovered");
        assert!(mempool.entry(&record.txid).is_some());
    }

    #[test]
    fn mempool_snapshot_replay_drops_confirmed_records_with_evidence() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([2_u8; 32]),
            vout: 0,
        };
        let record = snapshot_record(spend_transaction(previous_output, 499_000));
        let confirmed_output = OutPoint {
            txid: record.txid,
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(confirmed_output, 499_000);
        let snapshot = MempoolSnapshot {
            records: vec![record.clone()],
        };
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

        // Act
        let recovery = snapshot.replay_into_mempool(
            &mut mempool,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );

        // Assert
        assert_eq!(recovery[0].status, MempoolRecoveryStatus::DroppedConfirmed);
        assert_eq!(recovery[0].status.as_str(), "dropped_confirmed");
        assert!(mempool.entry(&record.txid).is_none());
    }

    #[test]
    fn mempool_snapshot_replay_drops_policy_incompatible_records_with_evidence() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([3_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let record = snapshot_record(spend_transaction(previous_output, 499_999));
        let snapshot = MempoolSnapshot {
            records: vec![record.clone()],
        };
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

        // Act
        let recovery = snapshot.replay_into_mempool(
            &mut mempool,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );

        // Assert
        assert_eq!(
            recovery[0].status,
            MempoolRecoveryStatus::DroppedPolicyIncompatible
        );
        assert_eq!(recovery[0].status.as_str(), "dropped_policy_incompatible");
        assert!(mempool.entry(&record.txid).is_none());
    }

    #[test]
    fn recovery_status_from_outcome_classifies_unexpected_errors_as_policy_incompatible() {
        // Arrange
        let error = open_bitcoin_mempool::MempoolError::Validation {
            reason: "bad-tx".to_string(),
        };

        // Act
        let status = recovery_status_from_outcome(Err(error));

        // Assert
        assert_eq!(status, MempoolRecoveryStatus::DroppedPolicyIncompatible);
    }
}
