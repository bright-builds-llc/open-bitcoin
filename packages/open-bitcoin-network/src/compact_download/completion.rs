// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{CodecError, block::wire::BlockView};
use open_bitcoin_primitives::InventoryType;

use super::*;

pub(crate) fn try_complete_compact_download_selected(
    block_hash: BlockHash,
    in_flight: &CompactDownloadInFlight,
    check_witness_root: bool,
) -> Result<CompactDownloadCompletionOutcome, CodecError> {
    selected_completion(block_hash, &in_flight.partial, check_witness_root)
}

fn selected_completion(
    block_hash: BlockHash,
    partial: &PartialCompactBlock,
    check_witness_root: bool,
) -> Result<CompactDownloadCompletionOutcome, CodecError> {
    Ok(match fill_block(partial) {
        Ok(block) if selected_block_mutated(&block, check_witness_root)? => {
            CompactDownloadCompletionOutcome::Fallback(FullBlockFetch { block_hash })
        }
        Ok(block) => CompactDownloadCompletionOutcome::Completed(block),
        Err(CompactReconstructionInvalidReason::IncompleteTransactions) => {
            CompactDownloadCompletionOutcome::Suppressed(
                CompactDownloadSuppressionReason::CompactReconstructionFailed,
            )
        }
        Err(_) => CompactDownloadCompletionOutcome::Fallback(FullBlockFetch { block_hash }),
    })
}

fn selected_block_mutated(
    block: &impl BlockView,
    check_witness_root: bool,
) -> Result<bool, CodecError> {
    open_bitcoin_consensus::block::is_block_mutated(block, check_witness_root)
}

pub(super) fn completion_actions_from_outcome(
    block_hash: BlockHash,
    outcome: CompactDownloadCompletionOutcome,
) -> Vec<CompactDownloadAction> {
    match outcome {
        CompactDownloadCompletionOutcome::Completed(block) => {
            vec![CompactDownloadAction::ReceivedBlock(block)]
        }
        CompactDownloadCompletionOutcome::Fallback(fetch) => {
            vec![CompactDownloadAction::RequestFullBlock(fetch)]
        }
        CompactDownloadCompletionOutcome::Suppressed(_) => {
            vec![CompactDownloadAction::RequestFullBlock(FullBlockFetch {
                block_hash,
            })]
        }
    }
}

pub(crate) fn compact_download_actions_to_peer_actions_selected(
    actions: Vec<CompactDownloadAction>,
    inventory_type: InventoryType,
) -> Vec<crate::PeerAction> {
    actions
        .into_iter()
        .map(|action| match action {
            CompactDownloadAction::SendGetBlockTxn(request) => {
                crate::PeerAction::Send(crate::WireNetworkMessage::GetBlockTxn(request))
            }
            CompactDownloadAction::RequestFullBlock(fetch) => {
                crate::PeerAction::Send(crate::WireNetworkMessage::GetData(
                    crate::InventoryList::new(vec![open_bitcoin_primitives::InventoryVector {
                        inventory_type,
                        object_hash: fetch.block_hash.into(),
                    }]),
                ))
            }
            CompactDownloadAction::ReceivedBlock(block) => crate::PeerAction::ReceivedBlock(block),
        })
        .collect()
}

pub(crate) fn init_compact_block_download_selected<'a>(
    (activation, peer_compact_capable): (BlockRelayActivationPolicy, bool),
    peer_state: &mut CompactDownloadPeerState,
    payload: &CompactBlockPayload,
    (eligibility, now_unix_seconds, check_witness_root): (
        CompactBlockDownloadEligibilityInput,
        i64,
        bool,
    ),
    candidates: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
    extra_transactions: impl IntoIterator<Item = (&'a Wtxid, &'a Transaction)>,
) -> Result<CompactBlockInitOutcome, CodecError> {
    if !activation.compact_relay.enabled || !peer_supports_compact_download(peer_compact_capable) {
        return Ok(CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactPeerIneligible,
        ));
    }

    let block_hash = open_bitcoin_consensus::block_hash(&payload.header);
    if evaluate_compact_block_download_eligibility(&payload.header, eligibility)
        != CompactBlockDownloadEligibility::Eligible
    {
        return Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
            block_hash,
        }));
    }

    let mut partial = PartialCompactBlock::new();
    let outcome = init_partial_compact_block(&mut partial, payload, candidates, extra_transactions);
    if peer_state.in_flight.contains_key(&block_hash) {
        return Ok(CompactBlockInitOutcome::Suppressed(
            CompactDownloadSuppressionReason::CompactBlockAlreadyInFlight,
        ));
    }

    Ok(match outcome {
        CompactReconstructionOutcome::Ready { missing_indexes } => {
            if missing_indexes.is_empty() {
                return super::completed_or_fallback_init_selected(
                    block_hash,
                    &partial,
                    check_witness_root,
                );
            }

            peer_state.in_flight.insert(
                block_hash,
                CompactDownloadInFlight {
                    partial,
                    getblocktxn_in_flight: false,
                    requested_indexes: Vec::new(),
                    started_at_unix: now_unix_seconds,
                },
            );
            let schedule = schedule_missing_transaction_request(
                activation,
                peer_compact_capable,
                peer_state,
                block_hash,
            );
            finalize_ready_init(block_hash, peer_state, schedule)
        }
        CompactReconstructionOutcome::Invalid(reason) => {
            CompactBlockInitOutcome::Misbehavior(reason)
        }
        CompactReconstructionOutcome::Failed(_) => {
            CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash })
        }
    })
}

