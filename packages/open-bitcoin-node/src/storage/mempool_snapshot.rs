// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{OutPoint, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{AdmissionContext, Mempool, MempoolEntryMetadata, MempoolOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolSnapshotRecord {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub transaction: Transaction,
    pub fee_sats: i64,
    pub virtual_size: usize,
    pub metadata: MempoolEntryMetadata,
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
                metadata: entry.metadata,
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|record| record.txid);

        Self { records }
    }

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
                    recovery_status_from_outcome(
                        mempool
                            .accept_transaction_transition_with_context(
                                record.transaction.clone(),
                                chainstate,
                                verify_flags,
                                consensus_params,
                                AdmissionContext::recovery(record.metadata),
                            )
                            .map(|transition| transition.outcome),
                    )
                };
                MempoolRecoveryRecord {
                    txid: record.txid,
                    status,
                }
            })
            .collect()
    }
}

pub(crate) fn transaction_is_confirmed(
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

pub(crate) fn recovery_status_from_outcome(
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
    use open_bitcoin_mempool::{
        MempoolAcceptanceTime, MempoolEntryMetadata, MempoolOrigin, PolicyConfig, PolicyTime,
        RelayIntent,
    };

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
            metadata: MempoolEntryMetadata::legacy_unknown(),
        }
    }

    fn known_local_requested(accepted_at: i64) -> MempoolEntryMetadata {
        MempoolEntryMetadata::new(
            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(accepted_at)),
            MempoolOrigin::Local,
            RelayIntent::Requested,
        )
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

    #[test]
    fn recovery_metadata_from_mempool_copies_exact_entry_metadata() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([11_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let transaction = spend_transaction(previous_output, 499_000);
        let expected = known_local_requested(90);
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());
        mempool
            .accept_transaction_transition_with_context(
                transaction,
                &chainstate,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
                open_bitcoin_mempool::AdmissionContext::recovery(expected),
            )
            .expect("admit known local");

        // Act
        let snapshot = MempoolSnapshot::from_mempool(&mempool);

        // Assert
        assert_eq!(snapshot.records.len(), 1);
        assert_eq!(snapshot.records[0].metadata, expected);
    }

    #[test]
    fn recovery_metadata_known_local_requested_replays_exactly_and_stays_retry_eligible() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([12_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let transaction = spend_transaction(previous_output, 499_000);
        let expected = known_local_requested(90);
        let mut record = snapshot_record(transaction);
        record.metadata = expected;
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
        let entry = mempool.entry(&record.txid).expect("recovered entry");
        assert_eq!(entry.metadata, expected);
        assert!(entry.metadata.is_retry_eligible(true));
        assert!(!entry.metadata.is_retry_eligible(false));
    }

    #[test]
    fn recovery_metadata_known_peer_and_reorg_remain_non_local_after_replay() {
        // Arrange
        let peer_previous = OutPoint {
            txid: Txid::from_byte_array([13_u8; 32]),
            vout: 0,
        };
        let reorg_previous = OutPoint {
            txid: Txid::from_byte_array([14_u8; 32]),
            vout: 0,
        };
        let mut utxos = HashMap::new();
        utxos.insert(
            peer_previous.clone(),
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(500_000).expect("valid amount"),
                    script_pubkey: p2sh_script(),
                },
                is_coinbase: false,
                created_height: 0,
                created_median_time_past: 0,
            },
        );
        utxos.insert(
            reorg_previous.clone(),
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(500_000).expect("valid amount"),
                    script_pubkey: p2sh_script(),
                },
                is_coinbase: false,
                created_height: 0,
                created_median_time_past: 0,
            },
        );
        let chainstate = ChainstateSnapshot::new(Vec::new(), utxos, HashMap::new());
        let peer_metadata = MempoolEntryMetadata::new(
            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(40)),
            MempoolOrigin::Peer,
            RelayIntent::NotRequested,
        );
        let reorg_metadata = MempoolEntryMetadata::new(
            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(80)),
            MempoolOrigin::Reorg,
            RelayIntent::NotRequested,
        );
        let mut peer_record = snapshot_record(spend_transaction(peer_previous, 499_000));
        peer_record.metadata = peer_metadata;
        let mut reorg_record = snapshot_record(spend_transaction(reorg_previous, 499_000));
        reorg_record.metadata = reorg_metadata;
        let snapshot = MempoolSnapshot {
            records: vec![peer_record.clone(), reorg_record.clone()],
        };
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

        // Act
        let _ = snapshot.replay_into_mempool(
            &mut mempool,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );

        // Assert
        assert_eq!(
            mempool.entry(&peer_record.txid).expect("peer").metadata,
            peer_metadata
        );
        assert_eq!(
            mempool.entry(&reorg_record.txid).expect("reorg").metadata,
            reorg_metadata
        );
        assert!(!peer_metadata.is_retry_eligible(true));
        assert!(!reorg_metadata.is_retry_eligible(true));
    }

    #[test]
    fn recovery_metadata_legacy_replays_fail_closed_not_restart_time() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([15_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let record = snapshot_record(spend_transaction(previous_output, 499_000));
        assert_eq!(record.metadata, MempoolEntryMetadata::legacy_unknown());
        let snapshot = MempoolSnapshot {
            records: vec![record.clone()],
        };
        let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

        // Act
        let _ = snapshot.replay_into_mempool(
            &mut mempool,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );

        // Assert
        let entry = mempool.entry(&record.txid).expect("legacy recovered");
        assert_eq!(entry.metadata, MempoolEntryMetadata::legacy_unknown());
        assert!(!entry.metadata.is_retry_eligible(true));
    }

    #[test]
    fn recovery_metadata_duplicate_does_not_rewrite_existing_canonical_metadata() {
        // Arrange
        let previous_output = OutPoint {
            txid: Txid::from_byte_array([16_u8; 32]),
            vout: 0,
        };
        let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
        let transaction = spend_transaction(previous_output, 499_000);
        let original = known_local_requested(90);
        let conflicting = MempoolEntryMetadata::new(
            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(999)),
            MempoolOrigin::Peer,
            RelayIntent::NotRequested,
        );
        let mut original_record = snapshot_record(transaction.clone());
        original_record.metadata = original;
        let mut duplicate_record = snapshot_record(transaction);
        duplicate_record.metadata = conflicting;
        let snapshot = MempoolSnapshot {
            records: vec![original_record.clone(), duplicate_record],
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
        assert_eq!(recovery[1].status, MempoolRecoveryStatus::DroppedDuplicate);
        assert_eq!(
            mempool
                .entry(&original_record.txid)
                .expect("original")
                .metadata,
            original
        );
    }
}