pub(crate) fn handle_block_transactions_selected(
    activation: BlockRelayActivationPolicy,
    peer_compact_capable: bool,
    peer_state: &mut CompactDownloadPeerState,
    response: &BlockTransactions,
    check_witness_root: bool,
) -> Result<CompactBlockTxnHandleOutcome, CodecError> {
    let Some(in_flight) = peer_state.in_flight.get_mut(&response.block_hash) else {
        return Ok(CompactBlockTxnHandleOutcome::NoMatchingInFlight);
    };
    if !in_flight.getblocktxn_in_flight {
        return Ok(CompactBlockTxnHandleOutcome::Misbehavior(
            CompactBlockTxnMisbehavior::DuplicateResponse,
        ));
    }
    let expected_block_hash = in_flight
        .partial
        .block_hash()
        .unwrap_or(response.block_hash);
    let apply_outcome =
        apply_block_transactions(&mut in_flight.partial, response, expected_block_hash);

    Ok(match apply_outcome {
        CompactBlockTxnOutcome::Applied { still_missing } => {
            if still_missing.is_empty() {
                return super::finish_applied_handle_selected(
                    peer_state,
                    response.block_hash,
                    expected_block_hash,
                    check_witness_root,
                );
            }
            in_flight.getblocktxn_in_flight = false;
            in_flight.requested_indexes.clear();
            let schedule = schedule_missing_transaction_request(
                activation,
                peer_compact_capable,
                peer_state,
                expected_block_hash,
            );
            match schedule {
                ScheduleMissingTransactionOutcome::Scheduled { request } => {
                    CompactBlockTxnHandleOutcome::Progress {
                        actions: vec![CompactDownloadAction::SendGetBlockTxn(request)],
                    }
                }
                ScheduleMissingTransactionOutcome::Suppressed(_) => {
                    peer_state.in_flight.remove(&expected_block_hash);
                    CompactBlockTxnHandleOutcome::Progress {
                        actions: vec![CompactDownloadAction::RequestFullBlock(FullBlockFetch {
                            block_hash: expected_block_hash,
                        })],
                    }
                }
            }
        }
        CompactBlockTxnOutcome::Misbehavior(reason) => {
            peer_state.in_flight.remove(&response.block_hash);
            match reason {
                CompactBlockTxnMisbehavior::UnexpectedBlockHash => {
                    CompactBlockTxnHandleOutcome::UnexpectedBlockHash
                }
                _ => CompactBlockTxnHandleOutcome::Misbehavior(reason),
            }
        }
    })
}

pub(crate) fn completed_or_fallback_init_selected(
    block_hash: BlockHash,
    partial: &PartialCompactBlock,
    check_witness_root: bool,
) -> Result<CompactBlockInitOutcome, CodecError> {
    Ok(
        match selected_completion(block_hash, partial, check_witness_root)? {
            CompactDownloadCompletionOutcome::Completed(block) => {
                CompactBlockInitOutcome::Completed { block }
            }
            _ => CompactBlockInitOutcome::Fallback(FullBlockFetch { block_hash }),
        },
    )
}

pub(crate) fn complete_applied_download_selected(
    peer_state: &mut CompactDownloadPeerState,
    response_block_hash: BlockHash,
    expected_block_hash: BlockHash,
    check_witness_root: bool,
) -> Result<Result<Vec<CompactDownloadAction>, CompactBlockTxnHandleOutcome>, CodecError> {
    let Some(in_flight) = peer_state.in_flight.get(&response_block_hash) else {
        return Ok(Err(CompactBlockTxnHandleOutcome::NoMatchingInFlight));
    };
    let outcome = super::try_complete_compact_download_selected(
        expected_block_hash,
        in_flight,
        check_witness_root,
    )?;
    peer_state.in_flight.remove(&response_block_hash);
    Ok(Ok(completion_actions_from_outcome(
        expected_block_hash,
        outcome,
    )))
}

pub(crate) fn finish_applied_handle_selected(
    peer_state: &mut CompactDownloadPeerState,
    response_block_hash: BlockHash,
    expected_block_hash: BlockHash,
    check_witness_root: bool,
) -> Result<CompactBlockTxnHandleOutcome, CodecError> {
    Ok(
        match super::complete_applied_download_selected(
            peer_state,
            response_block_hash,
            expected_block_hash,
            check_witness_root,
        )? {
            Ok(actions) => CompactBlockTxnHandleOutcome::Progress { actions },
            Err(outcome) => outcome,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_bitcoin_codec::PrefilledTransaction;
    use open_bitcoin_primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, TransactionInput,
        TransactionOutput, Txid,
    };

    fn sample_body() -> Block {
        let header = BlockHeader {
            version: 2,
            bits: 0x207f_ffff,
            ..BlockHeader::default()
        };
        let coinbase = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: ScriptBuf::default(),
            }],
            lock_time: 0,
        };
        crate::test_block_commitments::coherent_body(header, vec![coinbase], false)
    }

    fn complete_partial(payload: &CompactBlockPayload) -> PartialCompactBlock {
        let mut partial = PartialCompactBlock::new();
        let outcome = init_partial_compact_block(
            &mut partial,
            payload,
            std::iter::empty::<(&Wtxid, &Transaction)>(),
            std::iter::empty(),
        );
        assert!(matches!(
            outcome,
            CompactReconstructionOutcome::Ready { .. }
        ));
        partial
    }

    fn all_prefilled_payload(body: &Block) -> CompactBlockPayload {
        CompactBlockPayload {
            header: body.header.clone(),
            nonce: 5,
            short_ids: Vec::new(),
            prefilled_transactions: body
                .transactions
                .iter()
                .cloned()
                .map(|transaction| PrefilledTransaction {
                    index_delta: 0,
                    transaction,
                })
                .collect(),
        }
    }

    fn witness_spend() -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([7; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![vec![1, 2]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(5_000).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        }
    }

    #[test]
    fn phase167_compact_completion_merkle_mismatch_requires_full_fetch() {
        // Arrange
        let body = sample_body();
        let mut payload = all_prefilled_payload(&body);
        payload.prefilled_transactions[0].transaction.outputs[0].value =
            open_bitcoin_primitives::Amount::from_sats(1).expect("valid amount");
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);
        let partial = complete_partial(&payload);
        let in_flight = CompactDownloadInFlight {
            partial,
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 4,
        };

        // Act
        let outcome = try_complete_compact_download_selected(block_hash, &in_flight, true);

        // Assert
        assert_eq!(
            outcome,
            Ok(CompactDownloadCompletionOutcome::Fallback(FullBlockFetch {
                block_hash
            }))
        );
        assert_eq!(in_flight.partial.block_hash(), Some(block_hash));
    }

    #[test]
    fn phase167_compact_completion_matching_control_completes() {
        // Arrange
        let body = sample_body();
        let payload = all_prefilled_payload(&body);
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);
        let in_flight = CompactDownloadInFlight {
            partial: complete_partial(&payload),
            getblocktxn_in_flight: false,
            requested_indexes: Vec::new(),
            started_at_unix: 4,
        };

        // Act
        let outcome = try_complete_compact_download_selected(block_hash, &in_flight, true);

        // Assert
        assert_eq!(
            outcome,
            Ok(CompactDownloadCompletionOutcome::Completed(body))
        );
    }

    #[test]
    fn phase167_compact_completion_selected_witness_fact_changes_verdict() {
        // Arrange
        let base = sample_body();
        let body = crate::test_block_commitments::coherent_body(
            base.header,
            vec![base.transactions[0].clone(), witness_spend()],
            false,
        );
        let payload = all_prefilled_payload(&body);
        let partial = complete_partial(&payload);
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);

        // Act
        let selected = completed_or_fallback_init_selected(block_hash, &partial, true);
        let disabled = completed_or_fallback_init_selected(block_hash, &partial, false);

        // Assert
        assert_eq!(
            selected,
            Ok(CompactBlockInitOutcome::Completed { block: body })
        );
        assert_eq!(
            disabled,
            Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
                block_hash
            }))
        );
    }

    #[test]
    fn phase167_compact_completion_duplicate_leaf_falls_back() {
        // Arrange
        let base = sample_body();
        let spend = witness_spend();
        let mut body = Block {
            header: base.header,
            transactions: vec![base.transactions[0].clone(), spend.clone(), spend],
        };
        body.header.merkle_root = open_bitcoin_consensus::block_merkle_root(&body.transactions)
            .expect("txid root")
            .0;
        let payload = all_prefilled_payload(&body);
        let partial = complete_partial(&payload);
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);

        // Act
        let outcome = completed_or_fallback_init_selected(block_hash, &partial, false);

        // Assert
        assert_eq!(
            outcome,
            Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
                block_hash
            }))
        );
    }

    #[test]
    fn phase167_compact_completion_all_present_init_classifies_before_success() {
        // Arrange
        let body = sample_body();
        let mut payload = all_prefilled_payload(&body);
        payload.prefilled_transactions[0].transaction.lock_time += 1;
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);
        let eligibility = CompactBlockDownloadEligibilityInput {
            local_best_height: 0,
            best_tip_hash: payload.header.previous_block_hash,
            maybe_known_block_height: Some(1),
        };
        let mut peer_state = CompactDownloadPeerState::new();

        // Act
        let outcome = init_compact_block_download_selected(
            (activated_policy(), true),
            &mut peer_state,
            &payload,
            (eligibility, 9, true),
            std::iter::empty::<(&Wtxid, &Transaction)>(),
            std::iter::empty(),
        );

        // Assert
        assert_eq!(
            outcome,
            Ok(CompactBlockInitOutcome::Fallback(FullBlockFetch {
                block_hash
            }))
        );
        assert!(peer_state.in_flight.is_empty());
    }

    fn pending_blocktxn(transactions: &[Transaction]) -> (CompactDownloadPeerState, BlockHash) {
        let base = sample_body();
        let mut body_transactions = vec![base.transactions[0].clone()];
        body_transactions.extend_from_slice(transactions);
        let body =
            crate::test_block_commitments::coherent_body(base.header, body_transactions, false);
        let nonce = 42;
        let selector =
            open_bitcoin_codec::short_id_selector_from_header_and_nonce(&body.header, nonce);
        let payload = CompactBlockPayload {
            header: body.header,
            nonce,
            short_ids: transactions
                .iter()
                .map(|tx| {
                    let wtxid = open_bitcoin_consensus::transaction_wtxid(tx).expect("wtxid");
                    open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &wtxid)
                })
                .collect(),
            prefilled_transactions: vec![PrefilledTransaction {
                index_delta: 0,
                transaction: body.transactions[0].clone(),
            }],
        };
        let block_hash = open_bitcoin_consensus::block_hash(&payload.header);
        let partial = complete_partial(&payload);
        let requested_indexes = partial.missing_transaction_indexes();
        let mut peer_state = CompactDownloadPeerState::new();
        peer_state.in_flight.insert(
            block_hash,
            CompactDownloadInFlight {
                partial,
                getblocktxn_in_flight: true,
                requested_indexes,
                started_at_unix: 9,
            },
        );
        (peer_state, block_hash)
    }

    #[test]
    fn phase167_compact_completion_blocktxn_classifies_before_received_block() {
        // Arrange
        let missing = witness_spend();
        let (mut peer_state, block_hash) = pending_blocktxn(std::slice::from_ref(&missing));
        let mut altered = missing;
        altered.lock_time += 1;

        // Act
        let outcome = handle_block_transactions_selected(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash,
                transactions: vec![altered],
            },
            true,
        );

        // Assert
        assert!(
            matches!(outcome, Ok(CompactBlockTxnHandleOutcome::Progress { actions })
            if actions == vec![CompactDownloadAction::RequestFullBlock(FullBlockFetch { block_hash })])
        );
        assert!(peer_state.in_flight.is_empty());
    }

    #[test]
    fn phase167_compact_completion_blocktxn_missing_progress_reschedules_remaining() {
        // Arrange
        let first = witness_spend();
        let mut second = witness_spend();
        second.inputs[0].previous_output.txid = Txid::from_byte_array([8; 32]);
        let (mut peer_state, block_hash) = pending_blocktxn(&[first.clone(), second]);

        // Act
        let outcome = handle_block_transactions_selected(
            activated_policy(),
            true,
            &mut peer_state,
            &BlockTransactions {
                block_hash,
                transactions: vec![first],
            },
            true,
        );

        // Assert
        assert!(
            matches!(outcome, Ok(CompactBlockTxnHandleOutcome::Progress { actions })
            if matches!(actions.as_slice(), [CompactDownloadAction::SendGetBlockTxn(_)]))
        );
        let pending = peer_state
            .in_flight
            .get(&block_hash)
            .expect("rescheduled in-flight block");
        assert!(pending.getblocktxn_in_flight);
        assert_eq!(pending.requested_indexes, vec![2]);
        assert_eq!(pending.partial.missing_transaction_indexes(), vec![2]);
    }
}
